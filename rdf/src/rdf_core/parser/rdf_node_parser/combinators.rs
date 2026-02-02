use crate::rdf_core::{FocusRDF, RDFError, parser::rdf_node_parser::{RDFNodeParse, utils::parse_list_recursive, constructors::ValuesPropertyParser}};
use std::fmt::Debug;
use std::marker::PhantomData;
use iri_s::IriS;

// ============================================================================
// OPERATORS
// ============================================================================

/// Parses an optional value, returning `None` on failure without propagating the error.
///
/// This parser never fails; errors from the wrapped parser are converted to `None`.
///
/// # Type Parameters
///
/// * `P` - The wrapped parser type.
pub struct Optional<P> {
    /// The underlying parser to attempt.
    parser: P,
}

impl<P> Optional<P> {
    /// Creates a new optional parser.
    pub fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<RDF, P, T> RDFNodeParse<RDF> for Optional<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = T>,
{
    type Output = Option<T>;

    /// Attempts to parse a value, wrapping the result in `Option`.
    ///
    /// # Errors
    ///
    /// Never returns an error.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match self.parser.parse_focused(rdf) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None), 
        }
    }
}

/// Attempts the first parser, falling back to the second on failure.
///
/// Both parsers must produce the same output type. The second parser is only
/// attempted if the first returns an error.
///
/// # Type Parameters
///
/// * `P1` - The first parser to attempt.
/// * `P2` - The fallback parser.
pub struct Or<P1, P2> {
    /// The primary parser to try.
    first: P1,
    /// The fallback parser if the first fails.
    second: P2,
}

impl<P1, P2> Or<P1, P2> {
    /// Creates a new alternative parser.
    pub fn new(first: P1, second: P2) -> Self {
        Self { first, second }
    }
}

impl<RDF, P1, P2, T> RDFNodeParse<RDF> for Or<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = T>,
    P2: RDFNodeParse<RDF, Output = T>,
{
    type Output = T;

    /// Attempts the first parser, then the second if the first fails.
    ///
    /// # Errors
    ///
    /// Returns `RDFError::FailedOrError` containing both errors if both parsers fail.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match self.first.parse_focused(rdf) {
            Ok(v) => Ok(v),
            Err(err1) => self.second.parse_focused(rdf).map_err(|err2| RDFError::FailedOrError {
                err1: Box::new(err1),
                err2: Box::new(err2),
            }),
        }
    }
}

/// Parses two values sequentially, returning both results as a tuple.
///
/// Both parsers must succeed for the combined parser to succeed.
/// The results are returned as a tuple in the order parsed.
///
/// # Type Parameters
///
/// * `P1` - The first parser.
/// * `P2` - The second parser.
pub struct And<P1, P2> {
    /// The first parser to execute.
    first: P1,
    /// The second parser to execute.
    second: P2,
}

impl<P1, P2> And<P1, P2> {
    /// Creates a new sequential parser returning both results.
    pub fn new(first: P1, second: P2) -> Self {
        Self { first, second }
    }
}

impl<RDF, P1, P2, T1, T2> RDFNodeParse<RDF> for And<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = T1>,
    P2: RDFNodeParse<RDF, Output = T2>,
{
    type Output = (T1, T2);

    /// Executes both parsers and returns both results as a tuple.
    ///
    /// # Errors
    ///
    /// Returns the error from the first parser if it fails,
    /// or the error from the second parser if it fails.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let v1 = self.first.parse_focused(rdf)?;
        let v2 = self.second.parse_focused(rdf)?;
        Ok((v1, v2))
    }
}

/// Negates a parser's result, succeeding when the inner parser fails.
///
/// Requires the inner parser's output to implement `Debug` to report the
/// unexpected success value in error messages.
///
/// # Type Parameters
///
/// * `P` - The parser to negate.
pub struct Not<P> {
    /// The parser whose success/failure is inverted.
    parser: P,
}

impl<P> Not<P> {
    /// Creates a new negated parser.
    pub fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<RDF, P, T> RDFNodeParse<RDF> for Not<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = T>,
    T: Debug,
{
    type Output = ();

    /// Inverts the parse result.
    ///
    /// # Errors
    ///
    /// Returns `RDFError::FailedNotError` if the underlying parser succeeds,
    /// containing the debug representation of the parsed value.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match self.parser.parse_focused(rdf) {
            Ok(v) => Err(RDFError::FailedNotError { value: format!("{v:?}") }),
            Err(_) => Ok(()), 
        }
    }
}

