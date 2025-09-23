use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use iri_s::IriS;
use iri_s::iri;
use std::fmt::Debug;
use tracing::{debug, trace};

use crate::{
    FocusRDF, IriOrBlankNode, NeighsRDF, Object, PResult, RDF_NIL_STR, RDFParseError, Rdf,
    SHACLPath, SLiteral, Triple, matcher::Any, numeric_literal::NumericLiteral, rdf_first,
    rdf_parser, rdf_rest, rdf_type, sh_alternative_path, sh_inverse_path, sh_one_or_more_path,
    sh_zero_or_more_path, sh_zero_or_one_path,
};
use crate::{Iri as _, Literal as _};

/// By implementing the `RDFNodeParse` trait a type says that it can be used to parse RDF data which have a focus node.
/// RDF data with a focus node have to implement the [`FocusRDF`] trait.
pub trait RDFNodeParse<RDF: FocusRDF> {
    /// The type which is returned if the parser is successful.
    type Output;

    /// Entry point to the parser. It moves the focus node of `rdf` to `node` and runs the parser.
    ///
    /// Returns the parsed result if the parser succeeds, or an error otherwise.
    #[inline(always)]
    fn parse(&mut self, node: &IriS, mut rdf: RDF) -> PResult<Self::Output> {
        let iri: RDF::IRI = node.clone().into();
        let focus: RDF::Term = iri.into().into();
        rdf.set_focus(&focus);
        self.parse_impl(&mut rdf)
    }

    /// Borrows a parser instead of consuming it.
    ///
    /// Used to apply parser combinators on `self` without losing ownership.
    /// This is useful when you want to chain parsers or apply combinators without consuming the original parser.
    #[inline(always)]
    fn by_ref(&mut self) -> ByRef<'_, Self>
    where
        Self: core::marker::Sized,
    {
        ByRef::new(self)
    }

    /// Parses the current focus node without modifying the state
    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output>;

    /// Uses `f` to map over the output of `self`. If `f` returns an error the parser fails.
    ///
    /// ```
    /// # use iri_s::{IriS, iri};
    /// # use srdf::SRDFGraph;
    /// use srdf::{RDFNodeParse, RDFFormat, RDFParseError, ReaderMode, property_string, PResult};
    ///     let s = r#"prefix : <http://example.org/>
    ///     :x :p "1" .
    ///   "#;
    ///   let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///   let x = iri!("http://example.org/x");
    ///   let p = iri!("http://example.org/p");
    ///   fn cnv_int(s: String) -> PResult<isize> {
    ///      s.parse().map_err(|_| RDFParseError::Custom{ msg: format!("Error converting {s}")})
    ///   }
    ///   let mut parser = property_string(&p).flat_map(cnv_int);
    ///   assert_eq!(parser.parse(&x, graph).unwrap(), 1)
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
    /// use srdf::{RDFNodeParse, RDFFormat, RDFParseError, ReaderMode, property_string};
    /// let s = r#"prefix : <http://example.org/>
    ///        :x :p "1" .
    ///   "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
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
    /// # use iri_s::{IriS, iri};
    /// # use srdf::srdf_graph::SRDFGraph;
    /// use srdf::{RDFNodeParse, RDFFormat, ReaderMode, property_integer};
    /// let s = r#"prefix : <http://example.org/>
    ///          :x :p 1 .
    ///  "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
    /// let p = iri!("http://example.org/p");
    /// let mut parser = property_integer(&p).map(|n| n + 1);
    /// assert_eq!(parser.parse(&iri!("http://example.org/x"), graph).unwrap(), 2)
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
    /// # use srdf::{RDFNodeParse, RDFFormat, ReaderMode, property_bool, property_integer};
    /// let s = r#"prefix : <http://example.org/>
    ///       :x :p true ;
    ///          :q 1    .
    ///     "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
    /// let x = IriS::new_unchecked("http://example.org/x");
    /// let p = IriS::new_unchecked("http://example.org/p");
    /// let q = IriS::new_unchecked("http://example.org/q");
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
    /// use srdf::{RDFNodeParse, RDFFormat, ReaderMode, ok, property_integers};
    ///     let s = r#"prefix : <http://example.org/>
    ///       :x :p 1, 2, 3 .
    ///     "#;
    ///     let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///     let x = IriS::new_unchecked("http://example.org/x");
    ///     let p = IriS::new_unchecked("http://example.org/p");
    ///     let mut parser = property_integers(&p).then_mut(move |ns| {
    ///         ns.extend(vec![4, 5]);
    ///         ok(ns)
    ///      });
    ///     assert_eq!(parser.parse(&x, graph).unwrap(), HashSet::from([1, 2, 3, 4, 5]))
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
    /// # use srdf::{RDFNodeParse, RDFFormat, ReaderMode, property_bool};
    /// # use std::collections::HashSet;
    ///  let s = r#"prefix : <http://example.org/>
    ///       :x :p 1, 2 ;
    ///          :q true .
    ///     "#;
    ///  let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
    ///  let x = IriS::new_unchecked("http://example.org/x");
    ///  let p = IriS::new_unchecked("http://example.org/p");
    ///  let q = IriS::new_unchecked("http://example.org/q");
    ///  let mut parser = property_bool(&p).or(property_bool(&q));
    ///  assert_eq!(parser.parse(&x, graph).unwrap(), true)
    /// ```
    fn or<P2>(self, parser: P2) -> Or<Self, P2>
    where
        Self: Sized,
        P2: RDFNodeParse<RDF, Output = Self::Output>,
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

