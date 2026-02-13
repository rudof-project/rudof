use crate::rdf_core::{
    Matcher, RDFError,
    term::{
        BlankNode, Iri, IriOrBlankNode, Object, Subject, Term, Triple,
        literal::{ConcreteLiteral, Lang, Literal},
    },
};
use iri_s::IriS;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::fmt::Display;

/// A trait representing an RDF graph implementation with its associated types and operations.
///
/// This trait defines the core interface for working with RDF data structures, including
/// subjects, predicates, objects, literals, and triples. It provides type-safe conversions
/// between different RDF components and utility methods for common RDF operations.
///
/// # Associated Types
///
/// Implementors must define concrete types for all RDF components:
/// - `Subject`: Resources that can appear as triple subjects
/// - `IRI`: Internationalized Resource Identifiers
/// - `Term`: Any RDF term (IRIs, literals, blank nodes, or triples)
/// - `BNode`: Blank nodes (anonymous resources)
/// - `Literal`: Literal values (strings, numbers, dates, etc.)
/// - `Triple`: RDF statements (subject-predicate-object)
/// - `Err`: Error type for operations that can fail
pub trait Rdf: Sized {
    /// The subject type for this RDF implementation.
    type Subject: Subject
        + From<Self::IRI>
        + From<Self::BNode>
        + From<IriS>
        + From<IriOrBlankNode>
        + TryFrom<Self::Term>
        + TryInto<IriOrBlankNode>
        + TryFrom<Object>
        + Matcher<Self::Subject>;

    /// The IRI type for this RDF implementation.
    type IRI: Iri + From<IriS> + TryFrom<Self::Term> + Matcher<Self::IRI> + Into<IriS>;

    /// The term type representing any RDF component.
    type Term: Term
        + From<Self::Subject>
        + From<Self::IRI>
        + From<Self::BNode>
        + From<Self::Literal>
        + From<Self::Triple>
        + From<IriS>
        + From<Object>
        + TryInto<Object>
        + Matcher<Self::Term>
        + PartialEq;

    /// The blank node type for this RDF implementation.
    type BNode: BlankNode + TryFrom<Self::Term>;

    /// The literal type for representing data values.
    type Literal: Literal
        + From<bool>
        + From<String>
        + From<i128>
        + From<f64>
        + TryFrom<Self::Term>
        + From<ConcreteLiteral>
        + TryInto<ConcreteLiteral>;

    /// The triple type representing RDF statements.
    type Triple: Triple<Self::Subject, Self::IRI, Self::Term>;

    /// The error type for fallible operations.
    type Err: Display;

    /// Returns the prefixed name corresponding to an IRI.
    ///
    /// Converts a full IRI to its shortened form using registered namespace prefixes.
    ///
    /// # Parameters
    ///
    /// * `iri` - The IRI to qualify
    fn qualify_iri(&self, iri: &Self::IRI) -> String;

    /// Returns the prefixed representation of a subject.
    ///
    /// Converts a subject to its qualified string form, applying prefix mappings
    /// if the subject is an IRI.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject to qualify
    fn qualify_subject(&self, subj: &Self::Subject) -> String;

    /// Returns the prefixed representation of a term.
    ///
    /// Converts a term to its qualified string form, applying prefix mappings
    /// where applicable.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to qualify
    fn qualify_term(&self, term: &Self::Term) -> String;

    /// Returns the prefix map used by this RDF implementation.
    ///
    /// Returns `None` if no prefix map is configured.
    fn prefixmap(&self) -> Option<PrefixMap>;

    /// Resolves a prefix and local name to obtain the full IRI.
    ///
    /// Combines a namespace prefix with a local name to produce the complete IRI
    ///
    /// # Parameters
    ///
    /// * `prefix` - The namespace prefix
    /// * `local` - The local name
    ///
    /// # Errors
    ///
    /// Returns `PrefixMapError` if the prefix is not registered in the prefix map.
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;

    /// Extracts the numeric value from a term, if it represents a number.
    ///
    /// Attempts to convert the term to a literal and extract its numeric value
    /// as a `Decimal`. Returns `None` if the term is not a numeric literal.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to extract the numeric value from
    fn numeric_value(&self, term: &Self::Term) -> Option<Decimal> {
        let maybe_object: Result<Object, _> = term.clone().try_into();
        match maybe_object {
            Ok(object) => object.numeric_value().map(|n| n.to_decimal().unwrap()),
            Err(_) => None,
        }
    }

    /// Converts a term to a literal.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsLiteral` if the term is not a literal.
    fn term_as_literal(term: &Self::Term) -> Result<Self::Literal, RDFError> {
        Self::Literal::try_from(term.clone()).map_err(|_| RDFError::TermAsLiteral {
            term: term.to_string(),
        })
    }

