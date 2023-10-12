use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub struct BNode {
    value: String,
}

impl TryFrom<&str> for BNode {
    type Error = Void;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(BNode {
            value: s.to_string(),
        })
    }
}

impl Into<String> for BNode {
    fn into(self) -> String {
        self.value
    }
}
