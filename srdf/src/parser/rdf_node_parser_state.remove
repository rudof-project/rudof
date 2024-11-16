use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use iri_s::IriS;
use std::fmt::Debug;

use crate::{
    rdf_parser, FocusRDF, PResult, RDFParseError, RDF_FIRST, RDF_NIL, RDF_NIL_STR, RDF_REST,
    RDF_TYPE, SRDF, SRDFBasic,
};

/// By implementing the `RDFNodeParse` trait a type says that it can be used to parse RDF data which have a focus node. 
/// RDF data with a focus node have to implement the [`FocusRDF`] trait.
pub trait RDFNodeParse<RDF: FocusRDF> {

    /// The type which is returned if the parser is successful.
    type Output;

    /// The type of the internal state
    type State: Default + Clone;


    /// Entry point to the parser. It moves the focus node of `rdf` to `node` and runs the parser.
    /// 
    /// Returns the parsed result if the parser succeeds, or an error otherwise.
    fn parse(&mut self, node: &IriS, rdf: &mut RDF) -> Result<Self::Output, RDFParseError> {
        let focus = RDF::iri_as_term(RDF::iri_s2iri(node));
        rdf.set_focus(&focus);
        self.parse_impl(rdf)
    }

    /// Parses the current focus node without modifying the state
    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> { 
        let mut state = self.get_state();
        self.parse_impl_state(rdf, &mut state).and_then(|(output,s)| Ok(output))
    } 

    /// Parses the current focus node with the possibility of modifying the state
    /// 
    /// The default implementation ignores the state
    fn parse_impl_state(&mut self, rdf: &mut RDF, state: &mut Self::State) -> PResult<(Self::Output, Self::State)> {
        let output = self.parse_impl(rdf)?;
        Ok((output, state.clone()))
    }

    fn get_state(&mut self) -> Self::State {
        Default::default()
    }

    /// Uses `f` to map over the output of `self`. If `f` returns an error the parser fails.
    ///
    /// ```
    /// # use iri_s::{IriS, iri};
    /// # use srdf::SRDFGraph;
    /// use srdf::{RDFNodeParse, RDFFormat, RDFParseError, property_string, PResult};
    ///     let s = r#"prefix : <http://example.org/>
    ///     :x :p "1" .
    ///   "#;
    ///   let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    ///   let x = iri!("http://example.org/x");
    ///   let p = iri!("http://example.org/p");
    ///   fn cnv_int(s: String) -> PResult<isize> {
    ///      s.parse().map_err(|_| RDFParseError::Custom{ msg: format!("Error converting {s}")})
    ///   }
    ///   let mut parser = property_string::<SRDFGraph, ()>(&p).flat_map(cnv_int);
    ///   assert_eq!(parser.parse(&x, &mut graph).unwrap(), 1)
    /// ```
    fn flat_map<F, O>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> PResult<O>,
    {
        flat_map(self, f)
    }

    /// Parses with `self` and applies `f` on the result if `self` parses successfully.
    /// `f` may optionally fail with an error which is automatically converted to a `RDFParseError`.
    ///
    /// ```
    /// # use iri_s::{IriS, iri};
    /// # use srdf::srdf_graph::SRDFGraph;
    /// use srdf::{RDFNodeParse, RDFFormat, RDFParseError, property_string};
    /// let s = r#"prefix : <http://example.org/>
    ///        :x :p "1" .
    ///   "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    /// let x = iri!("http://example.org/x");
    /// let p = iri!("http://example.org/p");
    /// 
    /// 
    /// struct IntConversionError(String);
    /// 
    /// fn cnv_int(s: String) -> Result<isize, IntConversionError> {
    ///    s.parse().map_err(|_| IntConversionError(s))
    /// }
    /// 
    /// impl Into<RDFParseError> for IntConversionError {
    ///     fn into(self) -> RDFParseError {
    ///         RDFParseError::Custom { msg: format!("Int conversion error: {}", self.0)}
    ///     }
    /// }
    /// 
    /// let mut parser = property_string::<SRDFGraph, ()>(&p).and_then(cnv_int);
    /// assert_eq!(parser.parse(&x, &mut graph).unwrap(), 1)
    /// ```
    fn and_then<F, O, E>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<O, E>,
        E: Into<RDFParseError>,
    {
        and_then(self, f)
    }

    /// Uses `f` to map over the parsed value.
    ///
    /// ```
    /// # use iri_s::{IriS, iri};
    /// # use srdf::srdf_graph::SRDFGraph;
    /// use srdf::{RDFNodeParse, RDFFormat, property_integer};
    /// let s = r#"prefix : <http://example.org/>
    ///          :x :p 1 . 
    ///  "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    /// let p = iri!("http://example.org/p");
    /// let mut parser = property_integer::<SRDFGraph, ()>(&p).map(|n| n + 1);
    /// assert_eq!(parser.parse(&iri!("http://example.org/x"), &mut graph).unwrap(), 2)
    /// ```
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> B,
    {
        map(self, f)
    }

    /// Parses `self` followed by `p`.
    /// Succeeds if both parsers succeed, otherwise fails.
    /// Returns a tuple with both values on success.
    ///
    /// ```
    /// # use iri_s::IriS;
    /// # use srdf::srdf_graph::SRDFGraph;
    /// # use srdf::{RDFNodeParse, RDFFormat, property_bool, property_integer};
    /// let s = r#"prefix : <http://example.org/>
    ///       :x :p true ;
    ///          :q 1    .
    ///     "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    /// let x = IriS::new_unchecked("http://example.org/x");
    /// let p = IriS::new_unchecked("http://example.org/p");
    /// let q = IriS::new_unchecked("http://example.org/q");
    /// let mut parser = property_bool::<SRDFGraph, ()>(&p).and(property_integer(&q));
    /// assert_eq!(parser.parse(&x, &mut graph).unwrap(), (true, 1))
    /// ```
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

