
use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct Lang {
    lang: String
}

impl Lang {
    pub fn new(str: String) -> Lang {
        Lang { lang: str }
    }
}

impl Display for Lang {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.lang)
    }
}
