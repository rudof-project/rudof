use crate::rdf_core::{FocusRDF, RDFError, parser::rdf_node_parser::RDFNodeParse};
use std::marker::PhantomData;

/// Parser that returns the current focus node as a term.
///
/// This is the fundamental accessor for the focus position in the RDF graph.
/// It simply returns a clone of the current focus without modification.
#[derive(Debug, Clone)]
pub struct FocusParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> FocusParser<RDF> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

impl<RDF> Default for FocusParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> RDFNodeParse<RDF> for FocusParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)
    }
}

/// Parser that sets the focus to a specific term and returns ().
///
/// Note: While conceptually an effect rather than a parser, this enables
/// fluent navigation patterns in parser chains by allowing focus changes
/// to be composed with parsing operations.
#[derive(Debug, Clone)]
pub struct SetFocusParser<RDF>
where
    RDF: FocusRDF,
{
    target: RDF::Term,
}

impl<RDF> SetFocusParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(target: RDF::Term) -> Self {
        Self { target }
    }
}

impl<RDF> RDFNodeParse<RDF> for SetFocusParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = ();

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        rdf.set_focus(&self.target);
        Ok(())
    }
}
