use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum XsFacet {
    StringFacet,
    NumericFacet,
}