/// Parses two values sequentially, returning only the second result.
///
/// The first parser's result is computed but discarded. This is useful for
/// parsing syntax elements where only specific parts are relevant.
///
/// # Type Parameters
///
/// * `P1` - The parser producing the discarded value.
/// * `P2` - The parser producing the returned value.
pub struct With<P1, P2> {
    /// The parser to execute first.
    first: P1,
    /// The parser whose result is returned.
    second: P2,
}

impl<P1, P2> With<P1, P2> {
    /// Creates a new sequential parser.
    pub fn new(first: P1, second: P2) -> Self {
        Self { first, second }
    }
}

impl<RDF, P1, P2, T1, T2> RDFNodeParse<RDF> for With<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = T1>,
    P2: RDFNodeParse<RDF, Output = T2>,
{
    type Output = T2;

    /// Executes both parsers and returns the second result.
    ///
    /// # Errors
    ///
    /// Returns an error immediately if the first parser fails,
    /// otherwise returns the error from the second parser if it fails.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let _ = self.first.parse_focused(rdf)?;
        self.second.parse_focused(rdf)
    }
}

// ============================================================================
// COMBINATORS
// ============================================================================

/// Transforms a successful parse result using a mapping function.
///
/// # Type Parameters
///
/// * `P` - The underlying parser.
/// * `F` - The transformation function type.
#[derive(Copy, Clone)]
pub struct Map<P, F> {
    /// The parser producing the input value.
    parser: P,
    /// The function applied to transform the parsed value.
    function: F,
}

impl<P, F> Map<P, F> {
    /// Creates a new mapping parser.
    pub fn new(parser: P, function: F) -> Self {
        Self { parser, function }
    }
}

impl<RDF, A, B, P, F> RDFNodeParse<RDF> for Map<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
    F: Fn(A) -> B,
{
    type Output = B;

    /// Parses a value and applies the mapping function.
    ///
    /// # Errors
    ///
    /// Propagates errors from the underlying parser without transformation.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match self.parser.parse_focused(rdf) {
            Ok(a) => Ok((self.function)(a)),
            Err(e) => Err(e),
        }
    }
}

/// Chains a fallible transformation after a successful parse.
///
/// Similar to `FlatMap`, but named following `Result::and_then` conventions.
///
/// # Type Parameters
///
/// * `P` - The initial parser.
/// * `F` - The fallible transformation function.
#[derive(Copy, Clone)]
pub struct AndThen<P, F> {
    /// The parser producing the input value.
    parser: P,
    /// The function transforming the value, possibly failing.
    function: F,
}

impl<P, F> AndThen<P, F> {
    /// Creates a new chaining parser.
    pub fn new(parser: P, function: F) -> Self {
        Self { parser, function }
    }
}

impl<RDF, P, F, O> RDFNodeParse<RDF> for AndThen<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: Fn(P::Output) -> Result<O, RDFError>,
{
    type Output = O;

    /// Parses a value and applies the fallible transformation.
    ///
    /// # Errors
    ///
    /// Returns the error from the initial parser if it fails,
    /// or the error from the transformation function if it fails.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let value = self.parser.parse_focused(rdf)?;
        (self.function)(value)
    }
}

/// Chains a fallible transformation after a successful parse.
///
/// Functionally equivalent to `AndThen`, following monadic naming conventions.
///
/// # Type Parameters
///
/// * `P` - The initial parser.
/// * `F` - The fallible transformation function.
#[derive(Copy, Clone)]
pub struct FlatMap<P, F> {
    /// The parser producing the input value.
    parser: P,
    /// The function transforming the value, possibly failing.
    function: F,
}

impl<P, F> FlatMap<P, F> {
    /// Creates a new flat-mapping parser.
    pub fn new(parser: P, function: F) -> Self {
        Self { parser, function }
    }
}

impl<RDF, P, F, O> RDFNodeParse<RDF> for FlatMap<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: Fn(P::Output) -> Result<O, RDFError>,
{
    type Output = O;

    /// Parses a value and applies the fallible transformation.
    ///
    /// # Errors
    ///
    /// Returns the error from the initial parser if it fails,
    /// or the error from the transformation function if it fails.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let value = self.parser.parse_focused(rdf)?;
        (self.function)(value)
    }
}

