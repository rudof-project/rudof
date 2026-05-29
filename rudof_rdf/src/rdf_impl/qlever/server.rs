//! QLever HTTP server lifecycle.

use std::time::Duration;

use testcontainers::core::{ContainerPort, IntoContainerPort, Mount, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use tracing::{debug, info, warn};

use super::cli_probe;
use super::config::CONTAINER_WORKING_DIR;
use super::{CliKind, IndexHandle, QleverConfig, QleverError};

/// A running QLever container.
pub struct QleverServer {
    /// The live container. Stored in an Option so we can drop it inside
    /// `Drop` while a Tokio runtime is entered.
    container: Option<ContainerAsync<GenericImage>>,
    /// Runtime handle captured at startup for safe async drop.
    runtime: tokio::runtime::Handle,
    /// `http://host:mappedPort/`.
    endpoint_url: String,
    /// Host path of the index directory we (or a previous run) materialised.
    /// Used by `Drop` when `config.auto_delete_if_created` is set.
    index_dir_path: Option<std::path::PathBuf>,
    /// `true` if this run originally created the index dir on disk.
    created_index_dir: bool,
    /// Config snapshot used to start this server.
    config: QleverConfig,
}

impl Drop for QleverServer {
    fn drop(&mut self) {
        // Ensure the container's async drop runs within a Tokio runtime.
        let _guard = self.runtime.enter();
        if let Some(container) = self.container.take() {
            drop(container);
        }

        // The `ContainerAsync` `Drop` impl tears down the container itself.
        // We may additionally need to wipe the on-disk index if the user
        // opted into auto-delete and we created it.
        if self.config.auto_delete_if_created
            && self.created_index_dir
            && let Some(p) = &self.index_dir_path
            && let Err(e) = std::fs::remove_dir_all(p)
        {
            tracing::warn!("could not remove QLever index dir {}: {}", p.display(), e);
        }
    }
}

impl std::fmt::Debug for QleverServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QleverServer")
            .field("endpoint_url", &self.endpoint_url)
            .field("image", &self.config.image())
            .finish()
    }
}

impl QleverServer {
    /// Start a server serving the on-disk index at `handle`.
    pub async fn start(handle: &IndexHandle, config: &QleverConfig) -> Result<Self, QleverError> {
        // Pre-flight: Docker daemon reachable + image present.
        let docker = cli_probe::connect()?;
        cli_probe::ping(&docker).await?;
        cli_probe::ensure_image(&docker, &config.image()).await?;

        let cli_kind = cli_probe::probe(&config.image()).await?;
        let cmd = build_server_cmd(cli_kind, handle.name(), config);
        info!("starting QLever server: {}", cmd);

        let image = GenericImage::new(config.image_name.as_str(), config.image_tag.as_str())
            // Brief pause so the container is alive
            .with_wait_for(WaitFor::Duration {
                length: Duration::from_millis(750),
            })
            .with_exposed_port(config.container_port.tcp());

        // The image's entrypoint expects `["-c", "<cmd-string>"]` and a working directory of `/data` with something bound there.
        let mut request = image
            .with_cmd(["-c", cmd.as_str()])
            .with_working_dir(CONTAINER_WORKING_DIR.to_string())
            .with_mount(Mount::bind_mount(
                handle.path().display().to_string(),
                CONTAINER_WORKING_DIR.to_string(),
            ));

        if let Some(label) = &config.container_label {
            request = request.with_label("rudof.qlever.label", label.clone());
        }
        request = request.with_label("rudof.qlever", "true");

        if config.run_as_host_user
            && let Some(user) = host_uid_gid()
        {
            request = request.with_user(user);
        }

        let host_port_opt = config.host_port;
        let container_port = config.container_port;
        if host_port_opt.is_some() {
            request = request.with_host_config_modifier(move |hc| {
                if let Some(host_port) = host_port_opt {
                    use bollard::models::{PortBinding, PortMap};
                    let mut bindings: PortMap = std::collections::HashMap::new();
                    bindings.insert(
                        format!("{}/tcp", container_port),
                        Some(vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some(host_port.to_string()),
                        }]),
                    );
                    hc.port_bindings = Some(bindings);
                }
            });
        }

        let container = request.start().await?;
        let runtime = tokio::runtime::Handle::current();

        let mapped = container.get_host_port_ipv4(ContainerPort::Tcp(container_port)).await?;
        let endpoint_url = format!("http://127.0.0.1:{mapped}/");

        wait_until_responsive(&endpoint_url, config.server_readiness_timeout_secs).await?;
        debug!("QLever server ready at {}", endpoint_url);

        Ok(QleverServer {
            container: Some(container),
            runtime,
            endpoint_url,
            index_dir_path: Some(handle.path().to_path_buf()),
            created_index_dir: false,
            config: config.clone(),
        })
    }

    /// Mark that this run created the on-disk index.
    pub fn mark_created_index(&mut self, created: bool) {
        self.created_index_dir = created;
    }

    /// Endpoint URL — `http://host:mappedPort/`.
    pub fn endpoint(&self) -> &str {
        &self.endpoint_url
    }

    /// The config that brought this server up.
    pub fn config(&self) -> &QleverConfig {
        &self.config
    }
}

