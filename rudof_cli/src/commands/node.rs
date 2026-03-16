use crate::cli::parser::NodeArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

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

        ctx.rudof.load_data(
            &self.args.data, 
            Some(&self.args.data_format.into()), 
            self.args.base.as_deref(), 
            Some(&self.args.reader_mode.into()),
        );
        
        ctx.rudof.show_node_info(self.args.node.as_ref(), &mut ctx.writer)
            .with_predicates(&self.args.predicates)
            .with_show_node_mode(&self.args.show_node_mode.into())
            .with_depth(self.args.depth)
            .with_show_hyperlinks(self.args.show_hyperlinks)
            .execute()?;

        Ok(())
    }
}
