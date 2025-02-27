use serde::{Deserialize, Serialize};

use prefixmap::IriRef;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StartAction {
    #[serde(rename = "type")]
    type_: String,
    name: IriRef,
    code: String,
}
