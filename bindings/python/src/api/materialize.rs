use crate::{api::PyRudof, error::Result, formats::PyResultDataFormat, guard, output};
use pyo3::prelude::*;
use rudof_lib::formats::ResultDataFormat;
use std::path::PathBuf;

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a MapState from a JSON file.
    ///
    /// The MapState records the bindings produced by ShEx validation with Map semantic
    /// actions. It is required before calling :meth:`materialize`.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the JSON file containing the serialized MapState.
    ///
    /// Raises:
    ///     MapStateError: If the file cannot be read or the JSON is malformed.
    fn read_map_state(&mut self, py: Python<'_>, path: PathBuf) -> Result<()> {
        guard::detached(py, move || self.inner.load_map_state(&path).execute())?;
        Ok(())
    }

    /// Materializes an RDF graph from the current ShEx schema and MapState.
    ///
    /// Uses the Map semantic-action state (loaded via :meth:`read_map_state` or set
    /// after ShEx validation) to populate the triples defined by the ShEx schema's Map
    /// extensions.
    ///
    /// Args:
    ///     format (ResultDataFormat, optional): RDF serialization format for the output
    ///         graph. Defaults to ``ResultDataFormat.Turtle``.
    ///     node (str, optional): IRI string used as the root subject node of the
    ///         materialized graph. A fresh blank node is minted when omitted.
    ///
    /// Returns:
    ///     str: Serialized RDF graph.
    ///
    /// Raises:
    ///     MaterializeError: If no ShEx schema or MapState is loaded, if the node IRI is
    ///         invalid, or if materialization or serialization fails.
    #[pyo3(signature = (format = None, node = None))]
    fn materialize(&self, py: Python<'_>, format: Option<&PyResultDataFormat>, node: Option<&str>) -> Result<String> {
        let format: Option<ResultDataFormat> = format.map(Into::into);
        let node = node.map(str::to_owned);

        output::capture_string_detached(py, move |w| {
            let mut m = self.inner.materialize(w);
            if let Some(f) = &format {
                m = m.with_result_format(f);
            }
            if let Some(n) = &node {
                m = m.with_initial_node_iri(n);
            }
            m.execute()
        })
    }
}
