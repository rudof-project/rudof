use rudof_lib::formats::{DataFormat, DataReaderMode, ResultDataFormat};
use rudof_lib::types::{GraphRelation, SubgraphMode};

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
