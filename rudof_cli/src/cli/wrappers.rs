use clap::ValueEnum;
use rudof_lib::{
    ShaclValidationMode,
    compare::{InputCompareFormat, InputCompareMode, ResultCompareFormat},
    convert::{InputConvertFormat, InputConvertMode, OutputConvertFormat, OutputConvertMode},
    data_format::DataFormat,
    dctap_format::DCTapFormat,
    dctap_result_format::DCTapResultFormat,
    generate_schema_format::GenerateSchemaFormat,
    pgschema_format::{PgSchemaFormat, PgSchemaResultFormat},
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
use std::{fmt::{Display, Formatter, Result}};
use iri_s::MimeType;
use anyhow::{Result as AnyhowResult, bail};

/// CLI wrapper macro for rudof_lib types.
///
/// This macro creates a CLI-friendly enum wrapper around a core library type,
/// adding `clap::ValueEnum` support while delegating all logic to the lib type.
///
/// # What it generates:
/// - An enum with `#[derive(ValueEnum)]` for CLI parsing
/// - `From<&CliType> -> LibType` conversion using Display + FromStr
/// - `Display` implementation that delegates to the lib type
/// - Optional `MimeType` implementation (if `with_mime_type` is specified)
///
/// # Requirements:
/// The core library type must implement:
/// - `FromStr` (for parsing from strings)
/// - `Display` (for converting to strings)
/// - `MimeType` (if using `with_mime_type`)
///
/// # Syntax:
/// ```no_run
/// // Without MimeType
/// cli_wrapper!(CliTypeName, LibTypeName, { Variant1, Variant2, ... });
///
/// // With MimeType
/// cli_wrapper!(CliTypeName, LibTypeName, { Variant1, Variant2, ... }, with_mime_type);
/// ```
///
/// # Example:
/// ```no_run
/// // Regular wrapper
/// cli_wrapper!(
///     ShaclFormatCli,
///     ShaclFormat,
///     { Internal, Turtle, NTriples, RdfXml }
/// );
///
/// // With MimeType support
/// cli_wrapper!(
///     InputCompareFormatCli,
///     InputCompareFormat,
///     { ShExC, ShExJ, Turtle, RdfXml, NTriples },
///     with_mime_type
/// );
/// ```
#[macro_export]
macro_rules! cli_wrapper {
    // Version WITHOUT MimeType
    (
        $cli:ident,
        $core:ident,
        { $($variant:ident),* $(,)? }
    ) => {
        /// CLI wrapper enum with clap::ValueEnum support.
        #[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
        #[clap(rename_all = "lower")]
        pub enum $cli {
            $( $variant ),*
        }

        /// Convert CLI enum to core library type.
        impl From<&$cli> for $core {
            fn from(cli: &$cli) -> Self {
                let s = cli.to_string();
                s.parse().unwrap_or_else(|e| {
                    panic!(
                        "CLI enum variant {:?} doesn't match lib enum: {:?}",
                        cli, e
                    )
                })
            }
        }

        /// Display implementation that delegates to the core library type.
        impl Display for $cli {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let core_value: $core = self.into();
                write!(f, "{}", core_value)
            }
        }
    };

    // Version WITH MimeType
    (
        $cli:ident,
        $core:ident,
        { $($variant:ident),* $(,)? },
        with_mime_type
    ) => {
        /// CLI wrapper enum with clap::ValueEnum support.
        #[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
        #[clap(rename_all = "lower")]
        pub enum $cli {
            $( $variant ),*
        }

        /// Convert CLI enum to core library type.
        impl From<&$cli> for $core {
            fn from(cli: &$cli) -> Self {
                let s = cli.to_string();
                s.parse().unwrap_or_else(|e| {
                    panic!(
                        "CLI enum variant {:?} doesn't match lib enum: {:?}",
                        cli, e
                    )
                })
            }
        }

        /// Display implementation that delegates to the core library type.
        impl Display for $cli {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let core_value: $core = self.into();
                write!(f, "{}", core_value)
            }
        }

        /// MimeType implementation that delegates to the core library type.
        impl MimeType for $cli {
            fn mime_type(&self) -> &'static str {
                let core_value: $core = self.into();
                core_value.mime_type()
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
    },
    with_mime_type
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

// CLI wrapper for rudof_lib::pgschema_format::PgSchemaResultFormat.
cli_wrapper!(PgSchemaResultFormatCli, PgSchemaResultFormat, { Compact, Details, Json, Csv });

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
    },
    with_mime_type
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
    },
    with_mime_type
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

