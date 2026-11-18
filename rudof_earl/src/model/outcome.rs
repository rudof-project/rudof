use std::fmt::{Display, Formatter};
use std::str::FromStr;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::EarlVocab;
use crate::EarlError;

/// The outcome of carrying out a test
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Outcome {
    /// The subject passed the test.
    Passed,
    /// The subject failed the test.
    #[default]
    Failed,
    /// It is unclear whether the subject passed or failed.
    CantTell,
    /// The test is not applicable to the subject.
    Inapplicable,
    /// The test has not been carried out.
    Untested,
}

impl Outcome {
    /// The `earl:OutcomeValue` instance denoting this outcome
    pub fn iri(&self) -> IriS {
        match self {
            Outcome::Passed => EarlVocab::earl_passed(),
            Outcome::Failed => EarlVocab::earl_failed(),
            Outcome::CantTell => EarlVocab::earl_cant_tell(),
            Outcome::Inapplicable => EarlVocab::earl_inapplicable(),
            Outcome::Untested => EarlVocab::earl_untested(),
        }
    }

    /// The class this outcome's instance belongs to
    pub fn class_iri(&self) -> IriS {
        match self {
            Outcome::Passed => EarlVocab::earl_pass(),
            Outcome::Failed => EarlVocab::earl_fail(),
            Outcome::CantTell => EarlVocab::earl_cannot_tell(),
            Outcome::Inapplicable => EarlVocab::earl_not_applicable(),
            Outcome::Untested => EarlVocab::earl_not_tested(),
        }
    }
}

impl Display for Outcome {
    fn fmt(&self, dest: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Outcome::Passed => write!(dest, "passed"),
            Outcome::Failed => write!(dest, "failed"),
            Outcome::CantTell => write!(dest, "cantTell"),
            Outcome::Inapplicable => write!(dest, "inapplicable"),
            Outcome::Untested => write!(dest, "untested"),
        }
    }
}

impl From<Outcome> for IriS {
    fn from(value: Outcome) -> Self {
        value.iri()
    }
}

impl FromStr for Outcome {
    type Err = EarlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO - Maybe match the full uri
        match s.to_lowercase().as_str() {
            "passed" => Ok(Outcome::Passed),
            "failed" => Ok(Outcome::Failed),
            "canttell" => Ok(Outcome::CantTell),
            "inapplicable" => Ok(Outcome::Inapplicable),
            "untested" => Ok(Outcome::Untested),
            other => Err(EarlError::UnknownOutcome(other.to_string())),
        }
    }
}
