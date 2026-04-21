use crate::cli::parser::CommonArgsOutputForceOverWrite;
use clap::Args;
use clap_complete_command::Shell;

/// Arguments for the `completion` command
#[derive(Debug, Clone, Args)]
pub struct CompletionArgs {
    #[clap(value_parser = clap::value_parser!(Shell))]
    pub shell: Shell,

    #[command(flatten)]
    pub common: CommonArgsOutputForceOverWrite,
}