    /// Parses using `self` and then passes a reference to the value to `f` which returns a parser used to parse
    /// the rest of the input.
    fn then_ref<N, F>(self, f: F) -> ThenRef<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Output) -> N,
        N: RDFNodeParse<RDF>,
    {
        then_ref(self, f)
    }

    /// Parses using `self` and then passes a reference to the mutable value to `f` which returns a parser used to parse
    /// the rest of the input.
    ///
    /// ```
    /// # use iri_s::IriS;
    /// # use srdf::srdf_graph::SRDFGraph;
    /// # use oxrdf::Term;
    /// # use std::collections::HashSet;
    /// use srdf::{RDFNodeParse, RDFFormat, ok, property_integers};
    ///       let s = r#"prefix : <http://example.org/>
    ///       :x :p 1, 2, 3 .
    ///     "#;
    ///     let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    ///     let x = IriS::new_unchecked("http://example.org/x");
    ///     let p = IriS::new_unchecked("http://example.org/p");
    ///     let mut parser = property_integers::<SRDFGraph, ()>(&p).then_mut(move |ns| {
    ///         ns.extend(vec![4, 5]);
    ///         ok::<SRDFGraph, HashSet<isize>, ()>(ns)
    ///      });
    ///     assert_eq!(parser.parse(&x, &mut graph).unwrap(), HashSet::from([1, 2, 3, 4, 5]))
    /// ```
    fn then_mut<N, F>(self, f: F) -> ThenMut<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self::Output) -> N,
        N: RDFNodeParse<RDF>,
    {
        then_mut(self, f)
    }

    /// Returns a parser which attempts to parse using `self`. If `self` fails then it attempts `parser`.
    ///         
    /// ```
    /// # use iri_s::IriS;
    /// # use srdf::srdf_graph::SRDFGraph;
    /// # use srdf::{RDFNodeParse, RDFFormat, property_bool};
    /// # use std::collections::HashSet;
    ///  let s = r#"prefix : <http://example.org/>
    ///       :x :p 1, 2 ;
    ///          :q true .
    ///     "#;
    ///  let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    ///  let x = IriS::new_unchecked("http://example.org/x");
    ///  let p = IriS::new_unchecked("http://example.org/p");
    ///  let q = IriS::new_unchecked("http://example.org/q");
    ///  let mut parser = property_bool::<SRDFGraph, ()>(&p).or(property_bool(&q));
    ///  assert_eq!(parser.parse(&x, &mut graph).unwrap(), true)
    /// ```
    fn or<P2>(self, parser: P2) -> Or<Self, P2>
    where
        Self: Sized,
        P2: RDFNodeParse<RDF, Output = Self::Output, State = Self::State>,
    {
        or(self, parser)
    }

    /// Sets the focus node and returns ()
    fn focus(self, node: &RDF::Term) -> SetFocus<RDF, Self::State>
    where
        Self: Sized,
    {
        set_focus(node)
    }

    /// Discards the value of the current parser and returns the value of `parser`
    ///
    /// ```
    /// # use iri_s::IriS;
    /// # use srdf::{rdf_parser, RDFParser, RDF, RDFFormat, FocusRDF, satisfy, RDFNodeParse, SRDF, SRDFBasic, property_value, rdf_list, set_focus, parse_property_value_as_list, ok};
    /// # use srdf::srdf_graph::SRDFGraph;
    /// let s = r#"prefix : <http://example.org/>
    ///            :x :p :y .
    /// "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    /// let p = IriS::new_unchecked("http://example.org/p");
    /// let x = IriS::new_unchecked("http://example.org/x");
    /// assert_eq!(
    ///   property_value::<SRDFGraph, ()>(&p).with(ok(&1))
    ///   .parse(&x, &mut graph).unwrap(),
    ///   1
    /// )
    /// ```
    fn with<P, A>(self, parser: P) -> With<Self, P>
    where
        Self: Sized,
        P: RDFNodeParse<RDF, Output = A>,
    {
        with(self, parser)
    }

}


impl<RDF, P1, P2, A, B, S> RDFNodeParse<RDF> for (P1, P2)
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = A, State = S>,
    P2: RDFNodeParse<RDF, Output = B, State = S>,
    S: Default + Clone 
{
    type Output = (A, B);
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.0.parse_impl(rdf) {
            Ok(a) => match self.1.parse_impl(rdf) {
                Ok(b) => Ok((a, b)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

/// Applies a function `f` on the result of a parser
///
pub fn map<RDF, P, F, B>(parser: P, f: F) -> Map<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> B,
{
    Map { parser, f }
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
    type State = P::State;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(a) => Ok((self.f)(a)),
            Err(e) => Err(e),
        }
    }
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
    type State = P::State;

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
    type State = P::State;

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
pub struct Optional<P> {
    parser: P,
}

impl<RDF, P> RDFNodeParse<RDF> for Optional<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
{
    type Output = Option<P::Output>;
    type State = P::State;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => Ok(Some(value)),
            Err(_err) => Ok(None),
        }
    }
}

/// Equivalent to [`parser1.or(parser2)`].
///
/// /// [`parser1.or(parser2)`]: trait.RDFNodeParse.html#method.or
pub fn or<RDF, P1, P2>(parser1: P1, parser2: P2) -> Or<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF>,
    P2: RDFNodeParse<RDF>,
{
    Or { parser1, parser2 }
}

#[derive(Copy, Clone)]
pub struct Or<P1, P2> {
    parser1: P1,
    parser2: P2,
    // _marker_s: PhantomData<S>
}

impl<RDF, P1, P2, O, S> RDFNodeParse<RDF> for Or<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = O, State = S>,
    P2: RDFNodeParse<RDF, Output = O, State = S>,
    S: Default + Clone
{
    type Output = O;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser1.parse_impl(rdf) {
            Ok(value) => Ok(value),
            Err(err1) => match self.parser2.parse_impl(rdf) {
                Ok(value) => Ok(value),
                Err(err2) => Err(RDFParseError::FailedOr {
                    err1: Box::new(err1),
                    err2: Box::new(err2),
                })
            },
        }
    }
}

