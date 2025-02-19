use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxIri;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;

use crate::model::BlankNode;
use crate::model::Iri;
use crate::model::Literal;
use crate::model::RdfFormat;
use crate::model::Subject;
use crate::model::Term;
use crate::model::Triple;

pub mod oxgraph;
pub mod oxgraph_error;

impl Triple for OxTriple {
    type Subject = OxSubject;
    type Iri = OxIri;
    type Term = OxTerm;

    fn from_spo(
        subj: impl Into<Self::Subject>,
        pred: impl Into<Self::Iri>,
        obj: impl Into<Self::Term>,
    ) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    fn subject(&self) -> &Self::Subject {
        &self.subject
    }

    fn predicate(&self) -> &Self::Iri {
        &self.predicate
    }

    fn object(&self) -> &Self::Term {
        &self.object
    }

    fn into_spo(self) -> (Self::Subject, Self::Iri, Self::Term) {
        (self.subject, self.predicate, self.object)
    }
}

impl Subject for OxSubject {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    #[cfg(feature = "rdf-star")]
    type Triple = OxTriple;

    fn blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(blank_node) => Some(blank_node),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => None,
        }
    }

    fn iri(&self) -> Option<&Self::Iri> {
        match self {
            OxSubject::NamedNode(named_node) => Some(named_node),
            OxSubject::BlankNode(_) => None,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => None,
        }
    }

    #[cfg(feature = "rdf-star")]
    fn triple(&self) -> Option<&OxTriple> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(_) => None,
            OxSubject::Triple(triple) => Some(&triple),
        }
    }
}

impl Iri for OxIri {
    fn from_str(str: &str) -> Self {
        OxIri::new_unchecked(str)
    }

    fn into_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_ref().as_str().to_string())
    }
}

impl Term for OxTerm {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Literal = OxLiteral;
    #[cfg(feature = "rdf-star")]
    type Triple = OxTriple;

    fn blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(blank_node) => Some(blank_node),
            OxTerm::Literal(_) => None,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    fn iri(&self) -> Option<&Self::Iri> {
        match self {
            OxTerm::NamedNode(named_node) => Some(named_node),
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(_) => None,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    fn literal(&self) -> Option<&OxLiteral> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(literal) => Some(literal),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    #[cfg(feature = "rdf-star")]
    fn triple(&self) -> Option<&OxTriple> {
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
    fn datatype(&self) -> &str {
        self.datatype().as_str()
    }

    fn as_string(&self) -> Option<String> {
        Some(self.value().to_string())
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