// Convert SortByValidateCli -> SortByResultShapeMapCli
impl From<SortByValidateCli> for SortByResultShapeMapCli {
    fn from(val: SortByValidateCli) -> Self {
        match val {
            SortByValidateCli::Node => SortByResultShapeMapCli::Node,
            SortByValidateCli::Details => SortByResultShapeMapCli::Details,
        }
    }
}

// Convert SortByValidateCli -> SortByShaclValidationReportCli
impl From<SortByValidateCli> for SortByShaclValidationReportCli {
    fn from(val: SortByValidateCli) -> Self {
        match val {
            SortByValidateCli::Node => SortByShaclValidationReportCli::Node,
            SortByValidateCli::Details => SortByShaclValidationReportCli::Details,
        }
    }
}

// Convert ResultValidationFormatCli -> ResultShExValidationFormatCli
impl From<ResultValidationFormatCli> for ResultShExValidationFormatCli {
    fn from(val: ResultValidationFormatCli) -> Self {
        match val {
            ResultValidationFormatCli::Turtle => ResultShExValidationFormatCli::Turtle,
            ResultValidationFormatCli::NTriples => ResultShExValidationFormatCli::NTriples,
            ResultValidationFormatCli::RdfXml => ResultShExValidationFormatCli::RdfXml,
            ResultValidationFormatCli::TriG => ResultShExValidationFormatCli::TriG,
            ResultValidationFormatCli::N3 => ResultShExValidationFormatCli::N3,
            ResultValidationFormatCli::NQuads => ResultShExValidationFormatCli::NQuads,
            ResultValidationFormatCli::Compact => ResultShExValidationFormatCli::Compact,
            ResultValidationFormatCli::Details => ResultShExValidationFormatCli::Details,
            ResultValidationFormatCli::Json => ResultShExValidationFormatCli::Json,
            ResultValidationFormatCli::Csv => ResultShExValidationFormatCli::Csv,
        }
    }
}

// Convert ResultValidationFormatCli -> ResultShaclValidationFormatCli
impl From<ResultValidationFormatCli> for ResultShaclValidationFormatCli {
    fn from(val: ResultValidationFormatCli) -> Self {
        match val {
            ResultValidationFormatCli::Turtle => ResultShaclValidationFormatCli::Turtle,
            ResultValidationFormatCli::NTriples => ResultShaclValidationFormatCli::NTriples,
            ResultValidationFormatCli::RdfXml => ResultShaclValidationFormatCli::RdfXml,
            ResultValidationFormatCli::TriG => ResultShaclValidationFormatCli::TriG,
            ResultValidationFormatCli::N3 => ResultShaclValidationFormatCli::N3,
            ResultValidationFormatCli::NQuads => ResultShaclValidationFormatCli::NQuads,
            ResultValidationFormatCli::Compact => ResultShaclValidationFormatCli::Compact,
            ResultValidationFormatCli::Details => ResultShaclValidationFormatCli::Details,
            ResultValidationFormatCli::Json => ResultShaclValidationFormatCli::Json,
            ResultValidationFormatCli::Csv => ResultShaclValidationFormatCli::Csv,
        }
    }
}

impl TryFrom<ShExFormatCli> for ShaclFormatCli {
    type Error = anyhow::Error;

