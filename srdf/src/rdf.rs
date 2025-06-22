use std::cmp::Ordering;
use std::fmt::Display;

use iri_s::IriS;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;
use rust_decimal::Decimal;

use crate::lang::Lang;
use crate::matcher::Matcher;
use crate::BlankNode;
use crate::Iri;
use crate::Literal;
use crate::Object;
use crate::RDFError;
use crate::SLiteral;
use crate::Subject;
use crate::Term;
use crate::Triple;

pub trait Rdf: Sized {
    type Subject: Subject
        + From<Self::IRI>
        + From<Self::BNode>
        + From<IriS>
        + TryFrom<Self::Term>
        + TryFrom<Object>
        + Matcher<Self::Subject>;

    type IRI: Iri + From<IriS> + TryFrom<Self::Term> + Matcher<Self::IRI> + Into<IriS>;

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
        + From<SLiteral>
        + TryInto<SLiteral>;

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

    fn term_as_literal(term: &Self::Term) -> Result<Self::Literal, RDFError> {
        <Self::Term as TryInto<Self::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsLiteral {
                term: term.to_string(),
            }
        })
    }

    fn term_as_sliteral(term: &Self::Term) -> Result<SLiteral, RDFError> {
        let lit = <Self::Term as TryInto<Self::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsLiteral {
                term: term.to_string(),
            }
        })?;
        let slit = <Self::Literal as TryInto<SLiteral>>::try_into(lit.clone()).map_err(|_| {
            RDFError::LiteralAsSLiteral {
                literal: lit.to_string(),
            }
        })?;
        Ok(slit)
    }

    fn term_as_subject(term: &Self::Term) -> Result<Self::Subject, RDFError> {
        <Self::Term as TryInto<Self::Subject>>::try_into(term.clone()).map_err(|_e| {
            RDFError::TermAsSubject {
                term: term.to_string(),
            }
        })
    }

    fn subject_as_term(subj: &Self::Subject) -> Self::Term {
        subj.clone().into()
    }

    fn iris_as_term(iri: &IriS) -> Self::Term {
        Self::Term::from(Self::IRI::from(iri.clone()))
    }

    fn term_as_iri(term: &Self::Term) -> Result<Self::IRI, RDFError> {
        <Self::Term as TryInto<Self::IRI>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsIri {
                term: term.to_string(),
            }
        })
    }

    fn term_as_iris(term: &Self::Term) -> Result<IriS, RDFError> {
        let iri = <Self::Term as TryInto<Self::IRI>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsIriS {
                term: term.to_string(),
            }
        })?;
        let iri_s: IriS = iri.into();
        Ok(iri_s)
    }

    fn term_as_object(term: &Self::Term) -> Result<Object, RDFError> {
        <Self::Term as TryInto<Object>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsObject {
                term: format!("Converting term to object: {term}"),
            }
        })
    }

    fn subject_as_object(subj: &Self::Subject) -> Result<Object, RDFError> {
        let term = Self::subject_as_term(subj);
        <Self::Term as TryInto<Object>>::try_into(term.clone()).map_err(|_| {
            RDFError::TermAsObject {
                term: format!("Converting subject to object: {term}"),
            }
        })
    }

    fn subject_as_node(subject: &Self::Subject) -> Result<Object, RDFError> {
        let term = Self::subject_as_term(subject);
        let object = Self::term_as_object(&term)?;
        Ok(object)
    }

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

    /// The comparison should be compatible to SPARQL comparison:
    /// https://www.w3.org/TR/sparql11-query/#OperatorMapping
    fn compare(&self, term1: &Self::Term, term2: &Self::Term) -> Result<Ordering, RDFError> {
        // TODO: At this moment we convert the terms to object and perform the comparison within objects
        // This requires to clone but we should be able to optimize this later
        let obj1: Object = Self::term_as_object(term1)?;
        let obj2: Object = Self::term_as_object(term2)?;
        println!("Comparing objects: {obj1:?} {obj2:?}");
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
