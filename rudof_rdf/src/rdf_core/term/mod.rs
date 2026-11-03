mod blank_node;
mod iri;
mod iri_or_blanknode;
pub mod literal;
mod parse;
#[allow(clippy::module_inception)]
mod term;
mod triple;

pub use blank_node::{BlankNode, BlankNodeRef, ConcreteBlankNode};
pub use iri::Iri;
pub use iri_or_blanknode::IriOrBlankNode;
pub use parse::{TermParseError, parse_term};
pub use term::{Term, TermKind};
pub use triple::{ConcreteTriple, Object, Subject, Triple};

#[cfg(test)]
mod tests {
    mod parse_tests;
    mod schema_tests;
}
