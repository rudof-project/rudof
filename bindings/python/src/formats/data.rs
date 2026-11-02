use pyo3::prelude::*;
use rudof_lib::formats::{DataFormat, DataReaderMode, ResultDataFormat};
use rudof_lib::types::{DataDiff, GraphRelation, SubgraphMode};

pyenum! {
    /// RDF data serialization formats supported when reading or writing graphs.
    "RDFFormat": PyRDFFormat => DataFormat, default = Turtle, from_str {
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// N-Triples - line-based RDF format with one triple per line.
        NTriples => NTriples,
        /// RDF/XML - XML-based RDF serialization.
        RdfXml   => RdfXml,
        /// TriG - Turtle extended with named graphs.
        TriG     => TriG,
        /// Notation3 - superset of Turtle.
        N3       => N3,
        /// N-Quads - N-Triples extended with named graphs.
        NQuads   => NQuads,
        /// JSON-LD - JSON serialization for Linked Data.
        JsonLd   => JsonLd,
        /// Property Graph format - nodes and edges (non-RDF).
        Pg       => Pg,
    }
}

pyenum! {
    /// Output formats for serializing the currently loaded RDF data.
    #[allow(clippy::upper_case_acronyms)]
    "ResultDataFormat": PyResultDataFormat => ResultDataFormat, default = Compact, from_str {
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// N-Triples - line-based RDF format with one triple per line.
        NTriples => NTriples,
        /// JSON-LD - JSON serialization for Linked Data.
        JsonLd   => JsonLd,
        /// RDF/XML - XML-based RDF serialization.
        RdfXml   => RdfXml,
        /// TriG - Turtle extended with named graphs.
        TriG     => TriG,
        /// Notation3 - superset of Turtle.
        N3       => N3,
        /// N-Quads - N-Triples extended with named graphs.
        NQuads   => NQuads,
        /// Compact - concise textual summary of the graph.
        Compact  => Compact,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
        /// PlantUML - text-based UML diagram source.
        PlantUML => PlantUML,
        /// SVG - Scalable Vector Graphics image.
        Svg      => Svg,
        /// PNG - Portable Network Graphics image.
        Png      => Png,
    }
}

pyenum! {
    /// How strictly parsers react to malformed input.
    "ReaderMode": PyReaderMode => DataReaderMode, default = Lax, from_str {
        /// Ignore non-fatal errors and continue processing.
        Lax    => Lax,
        /// Fail immediately on the first parsing error.
        Strict => Strict,
    }
}

pyenum! {
    /// The relation between two RDF graphs, as returned by ``Rudof.compare_data``.
    /// Graphs are compared up to blank node renaming.
    "GraphRelation": PyGraphRelation => GraphRelation, default = Different {
        /// Both graphs are isomorphic: equal up to blank node renaming.
        Isomorphic => Isomorphic,
        /// Each graph is contained in the other without them being isomorphic
        /// (only with ``SubgraphMode.Homomorphic`` and ``SubgraphMode.Entailment``).
        Equivalent => Equivalent,
        /// The first graph is contained in the second, but not the other way round.
        Subgraph   => Subgraph,
        /// The second graph is contained in the first, but not the other way round.
        Supergraph => Supergraph,
        /// None of the above.
        Different  => Different,
    }
}

pyenum! {
    /// How blank nodes may be mapped when ``Rudof.compare_data`` checks whether
    /// one graph is contained in another.
    "SubgraphMode": PySubgraphMode => SubgraphMode, default = Injective, from_str {
        /// One-to-one onto blank nodes: the first graph is isomorphic to a subgraph of the second.
        Injective   => Injective,
        /// Onto blank nodes, several possibly onto the same one.
        Homomorphic => Homomorphic,
        /// Onto any term (IRI, literal, blank node or triple term): the second graph
        /// simply entails the first.
        Entailment  => Entailment,
    }
}

/// The differences between two RDF graphs, as returned by ``Rudof.diff_data``:
/// the triples of each graph that the other one does not have, up to blank
/// node renaming. Triples are in N-Triples, without the final ``.``;
/// ``str(diff)`` is a report with the triples written with the prefixes of
/// the documents.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, from_py_object, name = "GraphDiff", module = "pyrudof._pyrudof")]
#[derive(Clone)]
pub struct PyGraphDiff {
    pub(crate) inner: DataDiff,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyGraphDiff {
    /// The relation between both graphs.
    #[getter]
    fn relation(&self) -> PyGraphRelation {
        self.inner.relation.into()
    }

    /// The mode used to map blank nodes.
    #[getter]
    fn mode(&self) -> PySubgraphMode {
        self.inner.mode.into()
    }

    /// ``True`` if the blank node mapping is proven to be the best one. Always
    /// ``True`` when one graph is contained in the other.
    #[getter]
    fn exact(&self) -> bool {
        self.inner.exact
    }

    /// Triples of the first graph not matched in the second one, sorted.
    #[getter]
    fn only_in_first(&self) -> Vec<String> {
        self.inner.only_in_first.iter().map(ToString::to_string).collect()
    }

    /// Triples of the second graph not matched in the first one, sorted.
    #[getter]
    fn only_in_second(&self) -> Vec<String> {
        self.inner.only_in_second.iter().map(ToString::to_string).collect()
    }

    /// ``(blank node, term)`` pairs: how the blank nodes of the first graph map to
    /// terms of the second one.
    #[getter]
    fn first_to_second(&self) -> Vec<(String, String)> {
        pairs(&self.inner.first_to_second)
    }

    /// ``(blank node, term)`` pairs: how the blank nodes of the second graph map to
    /// terms of the first one.
    #[getter]
    fn second_to_first(&self) -> Vec<(String, String)> {
        pairs(&self.inner.second_to_first)
    }

    /// ``True`` if neither graph has unmatched triples.
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// ``{alias: IRI}``: the prefixes of the documents used to write ``str(diff)``.
    #[getter]
    fn prefixes(&self) -> std::collections::BTreeMap<String, String> {
        self.inner
            .used_prefixes()
            .iter()
            .map(|(alias, iri)| (alias.clone(), iri.to_string()))
            .collect()
    }

    /// The report: the relation, the ``PREFIX`` declarations used, the triples
    /// only in each graph (with prefixed names) and the blank node mapping.
    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    /// The diff as a JSON string.
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.inner.diff).expect("a diff always serializes")
    }

    fn __repr__(&self) -> String {
        format!(
            "GraphDiff(relation={}, only_in_first={}, only_in_second={}, exact={})",
            self.inner.relation,
            self.inner.only_in_first.len(),
            self.inner.only_in_second.len(),
            self.inner.exact
        )
    }
}

fn pairs<B: ToString, T: ToString>(ps: &[(B, T)]) -> Vec<(String, String)> {
    ps.iter().map(|(b, t)| (b.to_string(), t.to_string())).collect()
}
