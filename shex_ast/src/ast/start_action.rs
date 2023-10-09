use serde_derive::{Deserialize, Serialize};

use super::iri_ref::IriRef;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StartAction {
    #[serde(rename = "type")]
    type_: String,
    name: IriRef,
    code: String,
}
