use crate::EarlError;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::EarlVocab;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// How a test was carried out
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TestMode {
    /// Carried out automatically by a software tool.
    #[default]
    Automatic,
    /// Carried out by human evaluators.
    Manual,
    /// Carried out by a tool, with human judgement involved.
    SemiAuto,
    /// The process is not disclosed.
    Undisclosed,
    /// The process is unknown.
    UnknownMode,
}

impl TestMode {
    /// The `earl:TestMode` instance denoting this mode
    pub fn iri(&self) -> IriS {
        match self {
            TestMode::Automatic => EarlVocab::earl_automatic(),
            TestMode::Manual => EarlVocab::earl_manual(),
            TestMode::SemiAuto => EarlVocab::earl_semi_auto(),
            TestMode::Undisclosed => EarlVocab::earl_undisclosed(),
            TestMode::UnknownMode => EarlVocab::earl_unknown_mode(),
        }
    }
}

impl From<TestMode> for IriS {
    fn from(value: TestMode) -> Self {
        value.iri()
    }
}

impl Display for TestMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestMode::Automatic => write!(dest, "automatic"),
            TestMode::Manual => write!(dest, "manual"),
            TestMode::SemiAuto => write!(dest, "semiAuto"),
            TestMode::Undisclosed => write!(dest, "undisclosed"),
            TestMode::UnknownMode => write!(dest, "unknownMode"),
        }
    }
}

impl FromStr for TestMode {
    type Err = EarlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO - Maybe match the full uri?
        match s.to_lowercase().as_str() {
            "automatic" => Ok(TestMode::Automatic),
            "manual" => Ok(TestMode::Manual),
            "semiauto" => Ok(TestMode::SemiAuto),
            "undisclosed" => Ok(TestMode::Undisclosed),
            "unknownmode" | "unknown" => Ok(TestMode::UnknownMode),
            other => Err(EarlError::UnknownTestMode(other.to_owned())),
        }
    }
}
