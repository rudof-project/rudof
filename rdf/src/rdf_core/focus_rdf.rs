use crate::rdf_core::{
    NeighsRDF, RDFError, SHACLPath,
    parser::rdf_node_parser::{RDFNodeParse, constructors::ShaclPathParser},
};

/// A trait for RDF graphs that maintain a focus node for context-aware parsing.
///
/// `FocusRDF` extends [`NeighsRDF`] by adding the concept of a "focus node" - a current
/// point of reference within the RDF graph. This focus node serves as the starting point
/// for parsing operations, allowing parsers to navigate the graph structure from a specific
/// node without repeatedly specifying it.
///
/// The focus node pattern is commonly used in RDF validation (like SHACL) and graph
/// traversal scenarios where operations are performed relative to a particular node.
pub trait FocusRDF: NeighsRDF
where
    Self: 'static,
{
    /// Sets the current focus node.
    ///
    /// # Arguments
    ///
    /// * `focus` - The RDF term to set as the new focus node
    fn set_focus(&mut self, focus: &Self::Term);

    /// Retrieves the current focus node if one is set.
    ///
    /// # Returns
    ///
    /// A reference to `Some(term)` if a focus is set, or `None` otherwise
    fn get_focus(&self) -> Option<&Self::Term>;

    /// Retrieves the current focus node as a term, failing if no focus is set.
    ///
    /// This is a convenience method that unwraps the focus option and returns
    /// an error if no focus node is currently set. Useful when a focus is required
    /// for an operation to proceed.
    ///
    /// # Returns
    ///
    /// * `Ok(&term)` - A reference to the current focus node
    /// * `Err(RDFError::NoFocusNodeError)` - If no focus is currently set
    fn get_focus_as_term(&self) -> Result<&Self::Term, RDFError> {
        match self.get_focus() {
            None => Err(RDFError::NoFocusNodeError),
            Some(term) => Ok(term),
        }
    }

    /// Retrieves the current focus node as a subject, failing if not set or not a valid subject.
    ///
    /// # Returns
    ///
    /// * `Ok(subject)` - The current focus as a subject node
    /// * `Err(RDFError::NoFocusNodeError)` - If no focus is currently set
    /// * `Err(RDFError::ExpectedSubjectError)` - If the focus is a literal or cannot be converted to a subject
    fn get_focus_as_subject(&self) -> Result<Self::Subject, RDFError> {
        match self.get_focus() {
            None => Err(RDFError::NoFocusNodeError),
            Some(term) => {
                let subject =
                    Self::term_as_subject(term).map_err(|_| RDFError::ExpectedSubjectError {
                        node: format!("{term}"),
                        context: "get_focus_as_subject".to_string(),
                    })?;
                Ok(subject)
            }
        }
    }

    /// Parses and retrieves a SHACL path from a subject-predicate pair.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject node to query
    /// * `predicate` - The predicate whose object should be parsed as a SHACL path
    ///
    /// # Returns
    ///
    /// * `Ok(Some(path))` - If an object exists and was successfully parsed as a SHACL path
    /// * `Ok(None)` - If no object exists or parsing failed
    /// * `Err(e)` - If querying the graph for objects fails
    fn get_path_for(
        &mut self,
        subject: &Self::Term,
        predicate: &Self::IRI,
    ) -> Result<Option<SHACLPath>, RDFError> {
        match self.objects_for(subject, predicate)?.into_iter().next() {
            Some(term) => {
                let path = ShaclPathParser::new(term.clone())
                    .parse_focused(self)
                    .map_err(|e| RDFError::InvalidSHACLPathError {
                        node: format!("{term}"),
                        error: Box::new(e),
                    })?;
                Ok(Some(path))
            }
            None => Ok(None),
        }
    }
}
