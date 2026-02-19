use rudof_lib::{
    ShapeMapFormat, ShExFormat, ReaderMode, RDFFormat as DataFormat, ShaclValidationMode,
    result_shacl_validation_format::ResultShaclValidationFormat, result_shex_validation_format::ResultShExValidationFormat,
};
use crate::cli::formats::PgSchemaResultFormatCli;
use clap::ValueEnum;
use std::{str::FromStr, fmt::{Display, Formatter}};

#[macro_export]
macro_rules! cli_wrapper {
    // MODE Full: Enum, From, Display, FromStr, MIME
    (
        $cli:ident, 
        $core:ident, 
        { $($variant:ident),* $(,)? }, 
        { $($from:ident => $to:expr),* $(,)? },
        { $($str_in:literal => $str_out:ident),* $(,)? },
        { $($mime_var:ident => $mime_val:expr),* $(,)? }
    ) => {
        // Reuse Mode 3 (Base + FromStr)
        cli_wrapper!($cli, $core, { $($variant),* }, { $($from => $to),* }, { $($str_in => $str_out),* });

        impl $cli {
            /// Returns the officially recognized MIME type for this format.
            pub fn mime_type(&self) -> &'static str {
                match self {
                    $( $cli::$mime_var => $mime_val, )*
                }
            }
        }
    };

    // MODE Intermediate: Enum, From, Display, FromStr
    (
        $cli:ident, 
        $core:ident, 
        { $($variant:ident),* $(,)? }, 
        { $($from:ident => $to:expr),* $(,)? },
        { $($str_in:literal => $str_out:ident),* $(,)? }
    ) => {
        // Reuse Mode 2 (Base)
        cli_wrapper!($cli, $core, { $($variant),* }, { $($from => $to),* });

        /// Case-insensitive parsing from string for CLI arguments.
        impl FromStr for $cli {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    $( $str_in => Ok($cli::$str_out), )*
                    _ => Err(format!("Unknown format: {}", s)),
                }
            }
        }
    };

    // MODE Minimal: Enum, From, Display
    (
        $cli:ident, 
        $core:ident, 
        { $($variant:ident),* $(,)? }, 
        { $($from:ident => $to:expr),* $(,)? }
    ) => {
        #[derive(ValueEnum, Debug, Clone, PartialEq)]
        #[clap(rename_all = "lower")]
        pub enum $cli {
            $( $variant ),*
        }

        /// Map the CLI-specific enum back to the core library type.
        impl From<&$cli> for $core {
            fn from(format: &$cli) -> Self {
                match format {
                    $( $cli::$from => $to ),*
                }
            }
        }

        /// Standard display using the variant name in lowercase.
        impl std::fmt::Display for $cli {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( $cli::$variant => write!(f, "{}", stringify!($variant).to_lowercase()) ),*
                }
            }
        }
    };
}

// --- Implementations ---

// ShapeMapFormat
cli_wrapper!(
    ShapeMapFormatCli,
    ShapeMapFormat,
    { Compact, Internal, Json, Details, Csv },
    { 
        Compact  => ShapeMapFormat::Compact,
        Internal => ShapeMapFormat::Json,
        Json     => ShapeMapFormat::Json,
        Details  => ShapeMapFormat::Compact,
        Csv      => ShapeMapFormat::Csv
    },
    { "compact" => Compact, "internal" => Internal, "json" => Json, "details" => Details, "csv" => Csv },
    {
        Compact  => "text/shex-compact",
        Internal => "application/json",
        Json     => "application/json",
        Details  => "text/shex-details",
        Csv      => "text/csv"
    }
);

// ShExFormat
cli_wrapper!(
    ShExFormatCli,
    ShExFormat,
    { ShExC, ShExJ, Json, JsonLd },
    { 
        ShExC  => ShExFormat::ShExC,
        ShExJ  => ShExFormat::ShExJ,
        Json   => ShExFormat::ShExJ,
        JsonLd => ShExFormat::ShExJ,
    },
    { "shexc" => ShExC, "shexj" => ShExJ, "json" => Json, "jsonld" => JsonLd },
    {
        ShExC  => "text/shex",
        ShExJ  => "application/json",
        Json   => "application/json",
        JsonLd => "application/ld+json"
    }
);

// ReaderMode
cli_wrapper!(
    ReaderModeCli,
    ReaderMode,
    { Lax, Strict },
    { 
        Lax    => ReaderMode::Lax,
        Strict => ReaderMode::Strict,
    }
);

