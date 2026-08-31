use crate::cli::parser::NodeArgs;
use crate::cli::wrappers::resolve_backend;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::formats::{BackendSpec, IriNormalizationMode};

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

        let backend = resolve_backend(&self.args.common);
        let has_data_source =
            !self.args.data.is_empty() || matches!(backend, BackendSpec::Endpoint(_) | BackendSpec::Lbug);

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
        } else if !ctx.rudof.has_data()
            && let Some(uri) = dereferenceable_uri(&self.args.node)
        {
            ctx.rudof.dereference(uri).with_reader_mode(&reader_mode).execute()?;
        }

        let iri_mode = if self.args.strict_iris {
            IriNormalizationMode::Strict
        } else {
            IriNormalizationMode::Lax
        };

        let mut showing_node_info = ctx
            .rudof
            .show_node_info(self.args.node.as_ref(), &mut ctx.writer)
            .with_show_node_mode(&show_node_mode)
            .with_depth(self.args.depth)
            .with_iri_mode(iri_mode);
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

/// Extracts an absolute `http(s)://` IRI from a node selector string, if it
/// names one directly (optionally wrapped in `<>`), so `node` can fall back
/// to dereferencing it when no data or endpoint was given.
///
/// Prefixed names (`ex:Q80`) and blank nodes (`_:b1`) return `None` — there
/// is nothing to fetch without data already loaded to resolve the prefix.
fn dereferenceable_uri(node: &str) -> Option<&str> {
    let trimmed = node.trim();
    let inner = trimmed
        .strip_prefix('<')
        .and_then(|rest| rest.strip_suffix('>'))
        .unwrap_or(trimmed);
    if inner.starts_with("http://") || inner.starts_with("https://") {
        Some(inner)
    } else {
        None
    }
}
