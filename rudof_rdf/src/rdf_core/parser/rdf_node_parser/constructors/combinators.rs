use crate::rdf_core::{FocusRDF, RDFError, parser::rdf_node_parser::RDFNodeParse};
use std::marker::PhantomData;

/// Parser that chooses between two parsers based on a predicate.
///
/// evaluates the predicate on a stored value and executes either
/// the `then` parser or the `else` parser accordingly.
#[derive(Debug, Clone)]
pub struct IfThenElseParser<A, P, Then, Else> {
    value: A,
    predicate: P,
    then_parser: Then,
    else_parser: Else,
}

impl<A, P, Then, Else> IfThenElseParser<A, P, Then, Else>
where
    P: Fn(&A) -> bool,
{
    pub fn new(value: A, predicate: P, then_parser: Then, else_parser: Else) -> Self {
        Self {
            value,
            predicate,
            then_parser,
            else_parser,
        }
    }
}

impl<RDF, A, B, P, Then, Else> RDFNodeParse<RDF> for IfThenElseParser<A, P, Then, Else>
where
    RDF: FocusRDF,
    P: Fn(&A) -> bool,
    Then: RDFNodeParse<RDF, Output = B>,
    Else: RDFNodeParse<RDF, Output = B>,
{
    type Output = B;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        if (self.predicate)(&self.value) {
            self.then_parser.parse_focused(rdf)
        } else {
            self.else_parser.parse_focused(rdf)
        }
    }
}

/// Parser that applies a pure function to a stored value.
#[derive(Debug, Clone)]
pub struct ApplyParser<A, F, O> {
    value: A,
    function: F,
    _phantom: std::marker::PhantomData<O>,
}

impl<A, F, O> ApplyParser<A, F, O>
where
    F: Fn(&A) -> Result<O, RDFError>,
{
    pub fn new(value: A, function: F) -> Self {
        Self {
            value,
            function,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<RDF, A, F, O> RDFNodeParse<RDF> for ApplyParser<A, F, O>
where
    RDF: FocusRDF,
    F: Fn(&A) -> Result<O, RDFError>,
{
    type Output = O;

    fn parse_focused(&self, _rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        (self.function)(&self.value)
    }
}

/// Parser that applies a function directly to the RDF graph.
///
/// This is the "escape hatch" for arbitrary graph operations that
/// don't fit the standard parser patterns. The function receives
/// mutable access to the RDF graph and can perform any query or
/// modification.
#[derive(Debug, Clone)]
pub struct ApplyRdfParser<RDF, F> {
    function: F,
    _marker: PhantomData<RDF>,
}

impl<RDF, F> ApplyRdfParser<RDF, F> {
    pub fn new<A>(function: F) -> Self
    where
        RDF: FocusRDF,
        F: Fn(&mut RDF) -> Result<A, RDFError>,
    {
        Self {
            function,
            _marker: PhantomData,
        }
    }
}

impl<RDF, A, F> RDFNodeParse<RDF> for ApplyRdfParser<RDF, F>
where
    RDF: FocusRDF,
    F: Fn(&mut RDF) -> Result<A, RDFError>,
{
    type Output = A;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        (self.function)(rdf)
    }
}
