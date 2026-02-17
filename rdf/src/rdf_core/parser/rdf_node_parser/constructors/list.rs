use crate::rdf_core::{
    FocusRDF, RDFError,
    parser::rdf_node_parser::{RDFNodeParse, utils::parse_list_recursive},
};
use std::marker::PhantomData;

/// Parser that extracts all elements from an RDF list structure.
///
/// Traverses the RDF list using `rdf:first` and `rdf:rest` properties,
/// collecting elements until reaching `rdf:nil`. Detects cycles.
#[derive(Debug, Clone)]
pub struct ListParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> ListParser<RDF> {
    pub fn new() -> Self {
        ListParser { _marker: PhantomData }
    }
}

impl<RDF> Default for ListParser<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

impl<RDF> RDFNodeParse<RDF> for ListParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<RDF::Term>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus().ok_or(RDFError::NoFocusNodeError)?;
        parse_list_recursive::<RDF>(vec![focus.clone()], rdf)
    }
}
