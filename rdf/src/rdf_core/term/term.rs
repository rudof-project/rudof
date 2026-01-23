use std::{hash::Hash, fmt::{Debug, Display}};

/// Represents any RDF term that can appear in an RDF graph.
///
/// In RDF, terms are the fundamental building blocks of triples. A term can be
/// an IRI (Internationalized Resource Identifier), a blank node, a literal value,
/// or (in RDF-star) a quoted triple.
pub trait Term: Debug + Clone + Display + PartialEq + Eq + Hash {
    /// Returns the kind of RDF term this represents.
    ///
    /// This method allows distinguishing between different types of RDF terms
    /// at runtime.
    fn kind(&self) -> TermKind;

    /// Returns `true` if this term is an IRI.
    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    /// Returns `true` if this term is a blank node.
    fn is_blank_node(&self) -> bool {
        self.kind() == TermKind::BlankNode
    }

    /// Returns `true` if this term is a literal.
    fn is_literal(&self) -> bool {
        self.kind() == TermKind::Literal
    }

    /// Returns `true` if this term is a quoted triple (RDF-star).
    fn is_triple(&self) -> bool {
        self.kind() == TermKind::Triple
    }

    /// Returns the lexical representation of this term as a string.
    fn lexical_form(&self) -> String;
}

/// Represents the kind of RDF term.
#[derive(PartialEq)]
pub enum TermKind {
    Iri,
    BlankNode,
    Literal,
    Triple,
}