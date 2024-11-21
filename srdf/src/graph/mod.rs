use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
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
use crate::model::TSubjectRef;
use crate::model::Term;
use crate::model::TermKind;
use crate::model::Triple;

// pub mod oxgraph;
pub mod oxgraph_error;

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
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Triple = OxTriple;

    fn kind(&self) -> SubjectKind {
        match self {
            OxSubject::NamedNode(_) => SubjectKind::Iri,
            OxSubject::BlankNode(_) => SubjectKind::BlankNode,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => SubjectKind::Triple,
        }
    }

    fn into_blank_node(&self) -> Option<&OxBlankNode> {
        if let OxSubject::BlankNode(blank_node) = self {
            Some(blank_node)
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

    fn into_term<T: Triple>(&self) -> T::Term {
        self.into()
    }

    fn try_into_iri<T: Triple>(&self) -> Result<Self::IriRef<'_>, Self::TryIntoError> {
        todo!()
    }
}

impl Subject for OxSubjectRef<'_> {
    type SubjectRef<'x> = Self where Self: 'x;
    type BlankNodeRef<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;
    type TryIntoError;

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
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;

    fn into_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_str().to_string())
    }
}

impl Iri for OxIriRef<'_> {
    type IriRef<'x> = Self where Self: 'x;

    fn into_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_str().to_string())
    }
}

impl Term for OxTerm {
    type TermRef<'x> = OxTermRef<'x> where Self: 'x;
    type BlankNodeRef<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;
    type LiteralRef<'x> = OxLiteralRef<'x> where Self: 'x;
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;
    type TryIntoError;

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

    fn try_into_subject<T: Triple>(&self) -> Result<TSubjectRef<T>, Self::TryIntoError> {
        self.try_into()
    }
}

impl Term for OxTermRef<'_> {
    type TermRef<'x> = Self where Self: 'x;
    type BlankNodeRef<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;
    type LiteralRef<'x> = OxLiteralRef<'x> where Self: 'x;
    #[cfg(feature = "rdf-star")]
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;
    type TryIntoError;

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

    fn try_into_subject<T: Triple>(&self) -> Result<TSubjectRef<T>, Self::TryIntoError> {
        self.try_into()
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
