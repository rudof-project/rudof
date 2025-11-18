use clap::ValueEnum;
use shacl_validation::validation_report::report::SortModeReport;
use srdf::RDFFormat;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::RudofError;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ResultShaclValidationFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Minimal,
    Compact,
    #[default]
    Details,
    Json,
}

impl Display for ResultShaclValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultShaclValidationFormat::Turtle => write!(dest, "turtle"),
            ResultShaclValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultShaclValidationFormat::RDFXML => write!(dest, "rdfxml"),
            ResultShaclValidationFormat::TriG => write!(dest, "trig"),
            ResultShaclValidationFormat::N3 => write!(dest, "n3"),
            ResultShaclValidationFormat::NQuads => write!(dest, "nquads"),
            ResultShaclValidationFormat::Compact => write!(dest, "compact"),
            ResultShaclValidationFormat::Minimal => write!(dest, "minimal"),
            ResultShaclValidationFormat::Details => write!(dest, "details"),
            ResultShaclValidationFormat::Json => write!(dest, "json"),
        }
    }
}

pub fn result_format_to_rdf_format(result_format: &ResultShaclValidationFormat) -> Result<RDFFormat, RudofError> {
    match result_format {
        ResultShaclValidationFormat::Turtle => Ok(RDFFormat::Turtle),
        ResultShaclValidationFormat::NTriples => Ok(RDFFormat::NTriples),
        ResultShaclValidationFormat::RDFXML => Ok(RDFFormat::RDFXML),
        ResultShaclValidationFormat::TriG => Ok(RDFFormat::TriG),
        ResultShaclValidationFormat::N3 => Ok(RDFFormat::N3),
        ResultShaclValidationFormat::NQuads => Ok(RDFFormat::NQuads),
        _ => Err(RudofError::NotImplemented { msg: format!("Unsupported result format {}", result_format) }),
    }
}

impl FromStr for ResultShaclValidationFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultShaclValidationFormat::Turtle),
            "ntriples" => Ok(ResultShaclValidationFormat::NTriples),
            "rdfxml" => Ok(ResultShaclValidationFormat::RDFXML),
            "trig" => Ok(ResultShaclValidationFormat::TriG),
            "n3" => Ok(ResultShaclValidationFormat::N3),
            "nquads" => Ok(ResultShaclValidationFormat::NQuads),
            "minimal" => Ok(ResultShaclValidationFormat::Minimal),
            "compact" => Ok(ResultShaclValidationFormat::Compact),
            "details" => Ok(ResultShaclValidationFormat::Details),
            "json" => Ok(ResultShaclValidationFormat::Json),
            other => Err(format!("Unsupported result SHACL validation format: {other}")),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum SortByShaclValidationReport {
    #[default]
    Severity,
    Node,
    Component,
    Value,
    Path,
    SourceShape,
    Details,
}

impl Display for SortByShaclValidationReport {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SortByShaclValidationReport::Severity => write!(dest, "severity"),
            SortByShaclValidationReport::Node => write!(dest, "node"),
            SortByShaclValidationReport::Component => write!(dest, "component"),
            SortByShaclValidationReport::Value => write!(dest, "value"),
            SortByShaclValidationReport::Path => write!(dest, "path"),
            SortByShaclValidationReport::SourceShape => write!(dest, "sourceShape"),
            SortByShaclValidationReport::Details => write!(dest, "details"),
        }
    }
}

pub fn cnv_sort_mode_report(sort_by: &SortByShaclValidationReport) -> SortModeReport {
    match sort_by {
        SortByShaclValidationReport::Severity => SortModeReport::Severity,
        SortByShaclValidationReport::Node => SortModeReport::Node,
        SortByShaclValidationReport::Component => SortModeReport::Component,
        SortByShaclValidationReport::Value => SortModeReport::Value,
        SortByShaclValidationReport::Path => SortModeReport::Path,
        SortByShaclValidationReport::SourceShape => SortModeReport::Source,
        SortByShaclValidationReport::Details => SortModeReport::Details,
    }
}

impl std::str::FromStr for SortByShaclValidationReport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "severity" => Ok(SortByShaclValidationReport::Severity),
            "node" => Ok(SortByShaclValidationReport::Node),
            "component" => Ok(SortByShaclValidationReport::Component),
            "value" => Ok(SortByShaclValidationReport::Value),
            "path" => Ok(SortByShaclValidationReport::Path),
            "sourceshape" => Ok(SortByShaclValidationReport::SourceShape),
            "details" => Ok(SortByShaclValidationReport::Details),
            other => Err(format!("Unsupported sort mode for SHACL validation report: {other}")),
        }
    }
}
