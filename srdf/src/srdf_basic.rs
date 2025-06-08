use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::Subject as OxSubject;
use oxrdf::SubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;
use rust_decimal::Decimal;

use crate::lang::Lang;
use crate::literal::Literal as SRDFLiteral;
use crate::matcher::Matcher;
use crate::Object;
use crate::RDFError;

pub trait Rdf: Sized {
    type Subject: Subject
        + From<Self::IRI>
        + From<Self::BNode>
        + From<IriS>
        + TryFrom<Self::Term>
        + TryFrom<Object>
        + Matcher<Self::Subject>;

    type IRI: Iri + From<IriS> + TryFrom<Self::Term> + Matcher<Self::IRI>;

    type Term: Term
        + From<Self::Subject>
        + From<Self::IRI>
        + From<Self::BNode>
        + From<Self::Literal>
        + From<IriS>
        + From<Object>
        + TryInto<Object>
        + Matcher<Self::Term>
        + PartialEq;

    type BNode: BlankNode + TryFrom<Self::Term>;

    type Literal: Literal
        + From<bool>
        + From<String>
        + From<i128>
        + From<f64>
        + TryFrom<Self::Term>
        + From<SRDFLiteral>;

    type Triple: Triple<Self::Subject, Self::IRI, Self::Term>;

    type Err: Display;

    fn qualify_iri(&self, iri: &Self::IRI) -> String;
    fn qualify_subject(&self, subj: &Self::Subject) -> String;
    fn qualify_term(&self, term: &Self::Term) -> String;

    fn prefixmap(&self) -> Option<PrefixMap>;

    /// Resolves a a prefix and a local name and obtains the corresponding full `IriS`
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;

    fn numeric_value(&self, term: &Self::Term) -> Option<Decimal> {
        let maybe_object: Result<Object, _> = term.clone().try_into();
        match maybe_object {
            Ok(object) => object.numeric_value().map(|n| n.as_decimal()),
            Err(_) => None,
        }
    }

    fn term_as_literal(&self, term: &Self::Term) -> Result<Self::Literal, RDFError> {
        <Self::Term as TryInto<Self::Literal>>::try_into(term.clone())
            .map_err(|_| RDFError::ConversionError(format!("Converting term to literal: {term}")))
    }

    fn term_as_subject(&self, term: &Self::Term) -> Result<Self::Subject, RDFError> {
        <Self::Term as TryInto<Self::Subject>>::try_into(term.clone())
            .map_err(|_| RDFError::ConversionError(format!("Converting term to subject: {term}")))
    }

    fn term_as_iri(&self, term: &Self::Term) -> Result<Self::IRI, RDFError> {
        <Self::Term as TryInto<Self::IRI>>::try_into(term.clone())
            .map_err(|_| RDFError::ConversionError(format!("Converting term to iri: {term}")))
    }

    /// The comparison should be compatible to SPARQL comparison:
    /// https://www.w3.org/TR/sparql11-query/#OperatorMapping

    fn compare(&self, term1: &Self::Term, term2: &Self::Term) -> Result<Ordering, RDFError> {
        // TODO: At this moment we convert the terms to object and perform the comparison within objects
        // This requires to clone but we should be able to optimize this later
        let obj1: Object = term1.clone().try_into().map_err(|e| {
            RDFError::ConversionError(format!("Converting term to object: {term1}"))
        })?;
        let obj2: Object = term2.clone().try_into().map_err(|e| {
            RDFError::ConversionError(format!("Converting term to object: {term2}"))
        })?;
        obj1.partial_cmp(&obj2)
            .ok_or_else(|| RDFError::ComparisonError {
                term1: term1.lexical_form(),
                term2: term2.lexical_form(),
            })
    }

    /// Checks if the first term is equals to the second term
    /// This equality should be based on the euqlity defined for SPARQL
    /// https://www.w3.org/TR/sparql11-query/#OperatorMapping
    fn equals(&self, term1: &Self::Term, term2: &Self::Term) -> bool {
        term1 == term2
    }
}

#[derive(PartialEq)]
pub enum TermKind {
    Iri,
    BlankNode,
    Literal,
    Triple,
}

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

impl Subject for OxSubject {
    fn kind(&self) -> TermKind {
        match self {
            OxSubject::NamedNode(_) => TermKind::Iri,
            OxSubject::BlankNode(_) => TermKind::BlankNode,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => TermKind::Triple,
        }
    }
}

