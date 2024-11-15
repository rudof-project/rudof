use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;

use iri_s::IriS;

use crate::model::focus_rdf::FocusRdf;
use crate::model::rdf::Literal;
use crate::model::rdf::Object;
use crate::model::rdf::Predicate;
use crate::model::rdf::Rdf;
use crate::model::rdf::Subject;
use crate::model::Iri;
use crate::model::Literal as _;
use crate::model::Term as _;
use crate::model::Triple;
use crate::rdf_parser;
use crate::RDF_FIRST;
use crate::RDF_NIL;
use crate::RDF_NIL_STR;
use crate::RDF_REST;
use crate::RDF_TYPE;

use super::ParserResult;
use super::RDFParseError;

/// By implementing the `RDFNodeParse` trait a type says that it can be used for
/// parsing RDF data having a focus node. RDF data with a focus node has to
/// implement the [`FocusRdf`] trait.
pub trait RDFNodeParse<RDF: FocusRdf> {
    /// The type which is returned if the parser is successful.
    type Output;

    /// Entry point to the parser. It moves the focus node of `rdf` to `node`
    /// and runs the parser. Returns the parsed result if the parser succeeds,
    /// or an error otherwise.
    #[inline(always)]
    fn parse(&mut self, node: &IriS, mut rdf: RDF) -> ParserResult<Self::Output> {
        let iri = Predicate::<RDF>::new(node.as_str());
        rdf.set_focus(iri.into());
        self.parse_impl(&mut rdf)
    }

    #[inline(always)]
    fn by_ref(&mut self) -> ByRef<'_, Self>
    where
        Self: core::marker::Sized,
    {
        ByRef::new(self)
    }

    /// Parses the current focus node without modifying the state
    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output>;

    /// Uses `f` to map over the output of `self`. If `f` returns an error the
    /// parser fails.
    ///
    /// ```
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use srdf::{RDFNodeParse, RDFParseError, ParserResult};
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::Iri;
    /// use srdf::model::parse::ReaderMode;
    /// use srdf::property_string;
    /// use srdf::model::parse::RdfParse;
    /// use oxrdf::NamedNode as OxNamedNode;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p "1" .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///
    /// let x = OxNamedNode::new_unchecked("http://example.org/x").as_iri_s();
    /// let p = OxNamedNode::new_unchecked("http://example.org/p").as_iri_s();
    ///
    /// fn cnv_int(s: String) -> ParserResult<isize> {
    ///     s.parse().map_err(|_| RDFParseError::Custom{ msg: format!("Error converting {s}")})
    /// }
    ///
    /// let mut parser = property_string(&p).flat_map(cnv_int);
    /// assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    /// ```
    fn flat_map<F, O>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> ParserResult<O>,
    {
        flat_map(self, f)
    }

    /// Parses with `self` and applies `f` on the result if `self` parses
    /// successfully. `f` may optionally fail with an error which is
    /// automatically converted to a `RDFParseError`.
    ///
    /// ```
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use srdf::{RDFNodeParse, RDFParseError, property_string};
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::Iri;
    /// use srdf::model::parse::ReaderMode;
    /// use srdf::model::parse::RdfParse;
    /// use oxrdf::NamedNode as OxNamedNode;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p "1" .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///
    /// let x = OxNamedNode::new_unchecked("http://example.org/x").as_iri_s();
    /// let p = OxNamedNode::new_unchecked("http://example.org/p").as_iri_s();
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
    /// let mut parser = property_string(&p).and_then(cnv_int);
    /// assert_eq!(parser.parse(&x, graph).unwrap(), 1)
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
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use srdf::{RDFNodeParse, property_integer};
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::Iri;
    /// use srdf::model::parse::ReaderMode;
    /// use srdf::model::parse::RdfParse;
    /// use oxrdf::NamedNode as OxNamedNode;
    /// use srdf::property_string;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p 1 .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    /// let p = OxNamedNode::new_unchecked("http://example.org/p").as_iri_s();
    /// let mut parser = property_integer(&p).map(|n| n + 1);
    /// assert_eq!(parser.parse(&OxNamedNode::new_unchecked("http://example.org/x").as_iri_s(), graph).unwrap(), 2)
    /// ```
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> B,
    {
        map(self, f)
    }

    /// Parses `self` followed by `p`. Succeeds if both parsers succeed,
    /// otherwise fails. Returns a tuple with both values on success.
    ///
    /// ```
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use srdf::{RDFNodeParse, property_bool, property_integer};
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::parse::ReaderMode;
    /// use srdf::model::parse::RdfParse;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p true ;
    ///     :q 1    .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///
    /// let x = IriS::new_unchecked("http://example.org/x".to_string());
    /// let p = IriS::new_unchecked("http://example.org/p".to_string());
    /// let q = IriS::new_unchecked("http://example.org/q".to_string());
    ///
    /// let mut parser = property_bool(&p).and(property_integer(&q));
    /// assert_eq!(parser.parse(&x, graph).unwrap(), (true, 1))
    /// ```
    fn and<P2>(self, parser: P2) -> (Self, P2)
    where
        Self: Sized,
        P2: RDFNodeParse<RDF>,
    {
        (self, parser)
    }

    /// Parses using `self` and then passes the value to `f` which returns a
    /// parser used to parse the rest of the input.
    fn then<N, F>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> N,
        N: RDFNodeParse<RDF>,
    {
        then(self, f)
    }

