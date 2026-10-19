use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// The directional language-tagged string, the initial text direction of its lexical form.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum BaseDirection {
    /// The initial text direction is left-to-right.
    Ltr,
    /// The initial text direction is right-to-left.
    Rtl,
}

impl BaseDirection {
    /// Returns the base direction as it is written after the language tag.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            BaseDirection::Ltr => "ltr",
            BaseDirection::Rtl => "rtl",
        }
    }
}

impl FromStr for BaseDirection {
    type Err = BaseDirectionParseError;

    /// Reads `ltr` or `rtl`, the only two base directions RDF 1.2 defines.
    ///
    /// # Errors
    ///
    /// Returns `BaseDirectionParseError` for anything else.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ltr" => Ok(BaseDirection::Ltr),
            "rtl" => Ok(BaseDirection::Rtl),
            _ => Err(BaseDirectionParseError::InvalidBaseDirection(s.to_string())),
        }
    }
}

impl std::fmt::Display for BaseDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<oxrdf::BaseDirection> for BaseDirection {
    fn from(direction: oxrdf::BaseDirection) -> Self {
        match direction {
            oxrdf::BaseDirection::Ltr => BaseDirection::Ltr,
            oxrdf::BaseDirection::Rtl => BaseDirection::Rtl,
        }
    }
}

impl From<BaseDirection> for oxrdf::BaseDirection {
    fn from(direction: BaseDirection) -> Self {
        match direction {
            BaseDirection::Ltr => oxrdf::BaseDirection::Ltr,
            BaseDirection::Rtl => oxrdf::BaseDirection::Rtl,
        }
    }
}

/// Error type for base direction parsing failures.
#[derive(Error, Debug)]
pub enum BaseDirectionParseError {
    #[error("Invalid base direction: {0}, expected ltr or rtl")]
    InvalidBaseDirection(String),
}
