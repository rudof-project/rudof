use std::{marker::PhantomData, collections::HashSet};

use iri_s::IriS;

use crate::{FocusRDF, RDFParseError, RDF_NIL, SRDF, Vocab, rdf_parser, PResult};

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
    fn and<P2>(self, parser: P2) -> (Self, P2)
    where
        Self: Sized,
        P2: RDFNodeParse<RDF>,
    {
        (self, parser)
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

    /// Returns a parser which attempts to parse using `self`. If `self` fails then it attempts `parser`.
    fn or<P2>(self, parser: P2) -> Or<Self, P2>
    where
        Self: Sized,
        P2: RDFNodeParse<RDF, Output = Self::Output>
    {
        or(self, parser)
    }

    /// Sets the focus node and returns ()
    fn focus(self, node: &RDF::Term) -> SetFocus<RDF>
    where
        Self: Sized,
    {
        set_focus(node)
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

pub fn or<RDF, P1, P2>(parser1: P1, parser2: P2) -> Or<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF>,
    P2: RDFNodeParse<RDF>
{
    Or { parser1, parser2 }
}

#[derive(Copy, Clone)]
pub struct Or<P1, P2>{ parser1: P1, parser2: P2 }

impl<RDF, P1, P2, O> RDFNodeParse<RDF> for Or<P1, P2>
where RDF: FocusRDF, 
      P1: RDFNodeParse<RDF, Output = O>, 
      P2: RDFNodeParse<RDF, Output = O>
      {

    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser1.parse_impl(rdf) {
            Ok(value) => Ok(value),
            Err(err1) => match self.parser2.parse_impl(rdf) {
               Ok(value) => Ok(value),
               Err(err2) => Err(
                RDFParseError::FailedOr { 
                    err1: Box::new(err1), 
                    err2: Box::new(err2) 
                })
            }
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

#[derive(Debug, Clone)]
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

/// Parses the RDF list linked from the value of property `prop` at focus node
/// 
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
                    println!("Comparison with name: {} for term: {term}", self.predicate_name);
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

pub fn set_focus<RDF>(node: &RDF::Term) -> SetFocus<RDF> where RDF: FocusRDF {
    SetFocus {
        node: node.clone(), 
        _marker: PhantomData
    }
}

#[derive(Debug, Clone)]
pub struct SetFocus<RDF> where RDF: FocusRDF {
    node: RDF::Term,
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for SetFocus<RDF>
where
    RDF: FocusRDF,
{
    type Output = ();

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<()> {
        rdf.set_focus(&self.node);
        Ok(())
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

/* I would like the following code to work but it complains that: 
cannot move out of `parser`, a captured variable in an `FnMut` closure 
pub fn parse_rdf_list_for_property<RDF, P, A>(property: IriS, parser: P) -> impl RDFNodeParse<RDF, Output = Vec<A>> 
where 
   RDF: FocusRDF,
   P: RDFNodeParse<RDF, Output = A> + Clone
{
    property_value(&property).then(|ref node| {
        set_focus(node).and(
            parse_rdf_list::<RDF,P>(parser)).map(|(_, vs)| { vs }
        )
    })
}*/

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

/// Succeeds if current term is the expected IRI
pub fn is_iri<RDF>(expected_iri: IriS) -> impl RDFNodeParse<RDF, Output = ()>
where RDF: FocusRDF,
 {
    let name = format!("Is {}", expected_iri.as_str());
    satisfy(
      move |node: &RDF::Term| match RDF::object_as_iri(node) {
        Some(iri) => {
            let iri_s = RDF::iri2iri_s(&iri);
            iri_s == expected_iri
        }
        None => false,
    }, name.as_str()
   )    
}

/// Returns the node that is an instances of the expected IRI in the RDF data
/// It moves the focus to point to that node
pub fn instance_of<RDF>(expected: &IriS) -> impl RDFNodeParse<RDF, Output = RDF::Subject> 
where RDF: FocusRDF {
    instances_of(expected).flat_map(|vs| {
        let mut values = vs.into_iter();
        match values.next() {
            Some(value) => match values.next() {
                Some(_other_value) => todo!(),
                None => {
                    Ok(value)
                }
            },
            None => todo!()
        }
    })
}

pub fn set_focus_subject<RDF>(subject: RDF::Subject) -> impl RDFNodeParse<RDF, Output = ()> 
where RDF: FocusRDF {
   ApplyRDF {
    function: move |rdf: &mut RDF| {
        let term = RDF::subject_as_term(&subject);
        rdf.set_focus(&term);
        Ok(())
    }
   }
}

pub fn term_as_iri<RDF>(term: RDF::Term) -> impl RDFNodeParse<RDF, Output = IriS> 
where RDF: FocusRDF {
    ApplyRDF {
        function: move |_: &mut RDF| {
            match RDF::object_as_iri(&term) {
                Some(iri) => {
                    let iri_s = RDF::iri2iri_s(&iri);
                    Ok(iri_s)
                }
                None => todo!()
            }
        }
    }
}

/// Succeeds with a given value
pub fn ok<RDF, A>(value: &A) -> impl RDFNodeParse<RDF, Output = A> 
where RDF: FocusRDF,
      A: Clone {
    Ok { value: value.clone() }
}

#[derive(Debug, Clone)]
struct Ok<A> 
{
    value: A,
}

impl<RDF, A> RDFNodeParse<RDF> for Ok<A>
where
    RDF: FocusRDF,
    A: Clone
{
    type Output = A;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        Ok(self.value.clone())
    }
}

/// Applies a function and returns its result
pub fn apply<RDF, A, B>(value: &A, function: impl FnMut(&A) -> Result<B, RDFParseError>) -> impl RDFNodeParse<RDF, Output = B> 
where RDF: FocusRDF,
      A: Clone {
    Apply { value: value.clone(), function }
}

#[derive(Debug, Clone)]
struct Apply<A, F> 
{
    value: A,
    function: F
}

impl<RDF, A, B, F> RDFNodeParse<RDF> for Apply<A, F>
where
    RDF: FocusRDF,
    F: FnMut(&A) -> Result<B, RDFParseError>,
    A: Clone
{
    type Output = B;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match (self.function)(&self.value) {
            Ok(b) => Ok(b),
            Err(e) => Err(e)
        }
    }
}


/// Applies a function over the RDF graph and returns the result of that function
pub fn apply_rdf<RDF, A>(function: impl FnMut(&mut RDF) -> Result<A, RDFParseError>) -> impl RDFNodeParse<RDF, Output = A> 
where RDF: FocusRDF {
    ApplyRDF { function }
}

#[derive(Debug, Clone)]
struct ApplyRDF<F> {
    function: F
}

impl<RDF, A, F> RDFNodeParse<RDF> for ApplyRDF<F>
where
    RDF: FocusRDF,
    F: FnMut(&mut RDF) -> Result<A, RDFParseError>,
{
    type Output = A;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match (self.function)(rdf) {
            Ok(a) => Ok(a),
            Err(e) => Err(e)
        }
    }
}


/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn instances_of<RDF>(expected: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Subject>> 
where RDF: FocusRDF {
    let term = RDF::iri_s2term(expected);
    subjects_with_property_value(&Vocab::rdf_type(), &term)
}

pub fn rdf_type<RDF>() -> impl RDFNodeParse<RDF, Output = RDF::Term> 
where RDF: FocusRDF {
    property_value(&Vocab::rdf_type())
}

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn subjects_with_property_value<RDF>(property: &IriS, value: &RDF::Term) -> SubjectsPropertyValue<RDF> 
where RDF: FocusRDF {
  let iri = RDF::iri_s2iri(property);
  SubjectsPropertyValue {
    property: iri,
    value: value.clone(),
    _marker: PhantomData
  }
}

pub struct SubjectsPropertyValue<RDF: SRDF> {
    property: RDF::IRI,
    value: RDF::Term,
    _marker: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for SubjectsPropertyValue<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<RDF::Subject>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<RDF::Subject>> {
        let subjects = rdf.subjects_with_predicate_object(&self.property, &self.value).map_err(|e| {
           RDFParseError::ErrorSubjectsPredicateObject {
             property: format!("{}", self.property),
             value: format!("{}", self.value),
             err: e.to_string()
           }
        })?;
        let mut result = Vec::new();
        for s in subjects {
            result.push(s)
        }
        Ok(result)
    }
}


rdf_parser!{
    /// Parses the value of `property` as an RDF list
    pub fn parse_property_value_as_list['a, RDF](property: &'a IriS)(RDF) -> Vec<RDF::Term>
        where [
        ] { 
            property_value(&property)
            .then(|node| 
                set_focus(&node).then(|_| 
                    rdf_list())
             )
        }
    }