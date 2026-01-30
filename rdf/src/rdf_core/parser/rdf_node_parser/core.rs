use crate::rdf_core::{
    FocusRDF, RDFError
};
use iri_s::IriS;

/// A trait for parsing RDF data.
///
/// Types implementing `RDFNodeParse` can parse RDF graphs that maintain a focus node,
/// which represents the current node being examined during parsing. The RDF data structure
/// must implement the [`FocusRDF`] trait to support focus node operations.
///
/// This trait provides a combinator-based parsing API inspired by parser combinator libraries,
/// allowing complex parsers to be built by composing simpler ones.
///
/// # Type Parameters
///
/// * `RDF` - The RDF data structure type that implements [`FocusRDF`]
pub trait RDFNodeParse<RDF> 
where 
    RDF: FocusRDF,
{
    /// The type returned when parsing succeeds.
    type Output;
    
    /// Parses RDF data starting from the specified node.
    ///
    /// This is the main entry point for parsing. It sets the focus node of the RDF graph
    /// to `node` and then runs the parser implementation.
    ///
    /// # Arguments
    ///
    /// * `node` - The IRI of the node to set as the focus before parsing
    /// * `rdf` - The RDF graph data to parse
    fn parse(&self, node: &IriS, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = RDF::Term::from(RDF::IRI::from(node.clone()));
        rdf.set_focus(&focus);
        self.parse_focused(rdf)
    }
    
    /// The internal parsing implementation that operates on the current focus node.
    ///
    /// This method performs the actual parsing logic without modifying which node is focused.
    /// It is called by [`parse`](Self::parse) after the focus has been set.
    ///
    /// # Arguments
    ///
    /// * `rdf` - A mutable reference to the RDF graph with the focus already set
    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError>;
}