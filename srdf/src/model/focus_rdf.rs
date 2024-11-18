use super::rdf::Rdf;
use super::TObjectRef;
use super::Term;

/// Represents RDF graphs that contain a focus node.
///
/// This trait contains methods to get the focus node and for setting its value.
pub trait FocusRdf: Rdf {
    /// Set the value of the focus node
    fn set_focus<T: Term>(&mut self, focus: T);

    /// Get the focus node if it exists
    fn get_focus(&self) -> Option<TObjectRef<Self::Triple<'_>>>;
}
