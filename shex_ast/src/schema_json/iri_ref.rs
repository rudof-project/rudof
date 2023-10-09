use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq)]
#[serde(try_from = "String")]
pub struct IriRef {
    pub value: String,
}

impl TryFrom<String> for IriRef {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(IriRef { value: s })
    }
}
