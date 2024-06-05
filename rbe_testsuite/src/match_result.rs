use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum MatchResult {
    #[default]
    Pass,
    Fail,
}
