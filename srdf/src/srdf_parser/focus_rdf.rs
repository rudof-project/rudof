use crate::{Query, RDFParseError};

/// Represents RDF graphs that contain a focus node
///
/// The trait contains methods to get the focus node and to set its value
pub trait FocusRDF: Query {
    /// Set the value of the focus node
    fn set_focus(&mut self, focus: &Self::Term);

    /// Get the focus node if it exists
    fn get_focus(&self) -> &Option<Self::Term>;

    /// Get the current focus as a Term
    fn get_focus_as_term(&self) -> Result<&Self::Term, RDFParseError> {
        match self.get_focus() {
            None => Err(RDFParseError::NoFocusNode),
            Some(term) => Ok(term),
        }
    }

    /// Get the current focus as a Subject
    fn get_focus_as_subject(&self) -> Result<Self::Subject, RDFParseError> {
        match self.get_focus() {
            None => Err(RDFParseError::NoFocusNode),
            Some(term) => {
                let subject =
                    term.clone()
                        .try_into()
                        .map_err(|_| RDFParseError::ExpectedSubject {
                            node: format!("{term}"),
                        })?;
                Ok(subject)
            }
        }
    }
}
