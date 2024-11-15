use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::PrefixMap;

use crate::model::focus_rdf::FocusRdf;
use crate::model::rdf::Object;
use crate::model::rdf::Predicate;
use crate::model::rdf::Rdf;
use crate::model::rdf::Subject;
use crate::model::Iri;
use crate::model::Term;
use crate::model::Triple;
use crate::RDF_TYPE;

use super::rdf_node_parser::*;
use super::rdf_parser_error::RDFParseError;

/// The following code is an attempt to define parser combinators where the input
/// is an RDF graph instead of a sequence of characters. Some parts of this
/// code are inspired by [Combine](https://github.com/Marwes/combine)
///
/// Represents a generic parser of RDF data
pub trait RDFParse<RDF: Rdf> {
    /// The type which is returned if the parser is successful.
    type Output;

    fn parse(&mut self, rdf: RDF) -> Result<Self::Output, RDF::Error>;
}

/// Implements a concrete RDF parser
pub struct RDFParser<RDF: FocusRdf> {
    pub rdf: RDF,
}

impl<RDF: FocusRdf> RDFParser<RDF> {
    pub fn new(rdf: RDF) -> RDFParser<RDF> {
        RDFParser { rdf }
    }

    pub fn prefixmap(&self) -> Option<PrefixMap> {
        self.rdf.prefixmap()
    }

    pub fn iri_unchecked(str: &str) -> Predicate<RDF> {
        Predicate::<RDF>::new(str)
    }

    pub fn set_focus(&mut self, focus: &Object<RDF>) {
        self.rdf.set_focus(focus.clone())
    }

    pub fn set_focus_iri(&mut self, iri: &IriS) {
        let iri = Predicate::<RDF>::new(iri.as_str());
        self.rdf.set_focus(Object::<RDF>::from(iri))
    }

    pub fn term_iri_unchecked(str: &str) -> Object<RDF> {
        Object::<RDF>::from(Self::iri_unchecked(str))
    }

    #[inline]
    fn rdf_type() -> Predicate<RDF> {
        Predicate::<RDF>::new(RDF_TYPE.as_str())
    }

    pub fn instances_of<'a>(
        &'a self,
        object: &'a Object<RDF>,
    ) -> Result<impl Iterator<Item = Subject<RDF>>, RDFParseError> {
        let rdf_type = Self::rdf_type();

        let triples = match self
            .rdf
            .triples_matching(None, Some(&rdf_type), Some(object))
        {
            Ok(triples) => triples,
            Err(_) => {
                return Err(RDFParseError::SRDFError {
                    err: "Error obtaining the triples".to_string(),
                })
            }
        };

        let subjects = triples
            .map(Triple::subj)
            .map(Clone::clone)
            .collect::<Vec<_>>();

        Ok(subjects.into_iter())
    }

    pub fn instance_of(&self, object: &Object<RDF>) -> Result<Subject<RDF>, RDFParseError> {
        let mut values = self.instances_of(object)?;
        if let Some(value1) = values.next() {
            if let Some(value2) = values.next() {
                Err(RDFParseError::MoreThanOneInstanceOf {
                    object: format!("{object}"),
                    value1: format!("{value1}"),
                    value2: format!("{value2}"),
                })
            } else {
                // Only one value
                Ok(value1.clone())
            }
        } else {
            Err(RDFParseError::NoInstancesOf {
                object: format!("{object}"),
            })
        }
    }

    pub fn predicate_values(&mut self, pred: &IriS) -> Result<HashSet<Object<RDF>>, RDFParseError> {
        let mut p = property_values(pred);
        let vs = p.parse_impl(&mut self.rdf)?;
        Ok(vs)
    }

    pub fn predicate_value(&mut self, pred: &IriS) -> Result<Object<RDF>, RDFParseError>
    where
        RDF: FocusRdf,
    {
        property_value(pred).parse_impl(&mut self.rdf)
    }

    pub fn get_rdf_type(&mut self) -> Result<Object<RDF>, RDFParseError> {
        let value = self.predicate_value(&RDF_TYPE)?;
        Ok(value)
    }

    pub fn term_as_iri(term: &Object<RDF>) -> Result<IriS, RDFParseError> {
        match (term.is_iri(), term.is_blank_node(), term.is_literal()) {
            (true, false, false) => Ok(term.as_iri().unwrap().as_iri_s()),
            (false, true, false) => Err(RDFParseError::ExpectedIRIFoundBNode {
                bnode: term.to_string(),
            }),
            (false, false, true) => Err(RDFParseError::ExpectedIRIFoundLiteral {
                lit: term.to_string(),
            }),
            _ => unreachable!(),
        }
    }

    pub fn term_as_subject(term: &Object<RDF>) -> Result<Subject<RDF>, RDFParseError> {
        match term.clone().try_into() {
            Ok(subj) => Ok(subj),
            Err(_) => Err(RDFParseError::ExpectedSubject {
                node: format!("{term}"),
            }),
        }
    }

    pub fn parse_list_for_predicate(
        &mut self,
        pred: &IriS,
    ) -> Result<Vec<Object<RDF>>, RDFParseError> {
        let list_node = self.predicate_value(pred)?;
        self.rdf.set_focus(list_node);
        let values = rdf_list().parse_impl(&mut self.rdf)?;
        Ok(values)
    }
}
