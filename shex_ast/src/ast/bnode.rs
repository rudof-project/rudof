use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq)]
#[serde(try_from = "String")]
pub struct BNode {
    value: String,
}

impl TryFrom<String> for BNode {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(BNode { value: s })
    }
}
