use rudof_lib::formats::{DataFormat, DataReaderMode, ResultDataFormat};

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
