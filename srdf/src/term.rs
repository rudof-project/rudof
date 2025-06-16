use crate::TermKind;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

pub trait Term: Debug + Clone + Display + PartialEq + Eq + Hash {
    fn kind(&self) -> TermKind;

    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    fn is_blank_node(&self) -> bool {
        self.kind() == TermKind::BlankNode
    }

    fn is_literal(&self) -> bool {
        self.kind() == TermKind::Literal
    }

    fn is_triple(&self) -> bool {
        self.kind() == TermKind::Triple
    }

    fn lexical_form(&self) -> String;
}
