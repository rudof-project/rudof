use iri_s::IriS;

use super::rdf_parser_error::RDFParseError;
use crate::{Object, Vocab, RDF_TYPE, SRDF};
use std::{collections::VecDeque, error::Error, fmt::Display};

trait RDFParse<RDF: SRDF> {
    /// The type which is returned if the parser is successful.    
    type Output;

    fn parse(&mut self, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

trait RDFNodeParse<RDF: SRDF> {
    type Output;

    fn parse_subject(&mut self, node: RDF::Subject, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

/*fn parse_list_for_predicate<RDF>(
    node: RDF::Subject,
    pred: RDF::IRI,
    rdf: RDF,
) -> Result<Vec<RDF::Term>, RDF::Err>
where
    RDF: SRDF,
{
    let object = predicate_value(node, pred, rdf)?;
    let rest = parse_list_for_predicate_visited(object, pred, rdf, Vec::new())?;
}*/

pub struct RDFParser<RDF>
where
    RDF: SRDF,
{
    rdf: RDF,
}

impl<RDF> RDFParser<RDF>
where
    RDF: SRDF,
{
    pub fn new(rdf: RDF) -> RDFParser<RDF> {
        RDFParser { rdf }
    }

    pub fn iri_unchecked(str: &str) -> RDF::IRI {
        RDF::iri_s2iri(&IriS::new_unchecked(str))
    }

    pub fn term_iri_unchecked(str: &str) -> RDF::Term {
        RDF::iri_as_term(Self::iri_unchecked(str))
    }

    #[inline]
    fn rdf_type() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_type())
    }

    #[inline]
    fn rdf_first() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_first())
    }

    #[inline]
    fn rdf_rest() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_rest())
    }

    #[inline]
    fn rdf_nil() -> RDF::IRI {
        RDF::iri_s2iri(&Vocab::rdf_nil())
    }

    pub fn instances_of(
        &self,
        object: &RDF::Term,
    ) -> Result<impl Iterator<Item = RDF::Subject>, RDFParseError> {
        let values = self
            .rdf
            .subjects_with_predicate_object(&Self::rdf_type(), &object)
            .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?;
        Ok(values.into_iter())
    }

    pub fn instance_of(&self, object: &RDF::Term) -> Result<RDF::Subject, RDFParseError> {
        let mut values = self.instances_of(&object)?;
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

    pub fn predicate_values(
        &self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<impl Iterator<Item = RDF::Term>, RDFParseError>
    where
        RDF: SRDF,
    {
        let values = self
            .rdf
            .get_objects_for_subject_predicate(&node, &pred)
            .map_err(|e| RDFParseError::SRDFError {
                err: format!("{e}"),
            })?;
        Ok(values.into_iter())
    }

    pub fn predicate_value(
        &self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<RDF::Term, RDFParseError>
    where
        RDF: SRDF,
    {
        let mut values = self.predicate_values(&node, &pred)?;
        if let Some(value1) = values.next() {
            if let Some(value2) = values.next() {
                Err(RDFParseError::MoreThanOneValuePredicate {
                    node: format!("{node}"),
                    pred: format!("{pred}"),
                    value1: format!("{value1:?}"),
                    value2: format!("{value2:?}"),
                })
            } else {
                Ok(value1)
            }
        } else {
            /* Debug in case no value found */
            println!("Not found value for property {pred}");
            let preds = self.rdf.get_predicates_for_subject(&node);
            for pred in preds.iter() {
                println!("Other predicates: {pred:?}");
            }
            /* end debug */

            Err(RDFParseError::NoValuesPredicate {
                node: format!("{node}"),
                pred: format!("{pred}"),
            })
        }
    }

    pub fn get_rdf_type(&self, node: &RDF::Subject) -> Result<RDF::Term, RDFParseError> {
        let value = self.predicate_value(node, &Self::rdf_type())?;
        Ok(value)
    }

    pub fn term_as_iri(term: &RDF::Term) -> Result<IriS, RDFParseError> {
        let obj = RDF::term_as_object(term);
        match obj {
            Object::Iri { iri } => Ok(iri),
            Object::BlankNode(bnode) => Err(RDFParseError::ExpectedIRIFoundBNode { bnode }),
            Object::Literal(lit) => Err(RDFParseError::ExpectedIRIFoundLiteral { lit }),
        }
    }

    pub fn term_as_subject(term: &RDF::Term) -> Result<RDF::Subject, RDFParseError> {
        match RDF::term_as_subject(&term) {
            None => Err(RDFParseError::ExpectedSubject {
                node: format!("{term}"),
            }),
            Some(subj) => Ok(subj),
        }
    }

    pub fn parse_list_for_predicate(
        &self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<Vec<RDF::Term>, RDFParseError> {
        let list_node = self.predicate_value(&node, &pred)?;
        let list_node_subj = Self::term_as_subject(&list_node)?;
        let values = self.parse_list(&list_node_subj, vec![list_node])?;
        Ok(values)
    }

    fn parse_list(
        &self,
        list_node: &RDF::Subject,
        mut visited: Vec<RDF::Term>,
    ) -> Result<Vec<RDF::Term>, RDFParseError> {
        if Self::node_is_rdf_nil(&list_node) {
            Ok(Vec::new())
        } else {
            let value = self.predicate_value(&list_node, &Self::rdf_first())?;
            let rest = self.predicate_value(&list_node, &Self::rdf_rest())?;
            if visited.contains(&&rest) {
                Err(RDFParseError::RecursiveRDFList {
                    node: format!("{rest}"),
                })
            } else {
                visited.push(rest.clone());
                let rest_subj = Self::term_as_subject(&rest)?;
                let mut rest = Vec::new();
                rest.push(value);
                rest.extend(self.parse_list(&rest_subj, visited)?);
                Ok(rest)
            }
        }
    }

    fn node_is_rdf_nil(node: &RDF::Subject) -> bool {
        if let Some(iri) = RDF::subject_as_iri(&node) {
            iri == Self::rdf_nil()
        } else {
            false
        }
    }
}