    /// Attempts to convert a term into a concrete literal.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    /// # Errors
    ///
    /// Returns `RDFError::TermAsLiteral` if the term cannot be converted into a literal.
    /// Returns `RDFError::LiteralAsSLiteral` if the resulting literal cannot be converted into a concrete literal.
    fn term_as_sliteral(term: &Self::Term) -> Result<ConcreteLiteral, RDFError> {
        let lit = <Self::Term as TryInto<Self::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsLiteral {
                term: term.to_string(),
            }
        })?;
        let slit =
            <Self::Literal as TryInto<ConcreteLiteral>>::try_into(lit.clone()).map_err(|_| {
                RDFError::LiteralAsSLiteral {
                    literal: lit.to_string(),
                }
            })?;
        Ok(slit)
    }

    /// Converts a term to a subject.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsSubject` if the term cannot be used as a subject
    fn term_as_subject(term: &Self::Term) -> Result<Self::Subject, RDFError> {
        Self::Subject::try_from(term.clone()).map_err(|_| RDFError::TermAsSubject {
            term: term.to_string(),
        })
    }

    /// Converts a subject to a term.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject to convert
    fn subject_as_term(subj: &Self::Subject) -> Self::Term {
        subj.clone().into()
    }

    /// Converts a triple to a term (RDF-star support).
    ///
    /// In RDF-star, triples can be used as terms in other triples.
    ///
    /// # Parameters
    ///
    /// * `triple` - The triple to convert
    fn triple_as_term(triple: &Self::Triple) -> Self::Term {
        Self::Term::from(triple.clone())
    }

    /// Converts an `IriS` to a term.
    ///
    /// # Parameters
    ///
    /// * `iri` - The IriS to convert
    fn iris_as_term(iri: &IriS) -> Self::Term {
        Self::Term::from(Self::IRI::from(iri.clone()))
    }

    /// Converts a term to an IRI.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsIri` if the term is not an IRI.
    fn term_as_iri(term: &Self::Term) -> Result<Self::IRI, RDFError> {
        Self::IRI::try_from(term.clone()).map_err(|_| RDFError::TermAsIri {
            term: term.to_string(),
        })
    }

    /// Converts an IRI or blank node to a term.
    ///
    /// # Parameters
    ///
    /// * `ib` - The IRI or blank node to convert
    fn iri_or_bnode_as_term(ib: &IriOrBlankNode) -> Self::Term {
        let subject: Self::Subject = ib.clone().into();
        subject.into()
    }

    /// Converts a term to a blank node.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsBNode` if the term is not a blank node.
    fn term_as_bnode(term: &Self::Term) -> Result<Self::BNode, RDFError> {
        Self::BNode::try_from(term.clone()).map_err(|_| RDFError::TermAsBNode {
            term: term.to_string(),
        })
    }

    /// Converts a term to an `IriS`.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsIriS` if the term is not an IRI.
    fn term_as_iris(term: &Self::Term) -> Result<IriS, RDFError> {
        let iri = Self::IRI::try_from(term.clone()).map_err(|_| RDFError::TermAsIriS {
            term: term.to_string(),
        })?;
        let iri_s: IriS = iri.into();
        Ok(iri_s)
    }

    /// Converts a term to an `Object`.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsObject` if the conversion fails.
    fn term_as_object(term: &Self::Term) -> Result<Object, RDFError> {
        <Self::Term as TryInto<Object>>::try_into(term.clone()).map_err(|_e| {
            RDFError::TermAsObject {
                term: format!("Converting term to object: {term}"),
                error: "Error term_as_object".to_string(),
            }
        })
    }

    /// Converts an `Object` to a term.
    ///
    /// # Parameters
    ///
    /// * `object` - The object to convert
    fn object_as_term(object: &Object) -> Self::Term {
        Self::Term::from(object.clone())
    }

    /// Converts a subject to an `Object`.
    ///
    /// # Parameters
    ///
    /// * `subject` - The subject to convert
    ///
    /// # Errors
    ///
    /// Returns `RDFError` if the subject cannot be converted to an object.
    fn subject_as_node(subject: &Self::Subject) -> Result<Object, RDFError> {
        let term = Self::subject_as_term(subject);
        let object = Self::term_as_object(&term)?;
        Ok(object)
    }

    /// Extracts a language tag from a term.
    ///
    /// # Parameters
    ///
    /// * `term` - The term to extract the language tag from
    ///
    /// # Errors
    ///
    /// Returns `RDFError::TermAsLang` if the term is not a language-tagged literal.
    fn term_as_lang(term: &Self::Term) -> std::result::Result<Lang, RDFError> {
        if term.is_blank_node() {
            Err(RDFError::TermAsLang {
                term: term.to_string(),
            })
        } else if let Ok(literal) = Self::term_as_literal(term) {
            let lang = Lang::new(literal.lexical_form());
            match lang {
                Ok(lang) => Ok(lang),
                Err(_) => todo!(),
            }
        } else {
            todo!()
        }
    }

    /// Compares two terms according to SPARQL ordering semantics.
    ///
    /// The comparison follows the SPARQL 1.1 specification for operator mapping:
    /// <https://www.w3.org/TR/sparql11-query/#OperatorMapping>
    ///
    /// # Parameters
    ///
    /// * `term1` - The first term to compare
    /// * `term2` - The second term to compare
    ///
    /// # Errors
    ///
    /// Returns `RDFError::ComparisonError` if the terms cannot be compared
    fn compare(&self, term1: &Self::Term, term2: &Self::Term) -> Result<Ordering, RDFError> {
        // TODO: At this moment we convert the terms to object and perform the comparison within objects
        // This requires to clone but we should be able to optimize this later
        let obj1: Object = Self::term_as_object(term1)?;
        let obj2: Object = Self::term_as_object(term2)?;
        obj1.partial_cmp(&obj2)
            .ok_or_else(|| RDFError::ComparisonError {
                term1: term1.lexical_form(),
                term2: term2.lexical_form(),
            })
    }

    /// Checks if two terms are equal according to SPARQL semantics.
    ///
    /// The equality follows the SPARQL 1.1 specification for operator mapping:
    /// <https://www.w3.org/TR/sparql11-query/#OperatorMapping>
    ///
    /// # Parameters
    ///
    /// * `term1` - The first term
    /// * `term2` - The second term
    fn equals(&self, term1: &Self::Term, term2: &Self::Term) -> bool {
        term1 == term2
    }
}