/// Equivalent to [`p.then(f)`].
///
/// [`p.then(f)`]: trait.RDFNodeParse.html#method.then
pub fn then<RDF, P, F, N, S>(parser: P, function: F) -> Then<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, State = S>,
    F: FnMut(P::Output) -> N,
    N: RDFNodeParse<RDF>,
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
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;
    type State = N::State;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => (self.function)(value).parse_impl(rdf),
            Err(err) => Err(err),
        }
    }
}

/// Equivalent to [`p.then_ref(f)`].
///
/// [`p.then_ref(f)`]: trait.RDFNodeParse.html#method.then_ref
pub fn then_ref<RDF, P, F, N>(parser: P, function: F) -> ThenRef<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(&P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    ThenRef { parser, function }
}

#[derive(Copy, Clone)]
pub struct ThenRef<P, F> {
    parser: P,
    function: F,
}

impl<RDF, P, F, N> RDFNodeParse<RDF> for ThenRef<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(&P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;
    type State = N::State;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => (self.function)(&value).parse_impl(rdf),
            Err(err) => Err(err),
        }
    }
}

/// Equivalent to [`p.then_mut(f)`].
///
/// [`p.then_mut(f)`]: trait.RDFNodeParse.html#method.then_mut
pub fn then_mut<RDF, P, F, N>(parser: P, function: F) -> ThenMut<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(&mut P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    ThenMut { parser, function }
}

#[derive(Copy, Clone)]
pub struct ThenMut<P, F> {
    parser: P,
    function: F,
}

impl<RDF, P, F, N> RDFNodeParse<RDF> for ThenMut<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(&mut P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;
    type State = N::State;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(mut value) => (self.function)(&mut value).parse_impl(rdf),
            Err(err) => Err(err),
        }
    }
}


pub fn iri<RDF, S>() -> impl RDFNodeParse<RDF, Output = IriS, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    term().flat_map(|ref t| match RDF::term_as_iri(t) {
        None => Err(RDFParseError::ExpectedIRI {
            term: format!("{t}"),
        }),
        Some(v) => Ok(RDF::iri2iri_s(&v)),
    })
}