    /// Parses using `self` and then passes a reference to the value to `f`
    /// which returns a parser used to parse the rest of the input.
    fn then_ref<N, F>(self, f: F) -> ThenRef<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Output) -> N,
        N: RDFNodeParse<RDF>,
    {
        then_ref(self, f)
    }

    /// Parses using `self` and then passes a reference to the mutable value to
    /// `f` which returns a parser used to parse the rest of the input.
    ///
    /// ```
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use oxrdf::Term;
    /// use std::collections::HashSet;
    /// use srdf::{RDFNodeParse, ok, property_integers};
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::parse::ReaderMode;
    /// use srdf::model::parse::RdfParse;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p 1, 2, 3 .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///
    /// let x = IriS::new_unchecked("http://example.org/x".to_string());
    /// let p = IriS::new_unchecked("http://example.org/p".to_string());
    ///
    /// let mut parser = property_integers(&p).then_mut(
    ///     move |ns| {
    ///         ns.extend(vec![4, 5]);
    ///         ok(ns)
    ///      }
    /// );
    ///
    /// assert_eq!(parser.parse(&x, graph).unwrap(), HashSet::from([1, 2, 3, 4, 5]))
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
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use srdf::{RDFNodeParse, property_bool};
    /// use srdf::model::parse::ReaderMode;
    /// use std::collections::HashSet;
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::parse::RdfParse;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p 1, 2 ;
    ///     :q true .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///
    /// let x = IriS::new_unchecked("http://example.org/x".to_string());
    /// let p = IriS::new_unchecked("http://example.org/p".to_string());
    /// let q = IriS::new_unchecked("http://example.org/q".to_string());
    ///
    /// let mut parser = property_bool(&p).or(property_bool(&q));
    /// assert_eq!(parser.parse(&x, graph).unwrap(), true)
    /// ```
    fn or<P2>(self, parser: P2) -> Or<Self, P2>
    where
        Self: Sized,
        P2: RDFNodeParse<RDF, Output = Self::Output>,
    {
        or(self, parser)
    }

    /// Sets the focus node and returns ()
    fn focus(self, node: &Object<RDF>) -> SetFocus<RDF>
    where
        Self: Sized,
    {
        set_focus(node)
    }

    /// Discards the value of the current parser and returns the value of `parser`
    ///
    /// ```
    /// use iri_s::IriS;
    /// use srdf::oxgraph::OxGraph;
    /// use oxrdf::Term;
    /// use std::collections::HashSet;
    /// use srdf::{RDFNodeParse, ok, property_integers};
    /// use srdf::model::rdf_format::RdfFormat;
    /// use srdf::model::parse::ReaderMode;
    /// use srdf::model::parse::RdfParse;
    /// use srdf::property_value;
    ///
    /// let s = r#"
    ///     prefix : <http://example.org/>
    ///     :x :p :y .
    /// "#;
    ///
    /// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    /// let p = IriS::new_unchecked("http://example.org/p".to_string());
    /// let x = IriS::new_unchecked("http://example.org/x".to_string());
    /// assert_eq!(property_value(&p).with(ok(&1)).parse(&x, graph).unwrap(), 1)
    /// ```
    fn with<P, A>(self, parser: P) -> With<Self, P>
    where
        Self: Sized,
        P: RDFNodeParse<RDF, Output = A>,
    {
        with(self, parser)
    }
}

impl<RDF, P1, P2, A, B> RDFNodeParse<RDF> for (P1, P2)
where
    RDF: FocusRdf,
    P1: RDFNodeParse<RDF, Output = A>,
    P2: RDFNodeParse<RDF, Output = B>,
{
    type Output = (A, B);

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
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
pub fn map<RDF, P, F, B>(parser: P, f: F) -> Map<P, F>
where
    RDF: FocusRdf,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = A>,
    F: FnMut(A) -> B,
{
    type Output = B;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(a) => Ok((self.f)(a)),
            Err(e) => Err(e),
        }
    }
}

pub fn and_then<RDF, P, F, O, E>(parser: P, function: F) -> AndThen<P, F>
where
    RDF: FocusRdf,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> Result<O, E>,
    E: Into<RDFParseError>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> ParserResult<O>,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> ParserResult<O>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
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
    RDF: FocusRdf,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
{
    type Output = Option<P::Output>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(value) => Ok(Some(value)),
            Err(_err) => Ok(None),
        }
    }
}

/// Equivalent to [`parser1.or(parser2)`].
///
/// [`parser1.or(parser2)`]: trait.RDFNodeParse.html#method.or
pub fn or<RDF, P1, P2>(parser1: P1, parser2: P2) -> Or<P1, P2>
where
    RDF: FocusRdf,
    P1: RDFNodeParse<RDF>,
    P2: RDFNodeParse<RDF>,
{
    Or { parser1, parser2 }
}

#[derive(Copy, Clone)]
pub struct Or<P1, P2> {
    parser1: P1,
    parser2: P2,
}

impl<RDF, P1, P2, O> RDFNodeParse<RDF> for Or<P1, P2>
where
    RDF: FocusRdf,
    P1: RDFNodeParse<RDF, Output = O>,
    P2: RDFNodeParse<RDF, Output = O>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match self.parser1.parse_impl(rdf) {
            Ok(value) => Ok(value),
            Err(err1) => match self.parser2.parse_impl(rdf) {
                Ok(value) => Ok(value),
                Err(err2) => Err(RDFParseError::FailedOr {
                    err1: Box::new(err1),
                    err2: Box::new(err2),
                }),
            },
        }
    }
}

/// Equivalent to [`p.then(f)`].
///
/// [`p.then(f)`]: trait.RDFNodeParse.html#method.then
pub fn then<RDF, P, F, N>(parser: P, function: F) -> Then<P, F>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
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
    RDF: FocusRdf,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
    F: FnMut(&P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
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
    RDF: FocusRdf,
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
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
    F: FnMut(&mut P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(mut value) => (self.function)(&mut value).parse_impl(rdf),
            Err(err) => Err(err),
        }
    }
}

