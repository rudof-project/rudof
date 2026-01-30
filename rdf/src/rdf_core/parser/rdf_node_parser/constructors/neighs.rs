use crate::rdf_core::{
    FocusRDF, NeighsRDF, RDFError,
    parser::rdf_node_parser::RDFNodeParse,
};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

/// Parser that extracts all outgoing arcs (neighborhood) from the focus node.
#[derive(Debug, Clone)]
pub struct NeighborhoodParser<RDF> {
    _marker: PhantomData<RDF>,
}

impl<RDF> NeighborhoodParser<RDF> {
    pub fn new() -> Self {
        NeighborhoodParser { _marker: PhantomData }
    }
}

impl<RDF> RDFNodeParse<RDF> for NeighborhoodParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashMap<RDF::IRI, HashSet<RDF::Term>>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus().ok_or(RDFError::NoFocusNodeError)?;
        let subj = RDF::term_as_subject(&focus).map_err(|_| {
            RDFError::ExpectedFocusAsSubjectError {
                focus: focus.to_string(),
            }
        })?;
        rdf.outgoing_arcs(&subj).map_err(|e| RDFError::OutgoingArcsError {
            focus: focus.to_string(),
            error: e.to_string(),
        })
    }
}