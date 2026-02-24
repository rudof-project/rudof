use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rudof_rdf::rdf_core::RDFFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ResultDataFormat {
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

impl Display for ResultDataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultDataFormat::Turtle => write!(dest, "turtle"),
            ResultDataFormat::NTriples => write!(dest, "ntriples"),
            ResultDataFormat::RdfXml => write!(dest, "rdfxml"),
            ResultDataFormat::TriG => write!(dest, "trig"),
            ResultDataFormat::N3 => write!(dest, "n3"),
            ResultDataFormat::NQuads => write!(dest, "nquads"),
            ResultDataFormat::Compact => write!(dest, "compact"),
            ResultDataFormat::Json => write!(dest, "json"),
            ResultDataFormat::PlantUML => write!(dest, "plantuml"),
            ResultDataFormat::Svg => write!(dest, "svg"),
            ResultDataFormat::Png => write!(dest, "png"),
        }
    }
}

impl FromStr for ResultDataFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultDataFormat::Turtle),
            "ntriples" => Ok(ResultDataFormat::NTriples),
            "rdfxml" => Ok(ResultDataFormat::RdfXml),
            "trig" => Ok(ResultDataFormat::TriG),
            "n3" => Ok(ResultDataFormat::N3),
            "nquads" => Ok(ResultDataFormat::NQuads),
            "compact" => Ok(ResultDataFormat::Compact),
            "json" => Ok(ResultDataFormat::Json),
            "plantuml" => Ok(ResultDataFormat::PlantUML),
            "svg" => Ok(ResultDataFormat::Svg),
            "png" => Ok(ResultDataFormat::Png),
            _ => Err(format!("Unsupported data format: {}", s)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VisualFormat {
    PlantUML,
    Svg,
    Png,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CheckResultDataFormat {
    RDFFormat(RDFFormat),
    VisualFormat(VisualFormat),
}

impl TryFrom<ResultDataFormat> for CheckResultDataFormat {
    type Error = String;

    fn try_from(rdf_format: ResultDataFormat) -> Result<Self, Self::Error> {
        match rdf_format {
            ResultDataFormat::Turtle => Ok(CheckResultDataFormat::RDFFormat(RDFFormat::Turtle)),
            ResultDataFormat::N3 => Ok(CheckResultDataFormat::RDFFormat(RDFFormat::N3)),
            ResultDataFormat::NTriples => Ok(CheckResultDataFormat::RDFFormat(RDFFormat::NTriples)),
            ResultDataFormat::RdfXml => Ok(CheckResultDataFormat::RDFFormat(RDFFormat::Rdfxml)),
            ResultDataFormat::TriG => Ok(CheckResultDataFormat::RDFFormat(RDFFormat::TriG)),
            ResultDataFormat::NQuads => Ok(CheckResultDataFormat::RDFFormat(RDFFormat::NQuads)),
            ResultDataFormat::PlantUML => Ok(CheckResultDataFormat::VisualFormat(VisualFormat::PlantUML)),
            ResultDataFormat::Svg => Ok(CheckResultDataFormat::VisualFormat(VisualFormat::Svg)),
            ResultDataFormat::Png => Ok(CheckResultDataFormat::VisualFormat(VisualFormat::Png)),
            unsupported => Err(format!("Unsupported result data format (to do): {}", unsupported)),
        }
    }
}
