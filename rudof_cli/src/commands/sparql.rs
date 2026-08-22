use crate::cli::parser::SparqlArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `sparql` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Sparql logic.
pub struct SparqlCommand {
    /// Arguments specific to sparql.
    args: SparqlArgs,
}

impl SparqlCommand {
    pub fn new(args: SparqlArgs) -> Self {
        Self { args }
    }
}

impl Command for SparqlCommand {
    fn name(&self) -> &'static str {
        "sparql"
    }

    /// Executes the Sparql command.
    ///
    /// With no `--query`, there is nothing new to load, so this just
    /// re-serializes whatever query is already loaded in the session
    /// (useful in the interactive shell, where state persists across
    /// commands). This only shows the query — run it with the `query`
    /// command, which loads into (or reuses) the same state.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        if let Some(query) = &self.args.query {
            ctx.rudof.load_sparql_query(query).execute()?;
        }

        ctx.rudof.serialize_sparql_query(&mut ctx.writer).execute()?;

        Ok(())
    }
}
