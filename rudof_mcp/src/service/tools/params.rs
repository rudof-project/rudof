//! Enumerated tool parameters.
//!
//! Each type here is the MCP-facing counterpart of a `rudof_lib` format or mode
//! enum, narrowed to the values this server actually implements.

use rudof_lib::formats::{DataFormat, ResultDataFormat};
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
