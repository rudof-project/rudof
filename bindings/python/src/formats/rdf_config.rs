use rudof_lib::formats::{RdfConfigFormat, ResultRdfConfigFormat};

pyenum! {
    /// rdf-config input formats.
    "RdfConfigFormat": PyRdfConfigFormat => RdfConfigFormat, default = Yaml, from_str {
        /// YAML - the rdf-config specification format.
        Yaml => Yaml,
    }
}

pyenum! {
    /// rdf-config output formats.
    "ResultRdfConfigFormat": PyResultRdfConfigFormat => ResultRdfConfigFormat,
    default = Internal, from_str {
        /// Internal representation used for processing.
        Internal => Internal,
        /// YAML - the rdf-config specification format.
        Yaml     => Yaml,
    }
}
