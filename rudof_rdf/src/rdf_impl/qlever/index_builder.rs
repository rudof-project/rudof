//! One-shot Docker invocations that build a QLever index from one or more input files.
//!
//! The high-level lifecycle is:
//!
//! 1. Resolve the index directory on the host.
//! 2. If the index files already exist, return early (idempotent).
//! 3. Probe the image to find out which CLI it exposes.
//! 4. Assemble the `IndexBuilderMain` argv from [`QleverConfig`] + the inputs.
//! 5. Bind-mount the index directory as `/data` and each input directory as `/inputs/<n>` read-only.
//! 6. Run the container as a one-shot, propagating logs into `tracing`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use bollard::Docker;
use bollard::models::{ContainerCreateBody, HostConfig};
use bollard::query_parameters::{
    AttachContainerOptionsBuilder, CreateContainerOptions, LogsOptionsBuilder, RemoveContainerOptionsBuilder,
    StartContainerOptions, WaitContainerOptionsBuilder,
};
use futures::{StreamExt, TryStreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, info, warn};

use super::cli_probe;
use super::config::CONTAINER_WORKING_DIR;
use super::decompressor::{ResolvedDecompressor, decompressor_probe};
use super::{InputFile, NativeFormat, QleverConfig, QleverError};

/// Handle to a (possibly pre-existing) QLever on-disk index.
#[derive(Debug, Clone)]
pub struct IndexHandle {
    dir: PathBuf,
    name: String,
}

impl IndexHandle {
    pub(crate) fn new(dir: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self {
            dir: dir.into(),
            name: name.into(),
        }
    }

    /// Host path of the directory the index files live in.
    pub fn path(&self) -> &Path {
        &self.dir
    }

    /// The `-i` argument the QLever server expects.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// `true` if the index files already exist on disk (used to skip the indexing step on repeated runs).
    pub fn is_built(&self) -> bool {
        // QLever writes `<name>.meta-data.json` last, after every permutation and the vocabulary are flushed,
        // so its presence is a reliable "indexing finished" marker.
        self.dir.join(format!("{}.meta-data.json", self.name)).exists()
    }
}

/// Build the index for the given inputs.
///
/// Idempotent: if the on-disk index files are already present, this returns without doing any Docker work.
pub async fn build_index(inputs: &[InputFile], config: &QleverConfig) -> Result<IndexHandle, QleverError> {
    if inputs.is_empty() {
        return Err(QleverError::PreFlight("build_index called with no inputs".to_string()));
    }

    let fingerprint = fingerprint_inputs(inputs);
    let index_dir = config.resolve_index_dir(&fingerprint);
    std::fs::create_dir_all(&index_dir).map_err(|error| QleverError::IndexDirIo {
        path: index_dir.clone(),
        error,
    })?;

    let handle = IndexHandle::new(&index_dir, &config.index_name);
    if handle.is_built() {
        debug!("QLever index already built at {}", index_dir.display());
        return Ok(handle);
    }

    let docker = cli_probe::connect()?;
    cli_probe::ping(&docker).await?;
    cli_probe::ensure_image(&docker, &config.image()).await?;

    let cli_kind = cli_probe::probe(&config.image()).await?;

    let (argv, binds) = build_argv_and_binds(cli_kind, inputs, config, &index_dir)?;
    let cmd_string = shell_join(&argv);
    info!("running QLever index builder: {}", cmd_string);

    let streamable: Vec<&InputFile> = inputs.iter().filter(|i| i.compression.is_some()).collect();
    match streamable.len() {
        0 => {
            run_one_shot(&docker, config, binds, &cmd_string, "index-builder").await?;
        },
        1 => {
            let compression = streamable[0].compression.expect("filtered by is_some");
            let probe = decompressor_probe();
            let decomp = probe.for_compression(compression).ok_or_else(|| {
                let tried: Vec<&str> = compression.strategy().candidates().iter().map(|c| c.program).collect();
                QleverError::DecompressorMissing {
                    family: compression.strategy().family_name(),
                    tried: tried.join(", "),
                }
            })?;
            info!(
                "streaming compressed input ({}) via {}{}",
                decomp.family,
                decomp.program.display(),
                if decomp.parallel {
                    " (parallel)"
                } else {
                    " (single-threaded)"
                }
            );
            run_one_shot_with_stdin(
                &docker,
                config,
                binds,
                &cmd_string,
                "index-builder",
                &streamable[0].host_path,
                decomp,
            )
            .await?;
        },
        _ => unreachable!("rejected earlier in build_argv_and_binds"),
    }

    Ok(handle)
}