    /// Discards the value of the current parser and returns the value of `parser`
    ///
    /// ```
    /// # use iri_s::IriS;
    /// # use srdf::{rdf_parser, RDFParser, RDF, RDFFormat, FocusRDF, ReaderMode, satisfy, RDFNodeParse, Query, Rdf, property_value, rdf_list, set_focus, parse_property_value_as_list, ok};
    /// # use srdf::srdf_graph::SRDFGraph;
    /// let s = r#"prefix : <http://example.org/>
    ///            :x :p :y .
    /// "#;
    /// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
    /// let p = IriS::new_unchecked("http://example.org/p");
    /// let x = IriS::new_unchecked("http://example.org/x");
    /// assert_eq!(
    ///   property_value(&p).with(ok(&1))
    ///   .parse(&x, graph).unwrap(),
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

impl<RDF, P1, P2, A, B> RDFNodeParse<RDF> for (P1, P2)
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = A>,
    P2: RDFNodeParse<RDF, Output = B>,
{
    type Output = (A, B);

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

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(a) => Ok((self.f)(a)),
            Err(e) => Err(e),
        }
    }
}

///  Applies a function `func` on the result of a parser, which may fail with an error.
///
pub fn and_then<RDF, P, F, O, E>(parser: P, func: F) -> AndThen<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> Result<O, E>,
    E: Into<RDFParseError>,
{
    AndThen {
        parser,
        function: func,
    }
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
            Ok(value) => {
                trace!("FlatMap: got value, applying function");
                match (self.function)(value) {
                    Ok(result) => Ok(result),
                    Err(err) => {
                        trace!("FlatMap: function failed with error: {err}");
                        Err(err)
                    }
                }
            }
            Err(err) => {
                trace!("FlatMap: first parser failed with error: {err}");
                Err(err)
            }
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
}

impl<RDF, P1, P2, O> RDFNodeParse<RDF> for Or<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = O>,
    P2: RDFNodeParse<RDF, Output = O>,
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
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
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: FnMut(P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;

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

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Ok(mut value) => (self.function)(&mut value).parse_impl(rdf),
            Err(err) => Err(err),
        }
    }
}

/// Not parser succeeds if the `parser` fails and viceversa
/// Example:
/// ```
/// use iri_s::{IriS, iri};
/// use srdf::SRDFGraph;
/// use srdf::{literal, not, RDFFormat, RDFNodeParse};
///
/// let graph = SRDFGraph::new();
/// let x = iri!("http://example.org/x");
/// assert_eq!(not(literal()).parse(&x, graph).unwrap(), ())
/// ```
pub fn not<RDF, P>(parser: P) -> Not<P>
where
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = O>,
    O: Debug,
{
    type Output = ();

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match self.parser.parse_impl(rdf) {
            Err(_err) => Ok(()),
            Ok(value) => Err(RDFParseError::FailedNot {
                value: format!("{value:?}"),
            }),
        }
    }
}

