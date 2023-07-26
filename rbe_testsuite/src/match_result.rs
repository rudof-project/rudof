use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MatchResult {
  Pass,
  Fail
}

impl Default for MatchResult {
  fn default() -> Self { 
    MatchResult::Pass
  }
}
