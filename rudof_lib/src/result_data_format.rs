use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
