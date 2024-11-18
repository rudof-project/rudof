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
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;
    type Subject = OxSubject;
    type Iri = OxIri;
    type Term = OxTerm;

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

impl Subject for OxSubject {
    type SubjectRef<'x> = OxSubjectRef<'x>;

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

    fn blank_node(&self) -> Option<OxBlankNode> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(blank_node) => Some(blank_node),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => None,
        }
    }

    fn iri(&self) -> Option<Self::Iri> {
        match self {
            OxSubject::NamedNode(named_node) => Some(named_node),
            OxSubject::BlankNode(_) => None,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => None,
        }
    }

    #[cfg(feature = "rdf-star")]
    fn triple(&self) -> Option<OxTriple> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(_) => None,
            OxSubject::Triple(triple) => Some(triple),
        }
    }
}

impl Iri for OxIri {
    type IriRef<'x> = OxIriRef<'x>;

    fn new(str: &str) -> Self {
        OxIri::new_unchecked(str)
    }

    fn into_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_ref().as_str().to_string())
    }
}

impl Term for OxTerm {
    type TermRef<'x> = OxTermRef<'x>;
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

    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(blank_node) => Some(blank_node.as_ref()),
            OxTerm::Literal(_) => None,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    fn iri(&self) -> Option<OxIriRef<'_>> {
        match self {
            OxTerm::NamedNode(named_node) => Some(named_node.as_ref()),
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(_) => None,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    fn literal(&self) -> Option<OxLiteralRef<'_>> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(literal) => Some(literal.as_ref()),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => None,
        }
    }

    #[cfg(feature = "rdf-star")]
    fn triple(&self) -> Option<OxTripleRef<'_>> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Literal(_) => None,
            OxTerm::Triple(triple) => Some(triple.as_ref().into()),
        }
    }
}

impl Term for OxTermRef<'_> {
    type TermRef<'x> = Self where Self: 'x;
    type BlankNode = OxBlankNodeRef;
    type Iri = OxIriRef;
    type Literal = OxLiteralRef;
    type Triple = OxTripleRef;

    fn is_blank_node(&self) -> bool {
        todo!()
    }

    fn is_iri(&self) -> bool {
        todo!()
    }

    fn is_literal(&self) -> bool {
        todo!()
    }

    fn is_triple(&self) -> bool {
        todo!()
    }

    fn blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        todo!()
    }

    fn iri(&self) -> Option<OxIriRef<'_>> {
        todo!()
    }

    fn literal(&self) -> Option<OxLiteralRef<'_>> {
        todo!()
    }

    fn triple(&self) -> Option<OxTripleRef<'_>> {
        todo!()
    }
}

impl BlankNode for OxBlankNode {
    type BlankNodeRef<'x> = OxBlankNodeRef<'x>;

    fn label(&self) -> &str {
        self.as_str()
    }
}

impl BlankNode for OxBlankNodeRef<'_> {
    type BlankNodeRef<'x> = Self where Self: 'x;

    fn label(&self) -> &str {
        todo!()
    }
}

impl Literal for OxLiteral {
    type LiteralRef<'x> = OxLiteralRef<'x>;

    fn datatype(&self) -> &str {
        self.datatype().as_str()
    }

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

impl Literal for OxLiteralRef<'_> {
    type LiteralRef<'x> = Self where Self: 'x;

    fn datatype(&self) -> &str {
        todo!()
    }

    fn as_bool(&self) -> Option<bool> {
        todo!()
    }

    fn as_string(&self) -> Option<String> {
        todo!()
    }

    fn as_int(&self) -> Option<isize> {
        todo!()
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