/// Build the `IndexBuilderMain` argv plus the list of `host:container[:opt]` bind strings.
fn build_argv_and_binds(
    cli: super::CliKind,
    inputs: &[InputFile],
    config: &QleverConfig,
    index_dir: &Path,
) -> Result<(Vec<String>, Vec<String>), QleverError> {
    // Only one compressed input can be streamed per build, `IndexBuilderMain`
    // accepts at most one `-f -` (stdin) argument per invocation, and the
    // index is built once-and-for-all from all `-f` inputs in a single pass.
    if inputs.iter().filter(|i| i.compression.is_some()).count() > 1 {
        return Err(QleverError::PreFlight(
            "only one compressed input per build is supported".into(),
        ));
    }

    let mut argv: Vec<String> = vec![cli.index_builder_cmd().to_string()];
    argv.push("-i".into());
    argv.push(config.index_name.clone());

    if let Some(m) = &config.stxxl_memory {
        argv.push("-m".into());
        argv.push(m.clone());
    }
    if let Some(b) = &config.parser_buffer_size {
        argv.push("--parser-buffer-size".into());
        argv.push(b.clone());
    }
    if let Some(parallel) = config.parser_parallel {
        argv.push("--parse-parallel".into());
        argv.push(if parallel { "true".into() } else { "false".into() });
    }

    // index_dir → /data (read-write)
    let mut binds = vec![format!("{}:{}:rw", index_dir.display(), CONTAINER_WORKING_DIR)];

    // Bind each unique input host directory at a separate mount point so we
    // never have to copy possibly-huge files into the index dir.
    let mut dir_mounts: HashMap<PathBuf, String> = HashMap::new();
    for input in inputs {
        if input.compression.is_some() {
            // Streamed via stdin, no bind mount, container reads from `-`.
            argv.push("-f".into());
            argv.push("-".into());
            argv.push("-F".into());
            argv.push(input.format_ext.cli_arg().to_string());
            argv.push("-g".into());
            argv.push(input.graph_arg().to_string());
            continue;
        }

        let parent = input
            .host_path
            .parent()
            .ok_or_else(|| QleverError::PreFlight(format!("input has no parent dir: {}", input.host_path.display())))?
            .to_path_buf();

        let container_dir = match dir_mounts.get(&parent) {
            Some(d) => d.clone(),
            None => {
                let mount_name = if parent == index_dir {
                    CONTAINER_WORKING_DIR.to_string()
                } else {
                    let n = dir_mounts.len();
                    let d = format!("/inputs/{n}");
                    binds.push(format!("{}:{}:ro", parent.display(), d));
                    d
                };
                dir_mounts.insert(parent.clone(), mount_name.clone());
                mount_name
            },
        };

        let file_name = input
            .host_path
            .file_name()
            .ok_or_else(|| QleverError::PreFlight(format!("input has no file name: {}", input.host_path.display())))?
            .to_string_lossy()
            .to_string();
        let container_path = format!("{container_dir}/{file_name}");

        argv.push("-f".into());
        argv.push(container_path);
        argv.push("-F".into());
        argv.push(input.format_ext.cli_arg().to_string());
        argv.push("-g".into());
        argv.push(input.graph_arg().to_string());
    }

    Ok((argv, binds))
}

/// Run `cmd` inside `config.image()` once and remove the container after.
pub(crate) async fn run_one_shot(
    docker: &Docker,
    config: &QleverConfig,
    binds: Vec<String>,
    cmd: &str,
    what: &'static str,
) -> Result<(), QleverError> {
    let host_config = host_config_with_limits(config, binds)?;

    // The upstream `adfreiburg/qlever` image's entrypoint forwards `-c <cmd>`
    // to a login shell that has `/qlever` on PATH (so `IndexBuilderMain`
    // resolves). It also REQUIRES `working_dir=/data` and a bind there or it
    // prints a welcome message and exits 1.
    let body = ContainerCreateBody {
        image: Some(config.image()),
        cmd: Some(vec!["-c".into(), cmd.to_string()]),
        working_dir: Some(CONTAINER_WORKING_DIR.to_string()),
        user: user_string(config),
        host_config: Some(host_config),
        ..Default::default()
    };

    let create_options: Option<CreateContainerOptions> = None;
    let container = docker.create_container(create_options, body).await?;
    let id = container.id;

    docker.start_container(&id, None::<StartContainerOptions>).await?;

    let logs_options = LogsOptionsBuilder::new()
        .stdout(true)
        .stderr(true)
        .follow(true)
        .tail("all")
        .build();
    let mut log_buf = String::new();
    let mut log_stream = docker.logs(&id, Some(logs_options));
    while let Some(item) = log_stream.try_next().await.unwrap_or(None) {
        let chunk = item.into_bytes();
        let s = String::from_utf8_lossy(&chunk);
        debug!(target: "rudof_rdf::qlever", "{}", s.trim_end());
        log_buf.push_str(&s);
    }

    let wait_opts = WaitContainerOptionsBuilder::new().condition("not-running").build();
    let wait_result: Vec<_> = docker
        .wait_container(&id, Some(wait_opts))
        .try_collect::<Vec<_>>()
        .await
        .unwrap_or_default();
    let wait_code = wait_result.into_iter().next().map(|r| r.status_code);
    let (oom_killed, inspect_code) = inspect_post_exit(docker, &id).await;
    let exit_code = wait_code.or(inspect_code).unwrap_or(-1);

    let remove_opts = RemoveContainerOptionsBuilder::new().force(true).build();
    let _ = docker.remove_container(&id, Some(remove_opts)).await;

    if exit_code != 0 {
        return Err(container_nonzero_exit(what, exit_code, oom_killed, log_buf));
    }
    Ok(())
}

