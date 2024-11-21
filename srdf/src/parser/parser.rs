use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::PrefixMap;

use crate::model::rdf::FocusRdf;
use crate::model::rdf::Rdf;
use crate::model::rdf::Subjects;
use crate::model::rdf::TObjectRef;
use crate::model::rdf::TPredicateRef;
use crate::model::rdf::TSubjectRef;
use crate::model::Iri as _;
use crate::model::Term;
use crate::model::Triple;
use crate::RDF_TYPE;

use super::node_parser::*;
use super::parser_error::RdfParseError;

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

    pub fn iri_unchecked(str: &str) -> TPredicateRef<RDF> {
        TPredicateRef::<RDF>::from_str(str)
    }

    pub fn set_focus(&mut self, focus: TObjectRef<RDF>) {
        self.rdf.set_focus(focus)
    }

    pub fn set_focus_iri(&mut self, iri: &IriS) {
        let iri = TPredicateRef::<RDF>::from_str(iri.as_str());
        self.rdf.set_focus(TObjectRef::<RDF>::from(iri))
    }

    pub fn term_iri_unchecked(str: &str) -> TObjectRef<RDF> {
        TObjectRef::<RDF>::from(Self::iri_unchecked(str))
    }

    #[inline]
    fn rdf_type() -> TPredicateRef<'static, RDF> {
        TPredicateRef::<RDF>::from_str(RDF_TYPE.as_str())
    }

    pub fn instances_of(&self, object: TObjectRef<RDF>) -> Result<Subjects<RDF>, RdfParseError> {
        let triples = match self
            .rdf
            .triples_matching(None, Some(Self::rdf_type()), Some(object))
        {
            // TODO: can this match be removed?
            Ok(triples) => triples.map(Triple::as_subject),
            Err(_) => {
                return Err(RdfParseError::SRDFError {
                    err: "Error obtaining the triples".to_string(),
                })
            }
        };

        Ok(Box::new(triples))
    }

    pub fn instance_of(&self, object: TObjectRef<RDF>) -> Result<TSubjectRef<RDF>, RdfParseError> {
        let mut values = self.instances_of(object)?;
        if let Some(value1) = values.next() {
            if let Some(value2) = values.next() {
                Err(RdfParseError::MoreThanOneInstanceOf {
                    object: format!("{object}"),
                    value1: format!("{value1}"),
                    value2: format!("{value2}"),
                })
            } else {
                // Only one value
                Ok(value1.clone())
            }
        } else {
            Err(RdfParseError::NoInstancesOf {
                object: format!("{object}"),
            })
        }
    }

    pub fn predicate_values(
        &mut self,
        pred: &IriS,
    ) -> Result<HashSet<TObjectRef<RDF>>, RdfParseError> {
        let mut p = property_values(pred);
        let vs = p.parse_impl(&mut self.rdf)?;
        Ok(vs)
    }

    pub fn predicate_value(&mut self, pred: &IriS) -> Result<TObjectRef<RDF>, RdfParseError>
    where
        RDF: FocusRdf,
    {
        property_value(pred).parse_impl(&mut self.rdf)
    }

    pub fn get_rdf_type(&mut self) -> Result<TObjectRef<RDF>, RdfParseError> {
        let value = self.predicate_value(&RDF_TYPE)?;
        Ok(value)
    }

    pub fn term_as_iri(term: &TObjectRef<RDF>) -> Result<IriS, RdfParseError> {
        match (term.is_iri(), term.is_blank_node(), term.is_literal()) {
            (true, false, false) => Ok(term.into_iri().unwrap().into_iri_s()),
            (false, true, false) => Err(RdfParseError::ExpectedIRIFoundBNode {
                bnode: term.to_string(),
            }),
            (false, false, true) => Err(RdfParseError::ExpectedIRIFoundLiteral {
                lit: term.to_string(),
            }),
            _ => unreachable!(),
        }
    }

    pub fn term_as_subject(term: &TObjectRef<RDF>) -> Result<TSubjectRef<RDF>, RdfParseError> {
        match term.try_into() {
            Ok(subj) => Ok(subj),
            Err(_) => Err(RdfParseError::ExpectedSubject {
                node: format!("{term}"),
            }),
        }
    }

    pub fn parse_list_for_predicate(
        &mut self,
        pred: &IriS,
    ) -> Result<Vec<TObjectRef<RDF>>, RdfParseError> {
        let list_node = self.predicate_value(pred)?;
        self.rdf.set_focus(list_node);
        let values = rdf_list().parse_impl(&mut self.rdf)?;
        Ok(values)
    }
}
