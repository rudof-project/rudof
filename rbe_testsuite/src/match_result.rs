use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum MatchResult {
    #[default]
    Pass,
    Fail,
}