/// Creates a parser that returns the current focus node as a term
///
/// This is equivalent to [`get_focus`]
pub fn term<RDF, S>() -> Term<RDF, S>
where
    RDF: FocusRDF,
{
    Term {
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct Term<RDF, S> {
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>,
}

impl<RDF, S> RDFNodeParse<RDF> for Term<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = RDF::Term;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        match rdf.get_focus() {
            Some(focus) => Ok(focus.clone()),
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Parses the RDF list linked from the value of property `prop` at focus node
///
pub fn property_list<RDF, S>(prop: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Term>, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    property_value(prop).and(rdf_list()).map(|(_, ls)| ls)
}

/// Created a parser that returns the boolean associated with the current focus node for `property`
///
/// It doesn't move the current focus node
pub fn property_bool<RDF, S>(prop: &IriS) -> impl RDFNodeParse<RDF, Output = bool, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    property_value(prop).flat_map(|ref term| match RDF::term_as_boolean(term) {
        None => Err(RDFParseError::ExpectedBoolean {
            term: format!("{term}"),
        }),
        Some(b) => Ok(b),
    })
}

pub fn parse_rdf_nil<RDF, S>() -> impl RDFNodeParse<RDF, Output = (), State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    satisfy(
        |node: &RDF::Term| match RDF::term_as_iri(node) {
            Some(iri) => {
                let iri_s = RDF::iri2iri_s(&iri);
                iri_s.as_str() == RDF_NIL_STR
            }
            None => false,
        },
        "rdf_nil",
    )
}

/// Creates a parser that checks if the current node satisfies a predicate
///
/// The `predicate_name` argument is useful in case of failure to know which condition has failed
pub fn satisfy<RDF, P, S>(predicate: P, predicate_name: &str) -> Satisfy<RDF, P, S>
where
    RDF: SRDF,
    P: FnMut(&RDF::Term) -> bool,
{
    Satisfy {
        predicate,
        predicate_name: predicate_name.to_string(),
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

#[derive(Clone)]
pub struct Satisfy<RDF, P, S> {
    predicate: P,
    predicate_name: String,
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>
}

impl<RDF, P, S> RDFNodeParse<RDF> for Satisfy<RDF, P, S>
where
    RDF: FocusRDF,
    P: FnMut(&RDF::Term) -> bool,
    S: Default + Clone
{
    type Output = ();
    type State = S;

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

/// Return the integer values of `property` for the focus node
/// 
/// If some value is not an integer it fails, if there is no value returns an empty set
pub fn property_values_int<RDF, S>(property: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<isize>, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    property_values(property).flat_map(|values| {
        let ints: Vec<_> = values.iter().flat_map(|t| {
            let i = term_to_int::<RDF>(t)?;
            Ok::<isize, RDFParseError>(i)
        }
        ).collect();
        Ok(ints)
    })
}

/// Return the IRI values of `property` for the focus node
/// 
/// If some value is not an IRI it fails, if there is no value returns an empty set
pub fn property_values_iri<RDF, S>(property: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<IriS>, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    property_values(property).flat_map(|values| {
        let ints: Vec<_> = values.iter().flat_map(|t| {
            let iri = term_to_iri::<RDF>(t)?;
            Ok::<IriS, RDFParseError>(iri)
        }
        ).collect();
        Ok(ints)
    })
}



/// Returns the values of `property` for the focus node
///
/// If there is no value, it returns an empty set
pub fn property_values<RDF, S>(property: &IriS) -> PropertyValues<RDF, S>
where
    RDF: FocusRDF,
{
    PropertyValues {
        property: property.clone(),
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

pub struct PropertyValues<RDF: FocusRDF, S> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>
}

impl<RDF, S> RDFNodeParse<RDF> for PropertyValues<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = HashSet<RDF::Term>;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<HashSet<RDF::Term>> {
        let subject = rdf.get_focus_as_subject()?;
        let pred = RDF::iri_s2iri(&self.property);
        let values = rdf
            .objects_for_subject_predicate(&subject, &pred)
            .map_err(|e| RDFParseError::SRDFError {
                err: format!("{e}"),
            })?;
        Ok(values)
    }
}

/// Creates a parser that returns the value associated with the current focus node for `property`
///
/// It doesn't move the current focus node
pub fn property_value<RDF, S>(property: &IriS) -> PropertyValue<RDF, S>
where
    RDF: SRDF,
{
    PropertyValue {
        property: property.clone(),
        _marker_rdf: PhantomData,
        _marker_s: PhantomData
    }
}

pub struct PropertyValue<RDF: SRDF, S> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>,
}

impl<RDF, S> RDFNodeParse<RDF> for PropertyValue<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = RDF::Term;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        let mut p: PropertyValues<RDF, S> = property_values(&self.property);
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

/// Creates a parser that returns the value associated with the current focus node for `property`
///
/// It doesn't move the current focus node
/// 
/// This method can be used to debug the parser, because it is less efficient as in case that it fails, 
/// it shows the neighbourhood of the current node
pub fn property_value_debug<RDF, S>(property: &IriS) -> PropertyValueDebug<RDF, S>
where
    RDF: SRDF,
{
    let property = RDF::iri_s2iri(property);
    PropertyValueDebug {
        property,
        _marker_rdf: PhantomData,
        _marker_s: PhantomData
    }
}

pub struct PropertyValueDebug<RDF: SRDF, S> {
    property: RDF::IRI,
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>
}

impl<RDF, S> RDFNodeParse<RDF> for PropertyValueDebug<RDF, S>
where
    RDF: FocusRDF + Debug,
    S: Default + Clone
{
    type Output = RDF::Term;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        let mut p: Neighs<RDF, S> = neighs();
        let focus_node_str = match rdf.get_focus() {
            None => "No focus node".to_string(),
            Some(focus_node) => {
                format!("{focus_node:?}")
            }
        };
        let outgoing_arcs = p.parse_impl(rdf)?;
        if let Some(values) = outgoing_arcs.get(&self.property) {
            let mut values_iter = values.into_iter();
            if let Some(value1) = values_iter.next() {
                if let Some(value2) = values_iter.next() {
                    Err(RDFParseError::MoreThanOneValuePredicate {
                        node: format!("{focus_node_str}",),
                        pred: format!("{}", self.property),
                        value1: format!("{value1:?}"),
                        value2: format!("{value2:?}"),
                    })
                } else {
                    Ok(value1.clone())
                }
            } else {
                panic!("Internal error: Node {} has no value for predicate {}...but this case should be handled in the outer else...", focus_node_str, self.property)
            }
        }
        else {
            Err(RDFParseError::NoValuesPredicateDebug {
                node: format!("{focus_node_str}"),
                pred: format!("{}", self.property),
                outgoing_arcs: format!("{outgoing_arcs:?}")
            })
        }
    }
}

/// Creates a parser that returns the value associated with the current focus node for `property`
///
/// It doesn't move the current focus node
/// 
/// This method can be used to debug the parser, because it is less efficient as in case that it fails, 
/// it shows the neighbourhood of the current node
pub fn neighs<RDF, S>() -> Neighs<RDF, S>
where
    RDF: SRDF,
{
    Neighs {
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

pub struct Neighs<RDF: SRDF, S> {
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>,
}

impl<RDF, S> RDFNodeParse<RDF> for Neighs<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = HashMap<RDF::IRI, HashSet<RDF::Term>>;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<HashMap<RDF::IRI, HashSet<RDF::Term>>> {
        match rdf.get_focus() {
            Some(focus) => {
                let subj = RDF::term_as_subject(&focus).ok_or_else(|| 
                    RDFParseError::ExpectedFocusAsSubject { 
                        focus: format!("{focus}") 
                    })?;
                rdf.outgoing_arcs(&subj).map_err(|e| {
                  RDFParseError::Custom { 
                    msg: format!("Error obtaining outgoing arcs from {focus}: {e}") 
                  }
            })
            },
            None => todo!(),
        }
    }
}



/// Returns the integer values of `property` for the focus node
///
/// If there is no value, it returns an empty set
pub fn property_integers<RDF, S>(property: &IriS) -> impl RDFNodeParse<RDF, Output = HashSet<isize>, State = S>   
where 
 RDF: FocusRDF,
 S: Default + Clone 
{
    property_values(&property).flat_map(|terms| {
        let is = terms_to_ints::<RDF>(terms)?;
        Ok(is)
    })
}

/// Returns the integer value of `property` for the focus node
///
pub fn property_integer<RDF, S>(property: &IriS) -> impl RDFNodeParse<RDF, Output = isize, State = S>   
where 
 RDF: FocusRDF,
 S: Default + Clone
{
    property_value(&property).flat_map(|term| {
        let i = term_to_int::<RDF>(&term)?;
        Ok(i)
    })
}

/// Returns the string value of `property` for the focus node
///
pub fn property_string<RDF, S>(property: &IriS) -> impl RDFNodeParse<RDF, Output = String, State = S>   
where 
 RDF: FocusRDF,
 S: Default + Clone
{
    property_value(&property).flat_map(|term| {
        let i = term_to_string::<RDF>(&term)?;
        Ok(i)
    })
}

fn terms_to_ints<RDF>(terms: HashSet<RDF::Term>) -> Result<HashSet<isize>, RDFParseError> 
where RDF: SRDFBasic {
  let ints: HashSet<_> = terms.iter().flat_map(|t| term_to_int::<RDF>(t)).collect();
  Ok(ints)
}

fn term_to_int<RDF>(term: &RDF::Term) -> Result<isize, RDFParseError> 
where RDF: SRDFBasic {
    let n = RDF::term_as_integer(term).ok_or_else(|| 
        RDFParseError::ExpectedInteger { term: format!("{term}")}
    )?;
    Ok(n)
}

fn term_to_iri<RDF>(term: &RDF::Term) -> Result<IriS, RDFParseError> 
where RDF: SRDFBasic {
    let iri = RDF::term_as_iri(term).ok_or_else(|| 
        RDFParseError::ExpectedIRI { term: format!("{term}")}
    )?;
    Ok(RDF::iri2iri_s(&iri))
}

fn term_to_string<RDF>(term: &RDF::Term) -> Result<String, RDFParseError> 
where RDF: SRDFBasic {
    let n = RDF::term_as_string(term).ok_or_else(|| 
        RDFParseError::ExpectedString { term: format!("{term}")}
    )?;
    Ok(n)

}


/// Combines the results of parsers that return vectors of values
///
pub fn combine_vec<RDF, P1, P2, A>(parser1: P1, parser2: P2) -> CombineVec<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = Vec<A>>,
    P2: RDFNodeParse<RDF, Output = Vec<A>>,
{
    CombineVec { parser1, parser2 }
}

pub struct CombineVec<P1, P2> {
    parser1: P1,
    parser2: P2,
}

impl<RDF, P1, P2, A, S> RDFNodeParse<RDF> for CombineVec<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = Vec<A>, State = S>,
    P2: RDFNodeParse<RDF, Output = Vec<A>, State = S>,
    S: Default + Clone
{
    type Output = Vec<A>;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<A>> {
        match self.parser1.parse_impl(rdf) {
            Err(e) => Err(e),
            Ok(vs1) => match self.parser2.parse_impl(rdf) {
                Err(e) => Err(e),
                Ok(vs2) => {
                    let mut result = vs1;
                    result.extend(vs2);
                    Ok(result)
                }
            },
        }
    }
}

/// Parses a node as a bool
///
pub fn bool<RDF, S>() -> impl RDFNodeParse<RDF, Output = bool, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    get_focus().flat_map(|ref term| {
        match RDF::term_as_boolean(term) {
            Some(b) => {
                Ok(b)
            }
            None => {
                Err(RDFParseError::ExpectedBoolean {
                    term: format!("{term}"),
                })
            }
        }
    })
}

/// Parses a node as an RDF List
pub fn rdf_list<RDF, S>() -> RDFList<RDF, S>
where
    RDF: SRDF,
{
    RDFList {
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

/// Creates a parser that returns the focus node
pub fn get_focus<RDF, S>() -> GetFocus<RDF, S>
where
    RDF: FocusRDF,
{
    GetFocus {
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct GetFocus<RDF, S>
where
    RDF: FocusRDF,
{
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>,
}

impl<RDF, S> RDFNodeParse<RDF> for GetFocus<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = RDF::Term;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        match rdf.get_focus() {
            Some(focus) => Ok(focus.clone()),
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Creates a parser that sets the focus node and returns `()`
pub fn set_focus<RDF, S>(node: &RDF::Term) -> SetFocus<RDF, S>
where
    RDF: FocusRDF,
{
    SetFocus {
        node: node.clone(),
        _marker_rdf: PhantomData,
        _marker_s: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct SetFocus<RDF, S>
where
    RDF: FocusRDF,
{
    node: RDF::Term,
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>,
}

impl<RDF, S> RDFNodeParse<RDF> for SetFocus<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = ();
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<()> {
        rdf.set_focus(&self.node);
        Ok(())
    }
}

pub struct RDFList<RDF: SRDF, S> {
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>
}

impl<RDF, S> RDFNodeParse<RDF> for RDFList<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = Vec<RDF::Term>;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<RDF::Term>> {
        let focus = rdf.get_focus_as_term()?;
        let visited = vec![focus.clone()];
        parse_list::<RDF,S>(visited, rdf)
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
    ParseRDFList { parser }
}

#[derive(Copy, Clone)]
pub struct ParseRDFList<P> {
    parser: P,
}

impl<RDF, P, A> RDFNodeParse<RDF> for ParseRDFList<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;
    type State = P::State;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<A>> {
        let focus = rdf.get_focus_as_term()?;
        let visited = vec![focus.clone()];
        parse_list::<RDF,P::State>(visited, rdf).and_then(|nodes| {
            let mut result = Vec::new();
            for node in nodes {
                rdf.set_focus(&node);
                match self.parser.parse_impl(rdf) {
                    Ok(a) => result.push(a),
                    Err(e) => return Err(e),
                }
            }
            Ok(result)
        })
    }
}

// Auxiliary function to parse a node as an RDF list checking that the RDF list if non-cyclic
// by collecting a vector of visited terms
fn parse_list<RDF, S>(
    mut visited: Vec<RDF::Term>,
    rdf: &mut RDF,
) -> Result<Vec<RDF::Term>, RDFParseError>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    let focus = rdf.get_focus_as_term()?;
    if node_is_rdf_nil::<RDF>(focus) {
        Ok(Vec::new())
    } else {
        let value = property_value::<RDF, S>(&RDF_FIRST).parse_impl(rdf)?;
        let rest = property_value::<RDF, S>(&RDF_REST).parse_impl(rdf)?;
        if visited.contains(&&rest) {
            Err(RDFParseError::RecursiveRDFList {
                node: format!("{rest}"),
            })
        } else {
            visited.push(rest.clone());
            let mut rest_ls = vec![value];
            rdf.set_focus(&rest);
            rest_ls.extend(parse_list::<RDF,S>(visited, rdf)?);
            Ok(rest_ls)
        }
    }
}

fn node_is_rdf_nil<RDF>(node: &RDF::Term) -> bool
where
    RDF: SRDF,
{
    if let Some(iri) = RDF::term_as_iri(node) {
        RDF::iri2iri_s(&iri) == *RDF_NIL
    } else {
        false
    }
}

/// Succeeds if current term is the expected IRI
pub fn is_iri<RDF, S>(expected_iri: IriS) -> impl RDFNodeParse<RDF, Output = (), State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    let name = format!("Is {}", expected_iri.as_str());
    satisfy(
        move |node: &RDF::Term| match RDF::term_as_iri(node) {
            Some(iri) => {
                let iri_s = RDF::iri2iri_s(&iri);
                iri_s == expected_iri
            }
            None => false,
        },
        name.as_str(),
    )
}

/// Returns the node that is an instance of the expected IRI in the RDF data
/// It moves the focus to point to that node
pub fn instance_of<RDF, S>(expected: &IriS) -> impl RDFNodeParse<RDF, Output = RDF::Subject>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    instances_of::<RDF, S>(expected).flat_map(|vs| {
        let mut values = vs.into_iter();
        match values.next() {
            Some(value) => match values.next() {
                Some(_other_value) => todo!(),
                None => Ok(value),
            },
            None => todo!(),
        }
    })
}

pub fn set_focus_subject<RDF, S>(subject: RDF::Subject) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    ApplyRDF {
        function: move |rdf: &mut RDF| {
            let term = RDF::subject_as_term(&subject);
            rdf.set_focus(&term);
            Ok(())
        },
        _marker_s: PhantomData::<S>
    }
}

/*pub fn term_as_iri<RDF>(term: RDF::Term) -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRDF,
{
    ApplyRDF {
        function: move |_: &mut RDF| match RDF::object_as_iri(&term) {
            Some(iri) => {
                let iri_s = RDF::iri2iri_s(&iri);
                Ok(iri_s)
            }
            None => todo!(),
        },
    }
}*/

pub fn term_as_iri<RDF, S>(term: &RDF::Term) -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    apply::<RDF, RDF::Term, IriS, S>(term, |term| match &RDF::term_as_iri(term) {
        Some(iri) => {
            let iri_s = RDF::iri2iri_s(&iri);
            Ok(iri_s)
        }
        None => Err(RDFParseError::ExpectedIRI {
            term: format!("{term}"),
        }),
    })
}

/*fn term_as_iri_s<RDF>(term: &RDF::Term) -> PResult<IriS>
where
    RDF: FocusRDF,
{
    let iri = RDF::object_as_iri(term).ok_or_else(|| RDFParseError::Custom {
        msg: "Expected IRI".to_string(),
    })?;
    let iri_s = RDF::iri2iri_s(&iri);
    Ok(iri_s)
}*/

/// Succeeds with a given value
pub fn ok<RDF, A, S>(value: &A) -> impl RDFNodeParse<RDF, Output = A, State = S>
where
    RDF: FocusRDF,
    A: Clone,
    S: Default + Clone
{
    Ok {
        value: value.clone(),
        marker_s: PhantomData
    }
}

#[derive(Debug, Clone)]
struct Ok<A, S> {
    value: A,
    marker_s: PhantomData<S>
}

impl<RDF, A, S> RDFNodeParse<RDF> for Ok<A, S>
where
    RDF: FocusRDF,
    A: Clone,
    S: Default + Clone
{
    type Output = A;
    type State = S;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        Ok(self.value.clone())
    }
}

/// Fails with a given massage
pub fn fail_msg<A, RDF, S>(msg: String) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    Fail {
        msg: msg.clone(),
        _marker: PhantomData,
        _marker_s: PhantomData::<S>
    }
}

