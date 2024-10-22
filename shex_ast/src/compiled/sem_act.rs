use iri_s::IriS;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct SemAct {
    name: IriS,
    code: Option<String>,
}
