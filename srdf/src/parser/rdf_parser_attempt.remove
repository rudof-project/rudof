use iri_s::IriS;

use super::rdf_parser_error::RDFParseError;
use crate::{Object, Vocab, RDF_NIL, RDF_TYPE, SRDF};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
    marker::PhantomData,
};

type PResult<A> = Result<A, RDFParseError>;

/// The following code is an attempt to define parser combinators where the input is an RDF graph instead of a sequence of characters
/// Some parts of this code are inspired by [Combine](https://github.com/Marwes/combine)
///

/// Represents a generic parser of RDF data
pub trait RDFParse<RDF: SRDF> {
    /// The type which is returned if the parser is successful.    
    type Output;

    fn parse(&mut self, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

/// Represents a parser of RDF data from a pointed node in the graph
pub trait RDFNodeParse<RDF: SRDF> {
    type Output;

    fn focus(&self) -> RDF::Subject;

    fn parse(&mut self, rdf: &RDF) -> Result<Self::Output, RDFParseError> {
        self.parse_impl(rdf)
    }

    fn parse_impl(&mut self, rdf: &RDF) -> PResult<Self::Output>;
}

#[derive(Copy, Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}
impl<RDF, A, B, P, F> RDFNodeParse<RDF> for Map<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF, Output = A>,
    F: FnMut(A) -> B,
{
    type Output = B;

    fn parse_impl(&mut self, rdf: &RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(a) => Ok((self.f)(a)),
            Err(e) => Err(e),
        }
    }

    fn focus(&self) -> <RDF>::Subject {
        self.parser.focus()
    }
}

pub fn map<RDF, P, F, B>(parser: P, f: F) -> Map<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> B,
{
    Map { parser, f }
}

pub fn and_then<RDF, P, F, O, E>(parser: P, function: F) -> AndThen<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> Result<O, E>,
    E: Into<RDFParseError>,
{
    AndThen { parser, function }
}

#[derive(Copy, Clone)]
pub struct AndThen<P, F> {
    parser: P,
    function: F,
}

impl<RDF, P, F, O, E> RDFNodeParse<RDF> for AndThen<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> Result<O, E>,
    E: Into<RDFParseError>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &RDF) -> PResult<Self::Output> {
        match self.parser.parse(rdf) {
            Ok(value) => match (self.function)(value) {
                Ok(result) => Ok(result),
                Err(e) => Err(e.into()),
            },
            Err(err) => Err(err),
        }
    }

    fn focus(&self) -> <RDF>::Subject {
        self.parser.focus()
    }
}

pub fn flat_map<RDF, P, F, O>(parser: P, function: F) -> FlatMap<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> PResult<O>,
{
    FlatMap { parser, function }
}

#[derive(Copy, Clone)]
pub struct FlatMap<P, F> {
    parser: P,
    function: F,
}

impl<RDF, P, F, O> RDFNodeParse<RDF> for FlatMap<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> PResult<O>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &RDF) -> PResult<Self::Output> {
        match self.parser.parse(rdf) {
            Ok(value) => match (self.function)(value) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn focus(&self) -> <RDF>::Subject {
        self.parser.focus()
    }
}

pub fn parse_rdf_nil<RDF>() -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: SRDF,
{
    satisfy(
        |node: &RDF::Subject| match RDF::subject_as_iri(node) {
            Some(iri) => {
                let iri_s = RDF::iri2iri_s(&iri);
                iri_s.as_str() == RDF_NIL
            }
            None => false,
        },
        "rdf_nil",
    )
}

pub fn satisfy<RDF, P>(predicate: P, predicate_name: &str) -> Satisfy<RDF, P>
where
    RDF: SRDF,
    P: FnMut(&RDF::Subject) -> bool,
{
    Satisfy {
        predicate,
        predicate_name: predicate_name.to_string(),
        _marker: PhantomData,
    }
}

#[derive(Clone)]
pub struct Satisfy<RDF, P> {
    predicate: P,
    predicate_name: String,
    _marker: PhantomData<RDF>,
}

impl<RDF, P> RDFNodeParse<RDF> for Satisfy<RDF, P>
where
    RDF: SRDF,
    P: FnMut(&RDF::Subject) -> bool,
{
    type Output = ();

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> PResult<()> {
        if (self.predicate)(node) {
            Ok(())
        } else {
            Err(RDFParseError::NodeDoesntSatisfyCondition {
                condition_name: self.predicate_name.clone(),
                node: format!("{node}"),
            })
        }
    }
}

fn property_values<RDF>(property: &RDF::IRI) -> PropertyValues<RDF>
where
    RDF: SRDF,
{
    PropertyValues {
        property: property.clone(),
        _marker: PhantomData,
    }
}

struct PropertyValues<RDF: SRDF> {
    property: RDF::IRI,
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for PropertyValues<RDF>
where
    RDF: SRDF,
{
    type Output = HashSet<RDF::Term>;

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> PResult<HashSet<RDF::Term>> {
        let values = rdf
            .get_objects_for_subject_predicate(&node, &self.property)
            .map_err(|e| RDFParseError::SRDFError {
                err: format!("{e}"),
            })?;
        Ok(values)
    }
}

/*
fn f<RDF>(p: &RDF::IRI) -> impl FnMut(HashSet<RDF::Term>) -> PResult<RDF::Term> + '_
where
    RDF: SRDF,
{
    move |values| {
        let mut values_iter = values.into_iter();
        if let Some(value1) = values_iter.next() {
            if let Some(value2) = values_iter.next() {
                Err(RDFParseError::MoreThanOneValuePredicate {
                    // node: format!("{node}"),
                    pred: format!("{p}"),
                    value1: format!("{value1:?}"),
                    value2: format!("{value2:?}"),
                })
            } else {
                Ok(value1)
            }
        } else {
            Err(RDFParseError::NoValuesPredicate {
                // node: format!("{node}"),
                pred: format!("{p}"),
            })
        }
    }
}

pub fn property_value<RDF>(property: &RDF::IRI) -> impl RDFNodeParse<RDF, Output = RDF::Term>
where
    RDF: SRDF,
{
    and_then::<RDF, PropertyValues<RDF>, _, RDF::Term, RDFParseError>(
        property_values(property),
        f::<RDF>(property),
    )
} */

/*
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
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::srdf::SRDF;
    use crate::srdf_comparisons::SRDFBasic;
    use srdf_graph::SRDFGraph;

    #[test]
    fn test_rdf_nil() {
        let s = r#"prefix : <http://example.org/>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        
        :x :p rdf:nil .
        "#;

        let graph = SRDFGraph::from_str(s, None).unwrap();
        let p = IriS::new_unchecked("http://example.org/p");
        let x = IriS::new_unchecked("http://example.org/p");
    }
}
