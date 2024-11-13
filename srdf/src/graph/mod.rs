use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxIri;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;

use crate::model::BlankNode;
use crate::model::Iri;
use crate::model::Subject;
use crate::model::Term;
use crate::model::Triple;

pub mod error;
pub mod graph;
// pub mod lang;
// pub mod literal;
// pub mod numeric_literal;
// pub mod object;
pub mod serializer;
// TODO: move to shacl_ast
pub mod shacl_path;
// pub mod subject;
// pub mod triple;

impl Triple for OxTriple {
    type Subject = OxSubject;
    type Iri = OxIri;
    type Term = OxTerm;

    fn new(subj: Self::Subject, pred: Self::Iri, obj: Self::Term) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    fn subj(&self) -> &Self::Subject {
        &self.subject
    }

    fn pred(&self) -> &Self::Iri {
        &self.predicate
    }

    fn obj(&self) -> &Self::Term {
        &self.object
    }
}

impl Display for OxTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.subj(), self.pred(), self.obj())
    }
}

impl Subject for OxSubject {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Triple = OxTriple;

    fn is_blank_node(&self) -> bool {
        self.is_blank_node()
    }

    fn is_iri(&self) -> bool {
        self.is_named_node()
    }

    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn as_blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(blank_node) => Some(blank_node),
            OxSubject::Triple(_) => None,
        }
    }

    fn as_iri(&self) -> Option<&Self::Iri> {
        match self {
            OxSubject::NamedNode(named_node) => Some(named_node),
            OxSubject::BlankNode(_) => None,
            OxSubject::Triple(_) => None,
        }
    }

    fn as_triple(&self) -> Option<&OxTriple> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(_) => None,
            OxSubject::Triple(triple) => Some(&triple),
        }
    }
}

impl Iri for OxIri {
    fn new(str: &str) -> Self {
        OxIri::new_unchecked(str)
    }
}

impl Term for OxTerm {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Literal = OxLiteral;
    type Triple = OxTriple;

    fn is_blank_node(&self) -> bool {
        self.is_blank_node()
    }

    fn is_iri(&self) -> bool {
        self.is_named_node()
    }

    fn is_literal(&self) -> bool {
        self.is_literal()
    }

    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn as_blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(blank_node) => Some(blank_node),
            OxTerm::Triple(_) => None,
            OxTerm::Literal(_) => None,
        }
    }

    fn as_iri(&self) -> Option<&Self::Iri> {
        match self {
            OxTerm::NamedNode(named_node) => Some(named_node),
            OxTerm::BlankNode(_) => None,
            OxTerm::Triple(_) => None,
            OxTerm::Literal(_) => None,
        }
    }

    fn as_literal(&self) -> Option<&OxLiteral> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Triple(_) => None,
            OxTerm::Literal(literal) => Some(literal),
        }
    }

    fn as_triple(&self) -> Option<&OxTriple> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Triple(triple) => Some(&triple),
            OxTerm::Literal(_) => None,
        }
    }
}

impl BlankNode for OxBlankNode {
    fn label(&self) -> &str {
        self.as_str()
    }
}
