use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use crate::TermKind;

pub trait Subject: Debug + Display + PartialEq + Clone + Eq + Hash {
    fn kind(&self) -> TermKind;

    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    fn is_blank_node(&self) -> bool {
        self.kind() == TermKind::BlankNode
    }

    fn is_triple(&self) -> bool {
        self.kind() == TermKind::Triple
    }
}