#[derive(Debug, Clone)]
struct Fail<A, S> {
    msg: String,
    _marker: PhantomData<A>,
    _marker_s: PhantomData<S>
}

impl<A, RDF, S> RDFNodeParse<RDF> for Fail<A, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = A;
    type State = S;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        Err(RDFParseError::Custom {
            msg: self.msg.clone(),
        })
    }
}

/// Applies a function and returns its result
///
///
pub fn cond<RDF, A, S>(
    value: &A,
    pred: impl FnMut(&A) -> bool,
    fail_msg: String,
) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
    A: Clone,
    S: Default + Clone
{
    Cond {
        value: value.clone(),
        pred,
        fail_msg: fail_msg.clone(),
        _marker_s: PhantomData::<S>
    }
}

#[derive(Debug, Clone)]
struct Cond<A, P, S> {
    value: A,
    pred: P,
    fail_msg: String,
    _marker_s: PhantomData<S>
}

impl<RDF, A, P, S> RDFNodeParse<RDF> for Cond<A, P, S>
where
    RDF: FocusRDF,
    P: FnMut(&A) -> bool,
    A: Clone,
    S: Default + Clone
{
    type Output = ();
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match (self.pred)(&self.value) {
            true => Ok(()),
            false => Err(RDFParseError::Custom {
                msg: self.fail_msg.clone(),
            }),
        }
    }
}