/// Creates a parser that returns the current focus node as an Object
///
pub fn object<RDF>() -> ParseObject<RDF>
where
    RDF: FocusRDF,
{
    ParseObject {
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct ParseObject<RDF> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for ParseObject<RDF>
where
    RDF: FocusRDF,
{
    type Output = Object;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Object> {
        match rdf.get_focus() {
            Some(focus) => {
                let object: Object =
                    focus
                        .clone()
                        .try_into()
                        .map_err(|_| RDFParseError::TermToRDFNodeFailed {
                            term: focus.to_string(),
                        })?;
                Ok(object)
            }
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Creates a parser that returns the current focus node as a term
///
/// This is equivalent to [`get_focus`]
pub fn term<RDF>() -> ParseTerm<RDF>
where
    RDF: FocusRDF,
{
    ParseTerm {
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct ParseTerm<RDF> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for ParseTerm<RDF>
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

/// Creates a parser that returns the current focus node as a iri
///
/// ```
/// use iri_s::{IriS, iri};
/// use srdf::{SRDFGraph, iri, RDFNodeParse};
///
/// let graph = SRDFGraph::new();
/// let x = iri!("http://example.org/x");
/// assert_eq!(iri().parse(&x, graph).unwrap(), x)
/// ```
pub fn iri<RDF>() -> ParseIri<RDF>
where
    RDF: FocusRDF,
{
    ParseIri {
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct ParseIri<RDF> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for ParseIri<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::IRI;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::IRI> {
        match rdf.get_focus() {
            Some(focus) => {
                let iri: RDF::IRI =
                    RDF::term_as_iri(focus).map_err(|_| RDFParseError::ExpectedIRI {
                        term: format!("{focus}"),
                    })?;
                Ok(iri)
            }
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Creates a parser that returns the current focus node as a literal
///
/// ```
/// use iri_s::{IriS, iri};
/// use srdf::{SRDFGraph, iri, RDFNodeParse};
///
/// let graph = SRDFGraph::new();
/// let x = iri!("http://example.org/x");
/// assert_eq!(iri().parse(&x, graph).unwrap(), x)
/// ```
pub fn literal<RDF>() -> ParseLiteral<RDF>
where
    RDF: FocusRDF,
{
    ParseLiteral {
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct ParseLiteral<RDF> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for ParseLiteral<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Literal;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Literal> {
        match rdf.get_focus() {
            Some(focus) => {
                let iri: RDF::Literal =
                    RDF::term_as_literal(focus).map_err(|_| RDFParseError::ExpectedLiteral {
                        term: format!("{focus}"),
                    })?;
                Ok(iri)
            }
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Created a parser that returns the boolean associated with the current focus node for `property`
///
/// It doesn't move the current focus node
pub fn property_bool<R>(prop: &IriS) -> impl RDFNodeParse<R, Output = bool>
where
    R: FocusRDF,
{
    property_value(prop).flat_map(|term: R::Term| {
        let literal: R::Literal = <R::Term as TryInto<R::Literal>>::try_into(term.clone())
            .map_err(|_| RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            })?;
        literal
            .as_bool()
            .ok_or_else(|| RDFParseError::ExpectedBoolean {
                term: format!("{term}"),
            })
    })
}

pub fn parse_rdf_nil<R>() -> impl RDFNodeParse<R, Output = ()>
where
    R: FocusRDF,
{
    satisfy(
        |term: &R::Term| {
            let tmp: Result<R::IRI, _> = <R::Term as TryInto<R::IRI>>::try_into(term.clone());
            match tmp {
                Ok(iri) => iri.as_str() == RDF_NIL_STR,
                Err(_) => false,
            }
        },
        "rdf_nil",
    )
}

/// Creates a parser that checks if the current node satisfies a predicate
///
/// The `predicate_name` argument is useful in case of failure to know which condition has failed
pub fn satisfy<RDF, P>(predicate: P, predicate_name: &str) -> Satisfy<RDF, P>
where
    RDF: NeighsRDF,
    P: FnMut(&RDF::Term) -> bool,
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

/// Return the integer values of `property` for the focus node
///
/// If some value is not an integer it fails, if there is no value returns an empty set
pub fn property_values_int<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<isize>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let ints: Vec<_> = values
            .iter()
            .flat_map(|t| {
                let i = term_to_int::<RDF>(t)?;
                Ok::<isize, RDFParseError>(i)
            })
            .collect();
        Ok(ints)
    })
}

/// Return the boolean values of `property` for the focus node
///
/// If some value is not bool it fails, if there is no value returns an empty set
pub fn property_values_bool<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<bool>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let bools: Vec<_> = values
            .iter()
            .flat_map(|t| {
                let b = term_to_bool::<RDF>(t)?;
                Ok::<bool, RDFParseError>(b)
            })
            .collect();
        Ok(bools)
    })
}

/// Return the literal values of `property` for the focus node
///
/// If some value is not a literal it fails, if there is no value returns an empty set
pub fn property_values_literal<RDF>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Literal>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let lits: Vec<_> = values
            .iter()
            .flat_map(|t| {
                let lit: <RDF as Rdf>::Literal = term_to_literal::<RDF>(t)?;
                Ok::<RDF::Literal, RDFParseError>(lit)
            })
            .collect();
        Ok(lits)
    })
}

/// Return the string values of `property` for the focus node
///
/// If some value is not an ``xsd:string` it fails, if there is no value returns an empty set
pub fn property_values_string<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<String>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let strs: Vec<_> = values
            .iter()
            .flat_map(|t| {
                let i = term_to_string::<RDF>(t)?;
                Ok::<String, RDFParseError>(i)
            })
            .collect();
        Ok(strs)
    })
}

/// Return the IRI values of `property` for the focus node
///
/// If some value is not an IRI it fails, if there is no value returns an empty set
pub fn property_values_iri<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = HashSet<IriS>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let iris: HashSet<_> = values
            .iter()
            .flat_map(|t| {
                let iri = term_to_iri::<RDF>(t)?;
                Ok::<IriS, RDFParseError>(iri)
            })
            .collect();
        Ok(iris)
    })
}

/// Return the IRI or BNode values of `property` for the focus node
///
/// If some value is not an IRI or Blank Node it fails, if there is no value returns an empty set
pub fn property_values_iri_or_bnode<RDF>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = HashSet<IriOrBlankNode>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let nodes: HashSet<_> = values
            .iter()
            .flat_map(|t| {
                let node = term_to_iri_or_blanknode::<RDF>(t)?;
                Ok::<IriOrBlankNode, RDFParseError>(node)
            })
            .collect();
        Ok(nodes)
    })
}

/// Returns the values of `property` for the focus node
///
/// If there is no value, it returns an error
pub fn property_values_non_empty<RDF>(
    property: &IriS,
) -> impl RDFNodeParse<RDF, Output = HashSet<RDF::Term>>
where
    RDF: FocusRDF,
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

/// Returns the values of `property` for the focus node converted to concrete objects
///
/// For some values, it can fail if there is an error converting the term to an `Object`.
/// For example, if the term is something like `"x"^^xsd:integer` it will fail.
/// If there is no value, it returns an empty set
pub fn property_objects<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = HashSet<Object>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let rs: Result<HashSet<Object>, RDFParseError> = values
            .into_iter()
            .map(|t: RDF::Term| {
                let obj: Object = t.try_into().map_err(|_| RDFParseError::SRDFError {
                    err: "Error converting term to object".to_string(),
                })?;
                Ok::<Object, RDFParseError>(obj)
            })
            .collect();
        let objects = rs?;
        Ok(objects)
    })
}

/// Returns the values of `property` for the focus node as iris
///
/// If any of the values is not an IRI it fails
/// If there is no value, it returns an empty set
pub fn property_iris<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = HashSet<IriS>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|values| {
        let rs: Result<HashSet<IriS>, RDFParseError> =
            values.into_iter().map(value_to_iri::<RDF>).collect();
        let iris = rs?;
        Ok(iris)
    })
}

fn value_to_iri<RDF>(value: RDF::Term) -> Result<IriS, RDFParseError>
where
    RDF: FocusRDF,
{
    let iri: IriS = RDF::term_as_iri(&value)
        .map_err(|_| RDFParseError::ExpectedIRI {
            term: format!("{value}"),
        })?
        .into();
    Ok(iri)
}

