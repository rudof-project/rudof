use crate::cli::parser::NodeArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::{ReaderMode, rdf_reader_mode::RDFReaderMode};

/// Implementation of the `node` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Node command logic.
pub struct NodeCommand {
    /// Arguments specific to Node command.
    args: NodeArgs,
}

impl NodeCommand {
    pub fn new(args: NodeArgs) -> Self {
        Self { args }
    }
}

impl Command for NodeCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "node"
    }

    /// Executes the Node command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // Convert CLI types to library types
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();

        // Load RDF data into rudof
        ctx.rudof.load_data(
            &self.args.data,
            &(&self.args.data_format).into(),
            &self.args.base,
            &None,
            &reader_mode,
            false,
        )?;

        // Get node info and write to output
        ctx.rudof.show_node_info(
            &self.args.node,
            &self.args.predicates,
            &self.args.show_node_mode.to_string(), 
            self.args.depth,
            &mut ctx.writer,
        )?;

        Ok(())
    }
}
