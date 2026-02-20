use clap::ValueEnum;
use rudof_lib::{
    ShaclValidationMode,
    compare::{InputCompareFormat, InputCompareMode, ResultCompareFormat},
    convert::{InputConvertFormat, InputConvertMode, OutputConvertFormat, OutputConvertMode},
    data_format::DataFormat,
    dctap_format::DCTapFormat,
    dctap_result_format::DCTapResultFormat,
    generate_schema_format::GenerateSchemaFormat,
    pgschema_format::PgSchemaFormat,
    query_result_format::ResultQueryFormat,
    query_type::QueryType,
    rdf_config::{RdfConfigFormat, RdfConfigResultFormat},
    rdf_reader_mode::RDFReaderMode,
    result_data_format::ResultDataFormat,
    result_service_format::ResultServiceFormat,
    result_shacl_validation_format::{ResultShaclValidationFormat, SortByShaclValidationReport},
    result_shex_validation_format::ResultShExValidationFormat,
    result_validation_format::ResultValidationFormat,
    shacl_format::ShaclFormat,
    shapemap_format::ShapeMapFormat,
    shex_format::ShExFormat,
    show_node_mode::ShowNodeMode,
    sort_by::SortByValidate,
    sort_by_result_shape_map::SortByResultShapeMap,
    validation_mode::ValidationMode,
};
use std::fmt::{Display, Formatter, Result};

/// CLI wrapper macro for rudof_lib types.
///
/// This macro creates a CLI-friendly enum wrapper around a core library type,
/// adding `clap::ValueEnum` support while delegating all logic to the lib type.
///
/// # What it generates:
/// - An enum with `#[derive(ValueEnum)]` for CLI parsing
/// - `From<&CliType> -> LibType` conversion using Display + FromStr
/// - `Display` implementation that delegates to the lib type
///
/// # Requirements:
/// The core library type must implement:
/// - `FromStr` (for parsing from strings)
/// - `Display` (for converting to strings)
///
/// # Syntax:
/// ```no_run
/// cli_wrapper!(CliTypeName, LibTypeName, { Variant1, Variant2, ... });
/// ```
///
/// # Example:
/// ```no_run
/// cli_wrapper!(
///     ShaclFormatCli,
///     ShaclFormat,
///     { Internal, Turtle, NTriples, RdfXml }
/// );
/// ```
#[macro_export]
macro_rules! cli_wrapper {
    (
        $cli:ident,      // CLI enum name (e.g., ShaclFormatCli)
        $core:ident,     // Core lib type name (e.g., ShaclFormat)
        { $($variant:ident),* $(,)? }  // Enum variants
    ) => {
        /// CLI wrapper enum with clap::ValueEnum support.
        #[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
        #[clap(rename_all = "lower")]
        pub enum $cli {
            $( $variant ),*
        }

        /// Convert CLI enum to core library type.
        ///
        /// Uses Display -> FromStr round-trip to ensure consistency.
        /// Panics if variant names don't match between CLI and lib enums.
        impl From<&$cli> for $core {
            fn from(cli: &$cli) -> Self {
                // Convert CLI enum to string using its Display impl
                let s = cli.to_string();

                // Parse string into core lib type using its FromStr impl
                s.parse().unwrap_or_else(|e| {
                    panic!(
                        "CLI enum variant {:?} doesn't match lib enum: {:?}",
                        cli, e
                    )
                })
            }
        }

        /// Display implementation that delegates to the core library type.
        ///
        /// This ensures consistent string representation between CLI and lib.
        impl Display for $cli {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                // Convert to core type and use its Display
                let core_value: $core = self.into();
                write!(f, "{}", core_value)
            }
        }
    };
}

// CLI wrapper for rudof_lib::shapemap_format::ShapeMapFormat.
cli_wrapper!(
    ShapeMapFormatCli,
    ShapeMapFormat,
    {
        Compact,
        Internal,
        Json,
        Details,
        Csv
    }
);

// CLI wrapper for rudof_lib::shex_format::ShExFormat.
cli_wrapper!(
    ShExFormatCli,
    ShExFormat,
    {
        Internal,
        Simple,
        ShExC,
        ShExJ,
        Json,
        JsonLd,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads
    }
);

// CLI wrapper for rudof_lib::rdf_reader_mode::RDFReaderMode.
cli_wrapper!(
    RDFReaderModeCli,
    RDFReaderMode,
    {
        Lax,
        Strict
    }
);

// CLI wrapper for rudof_lib::pgschema_format::PgSchemaFormat.
cli_wrapper!(PgSchemaFormatCli, PgSchemaFormat, { PgSchemaC });

// CLI wrapper for rudof_lib::validation_mode::ValidationMode.
cli_wrapper!(
    ValidationModeCli,
    ValidationMode,
    {
        ShEx,
        Shacl,
        PGSchema
    }
);

// CLI wrapper for rudof_lib::sort_by::SortByValidate.
cli_wrapper!(
    SortByValidateCli,
    SortByValidate,
    {
        Node,
        Details
    }
);

// CLI wrapper for rudof_lib::data_format::DataFormat.
cli_wrapper!(
    DataFormatCli,
    DataFormat,
    {
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        JsonLd,
        Pg
    }
);

// CLI wrapper for rudof_lib::ShaclValidationMode.
cli_wrapper!(
    ShaclValidationModeCli,
    ShaclValidationMode,
    {
        Native,
        Sparql
    }
);

// CLI wrapper for rudof_lib::result_validation_format::ResultValidationFormat.
cli_wrapper!(
    ResultValidationFormatCli,
    ResultValidationFormat,
    {
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Compact,
        Details,
        Json,
        Csv
    }
);

