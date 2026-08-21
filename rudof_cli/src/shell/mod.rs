//! Interactive shell (REPL) for rudof.
//!
//! Implements the `rudof shell` command: a `rudof>` prompt that re-dispatches
//! typed lines to the same subcommands available on the `rudof` CLI, sharing a
//! single [`rudof_lib::Rudof`] session across commands (so e.g. `data` loaded
//! in one line stays loaded for a later `validate`).

mod completer;
mod repl;

use crate::cli::parser::ShellArgs;
use crate::commands::{Command, CommandContext};
use anyhow::Result;

pub struct ShellCommand;

impl ShellCommand {
    pub fn new(_args: ShellArgs) -> Self {
        Self
    }
}

impl Command for ShellCommand {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        repl::run(ctx)
    }
}