/// Returns the values of `property` for the focus node
///
/// If there is no value, it returns an empty set
pub fn property_values<RDF>(property: &IriS) -> PropertyValues<RDF>
where
    RDF: FocusRDF,
{
    PropertyValues {
        property: property.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValues<RDF: FocusRDF> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for PropertyValues<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<RDF::Term>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<HashSet<RDF::Term>> {
        if let Ok(subject) = rdf.get_focus_as_subject() {
            let pred: RDF::IRI = self.property.clone().into();
            let values: HashSet<_> = rdf
                .triples_matching(subject, pred, Any)
                .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?
                .map(Triple::into_object)
                .collect();
            Ok(values)
        } else {
            Ok(HashSet::new())
        }
    }
}

/// Returns the values of `property` for the focus node
///
/// If there is no value, it returns an empty set
/// This is a debug version which prints tracing information
///
pub fn property_values_debug<RDF>(property: &IriS) -> PropertyValuesDebug<RDF>
where
    RDF: FocusRDF,
{
    PropertyValuesDebug {
        property: property.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValuesDebug<RDF: FocusRDF> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for PropertyValuesDebug<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<RDF::Term>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<HashSet<RDF::Term>> {
        if let Ok(subject) = rdf.get_focus_as_subject() {
            let pred: RDF::IRI = self.property.clone().into();
            let values: HashSet<_> = rdf
                .triples_matching(subject.clone(), pred, Any)
                .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?
                .map(Triple::into_object)
                .collect();
            debug!(
                "property_values: Subject {}, Property {}: {}",
                subject,
                self.property,
                values
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            Ok(values)
        } else {
            Ok(HashSet::new())
        }
    }
}

/// Creates a parser that returns the value associated with the current focus node for `property`
///
/// It doesn't move the current focus node
pub fn property_value<RDF>(property: &IriS) -> PropertyValue<RDF>
where
    RDF: NeighsRDF,
{
    PropertyValue {
        property: property.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValue<RDF: NeighsRDF> {
    property: IriS,
    _marker_rdf: PhantomData<RDF>,
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

/// Creates a parser that returns the value associated with the current focus node for `property`
///
/// It doesn't move the current focus node
///
/// This method can be used to debug the parser, because it is less efficient as in case that it fails,
/// it shows the neighbourhood of the current node
pub fn property_value_debug<RDF>(property: &IriS) -> PropertyValueDebug<RDF>
where
    RDF: NeighsRDF,
{
    PropertyValueDebug {
        property: property.clone().into(),
        _marker_rdf: PhantomData,
    }
}

pub struct PropertyValueDebug<RDF: NeighsRDF> {
    property: RDF::IRI,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for PropertyValueDebug<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        let mut p: Neighs<RDF> = neighs();
        let focus_node_str = match rdf.get_focus() {
            None => "No focus node".to_string(),
            Some(focus_node) => {
                format!("{focus_node}")
            }
        };
        let outgoing_arcs = p.parse_impl(rdf)?;
        debug!("Property value: Focus node {focus_node_str}");
        if let Some(values) = outgoing_arcs.get(&self.property) {
            debug!(
                "Found values for property {} {}",
                &self.property,
                values
                    .iter()
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let mut values_iter = values.iter();
            if let Some(value1) = values_iter.next() {
                if let Some(value2) = values_iter.next() {
                    Err(RDFParseError::MoreThanOneValuePredicate {
                        node: focus_node_str.to_string(),
                        pred: format!("{}", self.property),
                        value1: format!("{value1:?}"),
                        value2: format!("{value2:?}"),
                    })
                } else {
                    Ok(value1.clone())
                }
            } else {
                panic!(
                    "Internal error: Node {} has no value for predicate {}...but this case should be handled in the outer else...",
                    focus_node_str, self.property
                )
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

/// Creates a parser that returns the value associated with the current focus node for `property`
///
/// It doesn't move the current focus node
///
/// This method can be used to debug the parser, because it is less efficient as in case that it fails,
/// it shows the neighbourhood of the current node
pub fn neighs<RDF>() -> Neighs<RDF>
where
    RDF: NeighsRDF,
{
    Neighs {
        _marker_rdf: PhantomData,
    }
}

pub struct Neighs<RDF: NeighsRDF> {
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for Neighs<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashMap<RDF::IRI, HashSet<RDF::Term>>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<HashMap<RDF::IRI, HashSet<RDF::Term>>> {
        match rdf.get_focus() {
            Some(focus) => {
                let subj = RDF::term_as_subject(focus).map_err(|_| {
                    RDFParseError::ExpectedFocusAsSubject {
                        focus: focus.to_string(),
                    }
                })?;
                rdf.outgoing_arcs(subj).map_err(|e| RDFParseError::Custom {
                    msg: format!("Error obtaining outgoing arcs from {focus}: {e}"),
                })
            }
            None => Err(RDFParseError::NoFocusNode),
        }
    }
}

/// Returns the integer values of `property` for the focus node
///
/// If there is no value, it returns an empty set
pub fn property_integers<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = HashSet<isize>>
where
    RDF: FocusRDF,
{
    property_values(property).flat_map(|terms| {
        let is = terms_to_ints::<RDF>(terms)?;
        Ok(is)
    })
}

/// Returns the IRI value of `property` for the focus node
///
pub fn property_iri<'a, RDF>(property: &'a IriS) -> impl RDFNodeParse<RDF, Output = IriS> + 'a
where
    RDF: FocusRDF + 'a,
{
    get_focus().then(move |focus| {
        property_value(property).flat_map(move |term| {
            let i =
                term_to_iri::<RDF>(&term).map_err(|e| RDFParseError::PropertyValueExpectedIRI {
                    focus: format!("{focus}"),
                    property: property.clone(),
                    error: format!("{e}"),
                })?;
            Ok(i)
        })
    })
}

/// Returns the IRI or Blank node value of `property` for the focus node
///
pub fn property_iri_or_bnode<'a, RDF>(
    property: &'a IriS,
) -> impl RDFNodeParse<RDF, Output = IriOrBlankNode> + 'a
where
    RDF: FocusRDF + 'a,
{
    get_focus().then(move |focus| {
        property_value(property).flat_map(move |term| {
            let ib = term_to_iri_or_blanknode::<RDF>(&term).map_err(|e| {
                RDFParseError::PropertyValueExpectedIRIOrBlankNode {
                    focus: format!("{focus}"),
                    property: property.clone(),
                    error: format!("{e}"),
                }
            })?;
            Ok(ib)
        })
    })
}

/// Returns the integer value of `property` for the focus node
///
pub fn property_integer<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = isize>
where
    RDF: FocusRDF,
{
    property_value(property).flat_map(|term| {
        let i = term_to_int::<RDF>(&term)?;
        Ok(i)
    })
}

/// Returns the integer value of `property` for the focus node
///
pub fn property_number<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = NumericLiteral>
where
    RDF: FocusRDF,
{
    // debug!("property_number: property={}", property);
    property_value(property).flat_map(|term| {
        let lit = term_to_number::<RDF>(&term);
        if lit.is_err() {
            trace!(
                "property_number: term is not a number: {}, err: {}",
                term,
                lit.as_ref().err().unwrap()
            );
        }
        lit
    })
}

/// Returns the string value of `property` for the focus node
///
pub fn property_string<RDF>(property: &IriS) -> impl RDFNodeParse<RDF, Output = String>
where
    RDF: FocusRDF,
{
    property_value(property).flat_map(|term| {
        let i = term_to_string::<RDF>(&term)?;
        Ok(i)
    })
}

fn terms_to_ints<RDF>(terms: HashSet<RDF::Term>) -> Result<HashSet<isize>, RDFParseError>
where
    RDF: Rdf,
{
    let ints: HashSet<_> = terms.iter().flat_map(|t| term_to_int::<RDF>(t)).collect();
    Ok(ints)
}

fn term_to_literal<R>(term: &R::Term) -> Result<R::Literal, RDFParseError>
where
    R: Rdf,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            }
        })?;
    Ok(literal)
}

fn term_to_int<R>(term: &R::Term) -> Result<isize, RDFParseError>
where
    R: Rdf,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            }
        })?;
    let n = literal
        .as_integer()
        .ok_or_else(|| RDFParseError::ExpectedInteger {
            term: format!("{term}"),
        })?;
    Ok(n)
}

