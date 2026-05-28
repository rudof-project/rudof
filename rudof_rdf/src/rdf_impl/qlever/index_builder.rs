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

use bollard::Docker;
use bollard::models::{ContainerCreateBody, HostConfig};
use bollard::query_parameters::{
    CreateContainerOptions, LogsOptionsBuilder, RemoveContainerOptionsBuilder, StartContainerOptions,
    WaitContainerOptionsBuilder,
};
use futures::TryStreamExt;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};

use super::cli_probe;
use super::{InputFile, NativeFormat, QleverConfig, QleverError};
use super::config::CONTAINER_WORKING_DIR;

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
        // QLever writes a `<name>.meta` file at the very end of indexing, along with several `*.permutation` files.
        self.dir.join(format!("{}.meta", self.name)).exists()
    }
}

/// Build the index for the given inputs.
///
/// Idempotent: if the on-disk index files are already present, this returns without doing any Docker work.
pub async fn build_index(inputs: &[InputFile], config: &QleverConfig) -> Result<IndexHandle, QleverError> {
    if inputs.is_empty() {
        return Err(QleverError::PreFlight(
            "build_index called with no inputs".to_string(),
        ));
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

    run_one_shot(&docker, config, binds, &cmd_string, "index-builder").await?;

    Ok(handle)
}

/// Build the `IndexBuilderMain` argv plus the list of `host:container[:opt]` bind strings.
fn build_argv_and_binds(
    cli: super::CliKind,
    inputs: &[InputFile],
    config: &QleverConfig,
    index_dir: &Path,
) -> Result<(Vec<String>, Vec<String>), QleverError> {
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

    // index_dir → /data (read-write)
    let mut binds = vec![format!("{}:{}:rw", index_dir.display(), CONTAINER_WORKING_DIR)];

    // Bind each unique input host directory at a separate mount point so we
    // never have to copy possibly-huge files into the index dir.
    let mut dir_mounts: HashMap<PathBuf, String> = HashMap::new();
    for input in inputs {
        let parent = input
            .host_path
            .parent()
            .ok_or_else(|| {
                QleverError::PreFlight(format!("input has no parent dir: {}", input.host_path.display()))
            })?
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
    let host_config = HostConfig {
        binds: Some(binds),
        auto_remove: Some(false),
        ..Default::default()
    };

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
    let exit_code = wait_result.into_iter().next().map(|r| r.status_code).unwrap_or(-1);

    let remove_opts = RemoveContainerOptionsBuilder::new().force(true).build();
    let _ = docker.remove_container(&id, Some(remove_opts)).await;

    if exit_code != 0 {
        return Err(QleverError::ContainerNonZeroExit {
            what: what.to_string(),
            status: exit_code,
            logs: log_buf,
        });
    }
    Ok(())
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
        RDFFormat::NQuads | RDFFormat::TriG | RDFFormat::JsonLd => {
            (NativeFormat::NQuads, OxRdfFormat::NQuads, "nq")
        },
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
        writer.serialize_quad(&quad).map_err(|e| QleverError::FormatConversion {
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
        let (argv, _) = build_argv_and_binds(super::super::CliKind::V1, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        // Find the -g and confirm next arg is "-"
        let g_pos = argv.iter().position(|a| a == "-g").unwrap();
        assert_eq!(argv[g_pos + 1], "-");
    }

    #[test]
    fn cli_kind_v2_uses_qlever_index_subcommand() {
        let config = QleverConfig::default();
        let inputs = vec![input("/tmp/data.ttl", NativeFormat::Turtle)];
        let (argv, _) = build_argv_and_binds(super::super::CliKind::V2, &inputs, &config, Path::new("/tmp/idx")).unwrap();
        assert_eq!(argv[0], "qlever-index");
    }

    #[test]
    fn is_built_detects_existing_meta_file() {
        let tmp = tempfile::tempdir().unwrap();
        let handle = IndexHandle::new(tmp.path(), "default");
        assert!(!handle.is_built());
        std::fs::write(tmp.path().join("default.meta"), b"").unwrap();
        assert!(handle.is_built());
    }
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
        RDFFormat::JsonLd => Ox::JsonLd { profile: Default::default() },
        // N3 is not in oxrdfio (fall back to Turtle).
        RDFFormat::N3 => Ox::Turtle,
    }
}
