use super::rdf_parser_error::RDFParseError;
use super::{PResult, rdf_node_parser::*};
use crate::Triple;
use crate::matcher::Any;
use crate::{FocusRDF, NeighsRDF, rdf_type};
use iri_s::IriS;
use prefixmap::PrefixMap;
use std::collections::HashSet;

/// The following code is an attempt to define parser combinators where the input is an RDF graph instead of a sequence of characters
/// Some parts of this code are inspired by [Combine](https://github.com/Marwes/combine)
///
/// Represents a generic parser of RDF data
pub trait RDFParse<RDF: NeighsRDF> {
    /// The type which is returned if the parser is successful.
    type Output;

    fn parse(&mut self, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

/// Implements a concrete RDF parser
pub struct RDFParser<RDF>
where
    RDF: FocusRDF,
{
    pub rdf: RDF,
}

impl<RDF> RDFParser<RDF>
where
    RDF: FocusRDF + 'static,
{
    pub fn new(rdf: RDF) -> RDFParser<RDF> {
        RDFParser { rdf }
    }

    pub fn prefixmap(&self) -> Option<PrefixMap> {
        self.rdf.prefixmap()
    }

    pub fn iri_unchecked(str: &str) -> RDF::IRI {
        IriS::new_unchecked(str).into()
    }

    pub fn set_focus(&mut self, focus: &RDF::Term) {
        self.rdf.set_focus(focus)
    }

    pub fn set_focus_iri(&mut self, iri: &IriS) {
        self.rdf.set_focus(&iri.clone().into())
    }

    pub fn term_iri_unchecked(str: &str) -> RDF::Term {
        Self::iri_unchecked(str).into().into()
    }

    #[inline]
    fn rdf_type_iri() -> RDF::IRI {
        rdf_type().clone().into()
    }

    pub fn instances_of(
        &self,
        object: &RDF::Term,
    ) -> PResult<impl Iterator<Item = RDF::Subject> + '_> {
        let values = self
            .rdf
            .triples_matching(Any, Self::rdf_type_iri(), object.clone())
            .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?
            .map(Triple::into_subject);
        Ok(values)
    }

    pub fn instance_of(&self, object: &RDF::Term) -> Result<RDF::Subject, RDFParseError> {
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
                Ok(value1)
            }
        } else {
            Err(RDFParseError::NoInstancesOf {
                object: format!("{object}"),
            })
        }
    }

    pub fn predicate_values(&mut self, pred: IriS) -> Result<HashSet<RDF::Term>, RDFParseError> {
        let mut p = property_values(pred);
        let vs = p.parse_impl(&mut self.rdf)?;
        Ok(vs)
    }

    pub fn predicate_value(&mut self, pred: IriS) -> Result<RDF::Term, RDFParseError>
    where
        RDF: FocusRDF,
    {
        property_value(pred).parse_impl(&mut self.rdf)
    }

    pub fn get_rdf_type(&mut self) -> Result<RDF::Term, RDFParseError> {
        let value = self.predicate_value(rdf_type().clone())?;
        Ok(value)
    }

    pub fn parse_list_for_predicate(
        &mut self,
        pred: IriS,
    ) -> Result<Vec<RDF::Term>, RDFParseError> {
        let list_node = self.predicate_value(pred)?;
        self.rdf.set_focus(&list_node);
        let values = rdf_list().parse_impl(&mut self.rdf)?;
        Ok(values)
    }
}