/// Chains two parsers where the second depends on the first's output.
///
/// This enables context-sensitive parsing where subsequent parsing
/// decisions depend on previously parsed values.
///
/// # Type Parameters
///
/// * `P` - The initial parser.
/// * `F` - The function producing the second parser.
#[derive(Copy, Clone)]
pub struct Then<P, F> {
    /// The first parser to execute.
    parser: P,
    /// The function generating the next parser from the first result.
    function: F,
}

impl<P, F> Then<P, F> {
    /// Creates a new dependent parser.
    pub fn new(parser: P, function: F) -> Self {
        Self { parser, function }
    }
}

impl<RDF, P, F, N> RDFNodeParse<RDF> for Then<P, F>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF>,
    F: Fn(P::Output) -> N,
    N: RDFNodeParse<RDF>,
{
    type Output = N::Output;

    /// Executes the first parser, then the dependent parser.
    ///
    /// # Errors
    ///
    /// Returns the error from the first parser if it fails,
    /// or the error from the second parser if it fails.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let value = self.parser.parse_focused(rdf)?;
        (self.function)(value).parse_focused(rdf)
    }
}

/// Concatenates results from two vector-producing parsers.
///
/// Both parsers must produce vectors of the same element type.
/// The results are concatenated in order.
///
/// # Type Parameters
///
/// * `P1` - The first parser producing a vector.
/// * `P2` - The second parser producing a vector.
pub struct Combine<P1, P2> {
    /// The first parser.
    parser1: P1,
    /// The second parser.
    parser2: P2,
}

impl<P1, P2> Combine<P1, P2> {
    /// Creates a new combining parser.
    pub fn new(parser1: P1, parser2: P2) -> Self {
        Self { parser1, parser2 }
    }
}

impl<RDF, P1, P2, A> RDFNodeParse<RDF> for Combine<P1, P2>
where
    RDF: FocusRDF,
    P1: RDFNodeParse<RDF, Output = Vec<A>>,
    P2: RDFNodeParse<RDF, Output = Vec<A>>,
{
    type Output = Vec<A>;

    /// Executes both parsers and concatenates their vector results.
    ///
    /// # Errors
    ///
    /// Returns the error from the first parser if it fails,
    /// or the error from the second parser if it fails.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let vs1 = self.parser1.parse_focused(rdf)?;
        let vs2 = self.parser2.parse_focused(rdf)?;
        let mut result = vs1;
        result.extend(vs2);
        Ok(result)
    }
}

/// Concatenates results from multiple vector-producing parsers.
///
/// Executes parsers sequentially, accumulating all results into a single vector.
///
/// # Type Parameters
///
/// * `P` - The parser type producing vectors.
pub struct CombineMany<P> {
    /// The collection of parsers to execute.
    parsers: Vec<P>,
}

impl<P> CombineMany<P> {
    /// Creates a new multi-combining parser.
    pub fn new(parsers: Vec<P>) -> Self {
        Self { parsers }
    }
}

impl<RDF, P, A> RDFNodeParse<RDF> for CombineMany<P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = Vec<A>>,
{
    type Output = Vec<A>;

    /// Executes all parsers and concatenates their results.
    ///
    /// # Errors
    ///
    /// Returns immediately with the first error encountered.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let mut result = Vec::new();
        for p in self.parsers.iter() {
            let vs = p.parse_focused(rdf)?;
            result.extend(vs);
        }
        Ok(result)
    }
}

/// Applies a parser to multiple focus nodes sequentially.
///
/// Iterates over a collection of nodes, setting each as the focus
/// and applying the parser, collecting all successful results.
///
/// # Type Parameters
///
/// * `RDF` - The RDF graph type.
/// * `P` - The parser to apply to each node.
#[derive(Clone)]
pub struct ForEach<RDF, P>
where
    RDF: FocusRDF,
{
    /// The nodes to process as focus points.
    nodes: Vec<RDF::Term>,
    /// The parser applied to each node.
    parser: P,
}

impl<RDF, P> ForEach<RDF, P>
where
    RDF: FocusRDF,
{
    /// Creates a new foreach parser.
    pub fn new(nodes: Vec<RDF::Term>, parser: P) -> Self {
        Self { nodes, parser }
    }
}