    fn try_from(val: ShExFormatCli) -> AnyhowResult<Self> {
        match val {
            ShExFormatCli::Internal => Ok(ShaclFormatCli::Internal),
            ShExFormatCli::Turtle => Ok(ShaclFormatCli::Turtle),
            ShExFormatCli::NTriples => Ok(ShaclFormatCli::NTriples),
            ShExFormatCli::RdfXml => Ok(ShaclFormatCli::RdfXml),
            ShExFormatCli::TriG => Ok(ShaclFormatCli::TriG),
            ShExFormatCli::N3 => Ok(ShaclFormatCli::N3),
            ShExFormatCli::NQuads => Ok(ShaclFormatCli::NQuads),
            ShExFormatCli::JsonLd => Ok(ShaclFormatCli::JsonLd),
            ShExFormatCli::Simple => bail!("Validation using SHACL mode doesn't support Simple format"),
            ShExFormatCli::ShExC => bail!("Validation using SHACL mode doesn't support ShExC format"),
            ShExFormatCli::ShExJ => bail!("Validation using SHACL mode doesn't support ShExJ format"),
            ShExFormatCli::Json => Ok(ShaclFormatCli::JsonLd),
        }
    }
}

// Convert ResultValidationFormatCli -> PgSchemaResultFormatCli
impl TryFrom<ResultValidationFormatCli> for PgSchemaResultFormatCli {
    type Error = anyhow::Error;

    fn try_from(val: ResultValidationFormatCli) -> AnyhowResult<Self, Self::Error> {
        match val {
            ResultValidationFormatCli::Compact => Ok(PgSchemaResultFormatCli::Compact),
            ResultValidationFormatCli::Details => Ok(PgSchemaResultFormatCli::Details),
            ResultValidationFormatCli::Json => Ok(PgSchemaResultFormatCli::Json),
            ResultValidationFormatCli::Csv => Ok(PgSchemaResultFormatCli::Csv),
            ResultValidationFormatCli::Turtle => todo!("PGSchema validation doesn't support Turtle result format"),
            ResultValidationFormatCli::NTriples => todo!("PGSchema validation doesn't support NTriples result format"),
            ResultValidationFormatCli::RdfXml => todo!("PGSchema validation doesn't support RDF/XML result format"),
            ResultValidationFormatCli::TriG => todo!("PGSchema validation doesn't support TriG result format"),
            ResultValidationFormatCli::N3 => todo!("PGSchema validation doesn't support N3 result format"),
            ResultValidationFormatCli::NQuads => todo!("PGSchema validation doesn't support NQuads result format"),
        }
    }
}

// Convert InputConvertFormatCli -> ShexFormatCli
impl TryFrom<InputConvertFormatCli> for ShExFormatCli {
    type Error = anyhow::Error;

    fn try_from(val: InputConvertFormatCli) -> AnyhowResult<Self, Self::Error> {
        match val {
            InputConvertFormatCli::ShExC => Ok(ShExFormatCli::ShExC),
            InputConvertFormatCli::ShExJ => Ok(ShExFormatCli::ShExJ),
            InputConvertFormatCli::Turtle => Ok(ShExFormatCli::Turtle),
            _ => bail!("The specified input format {:?} cannot be converted to a ShEx format", val),       
        }
    }
}

// Convert OutputConvertFormatCli -> ShexFormatCli
impl TryFrom<OutputConvertFormatCli> for ShExFormatCli {
    type Error = anyhow::Error;

    fn try_from(val: OutputConvertFormatCli) -> AnyhowResult<Self, Self::Error> {
        match val {
            OutputConvertFormatCli::ShExC => Ok(ShExFormatCli::ShExC),
            OutputConvertFormatCli::ShExJ => Ok(ShExFormatCli::ShExJ),
            OutputConvertFormatCli::Turtle => Ok(ShExFormatCli::Turtle),
            _ => bail!("The specified output format {:?} cannot be converted to a ShEx format", val),       
        }
    }
}

