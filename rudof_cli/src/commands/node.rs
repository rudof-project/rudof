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
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let show_node_mode = self.args.show_node_mode.into();

        let mut loading = ctx
            .rudof
            .load_data()
            .with_data(&self.args.data)
            .with_data_format(&data_format)
            .with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base.as_deref() {
            loading = loading.with_base(base);
        }
        if let Some(endpoint) = self.args.endpoint.as_deref() {
            loading = loading.with_endpoint(endpoint);
        }
        loading.execute()?;

        let mut showing_node_info = ctx
            .rudof
            .show_node_info(self.args.node.as_ref(), &mut ctx.writer)
            .with_show_node_mode(&show_node_mode)
            .with_depth(self.args.depth);
        if let Some(predicates) = self.args.predicates.as_deref() {
            showing_node_info = showing_node_info.with_predicates(predicates);
        }
        if let Some(show_hyperlinks) = self.args.show_hyperlinks {
            showing_node_info = showing_node_info.with_show_hyperlinks(show_hyperlinks);
        }
        showing_node_info.execute()?;

        Ok(())
    }
}
