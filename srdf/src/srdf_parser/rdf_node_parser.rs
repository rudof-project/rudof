use std::{marker::PhantomData, collections::HashSet};

use iri_s::IriS;

use crate::{FocusRDF, RDFParseError, RDF_NIL, SRDF, Vocab};

use super::PResult;

/// Represents a parser of RDF data from a pointed node in the graph
pub trait RDFNodeParse<RDF: FocusRDF> {
    type Output;

    fn parse(&mut self, node: &IriS, rdf: &mut RDF) -> Result<Self::Output, RDFParseError> {
        let focus = RDF::iri_as_term(RDF::iri_s2iri(node));
        rdf.set_focus(&focus);
        self.parse_impl(rdf)
    }

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output>;


    /// Uses `f` to map over the output of `self`. If `f` returns an error the parser fails.
    ///
    fn flat_map<F, O>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> PResult<O> {
        flat_map(self, f)
    }

    /// Parses with `self` and applies `f` on the result if `self` parses successfully.
    /// `f` may optionally fail with an error which is automatically converted to a `RDFParseError`.
    ///
    fn and_then<F, O, E>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<O, E>,
        E: Into<RDFParseError>
        {
        and_then(self, f)
    }

    /// Uses `f` to map over the parsed value.
    ///
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> B,
    {
        map(self, f)
    }

    /// Parses with `self` followed by `p`.
    /// Succeeds if both parsers succeed, otherwise fails.
    /// Returns a tuple with both values on success.
    ///
    fn and<P2>(self, p: P2) -> (Self, P2)
    where
        Self: Sized,
        P2: RDFNodeParse<RDF>,
    {
        (self, p)
    }

    /// Parses using `self` and then passes the value to `f` which returns a parser used to parse
    /// the rest of the input.
    fn then<N, F>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> N,
        N: RDFNodeParse<RDF>,
    {
        then(self, f)
    }
}

impl<RDF, P1, P2, A, B> RDFNodeParse<RDF> for (P1,P2) 
where RDF: FocusRDF,
      P1: RDFNodeParse<RDF, Output = A>,
      P2: RDFNodeParse<RDF, Output = B>,
{
type Output = (A, B);

fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
    match self.0.parse_impl(rdf) {
        Ok(a) => match self.1.parse_impl(rdf) {
            Ok(b) => Ok((a,b)),
            Err(e) => Err(e)
        },
        Err(e) => Err(e),
    }
}
}



#[derive(Copy, Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<RDF, A, B, P, F> RDFNodeParse<RDF> for Map<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
    F: FnMut(A) -> B,
{
    type Output = B;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(a) => Ok((self.f)(a)),
            Err(e) => Err(e),
        }
    }
}

pub fn map<RDF, P, F, B>(parser: P, f: F) -> Map<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> B,
{
    Map { parser, f }
}

pub fn and_then<RDF, P, F, O, E>(parser: P, function: F) -> AndThen<P, F>
where
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> Result<O, E>,
    E: Into<RDFParseError>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
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
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> PResult<O>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => match (self.function)(value) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

pub fn optional<RDF, P>(parser: P) -> Optional<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
{
    Optional { parser }
}

#[derive(Copy, Clone)]
pub struct Optional<P>{ parser: P }

impl<RDF, P> RDFNodeParse<RDF> for Optional<P>
where RDF: FocusRDF, P: RDFNodeParse<RDF> {
    type Output = Option<P::Output>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => Ok(Some(value)),
            Err(_err) => Ok(None),
        }
    }
}

pub fn then<RDF, P, F, N>(parser: P, function: F) -> Then<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> N,
    N: RDFNodeParse<RDF>
{
    Then { parser, function }
}

#[derive(Copy, Clone)]
pub struct Then<P, F> {
    parser: P,
    function: F,
}

impl<RDF, P, F, N> RDFNodeParse<RDF> for Then<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> N,
    N: RDFNodeParse<RDF>
{
    type Output = N::Output;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => (self.function)(value).parse_impl(rdf),
            Err(err) => Err(err),
        }
    }
}

pub fn iri<RDF>() -> impl RDFNodeParse<RDF, Output = IriS> 
where RDF: FocusRDF {
   term().flat_map(|ref t| {
      match RDF::object_as_iri(t) {
        None => Err(RDFParseError::ExpectedIRI { term: format!("{t}") }),
        Some(v) => Ok(RDF::iri2iri_s(&v))
      }
   })
}

pub fn term<RDF>() -> Term<RDF> 
where
    RDF: FocusRDF,
{
   Term { _marker: PhantomData }
} 

