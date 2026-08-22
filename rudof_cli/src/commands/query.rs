use crate::cli::parser::QueryArgs;
use crate::cli::wrappers::resolve_backend;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::formats::BackendSpec;

/// Implementation of the `query` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Query command logic.
pub struct QueryCommand {
    /// Arguments specific to Query command.
    args: QueryArgs,
}

impl QueryCommand {
    pub fn new(args: QueryArgs) -> Self {
        Self { args }
    }
}

impl Command for QueryCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "query"
    }

    /// Executes the Query command logic.
    ///
    /// With no `--query`, there is no new query to run, so this just
    /// re-serializes the results of the last query run in the session
    /// (useful in the interactive shell, where state persists across
    /// commands).
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let query_type = self.args.query_type.into();
        let result_query_format = self.args.result_query_format.into();

        let backend = resolve_backend(&self.args.common);
        let has_data_source = !self.args.data.is_empty() || matches!(backend, BackendSpec::Endpoint(_));

        if has_data_source {
            let mut loading = ctx
                .rudof
                .load_data()
                .with_data_format(&data_format)
                .with_reader_mode(&reader_mode)
                .with_backend(backend);
            if !self.args.data.is_empty() {
                loading = loading.with_data(&self.args.data);
            }
            if let Some(base) = self.args.base.as_deref() {
                loading = loading.with_base(base);
            }
            loading.execute()?;
        }

        match &self.args.query {
            Some(query) => {
                ctx.rudof
                    .load_sparql_query(query)
                    .with_query_type(&query_type)
                    .execute()?;

                ctx.rudof
                    .run_query()
                    .with_result_query_format(&result_query_format)
                    .execute()?;
            },
            None if ctx.rudof.query_results().is_none() && !ctx.rudof.has_sparql_query() => {
                anyhow::bail!("No query specified. Use --query/-q to provide a SPARQL query to run.");
            },
            // A query was loaded without being run yet (e.g. via `sparql
            // -q FILE`, which only loads and shows) — run it now, since
            // `sparql` and `query` share the same loaded-query state.
            None if ctx.rudof.query_results().is_none() => {
                ctx.rudof
                    .run_query()
                    .with_result_query_format(&result_query_format)
                    .execute()?;
            },
            None => {},
        }

        ctx.rudof
            .serialize_query_results(&mut ctx.writer)
            .with_result_query_format(&result_query_format)
            .execute()?;

        Ok(())
    }
}
