use crate::cli::parser::CommonArgsNoBackend;
use clap::Args;

/// Arguments for the `shell` command
#[derive(Debug, Clone, Args)]
pub struct ShellArgs {
    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
