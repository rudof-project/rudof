use tracing::debug;

use crate::{shacl_path_parse, NeighsRDF, RDFError, RDFNodeParse, RDFParseError, SHACLPath};

/// Represents RDF graphs that contain a focus node
///
/// The trait contains methods to get the focus node and to set its value
pub trait FocusRDF: NeighsRDF {
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
                    Self::term_as_subject(term).map_err(|_| RDFParseError::ExpectedSubject {
                        node: format!("{term}"),
                        context: "get_focus_as_subject".to_string(),
                    })?;
                Ok(subject)
            }
        }
    }

    fn get_path_for(
        &mut self,
        subject: &Self::Term,
        predicate: &Self::IRI,
    ) -> Result<Option<SHACLPath>, RDFError> {
        match self.objects_for(subject, predicate)?.into_iter().next() {
            Some(term) => match shacl_path_parse(term.clone()).parse_impl(self) {
                Ok(path) => Ok(Some(path)),
                Err(e) => {
                    debug!("Error parsing PATH from report...{e}");
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
}
