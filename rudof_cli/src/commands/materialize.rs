use crate::cli::parser::MaterializeArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `materialize` command.
///
/// Loads a ShEx schema and a MapState, then materializes an RDF graph using the
/// Map semantic-action state and writes the result to the configured output.
pub struct MaterializeCommand {
    args: MaterializeArgs,
}

impl MaterializeCommand {
    pub fn new(args: MaterializeArgs) -> Self {
        Self { args }
    }
}

impl Command for MaterializeCommand {
    fn name(&self) -> &'static str {
        "materialize"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let schema_format = self.args.schema_format.into();
        let reader_mode = self.args.reader_mode.into();
        let result_format = self.args.result_format.into();

        // 1. Load ShEx schema
        let mut shex_loading = ctx
            .rudof
            .load_shex_schema(&self.args.schema)
            .with_shex_schema_format(&schema_format)
            .with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base.as_deref() {
            shex_loading = shex_loading.with_base(base);
        }
        shex_loading.execute()?;

        // 2. Load MapState from file (if provided)
        if let Some(map_state_path) = &self.args.map_state {
            ctx.rudof.load_map_state(map_state_path).execute()?;
        }

        // 3. Materialize and write the RDF graph
        let mut materialize = ctx
            .rudof
            .materialize(&mut ctx.writer)
            .with_result_format(&result_format);
        if let Some(node_iri) = self.args.node.as_deref() {
            materialize = materialize.with_initial_node_iri(node_iri);
        }
        materialize.execute()?;

        Ok(())
    }
}