/// Not parser succeeds if the `parser` fails and viceversa.
///
/// ## Example:
/// ```
/// use iri_s::IriS;
/// use srdf::oxgraph::OxGraph;
/// use srdf::{literal, not, RDFNodeParse};
/// use oxrdf::NamedNode as OxNamedNode;
/// use srdf::model::rdf_format::RdfFormat;
/// use srdf::model::Iri;
///
/// let graph = OxGraph::default();
/// let x = OxNamedNode::new_unchecked("http://example.org/x".to_string()).as_iri_s();
/// assert_eq!(not(literal()).parse(&x, graph).unwrap(), ())
/// ```
pub fn not<RDF, P>(parser: P) -> Not<P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
{
    Not { parser }
}

#[derive(Copy, Clone)]
pub struct Not<P> {
    parser: P,
}

impl<RDF, P, O> RDFNodeParse<RDF> for Not<P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = O>,
    O: Debug,
{
    type Output = ();

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Err(_err) => Ok(()),
            Ok(value) => Err(RDFParseError::FailedNot {
                value: format!("{value:?}"),
            }),
        }
    }
}

/// Checks if the focus node is an IRI
/// ```
/// use iri_s::IriS;
/// use srdf::{iri, RDFNodeParse};
/// use srdf::oxgraph::OxGraph;
/// use oxrdf::NamedNode as OxNamedNode;
/// use crate::srdf::model::Iri;
///
/// let graph = OxGraph::default();
/// let x = OxNamedNode::new_unchecked("http://example.org/x").as_iri_s();
/// assert_eq!(iri().parse(&x, graph).unwrap(), x)
/// ```
pub fn iri<RDF: FocusRdf>() -> impl RDFNodeParse<RDF, Output = IriS> {
    term().flat_map(|t: Object<RDF>| match t.as_iri() {
        None => Err(RDFParseError::ExpectedIRI {
            term: format!("{t}"),
        }),
        Some(v) => Ok(v.as_iri_s()),
    })
}

/// Checks if the focus node is an IRI
/// ```
/// use iri_s::IriS;
/// use srdf::{iri, RDFNodeParse};
/// use srdf::oxgraph::OxGraph;
/// use oxrdf::NamedNode as OxNamedNode;
/// use crate::srdf::model::Iri;
///
/// let graph = OxGraph::default();
/// let x = OxNamedNode::new_unchecked("http://example.org/x").as_iri_s();
/// assert_eq!(iri().parse(&x, graph).unwrap(), x)
/// ```
pub fn literal<RDF: FocusRdf>() -> impl RDFNodeParse<RDF, Output = Literal<RDF::Triple>> {
    term().flat_map(|ref t: Object<RDF>| match t.as_literal() {
        Some(literal) => Ok(literal.to_owned()),
        None => Err(RDFParseError::ExpectedLiteral {
            term: format!("{t}"),
        }),
    })
}

#[derive(Debug, Clone)]
pub struct Term<RDF> {
    _marker_rdf: PhantomData<RDF>,
}

