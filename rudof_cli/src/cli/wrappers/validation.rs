use crate::cli_wrapper;
use rudof_lib_refactored::formats::{
    ValidationMode, ValidationSortByMode, ShaclValidationMode, ResultShaclValidationFormat,
    ShaclValidationSortByMode, ShExValidationSortByMode, ResultShExValidationFormat, ResultValidationFormat,
    ResultPgSchemaValidationFormat
};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

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
    ValidationSortByModeCli,
    ValidationSortByMode,
    {
        Node,
        Details
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
    ShaclValidationSortByModeCli,
    ShaclValidationSortByMode,
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
    ShExValidationSortByModeCli,
    ShExValidationSortByMode,
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

cli_wrapper!(ResultPgSchemaValidationFormatCli, ResultPgSchemaValidationFormat, { Compact, Details, Json, Csv });