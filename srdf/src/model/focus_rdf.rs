use super::rdf::TObject;
use super::rdf::Rdf;

/// Represents RDF graphs that contain a focus node.
///
/// The trait contains methods to get the focus node and for setting its value.
pub trait FocusRdf: Rdf {
    /// Set the value of the focus node
    fn set_focus(&mut self, focus: TObject<Self>);

    /// Get the focus node if it exists
    fn get_focus(&self) -> Option<&TObject<Self>>;
}
