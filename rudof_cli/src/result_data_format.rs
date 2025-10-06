use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultDataFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Compact,
    Json,
    PlantUML,
    SVG,
    PNG,
}

impl Display for ResultDataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultDataFormat::Turtle => write!(dest, "turtle"),
            ResultDataFormat::NTriples => write!(dest, "ntriples"),
            ResultDataFormat::RDFXML => write!(dest, "rdfxml"),
            ResultDataFormat::TriG => write!(dest, "trig"),
            ResultDataFormat::N3 => write!(dest, "n3"),
            ResultDataFormat::NQuads => write!(dest, "nquads"),
            ResultDataFormat::Compact => write!(dest, "compact"),
            ResultDataFormat::Json => write!(dest, "json"),
            ResultDataFormat::PlantUML => write!(dest, "plantuml"),
            ResultDataFormat::SVG => write!(dest, "svg"),
            ResultDataFormat::PNG => write!(dest, "png"),
        }
    }
}
