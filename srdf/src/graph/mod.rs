use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxIri;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;

use crate::model::rdf_format::RdfFormat;
use crate::model::BlankNode;
use crate::model::Iri;
use crate::model::Literal;
use crate::model::Subject;
use crate::model::Term;
use crate::model::Triple;

pub mod error;
pub mod oxgraph;
// pub mod lang;
// pub mod literal;
// pub mod numeric_literal;
// pub mod object;
pub mod serializer;
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

impl Subject for OxSubject {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    #[cfg(feature = "rdf-star")]
    type Triple = OxTriple;

    fn is_blank_node(&self) -> bool {
        self.is_blank_node()
    }

    fn is_iri(&self) -> bool {
        self.is_named_node()
    }

    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn as_blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(blank_node) => Some(blank_node),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => None,
        }
    }

    fn as_iri(&self) -> Option<&Self::Iri> {
        match self {
            OxSubject::NamedNode(named_node) => Some(named_node),
            OxSubject::BlankNode(_) => None,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => None,
        }
    }

    #[cfg(feature = "rdf-star")]
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

    fn as_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_ref().as_str().to_string())
    }
}

impl Term for OxTerm {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Literal = OxLiteral;
    #[cfg(feature = "rdf-star")]
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

    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn as_blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(blank_node) => Some(blank_node),
            OxTerm::Literal(_) => None,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    fn as_iri(&self) -> Option<&Self::Iri> {
        match self {
            OxTerm::NamedNode(named_node) => Some(named_node),
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(_) => None,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    fn as_literal(&self) -> Option<&OxLiteral> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(literal) => Some(literal),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    #[cfg(feature = "rdf-star")]
    fn as_triple(&self) -> Option<&OxTriple> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(_) => None,
            OxTerm::Triple(triple) => Some(&triple),
        }
    }
}

impl BlankNode for OxBlankNode {
    fn label(&self) -> &str {
        self.as_str()
    }
}

impl Literal for OxLiteral {
    fn as_bool(&self) -> Option<bool> {
        match self.value() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        Some(self.value().to_string())
    }

    fn as_int(&self) -> Option<isize> {
        match self.value().parse() {
            Ok(int) => Some(int),
            Err(_) => None,
        }
    }
}

impl From<RdfFormat> for oxrdfio::RdfFormat {
    fn from(value: RdfFormat) -> Self {
        match value {
            RdfFormat::Turtle => oxrdfio::RdfFormat::Turtle,
            RdfFormat::N3 => oxrdfio::RdfFormat::N3,
            RdfFormat::RdfXml => oxrdfio::RdfFormat::RdfXml,
            RdfFormat::NQuads => oxrdfio::RdfFormat::NQuads,
            RdfFormat::NTriples => oxrdfio::RdfFormat::NTriples,
            RdfFormat::TriG => oxrdfio::RdfFormat::TriG,
        }
    }
}