/// Cap for the bounded decompressor-stderr ring buffer (4 KiB tail).
const STDERR_TAIL_LIMIT: usize = 4 * 1024;

/// Run `cmd` inside `config.image()` once, streaming the decompressed bytes
/// of `compressed_host_path` into the container's stdin via a spawned host
/// decompressor (`decomp.program`).
pub(crate) async fn run_one_shot_with_stdin(
    docker: &Docker,
    config: &QleverConfig,
    binds: Vec<String>,
    cmd: &str,
    what: &'static str,
    compressed_host_path: &Path,
    decomp: &ResolvedDecompressor,
) -> Result<(), QleverError> {
    let host_config = host_config_with_limits(config, binds)?;

    let body = ContainerCreateBody {
        image: Some(config.image()),
        cmd: Some(vec!["-c".into(), cmd.to_string()]),
        working_dir: Some(CONTAINER_WORKING_DIR.to_string()),
        user: user_string(config),
        host_config: Some(host_config),
        open_stdin: Some(true),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        stdin_once: Some(true),
        tty: Some(false),
        ..Default::default()
    };

    let create_options: Option<CreateContainerOptions> = None;
    let container = docker.create_container(create_options, body).await?;
    let id = container.id;

    // Attach BEFORE start: otherwise the first bytes the container reads
    // from stdin race with our attach pipe being wired up.
    let attach_opts = AttachContainerOptionsBuilder::new()
        .stdin(true)
        .stdout(true)
        .stderr(true)
        .stream(true)
        .build();
    let attach = docker.attach_container(&id, Some(attach_opts)).await?;
    let mut input = attach.input;
    let mut output = attach.output;

    docker.start_container(&id, None::<StartContainerOptions>).await?;

    // Spawn the host decompressor.
    let mut child = tokio::process::Command::new(&decomp.program)
        .args(decomp.args)
        .arg(compressed_host_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let mut child_stdout = child.stdout.take().expect("requested stdout pipe");
    let mut child_stderr = child.stderr.take().expect("requested stderr pipe");

    let stdin_cancel = tokio_util::sync::CancellationToken::new();
    let stdin_cancel_watcher = stdin_cancel.clone();

    let copy_task = async move {
        let pump = async {
            tokio::io::copy(&mut child_stdout, &mut input).await?;
            input.shutdown().await?;
            Ok::<(), std::io::Error>(())
        };
        tokio::select! {
            biased;
            _ = stdin_cancel.cancelled() => Ok(()),
            res = pump => match res {
                Ok(()) => Ok(()),
                Err(e) if matches!(e.kind(), std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::WriteZero) => {
                    debug!(target: "rudof_rdf::qlever", "stdin pump: container closed pipe ({})", e.kind());
                    Ok(())
                },
                Err(e) => Err(e),
            },
        }
    };

    // 2. Drain container output into the log buffer.
    let output_task = async move {
        let mut log_buf = String::new();
        while let Some(item) = output.next().await {
            match item {
                Ok(chunk) => {
                    let s = String::from_utf8_lossy(chunk.as_ref());
                    debug!(target: "rudof_rdf::qlever", "{}", s.trim_end());
                    log_buf.push_str(&s);
                },
                Err(_) => break,
            }
        }
        Ok::<String, std::io::Error>(log_buf)
    };

    // 3. Drain decompressor stderr into a bounded ring buffer.
    let stderr_task = async move {
        let mut tail: Vec<u8> = Vec::with_capacity(STDERR_TAIL_LIMIT);
        let mut buf = [0u8; 4096];
        loop {
            let n = child_stderr.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            tail.extend_from_slice(&buf[..n]);
            if tail.len() > STDERR_TAIL_LIMIT {
                let drop = tail.len() - STDERR_TAIL_LIMIT;
                tail.drain(..drop);
            }
        }
        Ok::<String, std::io::Error>(String::from_utf8_lossy(&tail).into_owned())
    };

    // 4. Wait for the container to exit. As soon as it does, fire the cancel
    //    token so the stdin pump unblocks even if the daemon socket is
    //    backlogged. Run in parallel with the I/O tasks so a dead container
    //    never wedges us.
    let docker_for_wait = docker.clone();
    let id_for_wait = id.clone();
    let wait_task = async move {
        let wait_opts = WaitContainerOptionsBuilder::new().condition("not-running").build();
        let wait_result: Vec<_> = docker_for_wait
            .wait_container(&id_for_wait, Some(wait_opts))
            .try_collect::<Vec<_>>()
            .await
            .unwrap_or_default();
        let code = wait_result.into_iter().next().map(|r| r.status_code);
        // Unblock the stdin pump regardless of how the container exited.
        stdin_cancel_watcher.cancel();
        code
    };

    let (copy_res, log_buf_res, stderr_res, wait_code) = tokio::join!(copy_task, output_task, stderr_task, wait_task);
    let (oom_killed, inspect_code) = inspect_post_exit(docker, &id).await;
    let exit_code = wait_code.or(inspect_code).unwrap_or(-1);

    if let Err(e) = copy_res
        && exit_code == 0
    {
        return Err(QleverError::Io(e));
    }
    let log_buf = log_buf_res.unwrap_or_default();
    let stderr_tail = stderr_res.unwrap_or_default();

    let child_status = match tokio::time::timeout(std::time::Duration::from_secs(5), child.wait()).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => {
            warn!(target: "rudof_rdf::qlever", "decompressor wait failed: {e}");
            let _ = child.kill().await;
            std::process::ExitStatus::default()
        },
        Err(_) => {
            warn!(target: "rudof_rdf::qlever", "decompressor did not exit within 5s after container exit; killing");
            let _ = child.kill().await;
            let _ = child.wait().await;
            std::process::ExitStatus::default()
        },
    };

    let remove_opts = RemoveContainerOptionsBuilder::new().force(true).build();
    let _ = docker.remove_container(&id, Some(remove_opts)).await;

    // If the container exited non-zero, that's the headline error: the
    // decompressor's BrokenPipe is just downstream noise.
    if exit_code != 0 {
        return Err(container_nonzero_exit(what, exit_code, oom_killed, log_buf));
    }
    if !child_status.success() {
        return Err(QleverError::DecompressorExit {
            program: decomp.program.display().to_string(),
            status: child_status.code().unwrap_or(-1),
            stderr_tail,
        });
    }
    Ok(())
}