// CLI wrapper for rudof_lib::sort_by_result_shape_map::SortByResultShapeMap.
cli_wrapper!(
    SortByResultShapeMapCli,
    SortByResultShapeMap,
    {
        Node,
        Shape,
        Status,
        Details
    }
);

// CLI wrapper for rudof_lib::result_shex_validation_format::ResultShExValidationFormat.
cli_wrapper!(
    ResultShExValidationFormatCli,
    ResultShExValidationFormat,
    {
        Details,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Compact,
        Json,
        Csv,
    }
);

// CLI wrapper for rudof_lib::shacl_format::ShaclFormat.
cli_wrapper!(
    ShaclFormatCli,
    ShaclFormat,
    {
        Internal,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        JsonLd,
    }
);

// CLI wrapper for rudof_lib::result_shacl_validation_format::ResultShaclValidationFormat.
cli_wrapper!(
    ResultShaclValidationFormatCli,
    ResultShaclValidationFormat,
    {
        Details,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Minimal,
        Compact,
        Json,
        Csv,
    }
);

// CLI wrapper for rudof_lib::result_shacl_validation_format::SortByShaclValidationReport.
cli_wrapper!(
    SortByShaclValidationReportCli,
    SortByShaclValidationReport,
    {
        Severity,
        Node,
        Component,
        Value,
        Path,
        SourceShape,
        Details,
    }
);

// CLI wrapper for rudof_lib::result_data_format::ResultDataFormat.
cli_wrapper!(
    ResultDataFormatCli,
    ResultDataFormat,
    {
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Compact,
        Json,
        PlantUML,
        Svg,
        Png,
    }
);

// CLI wrapper for rudof_lib::show_node_mode::ShowNodeMode.
cli_wrapper!(
    ShowNodeModeCli,
    ShowNodeMode,
    {
        Outgoing,
        Incoming,
        Both,
    }
);

// CLI wrapper for rudof_lib::dctap_format::DCTapFormat.
cli_wrapper!(
    DCTapFormatCli,
    DCTapFormat,
    {
        Csv,
        Xlsx,
        Xlsb,
        Xlsm,
        Xls,
    }
);

// CLI wrapper for rudof_lib::dctap_result_format::DCTapResultFormat.
cli_wrapper!(
    DCTapResultFormatCli,
    DCTapResultFormat,
    {
        Internal,
        Json,
    }
);

// CLI wrapper for rudof_lib::convert::InputConvertMode.
cli_wrapper!(
    InputConvertModeCli,
    InputConvertMode,
    {
        Shacl,
        ShEx,
        Dctap,
    }
);

// CLI wrapper for rudof_lib::convert::InputConvertFormat.
cli_wrapper!(
    InputConvertFormatCli,
    InputConvertFormat,
    {
        Csv,
        Xlsx,
        ShExC,
        ShExJ,
        Turtle,
    }
);

// CLI wrapper for rudof_lib::convert::OutputConvertFormat.
cli_wrapper!(
    OutputConvertFormatCli,
    OutputConvertFormat,
    {
        Default,
        Internal,
        Json,
        ShExC,
        ShExJ,
        Turtle,
        PlantUML,
        Html,
        Svg,
        Png,
    }
);

// CLI wrapper for rudof_lib::convert::OutputConvertMode.
cli_wrapper!(
    OutputConvertModeCli,
    OutputConvertMode,
    {
        Sparql,
        ShEx,
        Uml,
        Html,
        Shacl,
    }
);

// CLI wrapper for rudof_lib::compare::InputCompareMode.
cli_wrapper!(
    InputCompareModeCli,
    InputCompareMode,
    {
        Shacl,
        ShEx,
        Dctap,
        Service,
    }
);

// CLI wrapper for rudof_lib::compare::InputCompareFormat.
cli_wrapper!(
    InputCompareFormatCli,
    InputCompareFormat,
    {
        ShExC,
        ShExJ,
        Turtle,
        RdfXml,
        NTriples,
    }
);

// CLI wrapper for rudof_lib::compare::ResultCompareFormat.
cli_wrapper!(
    ResultCompareFormatCli,
    ResultCompareFormat,
    {
        Internal,
        Json,
    }
);

// CLI wrapper for rudof_lib::rdf_config::RdfConfigFormat.
cli_wrapper!(RdfConfigFormatCli, RdfConfigFormat, { Yaml });

// CLI wrapper for rudof_lib::rdf_config::RdfConfigResultFormat.
cli_wrapper!(
    RdfConfigResultFormatCli,
    RdfConfigResultFormat,
    {
        Internal,
        Yaml,
    }
);

// CLI wrapper for rudof_lib::result_service_format::ResultServiceFormat.
cli_wrapper!(
    ResultServiceFormatCli,
    ResultServiceFormat,
    {
        Internal,
        Mie,
        Json,
    }
);

// CLI wrapper for rudof_lib::query_result_format::ResultQueryFormat.
cli_wrapper!(
    ResultQueryFormatCli,
    ResultQueryFormat,
    {
        Internal,
        Turtle,
        NTriples,
        JsonLd,
        RdfXml,
        Csv,
        TriG,
        N3,
        NQuads,
    }
);

// CLI wrapper for rudof_lib::query_type::QueryType.
cli_wrapper!(
    QueryTypeCli,
    QueryType,
    {
        Select,
        Construct,
        Ask,
        Describe,
    }
);

// CLI wrapper for rudof_lib::generate_schema_format::GenerateSchemaFormat.
cli_wrapper!(
    GenerateSchemaFormatCli,
    GenerateSchemaFormat,
    {
        Auto,
        ShEx,
        Shacl,
    }
);
