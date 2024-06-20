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
    prefixmap: Option<PrefixMap>,
    base: Option<IriS>,
    patterns: Vec<TriplePattern>,
}

impl SelectQuery {
    pub fn new() -> SelectQuery {
        SelectQuery {
            prefixmap: None,
            base: None,
            patterns: Vec::new(),
        }
    }

    pub fn with_prefixmap(mut self, prefixmap: Option<PrefixMap>) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    pub fn with_patterns(mut self, patterns: Vec<TriplePattern>) -> Self {
        self.patterns = patterns;
        self
    }
}

impl Display for SelectQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