/// Build a `HostConfig` with the user-supplied container memory limits applied.
///
/// Returns a `PreFlight` error if the strings can't be parsed.
fn host_config_with_limits(config: &QleverConfig, binds: Vec<String>) -> Result<HostConfig, QleverError> {
    let memory = match &config.container_memory {
        Some(s) => Some(
            parse_size_to_bytes(s)
                .map_err(|e| QleverError::PreFlight(format!("invalid container_memory={s:?}: {e}")))?,
        ),
        None => None,
    };
    let memory_swap = match &config.container_memory_swap {
        Some(s) => Some(
            parse_size_to_bytes(s)
                .map_err(|e| QleverError::PreFlight(format!("invalid container_memory_swap={s:?}: {e}")))?,
        ),
        None => None,
    };
    Ok(HostConfig {
        binds: Some(binds),
        auto_remove: Some(false),
        memory,
        memory_swap,
        ..Default::default()
    })
}

/// Best-effort lookup of `State.OOMKilled` and `State.ExitCode` after the
/// container has exited.
async fn inspect_post_exit(docker: &Docker, id: &str) -> (bool, Option<i64>) {
    match docker.inspect_container(id, None).await {
        Ok(resp) => {
            let state = resp.state;
            let oom = state.as_ref().and_then(|s| s.oom_killed).unwrap_or(false);
            let code = state.and_then(|s| s.exit_code);
            (oom, code)
        },
        Err(_) => (false, None),
    }
}

/// Build a `ContainerNonZeroExit`, prefixing logs with a one-line OOM hint
/// when the kernel killed the container so the error surfaces the cause.
fn container_nonzero_exit(what: &str, status: i64, oom_killed: bool, mut logs: String) -> QleverError {
    if oom_killed {
        let hint = "OOM-killed: the QLever container was terminated by the OOM killer. \
            Lower memory usage with QleverConfig::parser_parallel=Some(false), \
            QleverConfig::stxxl_memory, and/or cap with QleverConfig::container_memory.\n\
            ---\n";
        logs.insert_str(0, hint);
    }
    QleverError::ContainerNonZeroExit {
        what: what.to_string(),
        status,
        logs,
    }
}

