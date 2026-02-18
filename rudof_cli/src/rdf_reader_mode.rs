use clap::ValueEnum;
use rudof_rdf::rdf_impl::ReaderMode;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
#[clap(rename_all = "lower")]
pub enum RDFReaderMode {
    Lax,

    #[default]
    Strict,
}

impl From<&RDFReaderMode> for ReaderMode {
    fn from(value: &RDFReaderMode) -> Self {
        match value {
            RDFReaderMode::Strict => ReaderMode::Strict,
            RDFReaderMode::Lax => ReaderMode::Lax,
        }
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