/// Applies a function and returns its result
pub fn apply<RDF, A, B, S>(
    value: &A,
    function: impl FnMut(&A) -> Result<B, RDFParseError>,
) -> impl RDFNodeParse<RDF, Output = B>
where
    RDF: FocusRDF,
    A: Clone,
    S: Default + Clone
{
    Apply {
        value: value.clone(),
        function,
        _marker_s: PhantomData::<S>
    }
}

#[derive(Debug, Clone)]
struct Apply<A, F, S> {
    value: A,
    function: F,
    _marker_s: PhantomData<S>
}

impl<RDF, A, B, F, S> RDFNodeParse<RDF> for Apply<A, F, S>
where
    RDF: FocusRDF,
    F: FnMut(&A) -> Result<B, RDFParseError>,
    A: Clone,
    S: Default + Clone
{
    type Output = B;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match (self.function)(&self.value) {
            Ok(b) => Ok(b),
            Err(e) => Err(e),
        }
    }
}

/// Applies a function over the RDF graph and returns the result of that function
pub fn apply_rdf<RDF, A, S>(
    function: impl FnMut(&mut RDF) -> Result<A, RDFParseError>,
) -> impl RDFNodeParse<RDF, Output = A, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    ApplyRDF { function, _marker_s: PhantomData }
}

