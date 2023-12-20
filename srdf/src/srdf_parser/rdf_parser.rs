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

    fn parse(&mut self, node: &IriS, rdf: &RDF) -> Result<Self::Output, RDFParseError> {
        let subject = RDF::iri_as_subject(RDF::iri_s2iri(node));
        self.parse_impl(&subject, rdf)
    }

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> PResult<Self::Output>;
}

#[derive(Copy, Clone)]
pub struct Map<P, F>(P, F);
impl<RDF, A, B, P, F> RDFNodeParse<RDF> for Map<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF, Output = A>,
    F: FnMut(A) -> B,
{
    type Output = B;

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> PResult<Self::Output> {
        match self.0.parse_impl(node, rdf) {
            Ok(a) => Ok((self.1)(a)),
            Err(e) => Err(e),
        }
    }
}

pub fn map<RDF, P, F, B>(p: P, f: F) -> Map<P, F>
where
    RDF: SRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> B,
{
    Map(p, f)
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

    fn parse_impl(&mut self, node: &<RDF>::Subject, rdf: &RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(node, rdf) {
            Ok(value) => match (self.function)(value) {
                Ok(result) => Ok(result),
                Err(e) => Err(e.into()),
            },
            Err(err) => Err(err),
        }
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

    fn parse_impl(&mut self, node: &<RDF>::Subject, rdf: &RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(node, rdf) {
            Ok(value) => match (self.function)(value) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
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

pub struct PropertyValues<RDF: SRDF> {
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

fn property_value<RDF>(property: &IriS) -> PropertyValue<RDF>
where
    RDF: SRDF,
{
    let iri = RDF::iri_s2iri(property);
    PropertyValue {
        property: iri,
        _marker: PhantomData,
    }
}

pub struct PropertyValue<RDF: SRDF> {
    property: RDF::IRI,
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for PropertyValue<RDF>
where
    RDF: SRDF,
{
    type Output = RDF::Term;

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> PResult<RDF::Term> {
        let mut p = property_values(&self.property);
        let mut values_iter = p.parse_impl(node, rdf)?.into_iter();
        if let Some(value1) = values_iter.next() {
            if let Some(value2) = values_iter.next() {
                Err(RDFParseError::MoreThanOneValuePredicate {
                    node: format!("{node}"),
                    pred: format!("{}", self.property),
                    value1: format!("{value1:?}"),
                    value2: format!("{value2:?}"),
                })
            } else {
                Ok(value1)
            }
        } else {
            Err(RDFParseError::NoValuesPredicate {
                node: format!("{node}"),
                pred: format!("{}", self.property),
            })
        }
    }
}

fn parse_list_for_property<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output=Vec<RDF::Term>> 
where RDF: SRDF {

    flat_map(property_value(property), |value| Ok(rdf_list()))
//    let value = property_value(property).parse_impl(node, rdf)?;

}

/// Parses a node as an RDF List
fn rdf_list<RDF>() -> RDFList<RDF>
where
    RDF: SRDF,
{
    RDFList {
        _marker: PhantomData,
    }
}

pub struct RDFList<RDF: SRDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for RDFList<RDF>
where
    RDF: SRDF,
{
    type Output = Vec<RDF::Term>;

    fn parse_impl(&mut self, node: &RDF::Subject, rdf: &RDF) -> PResult<Vec<RDF::Term>> {
        let mut visited = vec![RDF::subject_as_term(node)];
        parse_list(node, visited, rdf)
    }
}

fn parse_list<RDF>(
    list_node: &RDF::Subject,
    mut visited: Vec<RDF::Term>,
    rdf: &RDF
) -> Result<Vec<RDF::Term>, RDFParseError> 
where RDF: SRDF {
    if node_is_rdf_nil::<RDF>(&list_node) {
        Ok(Vec::new())
    } else {
        let value = property_value(&Vocab::rdf_first()).parse_impl(list_node, rdf)?;
        let rest = property_value(&Vocab::rdf_rest()).parse_impl(list_node, rdf)?;
        if visited.contains(&rest) {
            Err(RDFParseError::RecursiveRDFList {
                node: format!("{rest}"),
            })
        } else {
            visited.push(rest.clone());
            let rest_subj = RDF::term_as_subject(&rest).ok_or_else(|| {
                RDFParseError::ExpectedSubject { node: format!("{rest}") }
            })?;
            let mut rest = Vec::new();
            rest.push(value);
            rest.extend(parse_list(&rest_subj, visited, rdf)?);
            Ok(rest)
        }
    }
}

fn node_is_rdf_nil<RDF>(node: &RDF::Subject) -> bool where RDF: SRDF {
    if let Some(iri) = RDF::subject_as_iri(&node) {
        RDF::iri2iri_s(&iri) == Vocab::rdf_nil() 
    } else {
        false
    }
}



/// Implements a concrete RDF parser
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

    pub fn predicate_values(&self,
        node: &RDF::Subject,
        pred: &RDF::IRI,
    ) -> Result<HashSet<RDF::Term>, RDFParseError> {
        let mut p = property_values(pred);
        let vs = p.parse_impl(node, &self.rdf)?;
        Ok(vs)
    }

    pub fn predicate_value(
        &self,
        node: &RDF::Subject,
        pred: &IriS,
    ) -> Result<RDF::Term, RDFParseError>
    where
        RDF: SRDF,
    {
        property_value(pred).parse_impl(node, &self.rdf)
    }

    pub fn get_rdf_type(&self, node: &RDF::Subject) -> Result<RDF::Term, RDFParseError> {
        let value = self.predicate_value(node, &Vocab::rdf_type())?;
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
        pred: &IriS,
    ) -> Result<Vec<RDF::Term>, RDFParseError> {
        let list_node = self.predicate_value(&node, pred)?;
        let list_node_subj = Self::term_as_subject(&list_node)?;
        let values = rdf_list().parse_impl(&list_node_subj, &self.rdf)?;
        Ok(values)
    }

    
}

