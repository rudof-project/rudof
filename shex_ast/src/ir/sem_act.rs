use iri_s::IriS;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct SemAct {
    name: IriS,
    code: Option<String>,
}

impl SemAct {
    pub fn new(name: IriS, code: Option<String>) -> Self {
        SemAct { name, code }
    }

    pub fn name(&self) -> &IriS {
        &self.name
    }

    pub fn code(&self) -> Option<&String> {
        self.code.as_ref()
    }
}