#[derive(Debug, Clone)]
struct ApplyRDF<F, S> {
    function: F,
    _marker_s: PhantomData<S>
}

impl<RDF, A, F, S> RDFNodeParse<RDF> for ApplyRDF<F, S>
where
    RDF: FocusRDF,
    F: FnMut(&mut RDF) -> Result<A, RDFParseError>,
    S: Default + Clone
{
    type Output = A;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match (self.function)(rdf) {
            Ok(a) => Ok(a),
            Err(e) => Err(e),
        }
    }
}

pub fn get_state<RDF, S>(
) -> impl RDFNodeParse<RDF, Output = S> 
where
    RDF: FocusRDF,
    S: Default + Clone
{
    GetState {
        _marker_rdf: PhantomData,
        _marker_s: PhantomData
    }
}

#[derive(Debug, Clone)]
pub struct GetState<RDF, S> 
{
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>
}

impl<RDF, S> RDFNodeParse<RDF> for GetState<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = S;
    type State = S;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        let state = self.get_state();
        Ok(state)
    }

}


pub fn set_state<RDF, S>(
    state: S,
) -> impl RDFNodeParse<RDF, Output = ()> 
where
    RDF: FocusRDF,
    S: Default + Clone
{
    SetState {
        state
    }
}

#[derive(Debug, Clone)]
pub struct SetState<S> 
{
    state: S,
}

impl<RDF, S> RDFNodeParse<RDF> for SetState<S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = ();
    type State = S;

    fn parse_impl_state(&mut self, _rdf: &mut RDF, state: &mut S) -> PResult<(Self::Output, Self::State)> {
        *state = self.state.clone();
        Ok(((), state.clone()))
    }

}

/// Changes the current applying a state changing function
/*pub fn change_state<RDF, S>(
    function: impl FnMut(&mut S) -> S,
) -> impl RDFNodeParse<RDF, Output = S> 
where
    RDF: FocusRDF,
    S: Default
{
    ChangeState {
        function,
        _marker_s: PhantomData::<S>
    }
}

#[derive(Debug, Clone)]
pub struct ChangeState<F, S> 
where F: FnMut(&mut S) -> S,
{
    function: F,
    state: S,
    _marker_s: PhantomData<S>,
}

impl<RDF, F, S> RDFNodeParse<RDF> for ChangeState<F, S>
where
    RDF: FocusRDF,
    F: FnMut(&mut S) -> S,
    S: Default
{
    type Output = S;
    type State = S;

    fn parse_impl_state(&mut self, _rdf: &mut RDF, state: &mut S) -> PResult<Self::Output> {
        let new_state = (self.function)(state);
        Ok(new_state)
    }

    fn set_state(&mut self, state: S) {
        self.state = state;
    }
}
*/

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn instances_of<RDF, S>(expected: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Subject>>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    let term = RDF::iri_s2term(expected);
    subjects_with_property_value::<RDF, S>(&RDF_TYPE, &term)
}