#[derive(Clone)]
pub struct Term<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for Term<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        match rdf.get_focus() {
            Some(focus) => Ok(focus.clone()),
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

pub fn property_list<RDF>(prop: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Term>> 
where RDF: FocusRDF {
    property_value(prop).and(rdf_list()).map(|(_,ls)| { ls })
}

pub fn parse_rdf_nil<RDF>() -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
{
    satisfy(
        |node: &RDF::Term| match RDF::object_as_iri(node) {
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
    P: FnMut(&RDF::Term) -> bool,
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
    RDF: FocusRDF,
    P: FnMut(&RDF::Term) -> bool,
{
    type Output = ();

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<()> {
        match rdf.get_focus() {
            Some(term) => {
                if (self.predicate)(term) {
                    Ok(())
                } else {
                    Err(RDFParseError::NodeDoesntSatisfyCondition {
                        condition_name: self.predicate_name.clone(),
                        node: format!("{term}"),
                    })
                }
            }
            None => todo!(),
        }
    }
}

pub fn property_values<RDF>(property: &RDF::IRI) -> PropertyValues<RDF>
where
    RDF: FocusRDF,
{
    PropertyValues {
        property: property.clone(),
        _marker: PhantomData,
    }
}

pub struct PropertyValues<RDF: FocusRDF> {
    property: RDF::IRI,
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for PropertyValues<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<RDF::Term>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<HashSet<RDF::Term>> {
        let subject = rdf.get_focus_as_subject()?;
        let values = rdf
            .get_objects_for_subject_predicate(&subject, &self.property)
            .map_err(|e| RDFParseError::SRDFError {
                err: format!("{e}"),
            })?;
        Ok(values)
    }
}

pub fn property_value<RDF>(property: &IriS) -> PropertyValue<RDF>
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
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        let mut p: PropertyValues<RDF> = property_values(&self.property);
        let focus_node_str = match rdf.get_focus() {
            None => "No focus node".to_string(),
            Some(focus_node) => {
                format!("{focus_node}")
            }
        };
        /*         let focus_node = rdf
        .get_focus()
        .map(|f| f.to_string())
        .unwrap_or_else(|| "No focus".to_string()); */
        let mut values_iter = p.parse_impl(rdf)?.into_iter();
        if let Some(value1) = values_iter.next() {
            if let Some(value2) = values_iter.next() {
                Err(RDFParseError::MoreThanOneValuePredicate {
                    node: format!("{focus_node_str}",),
                    pred: format!("{}", self.property),
                    value1: format!("{value1:?}"),
                    value2: format!("{value2:?}"),
                })
            } else {
                Ok(value1)
            }
        } else {
            Err(RDFParseError::NoValuesPredicate {
                node: format!("{focus_node_str}"),
                pred: format!("{}", self.property),
            })
        }
    }
}

/// Parses a node as an RDF List
pub fn rdf_list<RDF>() -> RDFList<RDF>
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
    RDF: FocusRDF,
{
    type Output = Vec<RDF::Term>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<RDF::Term>> {
        let focus = rdf.get_focus_as_term()?;
        let visited = vec![focus.clone()];
        parse_list(visited, rdf)
    }
}

/// Parses a node as an RDF List applying each element of the list a parser
pub fn parse_rdf_list<RDF, P>(parser: P) -> ParseRDFList<P>
where
    RDF: SRDF,
{
    ParseRDFList {
        parser
    }
}

#[derive(Copy, Clone)]
pub struct ParseRDFList<P> {
    parser: P,
}

impl<RDF, P, A> RDFNodeParse<RDF> for ParseRDFList<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output= A>
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<A>> {
        let focus = rdf.get_focus_as_term()?;
        let visited = vec![focus.clone()];
        parse_list(visited, rdf).and_then(|nodes| {
            let mut result = Vec::new();
            for node in nodes {
                rdf.set_focus(&node);
                match self.parser.parse_impl(rdf) {
                    Ok(a) => result.push(a),
                    Err(e) => return Err(e)
                }
            }
            Ok(result)
        })
    }
}

// Auxiliary function to parse a node as an RDF list checking that the RDF list if non-cyclic 
// by collecting a vector of visited terms
fn parse_list<RDF>(
    mut visited: Vec<RDF::Term>,
    rdf: &mut RDF,
) -> Result<Vec<RDF::Term>, RDFParseError>
where
    RDF: FocusRDF,
{
    let focus = rdf.get_focus_as_term()?;
    if node_is_rdf_nil::<RDF>(focus) {
        Ok(Vec::new())
    } else {
        let value = property_value(&Vocab::rdf_first()).parse_impl(rdf)?;
        let rest = property_value(&Vocab::rdf_rest()).parse_impl(rdf)?;
        if visited.contains(&&rest) {
            Err(RDFParseError::RecursiveRDFList {
                node: format!("{rest}"),
            })
        } else {
            visited.push(rest.clone());
            let mut rest_ls = vec![value];
            rdf.set_focus(&rest);
            rest_ls.extend(parse_list(visited, rdf)?);
            Ok(rest_ls)
        }
    }
}

fn node_is_rdf_nil<RDF>(node: &RDF::Term) -> bool
where
    RDF: SRDF,
{
    if let Some(iri) = RDF::object_as_iri(node) {
        RDF::iri2iri_s(&iri) == Vocab::rdf_nil()
    } else {
        false
    }
}