fn term_to_number<R>(term: &R::Term) -> Result<NumericLiteral, RDFParseError>
where
    R: Rdf,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            }
        })?;
    trace!("converted to literal: {:?}", literal);
    let slit: SLiteral = literal
        .try_into()
        .map_err(|_e| RDFParseError::ExpectedSLiteral {
            term: format!("{term}"),
        })?;
    match slit {
        SLiteral::NumericLiteral(n) => Ok(n),
        _ => Err(RDFParseError::ExpectedNumber {
            term: format!("{term}"),
        }),
    }
}

fn term_to_bool<R>(term: &R::Term) -> Result<bool, RDFParseError>
where
    R: Rdf,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            }
        })?;
    let n = literal
        .as_bool()
        .ok_or_else(|| RDFParseError::ExpectedBoolean {
            term: format!("{term}"),
        })?;
    Ok(n)
}

fn term_to_iri<R>(term: &R::Term) -> Result<IriS, RDFParseError>
where
    R: Rdf,
{
    let iri: R::IRI = <R::Term as TryInto<R::IRI>>::try_into(term.clone()).map_err(|_| {
        RDFParseError::ExpectedIRI {
            term: format!("{term}"),
        }
    })?;
    let iri_string = iri.as_str();
    Ok(iri!(iri_string))
}

fn term_to_iri_or_blanknode<R>(term: &R::Term) -> Result<IriOrBlankNode, RDFParseError>
where
    R: Rdf,
{
    let subj: R::Subject =
        <R::Term as TryInto<R::Subject>>::try_into(term.clone()).map_err(|_| {
            RDFParseError::ExpectedIriOrBlankNode {
                term: format!("{term}"),
                error: "Expected IRI or BlankNode".to_string(),
            }
        })?;
    let iri_or_bnode: IriOrBlankNode =
        subj.clone()
            .try_into()
            .map_err(|_| RDFParseError::SubjectToIriOrBlankNode {
                subject: format!("{subj}"),
            })?;
    Ok(iri_or_bnode)
}

fn term_to_string<R>(term: &R::Term) -> Result<String, RDFParseError>
where
    R: Rdf,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| {
            RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            }
        })?;
    Ok(literal.lexical_form().to_string())
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

impl<RDF, P1, P2, A> RDFNodeParse<RDF> for CombineVec<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = Vec<A>>,
    P2: RDFNodeParse<RDF, Output = Vec<A>>,
{
    type Output = Vec<A>;

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

/// Combines the results of parsers that return vectors of values
///
pub fn combine_parsers_vec<RDF, P, A>(parsers: Vec<P>) -> CombineParsers<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = Vec<A>>,
{
    CombineParsers { parsers }
}

pub struct CombineParsers<P> {
    parsers: Vec<P>,
}

impl<RDF, P, A> RDFNodeParse<RDF> for CombineParsers<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = Vec<A>>,
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<A>> {
        let mut result = Vec::new();
        for p in self.parsers.iter_mut() {
            match p.parse_impl(rdf) {
                Err(e) => return Err(e),
                Ok(vs) => {
                    result.extend(vs);
                }
            }
        }
        Ok(result)
    }
}

