use std::fmt::Display;

use iri_s::IriS;
use prefixmap::PrefixMap;

pub struct TriplePattern {
    subj: Var,
    pred: IriS,
    obj: Var,
}

impl TriplePattern {
    pub fn show_qualified(&self, prefixmap: PrefixMap) -> String {
        let pred = prefixmap.qualify(&self.pred);
        format!("{} {} {} .", self.subj, pred, self.obj)
    }
}

pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Var {
        Var {
            name: name.to_string(),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.name)
    }
}

pub struct SelectQuery {
    prefixmap: PrefixMap,
    base_iri: Option<IriS>,
    patterns: Vec<TriplePattern>,
}

impl SelectQuery {
    pub fn new() -> SelectQuery {
        SelectQuery {
            prefixmap: PrefixMap::new(),
            base_iri: None,
            patterns: Vec::new(),
        }
    }
}

impl Display for SelectQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
