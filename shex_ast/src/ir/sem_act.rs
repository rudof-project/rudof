use std::fmt::{self, Display};

use rudof_iri::IriS;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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

impl Display for SemAct {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(
            dest,
            "SemAct(name: {}, code: {})",
            self.name,
            self.code.as_deref().unwrap_or("None")
        )
    }
}
