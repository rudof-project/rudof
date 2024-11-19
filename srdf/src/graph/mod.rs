use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::BlankNodeRef as OxBlankNodeRef;
use oxrdf::Literal as OxLiteral;
use oxrdf::LiteralRef as OxLiteralRef;
use oxrdf::NamedNode as OxIri;
use oxrdf::NamedNodeRef as OxIriRef;
use oxrdf::Subject as OxSubject;
use oxrdf::SubjectRef as OxSubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::TermRef as OxTermRef;
use oxrdf::Triple as OxTriple;
use oxrdf::TripleRef as OxTripleRef;

use crate::model::BlankNode;
use crate::model::Iri;
use crate::model::Literal;
use crate::model::RdfFormat;
use crate::model::Subject;
use crate::model::SubjectKind;
use crate::model::Term;
use crate::model::TermKind;
use crate::model::Triple;

pub mod oxgraph;
pub mod oxgraph_error;
pub mod serializer;

impl Triple for OxTriple {
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;
    type Subject = OxSubject;
    type Iri = OxIri;
    type Term = OxTerm;

    fn from_spo(subject: Self::Subject, predicate: Self::Iri, object: Self::Term) -> Self {
        OxTriple::new(subject, predicate, object)
    }

    fn subject(&self) -> OxSubjectRef<'_> {
        self.subject.as_ref()
    }

    fn predicate(&self) -> OxIriRef<'_> {
        self.predicate.as_ref()
    }

    fn object(&self) -> OxTermRef<'_> {
        self.object.as_ref()
    }

    fn as_spo(self) -> (Self::Subject, Self::Iri, Self::Term) {
        (self.subject, self.predicate, self.object)
    }
}

impl<'a> Triple for OxTripleRef<'a> {
    type TripleRef<'x> = Self where Self: 'x;
    type Subject = OxSubjectRef<'a>;
    type Iri = OxIriRef<'a>;
    type Term = OxTermRef<'a>;

    fn from_spo(subject: Self::Subject, predicate: Self::Iri, object: Self::Term) -> Self {
        OxTripleRef::new(subject, predicate, object)
    }

    fn subject(&self) -> OxSubjectRef<'a> {
        self.subject
    }

    fn predicate(&self) -> OxIriRef<'a> {
        self.predicate
    }

    fn object(&self) -> OxTermRef<'a> {
        self.object
    }

    fn as_spo(self) -> (Self::Subject, Self::Iri, Self::Term) {
        (self.subject, self.predicate, self.object)
    }
}

impl Subject for OxSubject {
    type SubjectRef<'x> = OxSubjectRef<'x> where Self: 'x;
    type BlankNode<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type Iri<'x> = OxIriRef<'x> where Self: 'x;
    type Triple<'x> = OxTripleRef<'x> where Self: 'x;

    fn kind(&self) -> SubjectKind {
        match self {
            OxSubject::NamedNode(_) => SubjectKind::Iri,
            OxSubject::BlankNode(_) => SubjectKind::BlankNode,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => SubjectKind::Triple,
        }
    }

    fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        if let OxSubject::BlankNode(blank_node) = self {
            Some(blank_node.as_ref())
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<OxIriRef<'_>> {
        if let OxSubject::NamedNode(named_node) = self {
            Some(named_node.as_ref())
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<OxTripleRef<'_>> {
        if let OxSubject::Triple(triple) = self {
            Some(triple.as_ref().into())
        } else {
            None
        }
    }
}

impl Subject for OxSubjectRef<'_> {
    type SubjectRef<'x> = Self where Self: 'x;
    type BlankNode<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type Iri<'x> = OxIriRef<'x> where Self: 'x;
    type Triple<'x> = OxTripleRef<'x> where Self: 'x;

    fn kind(&self) -> SubjectKind {
        match self {
            OxSubjectRef::NamedNode(_) => SubjectKind::Iri,
            OxSubjectRef::BlankNode(_) => SubjectKind::BlankNode,
            #[cfg(feature = "rdf-star")]
            OxSubjectRef::Triple(_) => SubjectKind::Triple,
        }
    }

    fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        if let OxSubjectRef::BlankNode(blank_node) = self {
            Some(*blank_node)
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<OxIriRef<'_>> {
        if let OxSubjectRef::NamedNode(named_node) = self {
            Some(*named_node)
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<OxTripleRef<'_>> {
        if let OxSubjectRef::Triple(triple) = self {
            Some(triple.as_ref().into())
        } else {
            None
        }
    }
}

impl Iri for OxIri {
    type IriRef<'x> = OxIriRef<'x>;

    fn from_str(s: &str) -> Self {
        OxIri::new_unchecked(s)
    }

    fn as_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_str().to_string())
    }
}

impl Iri for OxIriRef<'_> {
    type IriRef<'x> = Self where Self: 'x;

    fn from_str(s: &str) -> OxIriRef<'_> {
        OxIriRef::new_unchecked(s)
    }

    fn as_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_str().to_string())
    }
}

impl Term for OxTerm {
    type TermRef<'x> = OxTermRef<'x> where Self: 'x;
    type BlankNode<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type Iri<'x> = OxIriRef<'x> where Self: 'x;
    type Literal<'x> = OxLiteralRef<'x> where Self: 'x;
    type Triple<'x> = OxTripleRef<'x> where Self: 'x;

    fn kind(&self) -> TermKind {
        match self {
            OxTerm::NamedNode(_) => TermKind::Iri,
            OxTerm::BlankNode(_) => TermKind::BlankNode,
            OxTerm::Literal(_) => TermKind::Literal,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => TermKind::Triple,
        }
    }

    fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        if let OxTerm::BlankNode(blank_node) = self {
            Some(blank_node.as_ref())
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<OxIriRef<'_>> {
        if let OxTerm::NamedNode(named_node) = self {
            Some(named_node.as_ref())
        } else {
            None
        }
    }

    fn into_literal(&self) -> Option<OxLiteralRef<'_>> {
        if let OxTerm::Literal(literal) = self {
            Some(literal.as_ref())
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<OxTripleRef<'_>> {
        if let OxTerm::Triple(triple) = self {
            Some(triple.as_ref().into())
        } else {
            None
        }
    }
}

impl Term for OxTermRef<'_> {
    type TermRef<'x> = Self where Self: 'x;
    type BlankNode<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type Iri<'x> = OxIriRef<'x> where Self: 'x;
    type Literal<'x> = OxLiteralRef<'x> where Self: 'x;
    #[cfg(feature = "rdf-star")]
    type Triple<'x> = OxTripleRef<'x> where Self: 'x    ;

    fn kind(&self) -> TermKind {
        match self {
            OxTermRef::NamedNode(_) => TermKind::Iri,
            OxTermRef::BlankNode(_) => TermKind::BlankNode,
            OxTermRef::Literal(_) => TermKind::Literal,
            #[cfg(feature = "rdf-star")]
            OxTermRef::Triple(_) => TermKind::Triple,
        }
    }

    fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        if let OxTermRef::BlankNode(blank_node) = self {
            Some(*blank_node)
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<OxIriRef<'_>> {
        if let OxTermRef::NamedNode(named_node) = self {
            Some(*named_node)
        } else {
            None
        }
    }

    fn into_literal(&self) -> Option<OxLiteralRef<'_>> {
        if let OxTermRef::Literal(literal) = self {
            Some(*literal)
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<OxTripleRef<'_>> {
        if let OxTermRef::Triple(triple) = self {
            Some(triple.as_ref().into())
        } else {
            None
        }
    }
}

impl BlankNode for OxBlankNode {
    fn id(&self) -> &str {
        self.as_str()
    }
}

impl BlankNode for OxBlankNodeRef<'_> {
    fn id(&self) -> &str {
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

impl Literal for OxLiteralRef<'_> {
    fn datatype(&self) -> &str {
        OxLiteralRef::datatype(*self).as_str()
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
