use crate::cli::parser::CommonArgsNoBackend;
use clap::Args;

/// Arguments for the `config` command
///
/// Dumps the effective configuration that rudof is currently using as TOML
#[derive(Debug, Clone, Args)]
pub struct ConfigArgs {
    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