/// Parse a human-readable byte count (`"2G"`, `"512M"`, `"1.5GiB"`, raw `"1073741824"`).
fn parse_size_to_bytes(s: &str) -> Result<i64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty".into());
    }
    let (num_str, mult): (&str, u64) = if let Some(rest) = s.strip_suffix("GiB").or_else(|| s.strip_suffix("Gi")) {
        (rest, 1024u64.pow(3))
    } else if let Some(rest) = s.strip_suffix("MiB").or_else(|| s.strip_suffix("Mi")) {
        (rest, 1024u64.pow(2))
    } else if let Some(rest) = s.strip_suffix("KiB").or_else(|| s.strip_suffix("Ki")) {
        (rest, 1024)
    } else if let Some(rest) = s.strip_suffix("TiB").or_else(|| s.strip_suffix("Ti")) {
        (rest, 1024u64.pow(4))
    } else if let Some(rest) = s.strip_suffix("GB") {
        (rest, 1_000_000_000)
    } else if let Some(rest) = s.strip_suffix("MB") {
        (rest, 1_000_000)
    } else if let Some(rest) = s.strip_suffix("KB") {
        (rest, 1_000)
    } else if let Some(rest) = s.strip_suffix("TB") {
        (rest, 1_000_000_000_000)
    } else if let Some(rest) = s.strip_suffix('G').or_else(|| s.strip_suffix('g')) {
        (rest, 1024u64.pow(3))
    } else if let Some(rest) = s.strip_suffix('M').or_else(|| s.strip_suffix('m')) {
        (rest, 1024u64.pow(2))
    } else if let Some(rest) = s.strip_suffix('K').or_else(|| s.strip_suffix('k')) {
        (rest, 1024)
    } else if let Some(rest) = s.strip_suffix('T').or_else(|| s.strip_suffix('t')) {
        (rest, 1024u64.pow(4))
    } else {
        (s, 1)
    };
    let num_str = num_str.trim();
    let value: f64 = num_str.parse().map_err(|e| format!("{e}"))?;
    if value < 0.0 || !value.is_finite() {
        return Err(format!("must be a finite, non-negative number, got {value}"));
    }
    let bytes = (value * mult as f64) as u64;
    i64::try_from(bytes).map_err(|_| format!("overflows i64: {bytes}"))
}

/// On Linux, format `<uid>:<gid>` so the QLever container writes index files owned by the host user. No-op on non-Unix.
fn user_string(config: &QleverConfig) -> Option<String> {
    if !config.run_as_host_user {
        return None;
    }
    host_uid_gid()
}

#[cfg(unix)]
fn host_uid_gid() -> Option<String> {
    let uid = nix::unistd::Uid::current().as_raw();
    let gid = nix::unistd::Gid::current().as_raw();
    Some(format!("{uid}:{gid}"))
}

#[cfg(not(unix))]
fn host_uid_gid() -> Option<String> {
    None
}

/// Convert a list of input host paths into a hex fingerprint that can be used
/// as the per-run subdirectory name under `~/.cache/rudof/qlever/`.
pub(crate) fn fingerprint_inputs(inputs: &[InputFile]) -> String {
    let mut h = Fnv1a::new();
    for input in inputs {
        h.write_path(&input.host_path);
        h.write_bytes(input.format_ext.cli_arg().as_bytes());
        if let Some(iri) = &input.graph_iri {
            h.write_bytes(iri.as_bytes());
        }
        if let Some(c) = input.compression {
            h.write_bytes(c.strategy().family_name().as_bytes());
        }
    }
    format!("{:016x}", h.finish())
}

/// Stable FNV-1a 64-bit hasher.
pub(crate) struct Fnv1a(u64);

impl Fnv1a {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    pub(crate) fn new() -> Self {
        Self(Self::OFFSET)
    }

    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 ^= u64::from(*b);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    pub(crate) fn write_path(&mut self, path: &Path) {
        // Hash the OS-level bytes when available so non-UTF-8 paths still produce a stable fingerprint.
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            self.write_bytes(path.as_os_str().as_bytes());
        }
        #[cfg(not(unix))]
        {
            self.write_bytes(path.to_string_lossy().as_bytes());
        }
        self.write_bytes(&[0]);
    }

    pub(crate) fn finish(self) -> u64 {
        self.0
    }
}

/// Best-effort shell quote (used to assemble a single command string).
fn shell_join(argv: &[String]) -> String {
    argv.iter().map(|a| shell_quote(a)).collect::<Vec<_>>().join(" ")
}

fn shell_quote(s: &str) -> String {
    if s.bytes().all(safe_char) {
        s.to_string()
    } else {
        // wrap in single quotes and escape any internal single quotes
        let escaped = s.replace('\'', "'\\''");
        format!("'{escaped}'")
    }
}

const fn safe_char(b: u8) -> bool {
    matches!(b,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' | b':' | b'='
    )
}

