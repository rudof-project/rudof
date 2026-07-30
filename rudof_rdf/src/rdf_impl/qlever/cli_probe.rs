//! Detect which CLI is exposed by the running QLever image.
//!
//! Probes the image at runtime because the `adfreiburg/qlever` Docker
//! image migrated from the v1 entry points (`IndexBuilderMain`, `ServerMain`)
//! to a v2 CLI (`qlever-index`, `qlever-server`). Both still ship in current
//! images for backwards compatibility, but only one set is preferred at any
//! given tag.

use bollard::Docker;
use bollard::models::{ContainerCreateBody, HostConfig};
use bollard::query_parameters::{
    CreateContainerOptions, RemoveContainerOptionsBuilder, StartContainerOptions, WaitContainerOptionsBuilder,
};
use futures::TryStreamExt;
use tracing::debug;

use super::QleverError;

/// Which CLI flavour the running image exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliKind {
    /// `IndexBuilderMain` / `ServerMain` (the v1 CLI).
    V1,
    /// `qlever-index` / `qlever-server` (the v2 CLI).
    V2,
}

impl CliKind {
    /// Subcommand to use when running the index builder.
    pub fn index_builder_cmd(self) -> &'static str {
        match self {
            CliKind::V1 => "IndexBuilderMain",
            CliKind::V2 => "qlever-index",
        }
    }

    /// Subcommand to use when running the server.
    pub fn server_cmd(self) -> &'static str {
        match self {
            CliKind::V1 => "ServerMain",
            CliKind::V2 => "qlever-server",
        }
    }
}

/// Probe `image` for which CLI is available. Tries v1 first, then v2.
///
/// Also pulls the image if it's not present locally.
pub async fn probe(image: &str) -> Result<CliKind, QleverError> {
    let docker = connect()?;
    ensure_image(&docker, image).await?;

    if probe_one(&docker, image, "IndexBuilderMain").await? {
        return Ok(CliKind::V1);
    }
    if probe_one(&docker, image, "qlever-index").await? {
        return Ok(CliKind::V2);
    }
    Err(QleverError::UnknownCliKind {
        image: image.to_string(),
    })
}

/// Connect to the local Docker daemon (surfaces a friendly error if the daemon socket is missing or unreachable).
pub(crate) fn connect() -> Result<Docker, QleverError> {
    Docker::connect_with_local_defaults().map_err(|e| QleverError::DockerUnreachable {
        message: format!("{e}"),
    })
}

/// Ping the daemon (used as a pre-flight check before any operation that would otherwise fail confusingly).
pub(crate) async fn ping(docker: &Docker) -> Result<(), QleverError> {
    docker
        .ping()
        .await
        .map(|_| ())
        .map_err(|e| QleverError::DockerUnreachable {
            message: format!("{e}"),
        })
}

/// Pull `image` if it's not already present locally.
///
/// Streams progress lines into `tracing` so users can see why the first run hangs for a minute (typically a ~1GB pull).
pub(crate) async fn ensure_image(docker: &Docker, image: &str) -> Result<(), QleverError> {
    if docker.inspect_image(image).await.is_ok() {
        return Ok(());
    }
    tracing::info!("QLever image {} not present locally; pulling (~1GB)", image);

    let options = bollard::query_parameters::CreateImageOptions {
        from_image: Some(image.to_string()),
        ..Default::default()
    };

    let mut stream = docker.create_image(Some(options), None, None);
    let mut last_status = String::new();
    while let Some(item) = stream.try_next().await? {
        if let Some(status) = item.status
            && status != last_status
        {
            tracing::info!("docker pull: {}", status);
            last_status = status;
        }
    }
    Ok(())
}

/// Run `<image> -c "<cmd> -h"` as a one-shot container and return whether
/// the container exited with status 0 (used as a presence check for the CLI).
async fn probe_one(docker: &Docker, image: &str, cmd: &str) -> Result<bool, QleverError> {
    debug!("probing QLever CLI: {} {} -h", image, cmd);

    // Bind a throwaway tempdir to /data so the entrypoint won't bail out with its welcome message.
    let tmp = tempfile::tempdir()?;
    let host_path = tmp.path().display().to_string();
    let binds = vec![format!("{host_path}:/data:rw")];

    let user = host_uid_gid();

    let create = ContainerCreateBody {
        image: Some(image.to_string()),
        cmd: Some(vec!["-c".into(), format!("{cmd} -h")]),
        working_dir: Some("/data".to_string()),
        user,
        host_config: Some(HostConfig {
            binds: Some(binds),
            auto_remove: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    };
    let create_options: Option<CreateContainerOptions> = None;
    let container = docker.create_container(create_options, create).await?;
    let id = container.id;

    docker.start_container(&id, None::<StartContainerOptions>).await?;

    let wait_opts = WaitContainerOptionsBuilder::new().condition("not-running").build();
    let wait_result: Vec<_> = docker
        .wait_container(&id, Some(wait_opts))
        .try_collect::<Vec<_>>()
        .await
        .unwrap_or_default();
    let exit_code = wait_result.into_iter().next().map(|r| r.status_code).unwrap_or(-1);

    let remove_opts = RemoveContainerOptionsBuilder::new().force(true).build();
    let _ = docker.remove_container(&id, Some(remove_opts)).await;

    Ok(exit_code == 0)
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
