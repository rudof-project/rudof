//! How data held by remote SPARQL endpoints is fetched during validation.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// How the validator fetches data from SPARQL endpoints. Has no effect on
/// data held in memory.
///
/// Either way, the validator never fetches the whole endpoint (data in memory
/// is indexed by class up front, which would mean asking an endpoint for every
/// triple), and keeps every answer it gets, so that the same lookup is not
/// sent twice.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FetchStrategy {
    /// Fetch the data of each focus node when the validation needs it: about
    /// one request per focus node and path.
    #[default]
    OnDemand,
    /// Fetch the data shape by shape before validating, many focus nodes per
    /// query, following *Validating SHACL constraints over a SPARQL endpoint*
    /// (Corman et al., ISWC 2019). See [`crate::validator::prefetch`].
    ByShape,
}

impl Display for FetchStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchStrategy::OnDemand => write!(f, "on-demand"),
            FetchStrategy::ByShape => write!(f, "by-shape"),
        }
    }
}

impl FromStr for FetchStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "on-demand" | "ondemand" => Ok(FetchStrategy::OnDemand),
            "by-shape" | "byshape" => Ok(FetchStrategy::ByShape),
            other => Err(format!(
                "unknown fetch strategy '{other}': expected 'on-demand' or 'by-shape'"
            )),
        }
    }
}
