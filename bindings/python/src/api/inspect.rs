use crate::{api::PyRudof, error::Result, formats::PyNodeNeighborhood, output};
use pyo3::prelude::*;
use rudof_lib::formats::{IriNormalizationMode, NodeInspectionMode};
use std::str::FromStr;

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Retrieves detailed information about a specific node in the RDF graph.
    ///
    /// Provides a neighborhood view of a node, including its properties, outgoing and
    /// incoming edges, and connected nodes up to a specified depth.
    ///
    /// Args:
    ///     node_selector (str): Node identifier. Can be:
    ///         - Full IRI: ``<http://example.org/alice>``
    ///         - Prefixed name: ``:alice``
    ///         - Blank node: ``_:b1``
    ///     predicates (list[str], optional): Filter by specific predicates. Empty list means all predicates.
    ///     mode (str, optional): Node inspection mode — ``"outgoing"``, ``"incoming"`` or ``"both"``. Defaults to ``"both"``.
    ///     show_colors (bool, optional): Use ANSI terminal colors in output. Defaults to ``True``.
    ///     depth (int, optional): Neighborhood distance (1 = direct neighbors, 2 = neighbors of neighbors, ...). 
    ///         Defaults to ``1``.
    ///
    /// Returns:
    ///     str: Formatted string with node information and neighborhood graph.
    ///
    /// Raises:
    ///     NodeInspectionError: If the mode is not one of the three accepted values, or
    ///         the node selector is invalid.
    ///
    /// Note:
    ///     Colors require a terminal with ANSI escape sequence support.
    #[pyo3(signature = (node_selector, predicates = None, mode = None, show_colors = None,
                        depth = None))]
    fn node_info(
        &mut self,
        py: Python<'_>,
        node_selector: &str,
        predicates: Option<Vec<String>>,
        mode: Option<&str>,
        show_colors: Option<bool>,
        depth: Option<usize>,
    ) -> Result<String> {
        let node_selector = node_selector.to_owned();
        let mode = mode.map(NodeInspectionMode::from_str).transpose()?;

        output::capture_string_detached(py, move |w| {
            let mut b = self.inner.show_node_info(&node_selector, w);
            if let Some(p) = predicates.as_deref() {
                b = b.with_predicates(p);
            }
            if let Some(m) = &mode {
                b = b.with_show_node_mode(m);
            }
            if let Some(c) = show_colors {
                b = b.with_show_colors(c);
            }
            if let Some(d) = depth {
                b = b.with_depth(d);
            }
            b.execute()
        })
    }

    /// Returns an iterator over the arcs around a node, in depth-first order.
    ///
    /// Args:
    ///     node_selector (str): Node identifier, as in :meth:`node_info`.
    ///     predicates (list[str], optional): Filter by specific predicates.
    ///     mode (str, optional): ``"outgoing"``, ``"incoming"`` or ``"both"``. Defaults to ``"both"``.
    ///     depth (int, optional): Neighborhood distance. Defaults to ``1``.
    ///     strict_iris (bool, optional): Require angle-bracketed IRIs instead of auto-wrapping bare ones. 
    ///         Defaults to ``False``.
    ///
    /// Returns:
    ///     NeighborArcIterator: The arcs around the node.
    ///
    /// Raises:
    ///     NodeInspectionError: If the mode is invalid.
    ///     DataError: If an arc cannot be retrieved.
    #[pyo3(signature = (node_selector, predicates = None, mode = None, depth = None,
                        strict_iris = None))]
    fn node_neighborhood(
        &self,
        py: Python<'_>,
        node_selector: &str,
        predicates: Option<Vec<String>>,
        mode: Option<&str>,
        depth: Option<usize>,
        strict_iris: Option<bool>,
    ) -> Result<PyNodeNeighborhood> {
        let node_selector = node_selector.to_owned();
        let mode = mode.map(NodeInspectionMode::from_str).transpose()?;

        let arcs = py.detach(move || {
            let mut b = self.inner.node_neighborhood(&node_selector);
            if let Some(p) = predicates.as_deref() {
                b = b.with_predicates(p);
            }
            if let Some(m) = &mode {
                b = b.with_mode(m);
            }
            if let Some(d) = depth {
                b = b.with_depth(d);
            }
            if strict_iris.unwrap_or(false) {
                b = b.with_iri_mode(IriNormalizationMode::Strict);
            }
            b.execute()?.collect::<std::result::Result<Vec<_>, _>>()
        })?;

        Ok(PyNodeNeighborhood::new(arcs))
    }
}