/// Creates a parser that returns the current focus node as a term. This is
/// equivalent to [`get_focus`].
pub fn term<RDF: FocusRdf>() -> Term<RDF> {
    Term {
        _marker_rdf: PhantomData,
    }
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for Term<RDF> {
    type Output = Object<RDF>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Object<RDF>> {
        match rdf.get_focus() {
            Some(focus) => Ok(focus.clone()),
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Parses the RDF list linked from the value of property `prop` at focus node
pub fn property_list<RDF: FocusRdf>(
    prop: &IriS,
) -> impl RDFNodeParse<RDF, Output = Vec<Object<RDF>>> {
    property_value(prop).and(rdf_list()).map(|(_, ls)| ls)
}

/// Created a parser that returns the boolean associated with the current focus
/// node for `property`. It doesn't move the current focus node.
pub fn property_bool<RDF: FocusRdf>(prop: &IriS) -> impl RDFNodeParse<RDF, Output = bool> {
    property_value(prop).flat_map(|ref term: Object<RDF>| match term.as_literal() {
        None => Err(RDFParseError::ExpectedLiteral {
            term: format!("{term}"),
        }),
        Some(literal) => match literal.as_bool() {
            Some(bool) => Ok(bool),
            None => Err(RDFParseError::ExpectedBoolean {
                term: format!("{term}"),
            }),
        },
    })
}

pub fn parse_rdf_nil<RDF: FocusRdf>() -> impl RDFNodeParse<RDF, Output = ()> {
    satisfy(
        |node: &Object<RDF>| match node.as_iri() {
            Some(iri) => {
                let iri_s = iri.as_iri_s();
                iri_s.as_str() == RDF_NIL_STR
            }
            None => false,
        },
        "rdf_nil",
    )
}

/// Creates a parser that checks if the current node satisfies a predicate. The
/// `predicate_name` argument is useful in case of failure to know which
/// condition has failed.
pub fn satisfy<RDF, P>(predicate: P, predicate_name: &str) -> Satisfy<RDF, P>
where
    RDF: Rdf,
    P: FnMut(&Object<RDF>) -> bool,
{
    Satisfy {
        predicate,
        predicate_name: predicate_name.to_string(),
        _marker_rdf: PhantomData,
    }
}

#[derive(Clone)]
pub struct Satisfy<RDF, P> {
    predicate: P,
    predicate_name: String,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF, P> RDFNodeParse<RDF> for Satisfy<RDF, P>
where
    RDF: FocusRdf,
    P: FnMut(&Object<RDF>) -> bool,
{
    type Output = ();

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<()> {
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

/// Return the integer values of `property` for the focus node. If some value
/// is not an integer it fails, if there is no value returns an empty set.
pub fn property_values_int<RDF: FocusRdf>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = Vec<isize>> {
    property_values(property).flat_map(|values| {
        let ints: Vec<_> = values
            .iter()
            .flat_map(|t: &Object<RDF>| match t.as_literal() {
                Some(literal) => match literal.as_int() {
                    Some(int) => Ok(int),
                    None => Err(RDFParseError::ExpectedInteger {
                        term: format!("{t}"),
                    }),
                },
                None => Err(RDFParseError::ExpectedLiteral {
                    term: format!("{t}"),
                }),
            })
            .collect();
        Ok(ints)
    })
}

/// Return the IRI values of `property` for the focus node. If some value is not
///  an IRI it fails, if there is no value returns an empty set.
pub fn property_values_iri<RDF: FocusRdf>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = Vec<IriS>> {
    property_values(property).flat_map(|values| {
        let ints: Vec<_> = values
            .iter()
            .flat_map(|t: &Object<RDF>| match t.as_iri() {
                Some(iri) => Ok::<IriS, RDFParseError>(iri.as_iri_s()),
                None => todo!(),
            })
            .collect();
        Ok(ints)
    })
}

/// Returns the values of `property` for the focus node. If there is no value,
/// it returns an error.
pub fn property_values_non_empty<RDF>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = HashSet<Object<RDF>>>
where
    RDF: FocusRdf,
{
    property_values(property).and_then(move |vs| {
        if vs.is_empty() {
            Err(RDFParseError::Custom {
                msg: "Property values are empty".to_string(),
            })
        } else {
            Ok(vs)
        }
    })
}

/// Returns the values of `property` for the focus node- If there is no value,
///  it returns an empty set
pub fn property_values<RDF>(property: &IriS) -> PropertyValues<RDF>
where
    RDF: FocusRdf,
{
    PropertyValues {
        property: property.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValues<RDF: FocusRdf> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for PropertyValues<RDF> {
    type Output = HashSet<Object<RDF>>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<HashSet<Object<RDF>>> {
        let subject = match rdf.get_focus() {
            Some(focus) => match focus.clone().try_into() {
                Ok(subject) => subject,
                Err(_) => {
                    return Err(RDFParseError::ExpectedFocusAsSubject {
                        focus: focus.to_string(),
                    })
                }
            },
            None => return Err(RDFParseError::NoFocusNode),
        };
        let pred = Predicate::<RDF>::new(self.property.as_str());
        let triples = match rdf.triples_matching(Some(&subject), Some(&pred), None) {
            Result::Ok(triples) => triples,
            Err(_) => {
                return Err(RDFParseError::SRDFError {
                    err: "Error obtaining the triples".to_string(),
                })
            }
        };
        let values = triples
            .map(Triple::obj)
            .map(Clone::clone)
            .collect::<HashSet<_>>();
        Ok(values)
    }
}

/// Creates a parser that returns the value associated with the current focus
/// node for `property`. It doesn't move the current focus node
pub fn property_value<RDF: Rdf>(property: &IriS) -> PropertyValue<RDF> {
    PropertyValue {
        property: property.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValue<RDF: Rdf> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for PropertyValue<RDF> {
    type Output = Object<RDF>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Object<RDF>> {
        let mut p: PropertyValues<RDF> = property_values(&self.property);
        let focus_node_str = match rdf.get_focus() {
            None => "No focus node".to_string(),
            Some(focus_node) => {
                format!("{focus_node}")
            }
        };
        let mut values_iter = p.parse_impl(rdf)?.into_iter();
        if let Some(value1) = values_iter.next() {
            // TODO: Add the following for strict parsing an rdf-list
            /* if let Some(value2) = values_iter.next() {
                Err(RDFParseError::MoreThanOneValuePredicate {
                    node: focus_node_str.to_string(),
                    pred: format!("{}", self.property),
                    value1: format!("{value1:?}"),
                    value2: format!("{value2:?}"),
                })
            } else { */
            Ok(value1)
            /* } */
        } else {
            Err(RDFParseError::NoValuesPredicate {
                node: focus_node_str.to_string(),
                pred: format!("{}", self.property),
            })
        }
    }
}

/// Creates a parser that returns the value associated with the current focus
/// node for `property`.
///
/// It doesn't move the current focus node
///
/// This method can be used to debug the parser, because it is less efficient
/// as in case that it fails, it shows the neighbourhood of the current node
pub fn property_value_debug<RDF: Rdf>(property: &IriS) -> PropertyValueDebug<RDF> {
    let property = Predicate::<RDF>::new(property.as_str());
    PropertyValueDebug {
        property,
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValueDebug<RDF: Rdf> {
    property: Predicate<RDF>,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf + Debug> RDFNodeParse<RDF> for PropertyValueDebug<RDF> {
    type Output = Object<RDF>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Object<RDF>> {
        let mut p: Neighs<RDF> = neighs();
        let focus_node_str = match rdf.get_focus() {
            None => "No focus node".to_string(),
            Some(focus_node) => {
                format!("{focus_node}")
            }
        };
        let outgoing_arcs = p.parse_impl(rdf)?;
        if let Some(values) = outgoing_arcs.get(&self.property) {
            let mut values_iter = values.iter();
            if let Some(value1) = values_iter.next() {
                if let Some(value2) = values_iter.next() {
                    Err(RDFParseError::MoreThanOneValuePredicate {
                        node: focus_node_str.to_string(),
                        pred: format!("{}", self.property),
                        value1: format!("{value1}"),
                        value2: format!("{value2}"),
                    })
                } else {
                    Ok(value1.clone())
                }
            } else {
                panic!("Internal error: Node {} has no value for predicate {}...but this case should be handled in the outer else...", focus_node_str, self.property)
            }
        } else {
            Err(RDFParseError::NoValuesPredicateDebug {
                node: focus_node_str.to_string(),
                pred: format!("{}", self.property),
                outgoing_arcs: format!("{outgoing_arcs:?}"),
            })
        }
    }
}

pub struct Neighs<RDF: Rdf> {
    _marker_rdf: PhantomData<RDF>,
}

/// Creates a parser that returns the value associated with the current focus
/// node for `property`. It doesn't move the current focus node.
///
/// This method can be used to debug the parser, because it is less efficient
/// as in case that it fails, it shows the neighbourhood of the current node.
pub fn neighs<RDF: Rdf>() -> Neighs<RDF> {
    Neighs {
        _marker_rdf: PhantomData,
    }
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for Neighs<RDF> {
    type Output = HashMap<Predicate<RDF>, HashSet<Object<RDF>>>;

    fn parse_impl(
        &mut self,
        rdf: &mut RDF,
    ) -> ParserResult<HashMap<Predicate<RDF>, HashSet<Object<RDF>>>> {
        match rdf.get_focus() {
            Some(focus) => {
                let subj = match focus.clone().try_into() {
                    Ok(subject) => subject,
                    Err(_) => {
                        return Err(RDFParseError::ExpectedSubject {
                            node: format!("{focus}"),
                        })
                    }
                };
                rdf.outgoing_arcs(&subj).map_err(|e| RDFParseError::Custom {
                    msg: format!("Error obtaining outgoing arcs from {focus}: {e}"),
                })
            }
            None => todo!(),
        }
    }
}

/// Returns the integer values of `property` for the focus node. If there is no
/// value, it returns an empty set.
pub fn property_integers<RDF: FocusRdf>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = HashSet<isize>> {
    property_values(property).flat_map(|terms| {
        let values = terms
            .iter()
            .flat_map(|term: &Object<RDF>| match term.as_literal() {
                Some(literal) => match literal.as_int() {
                    Some(int) => Ok(int),
                    None => Err(RDFParseError::ExpectedInteger {
                        term: format!("{term}"),
                    }),
                },
                None => Err(RDFParseError::ExpectedLiteral {
                    term: format!("{term}"),
                }),
            })
            .collect();
        Ok(values)
    })
}

/// Returns the IRI value of `property` for the focus node
pub fn property_iri<'a, RDF: FocusRdf + 'a>(
    property: &'a IriS,
) -> impl RDFNodeParse<RDF, Output = IriS> + 'a {
    get_focus().then(move |focus| {
        property_value(property).flat_map(move |term: Object<RDF>| match term.as_iri() {
            Some(iri) => Ok(iri.as_iri_s()),
            None => Err(RDFParseError::PropertyValueExpectedIRI {
                focus: format!("{focus}"),
                property: property.clone(),
                error: RDFParseError::ExpectedIRI {
                    term: term.to_string(),
                }
                .to_string(),
            }),
        })
    })
}

/// Returns the integer value of `property` for the focus node
pub fn property_integer<RDF: FocusRdf>(property: &IriS) -> impl RDFNodeParse<RDF, Output = isize> {
    property_value(property).flat_map(|term: Object<RDF>| match term.as_literal() {
        Some(literal) => match literal.as_int() {
            Some(int) => Ok(int),
            None => Err(RDFParseError::ExpectedInteger {
                term: format!("{term}"),
            }),
        },
        None => Err(RDFParseError::ExpectedLiteral {
            term: format!("{term}"),
        }),
    })
}

/// Returns the string value of `property` for the focus node
pub fn property_string<RDF: FocusRdf>(property: &IriS) -> impl RDFNodeParse<RDF, Output = String> {
    property_value(property).flat_map(|term: Object<RDF>| match term.as_literal() {
        Some(literal) => Ok(literal.as_string().unwrap()),
        None => Err(RDFParseError::ExpectedLiteral {
            term: format!("{term}"),
        }),
    })
}

// fn terms_to_ints<T>(terms: HashSet<T::Term>) -> Result<HashSet<isize>, RDFParseError>
// where
//     T: Triple,
// {
//     let ints: HashSet<_> = terms.iter().flat_map(|t| term_to_int::<T>(t)).collect();
//     Ok(ints)
// }

// fn term_to_int<T>(term: &T::Term) -> Result<isize, RDFParseError>
// where
//     T: Triple,
// {
//     let n = T::term_as_integer(term).ok_or_else(|| RDFParseError::ExpectedInteger {
//         term: format!("{term}"),
//     })?;
//     Ok(n)
// }

// fn term_to_iri<T>(term: &T::Term) -> Result<IriS, RDFParseError>
// where
//     T: Triple,
// {
//     let iri = T::term_as_iri(term).ok_or_else(|| RDFParseError::ExpectedIRI {
//         term: format!("{term}"),
//     })?;
//     Ok(T::iri2iri_s(iri))
// }

// fn term_to_string<T>(term: &T::Term) -> Result<String, RDFParseError>
// where
//     T: Triple,
// {
//     let n = T::term_as_string(term).ok_or_else(|| RDFParseError::ExpectedString {
//         term: format!("{term}"),
//     })?;
//     Ok(n)
// }

/// Combines the results of parsers that return vectors of values
///
pub fn combine_vec<RDF, P1, P2, A>(parser1: P1, parser2: P2) -> CombineVec<P1, P2>
where
    RDF: FocusRdf,
    P1: RDFNodeParse<RDF, Output = Vec<A>>,
    P2: RDFNodeParse<RDF, Output = Vec<A>>,
{
    CombineVec { parser1, parser2 }
}

pub struct CombineVec<P1, P2> {
    parser1: P1,
    parser2: P2,
}

impl<RDF, P1, P2, A> RDFNodeParse<RDF> for CombineVec<P1, P2>
where
    RDF: FocusRdf,
    P1: RDFNodeParse<RDF, Output = Vec<A>>,
    P2: RDFNodeParse<RDF, Output = Vec<A>>,
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Vec<A>> {
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
pub fn bool<RDF: FocusRdf>() -> impl RDFNodeParse<RDF, Output = bool> {
    get_focus().flat_map(|ref term: Object<RDF>| match term.as_literal() {
        Some(literal) => match literal.as_bool() {
            Some(bool) => Ok(bool),
            None => Err(RDFParseError::ExpectedBoolean {
                term: format!("{term}"),
            }),
        },
        None => Err(RDFParseError::ExpectedLiteral {
            term: format!("{term}"),
        }),
    })
}

/// Parses the current focus node as an RDF List
///
/// ```
/// use iri_s::IriS;
/// use srdf::oxgraph::OxGraph;
/// use srdf::{property_value, then, RDFNodeParse, rdf_list, set_focus};
/// use oxrdf::{Literal, Term};
/// use srdf::model::rdf_format::RdfFormat;
/// use srdf::model::Iri;
/// use srdf::model::parse::ReaderMode;
/// use srdf::model::parse::RdfParse;
/// use oxrdf::NamedNode as OxNamedNode;
///
/// let s = r#"prefix : <http://example.org/>
///  :x :p (1 2).
/// "#;
///
/// let graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
///
/// let x = OxNamedNode::new_unchecked("http://example.org/x").as_iri_s();
/// let p = OxNamedNode::new_unchecked("http://example.org/p").as_iri_s();
///
/// let mut parser = property_value(&p).then(move |obj| {
///   set_focus(&obj).with(rdf_list())
/// });
///
/// assert_eq!(
///     parser.parse(&x, graph).unwrap(),
///     vec![Term::from(Literal::from(1)), Term::from(Literal::from(2))]
/// )
/// ````
pub fn rdf_list<RDF: Rdf>() -> RDFList<RDF> {
    RDFList {
        _marker_rdf: PhantomData,
    }
}

/// Creates a parser that returns the focus node
pub fn get_focus<RDF: FocusRdf>() -> GetFocus<RDF> {
    GetFocus {
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct GetFocus<RDF: FocusRdf> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for GetFocus<RDF> {
    type Output = Object<RDF>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Object<RDF>> {
        match rdf.get_focus() {
            Some(focus) => Ok(focus.clone()),
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Creates a parser that sets the focus node and returns `()`
pub fn set_focus<RDF: FocusRdf>(node: &Object<RDF>) -> SetFocus<RDF> {
    SetFocus {
        node: node.clone(),
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct SetFocus<RDF: FocusRdf> {
    node: Object<RDF>,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for SetFocus<RDF> {
    type Output = ();

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<()> {
        rdf.set_focus(self.node.clone());
        Ok(())
    }
}

pub struct RDFList<RDF: Rdf> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for RDFList<RDF> {
    type Output = Vec<Object<RDF>>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Vec<Object<RDF>>> {
        let focus = match rdf.get_focus() {
            Some(focus) => focus.clone(),
            None => return Err(RDFParseError::NoFocusNode),
        };
        let visited = vec![focus];
        parse_list(visited, rdf)
    }
}

/* I would like the following code to work but it complains that:
cannot move out of `parser`, a captured variable in an `FnMut` closure
pub fn parse_rdf_list_for_property<RDF, P, A>(property: IriS, parser: P) -> impl RDFNodeParse<RDF, Output = Vec<A>>
where
   RDF: FocusRdf,
   P: RDFNodeParse<RDF, Output = A> + Clone
{
    property_value(&property).then(|ref node| {
        set_focus(node).and(
            parse_rdf_list::<RDF,P>(parser)).map(|(_, vs)| { vs }
        )
    })
}*/

/// Parses a node as an RDF List applying each element of the list a parser
pub fn parse_rdf_list<RDF: Rdf, P>(parser: P) -> ParseRDFList<P> {
    ParseRDFList { parser }
}

#[derive(Copy, Clone)]
pub struct ParseRDFList<P> {
    parser: P,
}

impl<RDF, P, A> RDFNodeParse<RDF> for ParseRDFList<P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Vec<A>> {
        let focus = match rdf.get_focus() {
            Some(focus) => focus.clone(),
            None => return Err(RDFParseError::NoFocusNode),
        };
        let visited = vec![focus.clone()];
        parse_list(visited, rdf).and_then(|nodes| {
            let mut result = Vec::new();
            for node in nodes {
                rdf.set_focus(node);
                match self.parser.parse_impl(rdf) {
                    Ok(a) => result.push(a),
                    Err(e) => return Err(e),
                }
            }
            Ok(result)
        })
    }
}

/// Auxiliary function to parse a node as an RDF list checking that the RDF list
/// if non-cyclic by collecting a vector of visited terms.
fn parse_list<RDF: FocusRdf>(
    mut visited: Vec<Object<RDF>>,
    rdf: &mut RDF,
) -> Result<Vec<Object<RDF>>, RDFParseError> {
    let focus = rdf.get_focus().unwrap();
    if node_is_rdf_nil::<RDF>(focus) {
        Ok(Vec::new())
    } else {
        let value = property_value(&RDF_FIRST).parse_impl(rdf)?;
        let rest = property_value(&RDF_REST).parse_impl(rdf)?;
        if visited.contains(&rest) {
            Err(RDFParseError::RecursiveRDFList {
                node: format!("{rest}"),
            })
        } else {
            visited.push(rest.clone());
            let mut rest_ls = vec![value];
            rdf.set_focus(rest);
            rest_ls.extend(parse_list(visited, rdf)?);
            Ok(rest_ls)
        }
    }
}

fn node_is_rdf_nil<RDF: Rdf>(node: &Object<RDF>) -> bool {
    if let Some(iri) = node.as_iri() {
        iri.as_iri_s() == *RDF_NIL
    } else {
        false
    }
}

/// Succeeds if current term is the expected IRI
pub fn is_expected_iri<RDF: FocusRdf>(expected_iri: IriS) -> impl RDFNodeParse<RDF, Output = ()> {
    let name = format!("Is {}", expected_iri.as_str());
    satisfy(
        move |node: &Object<RDF>| match node.as_iri() {
            Some(iri) => iri.as_iri_s() == expected_iri,
            None => false,
        },
        name.as_str(),
    )
}

/// Returns the node that is an instance of the expected IRI in the RDF data.
/// It moves the focus to point to that node
pub fn instance_of<RDF: FocusRdf>(
    expected: &IriS,
) -> impl RDFNodeParse<RDF, Output = Subject<RDF>> {
    // TODO: Review that this code seems to overlap with code at line 73 of rdf_parser.rs
    // We should probably replace this code by the other one
    let str = format!("{expected}");
    instances_of(expected).flat_map(move |vs| {
        let mut values = vs.into_iter();
        match values.next() {
            Some(value) => match values.next() {
                Some(_other_value) => todo!(),
                None => Ok(value),
            },
            None => Err(RDFParseError::NoInstancesOf {
                object: str.to_string(),
            }),
        }
    })
}

pub fn set_focus_subject<RDF: FocusRdf>(
    subject: Subject<RDF>,
) -> impl RDFNodeParse<RDF, Output = ()> {
    ApplyRDF {
        function: move |rdf: &mut RDF| {
            let term = subject.clone().into();
            rdf.set_focus(term);
            Ok(())
        },
    }
}

/*pub fn term_as_iri<RDF>(term: Object<RDF>) -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRdf,
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

// pub fn term_as_iri<RDF: FocusRdf>(term: &Object<RDF>) -> impl RDFNodeParse<RDF, Output = IriS> {
//     apply(term, |term| match &RDF::term_as_iri(term) {
//         Some(iri) => {
//             let iri_s = RDF::iri2iri_s(iri);
//             Ok(iri_s)
//         }
//         None => Err(RDFParseError::ExpectedIRI {
//             term: format!("{term}"),
//         }),
//     })
// }

/*fn term_as_iri_s<RDF>(term: &Object<RDF>) -> ParserResult<IriS>
where
    RDF: FocusRdf,
{
    let iri = RDF::object_as_iri(term).ok_or_else(|| RDFParseError::Custom {
        msg: "Expected IRI".to_string(),
    })?;
    let iri_s = RDF::iri2iri_s(&iri);
    Ok(iri_s)
}*/

/// Succeeds with a given value
pub fn ok<RDF, A>(value: &A) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRdf,
    A: Clone,
{
    Ok {
        value: value.clone(),
    }
}

#[derive(Debug, Clone)]
struct Ok<A> {
    value: A,
}

impl<RDF, A> RDFNodeParse<RDF> for Ok<A>
where
    RDF: FocusRdf,
    A: Clone,
{
    type Output = A;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> ParserResult<Self::Output> {
        Ok(self.value.clone())
    }
}

/// Fails with a given massage
pub fn fail_msg<A, RDF: FocusRdf>(msg: String) -> impl RDFNodeParse<RDF, Output = A> {
    Fail {
        msg: msg.clone(),
        _marker: PhantomData,
    }
}

#[derive(Debug, Clone)]
struct Fail<A> {
    msg: String,
    _marker: PhantomData<A>,
}

impl<A, RDF: FocusRdf> RDFNodeParse<RDF> for Fail<A> {
    type Output = A;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> ParserResult<Self::Output> {
        Err(RDFParseError::Custom {
            msg: self.msg.clone(),
        })
    }
}

/// Applies a function and returns its result
pub fn cond<RDF, A>(
    value: &A,
    pred: impl FnMut(&A) -> bool,
    fail_msg: String,
) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRdf,
    A: Clone,
{
    Cond {
        value: value.clone(),
        pred,
        fail_msg: fail_msg.clone(),
    }
}

#[derive(Debug, Clone)]
struct Cond<A, P> {
    value: A,
    pred: P,
    fail_msg: String,
}

impl<RDF, A, P> RDFNodeParse<RDF> for Cond<A, P>
where
    RDF: FocusRdf,
    P: FnMut(&A) -> bool,
    A: Clone,
{
    type Output = ();

    fn parse_impl(&mut self, _rdf: &mut RDF) -> ParserResult<Self::Output> {
        match (self.pred)(&self.value) {
            true => Ok(()),
            false => Err(RDFParseError::Custom {
                msg: self.fail_msg.clone(),
            }),
        }
    }
}

/// Applies a function and returns its result
pub fn apply<RDF, A, B>(
    value: &A,
    function: impl FnMut(&A) -> Result<B, RDFParseError>,
) -> impl RDFNodeParse<RDF, Output = B>
where
    RDF: FocusRdf,
    A: Clone,
{
    Apply {
        value: value.clone(),
        function,
    }
}

#[derive(Debug, Clone)]
struct Apply<A, F> {
    value: A,
    function: F,
}

impl<RDF, A, B, F> RDFNodeParse<RDF> for Apply<A, F>
where
    RDF: FocusRdf,
    F: FnMut(&A) -> Result<B, RDFParseError>,
    A: Clone,
{
    type Output = B;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> ParserResult<Self::Output> {
        match (self.function)(&self.value) {
            Ok(b) => Ok(b),
            Err(e) => Err(e),
        }
    }
}

/// Applies a function over the RDF graph and returns the result of that function
pub fn apply_rdf<A, RDF: FocusRdf>(
    function: impl FnMut(&mut RDF) -> Result<A, RDFParseError>,
) -> impl RDFNodeParse<RDF, Output = A> {
    ApplyRDF { function }
}

#[derive(Debug, Clone)]
struct ApplyRDF<F> {
    function: F,
}

impl<RDF, A, F> RDFNodeParse<RDF> for ApplyRDF<F>
where
    RDF: FocusRdf,
    F: FnMut(&mut RDF) -> Result<A, RDFParseError>,
{
    type Output = A;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match (self.function)(rdf) {
            Ok(a) => Ok(a),
            Err(e) => Err(e),
        }
    }
}

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn instances_of<RDF: FocusRdf>(
    expected: &IriS,
) -> impl RDFNodeParse<RDF, Output = Vec<Subject<RDF>>> {
    let iri = Predicate::<RDF>::new(expected.as_str());
    subjects_with_property_value::<RDF>(&RDF_TYPE, &iri.into())
}

pub fn parse_rdf_type<RDF: FocusRdf>() -> impl RDFNodeParse<RDF, Output = Object<RDF>> {
    property_value(&RDF_TYPE)
}

pub fn has_type<RDF: FocusRdf>(expected: IriS) -> impl RDFNodeParse<RDF, Output = ()> {
    let iri = Predicate::<RDF>::new(expected.as_str());
    parse_rdf_type().then(move |term: Object<RDF>| equals(term, iri.clone().into()))
}

pub fn equals<RDF: FocusRdf>(
    term: Object<RDF>,
    expected: Object<RDF>,
) -> impl RDFNodeParse<RDF, Output = ()> {
    let expected_str = format!("{expected}");
    cond(
        &term,
        move |t| t == &expected,
        format!("Term {term} not equals {}", expected_str),
    )
}

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn subjects_with_property_value<RDF: FocusRdf>(
    property: &IriS,
    value: &Object<RDF>,
) -> SubjectsPropertyValue<RDF> {
    SubjectsPropertyValue {
        property: Predicate::<RDF>::new(property.as_str()),
        value: value.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct SubjectsPropertyValue<RDF: Rdf> {
    property: Predicate<RDF>,
    value: Object<RDF>,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF: FocusRdf> RDFNodeParse<RDF> for SubjectsPropertyValue<RDF> {
    type Output = Vec<Subject<RDF>>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Vec<Subject<RDF>>> {
        let triples = match rdf.triples_matching(None, Some(&self.property), Some(&self.value)) {
            Result::Ok(triples) => triples,
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
        Ok(subjects)
    }
}

rdf_parser! {
    /// Parses the value of `property` as an RDF list
    pub fn parse_property_value_as_list['a, RDF](property: &'a IriS)(RDF) -> Vec<Object<RDF>>
        where [
        ] {
            property_value(property)
            .then(|node|
                set_focus(&node).then(|_|
                    rdf_list())
             )
    }
}

/// Apply a parser to an RDF node associated with the value of it's `rdf:type` property
pub fn parse_by_type<RDF, P, A>(
    values: Vec<(IriS, P)>,
    default: P,
) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = A>,
{
    ParseByType {
        values: HashMap::from_iter(values),
        default,
    }
}

pub struct ParseByType<I, P> {
    values: HashMap<I, P>,
    default: P,
}

impl<RDF, P, A> RDFNodeParse<RDF> for ParseByType<IriS, P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = A;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        let rdf_type = parse_rdf_type().parse_impl(rdf)?;
        let iri_type = match rdf_type.as_iri() {
            Some(iri) => iri.as_iri_s(),
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

/// Discards the value of the first parser and returns the value of the second
/// parser. It is equivalent to [`parser1.with(parser2)`]
pub fn with<RDF, P1, P2>(parser1: P1, parser2: P2) -> With<P1, P2>
where
    RDF: FocusRdf,
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

impl<RDF, A, B, P1, P2> RDFNodeParse<RDF> for With<P1, P2>
where
    RDF: FocusRdf,
    P1: RDFNodeParse<RDF, Output = A>,
    P2: RDFNodeParse<RDF, Output = B>,
{
    type Output = B;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        match self.parser1.parse_impl(rdf) {
            Ok(_) => match self.parser2.parse_impl(rdf) {
                Ok(b) => Ok(b),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

/// Applies a parser over a list of nodes and returns the list of values
pub fn parse_nodes<RDF, P>(nodes: Vec<Object<RDF>>, parser: P) -> ParserNodes<RDF, P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF>,
{
    ParserNodes { nodes, parser }
}

#[derive(Clone)]
pub struct ParserNodes<RDF: FocusRdf, P> {
    nodes: Vec<Object<RDF>>,
    parser: P,
}

impl<RDF, A, P> RDFNodeParse<RDF> for ParserNodes<RDF, P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        let mut results = Vec::new();
        for node in self.nodes.iter() {
            rdf.set_focus(node.clone());
            let value = self.parser.parse_impl(rdf)?;
            results.push(value)
        }
        Ok(results)
    }
}

/// Implementation of [`RDFNodeParse::by_ref`]
pub struct ByRef<'p, P> {
    p: &'p mut P,
}

impl<'p, P> ByRef<'p, P> {
    #[inline(always)]
    pub(crate) fn new(p: &'p mut P) -> Self {
        Self { p }
    }
}

impl<RDF, P, O> RDFNodeParse<RDF> for ByRef<'_, P>
where
    RDF: FocusRdf,
    P: RDFNodeParse<RDF, Output = O>,
{
    type Output = O;

    #[inline(always)]
    fn parse_impl(&mut self, rdf: &mut RDF) -> ParserResult<Self::Output> {
        self.p.parse_impl(rdf)
    }
}