impl<RDF, A, P> RDFNodeParse<RDF> for ForEach<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    /// Parses each node in sequence.
    ///
    /// # Errors
    ///
    /// Returns immediately with the first parsing error encountered.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let mut results = Vec::new();
        for node in self.nodes.iter() {
            rdf.set_focus(node);
            let value = self.parser.parse_focused(rdf)?;
            results.push(value);
        }
        Ok(results)
    }
}

/// Parses an RDF collection (rdf:first/rdf:rest list) into a vector.
///
/// Recursively traverses the RDF list structure starting from the current focus node,
/// applying the element parser to each list member.
///
/// # Type Parameters
///
/// * `RDF` - The RDF graph type.
/// * `P` - The parser for individual list elements.
#[derive(Debug, Clone)]
pub struct List<RDF, P> {
    /// The parser for list elements.
    parser: P,
    /// Phantom marker for the RDF type parameter.
    _marker: PhantomData<RDF>,
}

impl<RDF, P> List<RDF, P> {
    /// Creates a new list parser.
    pub fn new(parser: P) -> Self {
        List {
            parser,
            _marker: PhantomData,
        }
    }
}

impl<RDF, P, A> RDFNodeParse<RDF> for List<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    /// Parses the RDF list structure.
    ///
    /// # Errors
    ///
    /// Returns `RDFError::NoFocusNodeError` if no focus is set,
    /// or errors from the list parsing utility or element parser.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus()
            .ok_or(RDFError::NoFocusNodeError)?
            .clone();
        let nodes = parse_list_recursive::<RDF>(vec![focus], rdf)?;

        let mut results = Vec::with_capacity(nodes.len());
        for node in nodes {
            rdf.set_focus(&node);
            let value = self.parser.parse_focused(rdf)?;
            results.push(value);
        }
        Ok(results)
    }
}

/// Applies a parser to all values of a specific property.
///
/// Retrieves all objects of the given property from the current focus node,
/// then applies the parser to each value.
///
/// # Type Parameters
///
/// * `RDF` - The RDF graph type.
/// * `P` - The parser for property values.
#[derive(Debug, Clone)]
pub struct MapPropertyValuesParser<RDF, P> {
    /// The property IRI whose values are parsed.
    property: IriS,
    /// The parser applied to each property value.
    parser: P,
    /// Phantom marker for the RDF type parameter.
    _marker: PhantomData<RDF>,
}

impl<RDF, P> MapPropertyValuesParser<RDF, P> {
    /// Creates a new property values parser.
    pub fn new(property: IriS, parser: P) -> Self {
        Self {
            property,
            parser,
            _marker: PhantomData,
        }
    }
}

impl<RDF, P, A> RDFNodeParse<RDF> for MapPropertyValuesParser<RDF, P>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = A>,
{
    type Output = Vec<A>;

    /// Parses all values of the specified property.
    ///
    /// # Errors
    ///
    /// Returns errors from the value retrieval or element parsing.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let values = ValuesPropertyParser::new(self.property.clone()).parse_focused(rdf)?;
        let mut results = Vec::new();

        for node in values {
            rdf.set_focus(&node);
            results.push(self.parser.parse_focused(rdf)?);
        }
        Ok(results)
    }
}

// ============================================================================
// VALIDATION PARSERS
// ============================================================================

/// Validates the current focus node against a predicate.
///
/// Succeeds if the predicate returns true for the focus term,
/// otherwise fails with a descriptive error.
///
/// # Type Parameters
///
/// * `RDF` - The RDF graph type.
/// * `P` - The predicate function type.
#[derive(Clone)]
pub struct SatisfyParser<RDF, P> {
    /// The validation predicate.
    predicate: P,
    /// The name of the condition for error reporting.
    condition_name: String,
    /// Phantom marker for the RDF type parameter.
    _marker: PhantomData<RDF>,
}

impl<RDF, P> SatisfyParser<RDF, P>
where
    RDF: FocusRDF,
    P: Fn(&RDF::Term) -> bool,
{
    /// Creates a new validation parser.
    ///
    /// # Arguments
    ///
    /// * `predicate` - The validation function.
    /// * `condition_name` - A descriptive name for the condition used in error messages.
    pub fn new(predicate: P, condition_name: impl Into<String>) -> Self {
        Self {
            predicate,
            condition_name: condition_name.into(),
            _marker: PhantomData,
        }
    }
}

