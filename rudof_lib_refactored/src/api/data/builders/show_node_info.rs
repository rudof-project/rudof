use crate::{Rudof, Result, api::data::DataOperations, formats::NodeInspectionMode};
use std::io;

/// Builder for `show_node_info` operation.
///
/// Provides a fluent interface for configuring and executing node inspection
/// operations with optional parameters.
pub struct ShowNodeInfoBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    node: &'a str,
    writer: &'a mut W,
    predicates: Option<&'a [String]>,
    show_node_mode: Option<&'a NodeInspectionMode>,
    depth: Option<usize>,
    show_hyperlinks: Option<bool>,
    show_colors: Option<bool>,
}

impl<'a, W: io::Write> ShowNodeInfoBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::show_node_info()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, node: &'a str, writer: &'a mut W) -> Self {
        Self {
            rudof,
            node,
            writer,
            predicates: None,
            show_node_mode: None,
            depth: None,
            show_hyperlinks: None,
            show_colors: None,
        }
    }

    /// Sets the predicates to filter displayed relations.
    ///
    /// # Arguments
    ///
    /// * `predicates` - List of predicate names to include in the output
    pub fn with_predicates(mut self, predicates: &'a [String]) -> Self {
        self.predicates = Some(predicates);
        self
    }

    /// Sets the node inspection mode.
    ///
    /// # Arguments
    ///
    /// * `show_node_mode` - The level of detail for node inspection
    pub fn with_show_node_mode(mut self, show_node_mode: &'a NodeInspectionMode) -> Self {
        self.show_node_mode = Some(show_node_mode);
        self
    }

    /// Sets the maximum traversal depth.
    ///
    /// # Arguments
    ///
    /// * `depth` - Maximum depth when expanding related nodes
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = Some(depth);
        self
    }

    /// Sets whether to include hyperlinks in the output.
    ///
    /// # Arguments
    ///
    /// * `show_hyperlinks` - Whether to include hyperlinks
    pub fn with_show_hyperlinks(mut self, show_hyperlinks: bool) -> Self {
        self.show_hyperlinks = Some(show_hyperlinks);
        self
    }

    /// Sets whether to use colored output.
    /// 
    /// # Arguments
    /// 
    /// * `show_colors` - Whether to use colored output
    pub fn with_show_colors(mut self, show_colors: bool) -> Self {
        self.show_colors = Some(show_colors);
        self
    }

    /// Executes the node inspection operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the node information cannot be retrieved or serialized.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::show_node_info(
            self.rudof,
            self.node,
            self.predicates,
            self.show_node_mode,
            self.depth,
            self.show_hyperlinks,
            self.show_colors,
            self.writer,
        )
    }
}