// DataFormat
cli_wrapper!( 
    DataFormatCli,
    DataFormat,
    { Turtle, NTriples, RdfXml, TriG, NQuads, JsonLd },
    { 
        Turtle   => DataFormat::Turtle,
        NTriples => DataFormat::NTriples,
        RdfXml   => DataFormat::Rdfxml,
        TriG     => DataFormat::TriG,
        NQuads   => DataFormat::NQuads,
        JsonLd   => DataFormat::JsonLd,
    },
    { 
        "turtle" => Turtle, 
        "ntriples" => NTriples, 
        "rdfxml" => RdfXml, 
        "trig" => TriG, 
        "nquads" => NQuads, 
        "jsonld" => JsonLd 
    },
    {
        Turtle  => "text/turtle",
        NTriples => "application/n-triples",
        RdfXml   => "application/rdf+xml",
        TriG     => "application/trig",
        NQuads   => "application/n-quads",
        JsonLd   => "application/ld+json"
    }
);

// ShaclValidationMode
cli_wrapper!(
    ShaclValidationModeCli,
    ShaclValidationMode,
    { Native, Sparql },
    { 
        Native => ShaclValidationMode::Native,
        Sparql => ShaclValidationMode::Sparql,
    },
    { 
        "native" => Native, 
        "sparql" => Sparql 
    }
);

// PgSchemaResultFormat, ResultShExValidationFormat, ResultShaclValidationFormat
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultValidationFormatCli {
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    Compact,
    Details,
    Json,
    Csv,
}

impl ResultValidationFormatCli {
    pub fn to_shex_result_format(self) -> ResultShExValidationFormat {
        match self {
            ResultValidationFormatCli::Turtle => ResultShExValidationFormat::Turtle,
            ResultValidationFormatCli::NTriples => ResultShExValidationFormat::NTriples,
            ResultValidationFormatCli::RdfXml => ResultShExValidationFormat::RdfXml,
            ResultValidationFormatCli::TriG => ResultShExValidationFormat::TriG,
            ResultValidationFormatCli::N3 => ResultShExValidationFormat::N3,
            ResultValidationFormatCli::NQuads => ResultShExValidationFormat::NQuads,
            ResultValidationFormatCli::Compact => ResultShExValidationFormat::Compact,
            ResultValidationFormatCli::Details => ResultShExValidationFormat::Details,
            ResultValidationFormatCli::Json => ResultShExValidationFormat::Json,
            ResultValidationFormatCli::Csv => ResultShExValidationFormat::Csv,
        }
    }

    pub fn to_shacl_result_format(self) -> ResultShaclValidationFormat {
        match self {
            ResultValidationFormatCli::Turtle => ResultShaclValidationFormat::Turtle,
            ResultValidationFormatCli::NTriples => ResultShaclValidationFormat::NTriples,
            ResultValidationFormatCli::RdfXml => ResultShaclValidationFormat::RdfXml,
            ResultValidationFormatCli::TriG => ResultShaclValidationFormat::TriG,
            ResultValidationFormatCli::N3 => ResultShaclValidationFormat::N3,
            ResultValidationFormatCli::NQuads => ResultShaclValidationFormat::NQuads,
            ResultValidationFormatCli::Compact => ResultShaclValidationFormat::Compact,
            ResultValidationFormatCli::Details => ResultShaclValidationFormat::Details,
            ResultValidationFormatCli::Json => ResultShaclValidationFormat::Json,
            ResultValidationFormatCli::Csv => ResultShaclValidationFormat::Csv,
        }
    }

    pub fn to_pgschema_result_format(self) -> PgSchemaResultFormatCli {
        match self {
            ResultValidationFormatCli::Compact => PgSchemaResultFormatCli::Compact,
            ResultValidationFormatCli::Details => PgSchemaResultFormatCli::Details,
            ResultValidationFormatCli::Json => PgSchemaResultFormatCli::Json,
            ResultValidationFormatCli::Csv => PgSchemaResultFormatCli::Csv,
            _ => todo!(),
        }
    }
}

impl Display for ResultValidationFormatCli {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultValidationFormatCli::Turtle => write!(dest, "turtle"),
            ResultValidationFormatCli::NTriples => write!(dest, "ntriples"),
            ResultValidationFormatCli::RdfXml => write!(dest, "rdfxml"),
            ResultValidationFormatCli::TriG => write!(dest, "trig"),
            ResultValidationFormatCli::N3 => write!(dest, "n3"),
            ResultValidationFormatCli::NQuads => write!(dest, "nquads"),
            ResultValidationFormatCli::Compact => write!(dest, "compact"),
            ResultValidationFormatCli::Json => write!(dest, "json"),
            ResultValidationFormatCli::Details => write!(dest, "details"),
            ResultValidationFormatCli::Csv => write!(dest, "csv"),
        }
    }
}