/// Convert an input file in a non-native format into QLever's nearest native format.
///
/// QLever's `-F` flag only accepts `ttl`/`nt`/`nq`, so anything else needs
/// rewriting before indexing. The conversion target is chosen to preserve as
/// much of the source as QLever can index:
///
/// - **Quad-bearing sources** (`NQuads`, `TriG`, `JsonLd`) become **N-Quads**.
/// - **Triple-only sources** (`Turtle`, `NTriples`, `Rdfxml`, `N3`) become **N-Triples**, since there are no quads to preserve.
///
/// Returns the path of the converted file plus the [`NativeFormat`] that should be passed to `-F` for it.
pub async fn convert_to_native(
    source: &Path,
    source_format: &crate::rdf_core::RDFFormat,
    dest_dir: &Path,
) -> Result<(PathBuf, NativeFormat), QleverError> {
    use crate::rdf_core::RDFFormat;
    use oxrdfio::{RdfFormat as OxRdfFormat, RdfParser, RdfSerializer};
    use tokio::io::AsyncReadExt;

    std::fs::create_dir_all(dest_dir).map_err(|error| QleverError::IndexDirIo {
        path: dest_dir.to_path_buf(),
        error,
    })?;

    let (target_native, target_ox, target_ext) = match source_format {
        RDFFormat::NQuads | RDFFormat::TriG | RDFFormat::JsonLd => (NativeFormat::NQuads, OxRdfFormat::NQuads, "nq"),
        _ => (NativeFormat::NTriples, OxRdfFormat::NTriples, "nt"),
    };

    let target_name = source
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "input".to_string());
    let target = dest_dir.join(format!("{target_name}.{target_ext}"));

    let source_format_ox = rdf_format_to_oxrdfio(source_format);
    let mut bytes = Vec::new();
    let mut f = tokio::fs::File::open(source).await?;
    f.read_to_end(&mut bytes).await?;

    let parser = RdfParser::from_format(source_format_ox);
    let serializer = RdfSerializer::from_format(target_ox);

    let mut out_bytes: Vec<u8> = Vec::new();
    let mut writer = serializer.for_writer(&mut out_bytes);
    for quad in parser.for_reader(bytes.as_slice()) {
        let quad = quad.map_err(|e| QleverError::FormatConversion {
            source_name: source.display().to_string(),
            error: format!("{e}"),
        })?;
        writer
            .serialize_quad(&quad)
            .map_err(|e| QleverError::FormatConversion {
                source_name: source.display().to_string(),
                error: format!("{e}"),
            })?;
    }
    writer.finish().map_err(|e| QleverError::FormatConversion {
        source_name: source.display().to_string(),
        error: format!("{e}"),
    })?;

    let mut out = tokio::fs::File::create(&target).await?;
    out.write_all(&out_bytes).await?;
    out.flush().await?;

    Ok((target, target_native))
}