pub fn opaque<RDF, F, O>(f: F) -> Opaque<F, RDF, O>
where
    RDF: FocusRDF,
    F: FnMut(&mut dyn FnMut(&mut dyn RDFNodeParse<RDF, Output = O>)),
{
    Opaque(f, PhantomData)
}

#[derive(Copy, Clone)]
pub struct Opaque<F, RDF, O>(F, PhantomData<fn(&mut RDF) -> O>);

impl<RDF, F, O> RDFNodeParse<RDF> for Opaque<F, RDF, O>
where
    RDF: FocusRDF,
    F: FnMut(&mut dyn FnMut(&mut dyn RDFNodeParse<RDF, Output = O>)),
{
    type Output = O;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        let mut result = Err(RDFParseError::Custom {
            msg: "Opaque parser failed".to_string(),
        });
        (self.0)(&mut |parser| {
            result = parser.parse_impl(rdf);
        });
        result
    }
}

/// Parses a node as a bool
///
pub fn bool<R>() -> impl RDFNodeParse<R, Output = bool>
where
    R: FocusRDF,
{
    get_focus().flat_map(|term: R::Term| {
        let literal: R::Literal = <R::Term as TryInto<R::Literal>>::try_into(term.clone())
            .map_err(|_| RDFParseError::ExpectedLiteral {
                term: format!("{term}"),
            })?;
        let boolean = literal
            .as_bool()
            .ok_or_else(|| RDFParseError::ExpectedInteger {
                term: format!("{term}"),
            })?;
        Ok(boolean)
    })
}

/// Parses the current focus node as an RDF List
///
/// ```
/// use iri_s::{IriS, iri};
/// use srdf::SRDFGraph;
/// use srdf::{property_value, then, RDFFormat, ReaderMode, RDFNodeParse, rdf_list, set_focus};
/// use oxrdf::{Literal, Term};
///
/// let s = r#"prefix : <http://example.org/>
///  :x :p (1 2).
/// "#;
/// let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
/// let x = iri!("http://example.org/x");
/// let p = iri!("http://example.org/p");
/// let mut parser = property_value(&p).then(move |obj| {
///   set_focus(&obj).with(rdf_list())
/// });
/// assert_eq!(parser.parse(&x, graph).unwrap(),
///   vec![Term::from(Literal::from(1)), Term::from(Literal::from(2))])
/// ```
pub fn rdf_list<RDF>() -> RDFList<RDF>
where
    RDF: NeighsRDF,
{
    RDFList {
        _marker_rdf: PhantomData,
    }
}

/// Gets the current focus node expecting it to be an IRI or Blanknode
pub fn get_focus_iri_or_bnode<RDF>() -> impl RDFNodeParse<RDF, Output = IriOrBlankNode>
where
    RDF: FocusRDF,
{
    get_focus().flat_map(|term: RDF::Term| {
        let node = term_to_iri_or_blanknode::<RDF>(&term).map_err(|e| {
            trace!("Error converting term to IRI or BlankNode: {}", e);
            RDFParseError::ExpectedIriOrBlankNode {
                term: term.to_string(),
                error: e.to_string(),
            }
        });
        node
    })
}