pub fn parse_rdf_type<RDF, S>() -> impl RDFNodeParse<RDF, Output = RDF::Term, State = S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    property_value(&RDF_TYPE)
}

pub fn has_type<RDF, S>(expected: IriS) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    parse_rdf_type::<RDF, S>().then(move |term: RDF::Term| 
        equals::<RDF,S>(term.clone(), RDF::iri_s2term(&expected))
    )
}

pub fn equals<RDF, S>(term: RDF::Term, expected: RDF::Term) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    let expected_str = format!("{expected}");
    cond::<RDF, RDF::Term, S>(
        &term,
        move |ref t| RDF::term_as_object(*t) == RDF::term_as_object(&expected),
        format!("Term {term} not equals {}", expected_str),
    )
}

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn subjects_with_property_value<RDF, S>(
    property: &IriS,
    value: &RDF::Term,
) -> SubjectsPropertyValue<RDF, S>
where
    RDF: FocusRDF,
    S: Default
{
    let iri = RDF::iri_s2iri(property);
    SubjectsPropertyValue {
        property: iri,
        value: value.clone(),
        _marker_rdf: PhantomData,
        _marker_s: PhantomData
    }
}

pub struct SubjectsPropertyValue<RDF: SRDF, S> {
    property: RDF::IRI,
    value: RDF::Term,
    _marker_rdf: PhantomData<RDF>,
    _marker_s: PhantomData<S>,
}

impl<RDF, S> RDFNodeParse<RDF> for SubjectsPropertyValue<RDF, S>
where
    RDF: FocusRDF,
    S: Default + Clone
{
    type Output = Vec<RDF::Subject>;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<RDF::Subject>> {
        let subjects = rdf
            .subjects_with_predicate_object(&self.property, &self.value)
            .map_err(|e| RDFParseError::ErrorSubjectsPredicateObject {
                property: format!("{}", self.property),
                value: format!("{}", self.value),
                err: e.to_string(),
            })?;
        let mut result = Vec::new();
        for s in subjects {
            result.push(s)
        }
        Ok(result)
    }
}

rdf_parser! {
    /// Parses the value of `property` as an RDF list
    pub fn parse_property_value_as_list['a, RDF, S](property: &'a IriS)(RDF)(S) -> Vec<RDF::Term>
        where [
        ] {
            property_value::<RDF,S>(&property)
            .then(|node|
                set_focus::<RDF,S>(&node).then(|_|
                    rdf_list())
             )
    }
}

/// Apply a parser to an RDF node associated with the value of it's `rdf:type` property
pub fn parse_by_type<'a, RDF, P, A, S>(
    values: Vec<(IriS, P)>,
    default: P,
) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A, State = S>,
    S: Default + Clone
{
    ParseByType {
        values: HashMap::from_iter(values.into_iter()),
        default,
    }
}

pub struct ParseByType<I, P> {
    values: HashMap<I, P>,
    default: P,
}

impl<RDF, P, A, S> RDFNodeParse<RDF> for ParseByType<IriS, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A, State = S>,
    S: Default + Clone
{
    type Output = A;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        let rdf_type = parse_rdf_type::<RDF, S>().parse_impl(rdf)?;
        let iri_type = match RDF::term_as_iri(&rdf_type) {
            Some(iri) => RDF::iri2iri_s(&iri),
            None => {
                return Err(RDFParseError::ExpectedIRI {
                    term: format!("{rdf_type}"),
                })
            }
        };
        match self.values.get_mut(&iri_type) {
            Some(p) => p.parse_impl(rdf),
            None => self.default.parse_impl(rdf),
        }
    }
}

/// Equivalent to [`parser1.with(parser2)`]
///
/// Discards the value of the first parser and returns the value of the second parser
///
pub fn with<RDF, P1, P2>(parser1: P1, parser2: P2) -> With<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF>,
    P2: RDFNodeParse<RDF>,
{
    With { parser1, parser2 }
}

#[derive(Copy, Clone)]
pub struct With<P1, P2> {
    parser1: P1,
    parser2: P2,
}

impl<RDF, A, B, P1, P2, S> RDFNodeParse<RDF> for With<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = A, State = S>,
    P2: RDFNodeParse<RDF, Output = B, State = S>,
    S: Default + Clone
{
    type Output = B;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser1.parse_impl(rdf) {
            Ok(a) => match self.parser2.parse_impl(rdf) {
                Ok(b) => Ok(b),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

/// Applies a parser over a list of nodes and returns the list of values
///
pub fn parse_nodes<RDF, P>(nodes: Vec<RDF::Term>, parser: P) -> ParserNodes<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
{
    ParserNodes { nodes, parser }
}

#[derive(Clone)]
pub struct ParserNodes<RDF, P>
where
    RDF: FocusRDF,
{
    nodes: Vec<RDF::Term>,
    parser: P,
}

impl<RDF, A, P, S> RDFNodeParse<RDF> for ParserNodes<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A, State = S>,
    S: Default + Clone
{
    type Output = Vec<A>;
    type State = S;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        let mut results = Vec::new();
        for node in self.nodes.iter() {
            rdf.set_focus(&node);
            let value = self.parser.parse_impl(rdf)?;
            results.push(value)
        }
        Ok(results)
    }
}