/// Construct the single command string that the container's shell will run.
/// Translates the [`QleverConfig`] flags to `ServerMain` / `qlever-server`
pub(crate) fn build_server_cmd(cli: CliKind, index_name: &str, config: &QleverConfig) -> String {
    let mut argv: Vec<String> = vec![cli.server_cmd().to_string()];
    argv.push("-i".into());
    argv.push(index_name.to_string());
    argv.push("-p".into());
    argv.push(config.container_port.to_string());

    if let Some(t) = &config.access_token {
        argv.push("-a".into());
        argv.push(t.clone());
    }
    if let Some(n) = config.num_simultaneous_queries {
        argv.push("-j".into());
        argv.push(n.to_string());
    }
    if let Some(m) = &config.memory_max_size {
        argv.push("-m".into());
        argv.push(m.clone());
    }
    if let Some(c) = &config.cache_max_size {
        argv.push("-c".into());
        argv.push(c.clone());
    }
    if let Some(e) = &config.cache_max_size_single_entry {
        argv.push("-e".into());
        argv.push(e.clone());
    }
    if let Some(e) = &config.lazy_result_max_cache_size {
        argv.push("-E".into());
        argv.push(e.clone());
    }
    if let Some(k) = config.cache_max_num_entries {
        argv.push("-k".into());
        argv.push(k.to_string());
    }
    if config.no_patterns {
        argv.push("-P".into());
    }
    if config.no_pattern_trick {
        argv.push("-T".into());
    }
    if config.text {
        argv.push("-t".into());
    }
    if config.only_pso_and_pos_permutations {
        argv.push("-o".into());
    }
    if let Some(s) = &config.default_query_timeout {
        argv.push("-s".into());
        argv.push(s.clone());
    }
    if let Some(s) = config.service_max_value_rows {
        argv.push("-S".into());
        argv.push(s.to_string());
    }
    if config.throw_on_unbound_variables {
        argv.push("--throw-on-unbound-variables".into());
        argv.push("true".into());
    }

    argv.iter().map(|s| shell_quote(s)).collect::<Vec<_>>().join(" ")
}

/// Block until a SPARQL `ASK` against `endpoint` returns 200 OK, or `timeout_secs` elapses.
async fn wait_until_responsive(endpoint: &str, timeout_secs: u64) -> Result<(), QleverError> {
    let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build()?;
    let deadline = std::time::Instant::now() + Duration::from_secs(timeout_secs);
    let probe_query = "ASK { ?s ?p ?o }";

    let mut backoff = Duration::from_millis(100);
    loop {
        if let Ok(url) = url::Url::parse_with_params(endpoint, &[("query", probe_query)]) {
            match client.get(url).send().await {
                Ok(r) if r.status().is_success() => return Ok(()),
                Ok(r) => debug!("QLever readiness probe got status {}", r.status()),
                Err(e) => debug!("QLever readiness probe error: {}", e),
            }
        }

        if std::time::Instant::now() >= deadline {
            warn!("QLever did not respond within {}s", timeout_secs);
            return Err(QleverError::ServerStartupTimeout {
                endpoint: endpoint.to_string(),
                timeout_secs,
            });
        }

        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_secs(2));
    }
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

fn shell_quote(s: &str) -> String {
    if s.bytes().all(|b| {
        matches!(b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' | b':' | b'='
        )
    }) {
        s.to_string()
    } else {
        let escaped = s.replace('\'', "'\\''");
        format!("'{escaped}'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_cmd_starts_with_correct_subcommand() {
        let config = QleverConfig::default();
        let cmd = build_server_cmd(CliKind::V1, "default", &config);
        assert!(cmd.starts_with("ServerMain"), "got: {cmd}");
        assert!(cmd.contains("-i default"));
        assert!(cmd.contains("-p 7001"));
        assert!(cmd.contains("-m 5G"));

        let cmd_v2 = build_server_cmd(CliKind::V2, "default", &config);
        assert!(cmd_v2.starts_with("qlever-server"));
    }

    #[test]
    fn server_cmd_omits_unset_flags() {
        let config = QleverConfig {
            memory_max_size: None,
            cache_max_size: None,
            cache_max_size_single_entry: None,
            ..Default::default()
        };
        let cmd = build_server_cmd(CliKind::V1, "default", &config);
        assert!(!cmd.contains("-m"), "leaked memory flag: {cmd}");
        // -c is a valid prefix of nothing else; check space.
        assert!(!cmd.contains(" -c "), "leaked cache flag: {cmd}");
    }

    #[test]
    fn server_cmd_includes_throw_on_unbound() {
        let config = QleverConfig {
            throw_on_unbound_variables: true,
            ..Default::default()
        };
        let cmd = build_server_cmd(CliKind::V1, "default", &config);
        assert!(cmd.contains("--throw-on-unbound-variables true"), "got: {cmd}");
    }
}