pub(super) fn rdf_format_to_oxrdfio(f: &crate::rdf_core::RDFFormat) -> oxrdfio::RdfFormat {
    use crate::rdf_core::RDFFormat;
    use oxrdfio::RdfFormat as Ox;
    match f {
        RDFFormat::Turtle => Ox::Turtle,
        RDFFormat::NTriples => Ox::NTriples,
        RDFFormat::Rdfxml => Ox::RdfXml,
        RDFFormat::NQuads => Ox::NQuads,
        RDFFormat::TriG => Ox::TriG,
        RDFFormat::JsonLd => Ox::JsonLd {
            profile: Default::default(),
        },
        // N3 is not in oxrdfio (fall back to Turtle).
        RDFFormat::N3 => Ox::Turtle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn input(host: &str, fmt: NativeFormat) -> InputFile {
        InputFile {
            host_path: PathBuf::from(host),
            in_container_name: host.rsplit('/').next().unwrap().to_string(),
            format_ext: fmt,
            graph_iri: None,
            compression: None,
        }
    }

    fn input_compressed(host: &str, fmt: NativeFormat, c: super::super::decompressor::Compression) -> InputFile {
        InputFile {
            host_path: PathBuf::from(host),
            in_container_name: host.rsplit('/').next().unwrap().to_string(),
            format_ext: fmt,
            graph_iri: None,
            compression: Some(c),
        }
    }

    #[test]
    fn argv_includes_input_flags_for_each_file() {
        let config = QleverConfig::default();
        let inputs = vec![
            input("/tmp/work/data.ttl", NativeFormat::Turtle),
            input("/tmp/work/extra.nq", NativeFormat::NQuads),
        ];
        let (argv, binds) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();

        assert_eq!(argv[0], "IndexBuilderMain");
        assert_eq!(argv[1], "-i");
        assert_eq!(argv[2], config.index_name);

        // Each input gets -f, -F, -g
        assert_eq!(argv.iter().filter(|a| *a == "-f").count(), 2);
        assert_eq!(argv.iter().filter(|a| *a == "-F").count(), 2);
        assert_eq!(argv.iter().filter(|a| *a == "-g").count(), 2);

        // Inputs from the same parent dir share one bind mount.
        assert_eq!(binds.iter().filter(|b| b.starts_with("/tmp/work")).count(), 1);
        assert!(binds[0].starts_with("/tmp/idx:/data"));
    }

    #[test]
    fn argv_passes_dash_for_default_graph() {
        let config = QleverConfig::default();
        let inputs = vec![input("/tmp/data.ttl", NativeFormat::Turtle)];
        let (argv, _) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        // Find the -g and confirm next arg is "-"
        let g_pos = argv.iter().position(|a| a == "-g").unwrap();
        assert_eq!(argv[g_pos + 1], "-");
    }

    #[test]
    fn cli_kind_v2_uses_qlever_index_subcommand() {
        let config = QleverConfig::default();
        let inputs = vec![input("/tmp/data.ttl", NativeFormat::Turtle)];
        let (argv, _) =
            build_argv_and_binds(super::super::CliKind::V2, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        assert_eq!(argv[0], "qlever-index");
    }

    #[test]
    fn is_built_detects_existing_meta_file() {
        let tmp = tempfile::tempdir().unwrap();
        let handle = IndexHandle::new(tmp.path(), "default");
        assert!(!handle.is_built());
        std::fs::write(tmp.path().join("default.meta-data.json"), b"{}").unwrap();
        assert!(handle.is_built());
    }

    #[test]
    fn argv_uses_stdin_dash_for_compressed_bz2() {
        use super::super::decompressor::Compression;
        let config = QleverConfig::default();
        let inputs = vec![input_compressed(
            "/tmp/work/dump.nt.bz2",
            NativeFormat::NTriples,
            Compression::Bzip2,
        )];
        let (argv, binds) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();

        // Exactly one `-f -` pair, no `-f /inputs/...` for this input.
        let f_positions: Vec<usize> = argv
            .iter()
            .enumerate()
            .filter(|(_, a)| *a == "-f")
            .map(|(i, _)| i)
            .collect();
        assert_eq!(f_positions.len(), 1);
        assert_eq!(argv[f_positions[0] + 1], "-");

        // The compressed file's parent dir must NOT be bind-mounted —
        // bytes come over stdin, the file is read directly on the host.
        assert!(
            !binds.iter().any(|b| b.starts_with("/tmp/work")),
            "compressed input parent should not be bind-mounted, got: {binds:?}"
        );
        // The index_dir bind is still present.
        assert!(binds[0].starts_with("/tmp/idx:/data"));
    }

    #[test]
    fn argv_uses_stdin_dash_for_compressed_xz() {
        use super::super::decompressor::Compression;
        let config = QleverConfig::default();
        let inputs = vec![input_compressed(
            "/tmp/work/dump.nt.xz",
            NativeFormat::NTriples,
            Compression::Xz,
        )];
        let (argv, _) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        let f_pos = argv.iter().position(|a| a == "-f").unwrap();
        assert_eq!(argv[f_pos + 1], "-");
    }

    #[test]
    fn argv_mixes_stdin_dash_with_mounted_inputs() {
        use super::super::decompressor::Compression;
        let config = QleverConfig::default();
        let inputs = vec![
            input("/tmp/work/extra.ttl", NativeFormat::Turtle),
            input_compressed("/tmp/big/dump.nt.bz2", NativeFormat::NTriples, Compression::Bzip2),
        ];
        let (argv, binds) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();

        // Two `-f`: one mounted path, one `-`.
        assert_eq!(argv.iter().filter(|a| *a == "-f").count(), 2);
        assert!(argv.iter().any(|a| a == "-"));
        assert!(argv.iter().any(|a| a.starts_with("/inputs/")));

        // Only the uncompressed input's parent is bind-mounted.
        assert!(binds.iter().any(|b| b.starts_with("/tmp/work")));
        assert!(!binds.iter().any(|b| b.starts_with("/tmp/big")));
    }

    #[test]
    fn rejects_two_compressed_inputs() {
        use super::super::decompressor::Compression;
        let config = QleverConfig::default();
        let inputs = vec![
            input_compressed("/tmp/a.nt.bz2", NativeFormat::NTriples, Compression::Bzip2),
            input_compressed("/tmp/b.nt.xz", NativeFormat::NTriples, Compression::Xz),
        ];
        let err = build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap_err();
        match err {
            QleverError::PreFlight(msg) => assert!(msg.contains("one compressed input"), "got: {msg}"),
            other => panic!("expected PreFlight, got: {other:?}"),
        }
    }

    #[test]
    fn argv_passes_parse_parallel_false_when_set() {
        let config = QleverConfig::default().with_parser_parallel(false);
        let inputs = vec![input("/tmp/data.ttl", NativeFormat::Turtle)];
        let (argv, _) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        let pos = argv
            .iter()
            .position(|a| a == "--parse-parallel")
            .expect("--parse-parallel missing");
        assert_eq!(argv[pos + 1], "false");
    }

    #[test]
    fn argv_omits_parser_flags_when_unset() {
        let config = QleverConfig::default();
        let inputs = vec![input("/tmp/data.ttl", NativeFormat::Turtle)];
        let (argv, _) =
            build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        assert!(!argv.iter().any(|a| a == "--parse-parallel"));
    }

    #[test]
    fn parse_size_to_bytes_understands_common_suffixes() {
        assert_eq!(parse_size_to_bytes("0").unwrap(), 0);
        assert_eq!(parse_size_to_bytes("1024").unwrap(), 1024);
        assert_eq!(parse_size_to_bytes("2G").unwrap(), 2 * 1024 * 1024 * 1024);
        assert_eq!(parse_size_to_bytes("2GiB").unwrap(), 2 * 1024 * 1024 * 1024);
        assert_eq!(parse_size_to_bytes("2GB").unwrap(), 2_000_000_000);
        assert_eq!(parse_size_to_bytes("512M").unwrap(), 512 * 1024 * 1024);
        assert_eq!(
            parse_size_to_bytes("1.5G").unwrap(),
            (1.5_f64 * 1024.0 * 1024.0 * 1024.0) as i64
        );
        assert!(parse_size_to_bytes("").is_err());
        assert!(parse_size_to_bytes("nope").is_err());
        assert!(parse_size_to_bytes("-1G").is_err());
    }

    #[test]
    fn host_config_propagates_memory_limit() {
        let config = QleverConfig::default()
            .with_container_memory("2G")
            .with_container_memory_swap("2G");
        let hc = host_config_with_limits(&config, vec![]).unwrap();
        assert_eq!(hc.memory, Some(2 * 1024 * 1024 * 1024));
        assert_eq!(hc.memory_swap, Some(2 * 1024 * 1024 * 1024));
    }

    #[test]
    fn host_config_leaves_memory_unset_by_default() {
        let config = QleverConfig::default();
        let hc = host_config_with_limits(&config, vec![]).unwrap();
        assert_eq!(hc.memory, None);
        assert_eq!(hc.memory_swap, None);
    }

    #[test]
    fn host_config_rejects_invalid_memory_string() {
        let config = QleverConfig::default().with_container_memory("not-a-size");
        let err = host_config_with_limits(&config, vec![]).unwrap_err();
        match err {
            QleverError::PreFlight(msg) => assert!(msg.contains("container_memory"), "got: {msg}"),
            other => panic!("expected PreFlight, got: {other:?}"),
        }
    }

    #[test]
    fn container_nonzero_exit_prepends_oom_hint() {
        let err = container_nonzero_exit("index-builder", 137, true, "WARN: parallel parser\n".to_string());
        match err {
            QleverError::ContainerNonZeroExit { logs, status, .. } => {
                assert_eq!(status, 137);
                assert!(logs.contains("OOM-killed"), "logs missing hint: {logs}");
                assert!(
                    logs.contains("parser_parallel"),
                    "logs missing remediation hint: {logs}"
                );
            },
            other => panic!("expected ContainerNonZeroExit, got: {other:?}"),
        }
    }

    #[test]
    fn container_nonzero_exit_no_hint_when_not_oom() {
        let err = container_nonzero_exit("index-builder", 1, false, "boom\n".to_string());
        match err {
            QleverError::ContainerNonZeroExit { logs, .. } => {
                assert!(!logs.contains("OOM-killed"), "unexpected OOM hint: {logs}");
            },
            other => panic!("expected ContainerNonZeroExit, got: {other:?}"),
        }
    }

    #[test]
    fn fingerprint_distinguishes_compressed_from_uncompressed() {
        use super::super::decompressor::Compression;
        // Same host_path on purpose — the compression salt must still flip the fingerprint.
        let a = input("/tmp/dump.nt", NativeFormat::NTriples);
        let b = input_compressed("/tmp/dump.nt", NativeFormat::NTriples, Compression::Bzip2);
        let c = input_compressed("/tmp/dump.nt", NativeFormat::NTriples, Compression::Xz);
        let fa = fingerprint_inputs(&[a]);
        let fb = fingerprint_inputs(&[b]);
        let fc = fingerprint_inputs(&[c]);
        assert_ne!(fa, fb);
        assert_ne!(fa, fc);
        assert_ne!(fb, fc);
    }
}
