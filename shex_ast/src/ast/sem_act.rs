use serde_derive::{Deserialize, Serialize};

use super::iri_ref::IriRef;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SemAct {
    name: IriRef,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}
