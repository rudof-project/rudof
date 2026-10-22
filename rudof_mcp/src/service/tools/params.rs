//! Enumerated tool parameters.
//!
//! Each type here is the MCP-facing counterpart of a `rudof_lib` format or mode
//! enum, narrowed to the values this server actually implements.

use rudof_lib::formats::{
    DataFormat, NodeInspectionMode, ResultDataFormat, ResultQueryFormat, ResultShExValidationFormat,
    ResultShaclValidationFormat, ShExFormat, ShExValidationSortByMode, ShaclFormat, ShaclValidationMode,
    ShaclValidationSortByMode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Declare an MCP tool parameter enum wrapping a `rudof_lib` type.
macro_rules! mcp_param {
    (
        $(#[$meta:meta])*
        $param:ident, $lib:ident, { $( $variant:ident => $wire:literal ),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
        pub enum $param {
            $( #[serde(rename = $wire)] $variant ),+
        }

        impl $param {
            /// Every value this parameter accepts.
            pub const ALL: &'static [$param] = &[$( $param::$variant ),+];

            /// The wire value, as it appears in the input schema.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $param::$variant => $wire ),+
                }
            }
        }

        impl Display for $param {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl From<$param> for $lib {
            fn from(param: $param) -> Self {
                param.as_str().parse().unwrap_or_else(|e| {
                    panic!(
                        "{} value '{}' does not parse as {}: {}",
                        stringify!($param),
                        param.as_str(),
                        stringify!($lib),
                        e
                    )
                })
            }
        }
    };
}

mcp_param!(
    /// RDF serialization the data sources are parsed as.
    DataFormatParam,
    DataFormat,
    {
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        JsonLd => "jsonld",
        TriG => "trig",
        NQuads => "nquads",
        N3 => "n3",
    }
);

mcp_param!(
    /// RDF serialization to write the graph in. Images and diagrams are not
    /// here: use `export_image` or `export_plantuml` for those.
    ResultDataFormatParam,
    ResultDataFormat,
    {
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        JsonLd => "jsonld",
        TriG => "trig",
        NQuads => "nquads",
        N3 => "n3",
    }
);

mcp_param!(
    /// Image format to render the graph as.
    ImageFormatParam,
    ResultDataFormat,
    {
        Svg => "svg",
        Png => "png",
    }
);

impl ImageFormatParam {
    /// The MIME type of the rendered image.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Svg => "image/svg+xml",
            Self::Png => "image/png",
        }
    }
}

mcp_param!(
    /// Serialization a ShEx schema is read from. Beyond ShExC and ShExJ these
    /// are the ShExR formats, a ShEx schema encoded as RDF.
    ShExFormatParam,
    ShExFormat,
    {
        ShExC => "shexc",
        ShExJ => "shexj",
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        NQuads => "nquads",
    }
);

mcp_param!(
    /// Serialization a ShEx schema is written in.
    ResultShExFormatParam,
    ShExFormat,
    {
        ShExC => "shexc",
        ShExJ => "shexj",
        Json => "json",
        JsonLd => "jsonld",
        Internal => "internal",
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        TriG => "trig",
        N3 => "n3",
        NQuads => "nquads",
    }
);

mcp_param!(
    /// Serialization a SHACL shapes graph is read from.
    ShaclFormatParam,
    ShaclFormat,
    {
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        JsonLd => "jsonld",
        TriG => "trig",
        NQuads => "nquads",
        N3 => "n3",
    }
);

mcp_param!(
    /// Engine SHACL validation runs on.
    ShaclModeParam,
    ShaclValidationMode,
    {
        Native => "native",
        Sparql => "sparql",
    }
);

mcp_param!(
    /// Arc directions to report for a node.
    NodeModeParam,
    NodeInspectionMode,
    {
        Both => "both",
        Outgoing => "outgoing",
        Incoming => "incoming",
    }
);

mcp_param!(
    /// Serialization a ShEx validation result is written in.
    ResultShExValidationFormatParam,
    ResultShExValidationFormat,
    {
        Details => "details",
        Compact => "compact",
        Json => "json",
        Csv => "csv",
    }
);

mcp_param!(
    /// Serialization a SHACL validation report is written in.
    ResultShaclValidationFormatParam,
    ResultShaclValidationFormat,
    {
        Details => "details",
        Compact => "compact",
        Minimal => "minimal",
        Csv => "csv",
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        TriG => "trig",
        N3 => "n3",
        NQuads => "nquads",
    }
);

mcp_param!(
    /// Order ShEx validation results are sorted in.
    ShExSortByParam,
    ShExValidationSortByMode,
    {
        Node => "node",
        Shape => "shape",
        Status => "status",
        Details => "details",
    }
);

mcp_param!(
    /// Order SHACL validation results are sorted in.
    ShaclSortByParam,
    ShaclValidationSortByMode,
    {
        Severity => "severity",
        Node => "node",
        Component => "component",
        Value => "value",
        Path => "path",
        SourceShape => "sourceshape",
        Details => "details",
    }
);

mcp_param!(
    /// Serialization SPARQL query results are written in.
    ResultQueryFormatParam,
    ResultQueryFormat,
    {
        Internal => "internal",
        Turtle => "turtle",
        NTriples => "ntriples",
        RdfXml => "rdfxml",
        JsonLd => "jsonld",
        TriG => "trig",
        N3 => "n3",
        NQuads => "nquads",
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_every_variant_converts {
        ($param:ident, $lib:ident) => {
            for param in $param::ALL {
                let _: $lib = (*param).into();
            }
        };
    }

    #[test]
    fn every_variant_converts_to_its_library_type() {
        assert_every_variant_converts!(DataFormatParam, DataFormat);
        assert_every_variant_converts!(ResultDataFormatParam, ResultDataFormat);
        assert_every_variant_converts!(ImageFormatParam, ResultDataFormat);
        assert_every_variant_converts!(ShExFormatParam, ShExFormat);
        assert_every_variant_converts!(ResultShExFormatParam, ShExFormat);
        assert_every_variant_converts!(ShaclFormatParam, ShaclFormat);
        assert_every_variant_converts!(ShaclModeParam, ShaclValidationMode);
        assert_every_variant_converts!(NodeModeParam, NodeInspectionMode);
        assert_every_variant_converts!(ResultShExValidationFormatParam, ResultShExValidationFormat);
        assert_every_variant_converts!(ResultShaclValidationFormatParam, ResultShaclValidationFormat);
        assert_every_variant_converts!(ShExSortByParam, ShExValidationSortByMode);
        assert_every_variant_converts!(ShaclSortByParam, ShaclValidationSortByMode);
        assert_every_variant_converts!(ResultQueryFormatParam, ResultQueryFormat);
    }

    #[test]
    fn the_wire_value_is_what_serde_writes() {
        for param in DataFormatParam::ALL {
            let json = serde_json::to_value(param).expect("a parameter should serialize");
            assert_eq!(json, serde_json::json!(param.as_str()));
        }
    }

    #[test]
    fn an_image_format_knows_its_mime_type() {
        assert_eq!(ImageFormatParam::Svg.mime_type(), "image/svg+xml");
        assert_eq!(ImageFormatParam::Png.mime_type(), "image/png");
    }
}