impl Subject for SubjectRef<'_> {
    fn kind(&self) -> TermKind {
        match self {
            SubjectRef::NamedNode(_) => TermKind::Iri,
            SubjectRef::BlankNode(_) => TermKind::BlankNode,
            #[cfg(feature = "rdf-star")]
            SubjectRef::Triple(_) => TermKind::Triple,
        }
    }
}

impl Matcher<OxSubject> for OxSubject {
    fn value(&self) -> Option<OxSubject> {
        Some(self.clone())
    }
}

pub trait Iri: Debug + Display + Hash + Eq + Ord + Clone {
    fn as_str(&self) -> &str;
}

impl Iri for OxNamedNode {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl Matcher<OxNamedNode> for OxNamedNode {
    fn value(&self) -> Option<OxNamedNode> {
        Some(self.clone())
    }
}

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

impl Term for OxTerm {
    fn kind(&self) -> TermKind {
        match self {
            OxTerm::NamedNode(_) => TermKind::Iri,
            OxTerm::BlankNode(_) => TermKind::BlankNode,
            OxTerm::Literal(_) => TermKind::Literal,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => TermKind::Triple,
        }
    }
    fn lexical_form(&self) -> String {
        match self {
            OxTerm::NamedNode(iri) => iri.as_str().to_string(),
            OxTerm::BlankNode(bnode) => bnode.as_str().to_string(),
            OxTerm::Literal(literal) => literal.value().to_string(),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(triple) => triple.to_string(),
        }
    }
}

impl Matcher<OxTerm> for OxTerm {
    fn value(&self) -> Option<OxTerm> {
        Some(self.clone())
    }
}

pub trait Literal: Debug + Clone + Display + PartialEq + Eq + Hash {
    fn lexical_form(&self) -> &str;

    fn lang(&self) -> Option<&str>;

    fn datatype(&self) -> &str;

    fn as_bool(&self) -> Option<bool> {
        match self.lexical_form() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    fn as_integer(&self) -> Option<isize> {
        self.lexical_form().parse().ok()
    }

    fn as_double(&self) -> Option<f64> {
        self.lexical_form().parse().ok()
    }

    fn as_decimal(&self) -> Option<Decimal> {
        self.lexical_form().parse().ok()
    }

    fn as_literal(&self) -> SRDFLiteral {
        if let Some(bool) = self.as_bool() {
            SRDFLiteral::boolean(bool)
        } else if let Some(int) = self.as_integer() {
            SRDFLiteral::integer(int)
        } else if let Some(decimal) = self.as_double() {
            SRDFLiteral::double(decimal)
        } else if let Some(decimal) = self.as_decimal() {
            SRDFLiteral::decimal(decimal)
        } else if let Some(lang) = self.lang() {
            SRDFLiteral::lang_str(self.lexical_form(), Lang::new_unchecked(lang))
        } else {
            SRDFLiteral::str(self.lexical_form())
        }
    }
}

impl Literal for OxLiteral {
    fn lexical_form(&self) -> &str {
        self.value()
    }

    fn lang(&self) -> Option<&str> {
        self.language()
    }

    fn datatype(&self) -> &str {
        self.datatype().as_str()
    }
}

pub trait BlankNode: Debug + Display + PartialEq {
    fn new(id: impl Into<String>) -> Self;
    fn id(&self) -> &str;
}

impl BlankNode for OxBlankNode {
    fn new(id: impl Into<String>) -> Self {
        OxBlankNode::new_unchecked(id)
    }

    fn id(&self) -> &str {
        self.as_str()
    }
}

pub trait Triple<S, P, O>: Debug + Clone + Display
where
    S: Subject,
    P: Iri,
    O: Term,
{
    fn new(subj: impl Into<S>, pred: impl Into<P>, obj: impl Into<O>) -> Self;

    fn subj(&self) -> S;
    fn pred(&self) -> P;
    fn obj(&self) -> O;

    fn into_components(self) -> (S, P, O);

    fn into_subject(self) -> S {
        self.into_components().0
    }

    fn into_predicate(self) -> P {
        self.into_components().1
    }

    fn into_object(self) -> O {
        self.into_components().2
    }
}

impl Triple<OxSubject, OxNamedNode, OxTerm> for OxTriple {
    fn new(
        subj: impl Into<OxSubject>,
        pred: impl Into<OxNamedNode>,
        obj: impl Into<OxTerm>,
    ) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    fn subj(&self) -> OxSubject {
        self.subject.clone()
    }

    fn pred(&self) -> OxNamedNode {
        self.predicate.clone()
    }

    fn obj(&self) -> OxTerm {
        self.object.clone()
    }

    fn into_components(self) -> (OxSubject, OxNamedNode, OxTerm) {
        (self.subject, self.predicate, self.object)
    }
}