/// Creates a parser that returns the focus node
pub fn get_focus<RDF>() -> GetFocus<RDF>
where
    RDF: FocusRDF,
{
    GetFocus {
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct GetFocus<RDF>
where
    RDF: FocusRDF,
{
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for GetFocus<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<RDF::Term> {
        match rdf.get_focus() {
            Some(focus) => {
                trace!("Focus node: {}", focus);
                Ok(focus.clone())
            }
            None => {
                trace!("No focus node");
                Err(RDFParseError::NoFocusNode)
            }
        }
    }
}

/// Sets the focus node from an IRI or Blank node and returns ()
pub fn set_focus_iri_or_bnode<RDF>(node: &IriOrBlankNode) -> SetFocus<RDF>
where
    RDF: FocusRDF,
{
    let term: RDF::Term = RDF::iri_or_bnode_as_term(&node.clone());
    set_focus(&term)
}

/// Creates a parser that sets the focus node and returns `()`
pub fn set_focus<RDF>(node: &RDF::Term) -> SetFocus<RDF>
where
    RDF: FocusRDF,
{
    SetFocus {
        node: node.clone(),
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct SetFocus<RDF>
where
    RDF: FocusRDF,
{
    node: RDF::Term,
    _marker_rdf: PhantomData<RDF>,
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

pub struct RDFList<RDF: NeighsRDF> {
    _marker_rdf: PhantomData<RDF>,
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
    RDF: NeighsRDF,
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

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<A>> {
        let focus = rdf.get_focus_as_term()?;
        let visited = vec![focus.clone()];
        parse_list(visited, rdf).and_then(|nodes| {
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
        let value = property_value(rdf_first()).parse_impl(rdf)?;
        let rest = property_value(rdf_rest()).parse_impl(rdf)?;
        if visited.contains(&rest) {
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

fn node_is_rdf_nil<R>(node: &R::Term) -> bool
where
    R: NeighsRDF,
{
    let tmp: Result<R::IRI, _> = <R::Term as TryInto<R::IRI>>::try_into(node.clone());
    match tmp {
        Ok(iri) => iri.as_str() == RDF_NIL_STR,
        Err(_) => false,
    }
}

/// Succeeds if current term is the expected IRI
pub fn is_iri<RDF>(expected_iri: IriS) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
{
    let name = format!("Is {}", expected_iri.as_str());
    satisfy(
        // TODO: this clone can be removed
        move |node: &RDF::Term| {
            let tmp: Result<RDF::IRI, _> = <RDF::Term as TryInto<RDF::IRI>>::try_into(node.clone());
            match tmp {
                Ok(iri) => iri.as_str() == expected_iri.as_str(),
                Err(_) => false,
            }
        },
        name.as_str(),
    )
}

/// Returns the node that is an instance of the expected IRI in the RDF data
/// It moves the focus to point to that node
pub fn instance_of<RDF>(expected: &IriS) -> impl RDFNodeParse<RDF, Output = RDF::Subject>
where
    RDF: FocusRDF,
{
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

pub fn set_focus_subject<RDF>(subject: RDF::Subject) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
{
    ApplyRDF {
        function: move |rdf: &mut RDF| {
            let term = subject.clone().into();
            rdf.set_focus(&term);
            Ok(())
        },
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

pub fn term_as_iri<RDF>(term: &RDF::Term) -> impl RDFNodeParse<RDF, Output = IriS>
where
    RDF: FocusRDF,
{
    apply(term, |term| {
        let iri: RDF::IRI =
            <RDF::Term as TryInto<RDF::IRI>>::try_into(term.clone()).map_err(|_| {
                RDFParseError::ExpectedIRI {
                    term: format!("{term}"),
                }
            })?;
        let iri_string = iri.as_str();
        Ok(iri!(iri_string))
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
pub fn ok<RDF, A>(value: &A) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    A: Clone,
{
    type Output = A;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        Ok(self.value.clone())
    }
}

/// Fails with a given massage
pub fn fail_msg<A, RDF>(msg: String) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRDF,
{
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

impl<A, RDF> RDFNodeParse<RDF> for Fail<A>
where
    RDF: FocusRDF,
{
    type Output = A;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        Err(RDFParseError::Custom {
            msg: self.msg.clone(),
        })
    }
}

/// Applies a function and returns its result
///
///
pub fn cond<RDF, A>(
    value: &A,
    pred: impl FnMut(&A) -> bool,
    fail_msg: String,
) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    P: FnMut(&A) -> bool,
    A: Clone,
{
    type Output = ();

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
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
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    F: FnMut(&A) -> Result<B, RDFParseError>,
    A: Clone,
{
    type Output = B;

    fn parse_impl(&mut self, _rdf: &mut RDF) -> PResult<Self::Output> {
        match (self.function)(&self.value) {
            Ok(b) => Ok(b),
            Err(e) => Err(e),
        }
    }
}

/// Applies a function over the RDF graph and returns the result of that function
pub fn apply_rdf<RDF, A>(
    function: impl FnMut(&mut RDF) -> Result<A, RDFParseError>,
) -> impl RDFNodeParse<RDF, Output = A>
where
    RDF: FocusRDF,
{
    ApplyRDF { function }
}

#[derive(Debug, Clone)]
struct ApplyRDF<F> {
    function: F,
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
            Err(e) => Err(e),
        }
    }
}

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn instances_of<RDF>(expected: &IriS) -> impl RDFNodeParse<RDF, Output = Vec<RDF::Subject>>
where
    RDF: FocusRDF,
{
    subjects_with_property_value::<RDF>(rdf_type(), &expected.clone().into())
}

pub fn parse_rdf_type<RDF>() -> impl RDFNodeParse<RDF, Output = RDF::Term>
where
    RDF: FocusRDF,
{
    property_value(rdf_type())
}

pub fn has_type<RDF>(expected: IriS) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
{
    parse_rdf_type().then(move |term: RDF::Term| equals(term.clone(), expected.clone().into()))
}

pub fn equals<RDF>(term: RDF::Term, expected: RDF::Term) -> impl RDFNodeParse<RDF, Output = ()>
where
    RDF: FocusRDF,
{
    let expected_str = format!("{expected}");
    cond(
        &term,
        move |t| t == &expected,
        format!("Term {term} not equals {expected_str}"),
    )
}

/// Returns all nodes that are instances of the expected IRI in the RDF data
pub fn subjects_with_property_value<RDF>(
    property: &IriS,
    value: &RDF::Term,
) -> SubjectsPropertyValue<RDF>
where
    RDF: FocusRDF,
{
    SubjectsPropertyValue {
        property: property.clone().into(),
        value: value.clone(),
        _marker_rdf: PhantomData,
    }
}

pub struct SubjectsPropertyValue<RDF: NeighsRDF> {
    property: RDF::IRI,
    value: RDF::Term,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for SubjectsPropertyValue<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<RDF::Subject>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Vec<RDF::Subject>> {
        let subjects = rdf
            .triples_matching(Any, self.property.clone(), self.value.clone())
            .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?
            .map(Triple::into_subject)
            .collect();
        Ok(subjects)
    }
}

rdf_parser! {
    /// Parses the value of `property` as an RDF list
    pub fn property_value_as_list['a, RDF](property: &'a IriS)(RDF) -> Vec<RDF::Term>
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
    RDF: FocusRDF,
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
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = A;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        let rdf_type = parse_rdf_type().parse_impl(rdf)?;
        let iri: RDF::IRI =
            <RDF::Term as TryInto<RDF::IRI>>::try_into(rdf_type.clone()).map_err(|_| {
                RDFParseError::ExpectedIRI {
                    term: format!("{rdf_type}"),
                }
            })?;
        let iri_string = iri.as_str();
        match self.values.get_mut(&iri!(iri_string)) {
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

impl<RDF, A, B, P1, P2> RDFNodeParse<RDF> for With<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = A>,
    P2: RDFNodeParse<RDF, Output = B>,
{
    type Output = B;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
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
///
/// Moves the focus to each node in the list and applies the parser to it.
/// Returns a vector of results of each parser
pub fn parse_nodes<RDF, P>(nodes: Vec<RDF::Term>, parser: P) -> ParserNodes<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
{
    ParserNodes { nodes, parser }
}

pub fn parse_property_values<RDF, P>(property: &IriS, parser: P) -> ParserPropertyValues<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
{
    ParserPropertyValues {
        property: property.clone(),
        parser,
        _marker_rdf: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct ParserPropertyValues<RDF, P>
where
    RDF: FocusRDF,
{
    property: IriS,
    parser: P,
    _marker_rdf: PhantomData<RDF>,
}

impl<RDF, A, P> RDFNodeParse<RDF> for ParserPropertyValues<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        let values = property_values(&self.property).parse_impl(rdf)?;
        let mut results = Vec::new();
        for node in values.iter() {
            rdf.set_focus(node);
            let value = self.parser.parse_impl(rdf)?;
            results.push(value)
        }
        Ok(results)
    }
}

#[derive(Clone)]
pub struct ParserNodes<RDF, P>
where
    RDF: FocusRDF,
{
    nodes: Vec<RDF::Term>,
    parser: P,
}

impl<RDF, A, P> RDFNodeParse<RDF> for ParserNodes<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        let mut results = Vec::new();
        for node in self.nodes.iter() {
            rdf.set_focus(node);
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
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = O>,
{
    type Output = O;

    #[inline(always)]
    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        self.p.parse_impl(rdf)
    }
}

/// Parses the current focus node as a SHACL path
pub fn shacl_path_parse<RDF>(term: RDF::Term) -> impl RDFNodeParse<RDF, Output = SHACLPath>
where
    RDF: FocusRDF,
{
    ShaclPathParser::<RDF> {
        _marker_rdf: PhantomData,
        term,
    }
}

pub struct ShaclPathParser<RDF: FocusRDF> {
    _marker_rdf: PhantomData<RDF>,
    term: RDF::Term,
}

impl<RDF> RDFNodeParse<RDF> for ShaclPathParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = SHACLPath;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<SHACLPath> {
        rdf.set_focus(&self.term);
        if let Ok(iri) = RDF::term_as_iri(&self.term) {
            Ok(SHACLPath::iri(IriS::new_unchecked(iri.as_str())))
        } else if let Ok(_bnode) = RDF::term_as_bnode(&self.term) {
            // TODO: Refactor this code to use something like an or
            match sequence(rdf) {
                Ok(sequence) => Ok(sequence),
                Err(_err) => match alternative(rdf) {
                    Ok(alternative) => Ok(alternative),
                    Err(_err) => match zero_or_more_path(rdf) {
                        Ok(zero_or_more) => Ok(zero_or_more),
                        Err(_err) => match one_or_more_path(rdf) {
                            Ok(one_or_more) => Ok(one_or_more),
                            Err(_err) => match zero_or_one_path(rdf) {
                                Ok(zero_or_one) => Ok(zero_or_one),
                                Err(_err) => match inverse_path(rdf) {
                                    Ok(inverse) => Ok(inverse),
                                    Err(err) => Err(RDFParseError::Custom {
                                        msg: format!("Error parsing SHACL Path: {}", err),
                                    }),
                                },
                            },
                        },
                    },
                },
            }
        } else {
            Err(RDFParseError::UnexpectedLiteral {
                term: self.term.to_string(),
            })
        }
    }
}

fn sequence<RDF>(rdf: &mut RDF) -> std::result::Result<SHACLPath, RDFParseError>
where
    RDF: FocusRDF,
{
    let ls = rdf_list().parse_impl(rdf)?;
    let mut r = Vec::new();
    for t in ls {
        let p = shacl_path_parse::<RDF>(t).parse_impl(rdf)?;
        r.push(p);
    }
    Ok(SHACLPath::sequence(r))
}

fn alternative<RDF>(rdf: &mut RDF) -> std::result::Result<SHACLPath, RDFParseError>
where
    RDF: FocusRDF,
{
    let ls = property_value_as_list(sh_alternative_path())
        .parse_impl(rdf)?
        .into_iter()
        .map(|t| shacl_path_parse::<RDF>(t).parse_impl(rdf));
    let ls_iter: std::result::Result<Vec<SHACLPath>, RDFParseError> = ls.into_iter().collect();
    let ls = ls_iter?;
    Ok(SHACLPath::alternative(ls))
}

fn zero_or_more_path<RDF>(rdf: &mut RDF) -> std::result::Result<SHACLPath, RDFParseError>
where
    RDF: FocusRDF,
{
    let term = property_value(sh_zero_or_more_path()).parse_impl(rdf)?;
    let p = shacl_path_parse::<RDF>(term).parse_impl(rdf)?;
    Ok(SHACLPath::zero_or_more(p))
}

fn one_or_more_path<RDF>(rdf: &mut RDF) -> std::result::Result<SHACLPath, RDFParseError>
where
    RDF: FocusRDF,
{
    let term = property_value(sh_one_or_more_path()).parse_impl(rdf)?;
    let p = shacl_path_parse::<RDF>(term).parse_impl(rdf)?;
    Ok(SHACLPath::one_or_more(p))
}

fn zero_or_one_path<RDF>(rdf: &mut RDF) -> std::result::Result<SHACLPath, RDFParseError>
where
    RDF: FocusRDF,
{
    let term = property_value(sh_zero_or_one_path()).parse_impl(rdf)?;
    let p = shacl_path_parse::<RDF>(term).parse_impl(rdf)?;
    Ok(SHACLPath::zero_or_one(p))
}

fn inverse_path<RDF>(rdf: &mut RDF) -> std::result::Result<SHACLPath, RDFParseError>
where
    RDF: FocusRDF,
{
    let term = property_value(sh_inverse_path()).parse_impl(rdf)?;
    let p = shacl_path_parse::<RDF>(term).parse_impl(rdf)?;
    Ok(SHACLPath::inverse(p))
}
