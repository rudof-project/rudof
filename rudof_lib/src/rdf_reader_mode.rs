use rudof_rdf::rdf_impl::ReaderMode;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub enum RDFReaderMode {
    Lax,

    #[default]
    Strict,
}

impl From<RDFReaderMode> for ReaderMode {
    fn from(format: RDFReaderMode) -> Self {
        match format {
            RDFReaderMode::Strict => ReaderMode::Strict,
            RDFReaderMode::Lax => ReaderMode::Lax,
        }
    }
}

impl From<&RDFReaderMode> for ReaderMode {
    fn from(format: &RDFReaderMode) -> Self {
        (*format).into()
    }
}

impl Display for RDFReaderMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            RDFReaderMode::Strict => write!(dest, "strict"),
            RDFReaderMode::Lax => write!(dest, "lax"),
        }
    }
}

impl FromStr for RDFReaderMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strict" => Ok(RDFReaderMode::Strict),
            "lax" => Ok(RDFReaderMode::Lax),
            other => Err(format!("Unknown reader mode: {}", other)),
        }
    }
}
