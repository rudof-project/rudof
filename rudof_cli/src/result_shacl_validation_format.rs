use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultShaclValidationFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Compact,
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
            ResultShaclValidationFormat::Json => write!(dest, "json"),
        }
    }
}