impl<RDF, P> RDFNodeParse<RDF> for SatisfyParser<RDF, P>
where
    RDF: FocusRDF,
    P: Fn(&RDF::Term) -> bool,
{
    type Output = ();

    /// Validates the focus node against the predicate.
    ///
    /// # Errors
    ///
    /// Returns `RDFError::NoFocusNodeError` if no focus is set,
    /// or `RDFError::NodeDoesntSatisfyConditionError` if the predicate returns false.
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus()
            .ok_or(RDFError::NoFocusNodeError)?;
            
        if (self.predicate)(&focus) {
            Ok(())
        } else {
            Err(RDFError::NodeDoesntSatisfyConditionError {
                condition_name: self.condition_name.clone(),
                node: focus.to_string(),
            })
        }
    }
}

// ============================================================================
// PARSER EXTENSION TRAIT (Fluent API)
// ============================================================================

/// Extension trait providing fluent combinator methods for parsers.
///
/// Automatically implemented for all types implementing `RDFNodeParse<RDF>`,
/// enabling method chaining for parser composition.
pub trait ParserExt<RDF>: RDFNodeParse<RDF> + Sized 
where 
    RDF: FocusRDF,
{
    /// Transforms successful results using a function.
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Output) -> B,
    {
        Map::new(self, f)
    }

    /// Chains a fallible transformation after this parser.
    fn and_then<F, O>(self, func: F) -> AndThen<Self, F>
    where
        F: Fn(Self::Output) -> Result<O, RDFError>,
    {
        AndThen::new(self, func)
    }

    /// Chains a fallible transformation (monadic bind).
    fn flat_map<F, O>(self, function: F) -> FlatMap<Self, F>
    where
        F: Fn(Self::Output) -> Result<O, RDFError>,
    {
        FlatMap::new(self, function)
    }

    /// Creates a dependent parser consuming this parser's output.
    fn then<F, N>(self, function: F) -> Then<Self, F>
    where
        F: Fn(Self::Output) -> N,
        N: RDFNodeParse<RDF>,
    {
        Then::new(self, function)
    }

    /// Makes the parser optional, converting failure to `None`.
    fn optional(self) -> Optional<Self> {
        Optional::new(self)
    }

    /// Provides an alternative parser if this one fails.
    fn or<P>(self, other: P) -> Or<Self, P>
    where
        P: RDFNodeParse<RDF, Output = Self::Output>,
    {
        Or::new(self, other)
    }

    /// Combines this parser with another, returning both results as a tuple.
    fn and<P, B>(self, other: P) -> And<Self, P>
    where
        P: RDFNodeParse<RDF, Output = B>,
    {
        And::new(self, other)
    }

    /// Negates this parser's result.
    fn not(self) -> Not<Self>
    where
        Self::Output: Debug,
    {
        Not::new(self)
    }

    /// Runs this parser for side effects, then returns the result of another.
    fn with<P, B>(self, other: P) -> With<Self, P>
    where
        P: RDFNodeParse<RDF, Output = B>,
    {
        With::new(self, other)
    }

    /// Combines two vector-producing parsers.
    fn combine<P, A>(self, other: P) -> Combine<Self, P>
    where
        Self: RDFNodeParse<RDF, Output = Vec<A>>,
        P: RDFNodeParse<RDF, Output = Vec<A>>,
    {
        Combine::new(self, other)
    }

    /// Combines multiple vector-producing parsers.
    fn combine_many<A>(self, others: Vec<Self>) -> CombineMany<Self>
    where
        Self: RDFNodeParse<RDF, Output = Vec<A>>,
    {
        let mut all = vec![self];
        all.extend(others);
        CombineMany::new(all)
    }

    /// Applies this parser to multiple nodes.
    fn for_each(self, nodes: Vec<RDF::Term>) -> ForEach<RDF, Self>
    where
        Self: Clone,
    {
        ForEach::new(nodes, self)
    }

    /// Parses an RDF list using this parser for elements.
    fn list<A>(self) -> List<RDF, Self>
    where 
        Self: RDFNodeParse<RDF, Output = A>,
    {
        List::new(self)
    }

    /// Applies this parser to all values of a property.
    fn map_property(self, property: IriS) -> MapPropertyValuesParser<RDF, Self>
    where
        Self: RDFNodeParse<RDF>,
    {
        MapPropertyValuesParser::new(property, self)
    }
}

impl<RDF, T> ParserExt<RDF> for T
where
    T: RDFNodeParse<RDF> + Sized,
    RDF: FocusRDF,
{
}