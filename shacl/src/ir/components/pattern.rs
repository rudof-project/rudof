use crate::ir::error::IRError;
use rudof_rdf::rdf_core::utils::RDFRegex;
use std::fmt::{Display, Formatter};

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
#[derive(Debug, Clone)]
// TODO - Maybe remove pattern and flags and pick them from RDFRegex
pub(crate) struct Pattern {
    pattern: String,
    flags: Option<String>,
    regex: RDFRegex,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Result<Self, Box<IRError>> {
        let regex = RDFRegex::new(&pattern, flags.as_deref()).map_err(|e| IRError::InvalidRegex {
            pattern: pattern.clone(),
            flags: flags.clone(),
            error: Box::new(e),
        })?;

        Ok(Pattern { pattern, flags, regex })
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn flags(&self) -> Option<&String> {
        self.flags.as_ref()
    }

    pub fn regex(&self) -> &RDFRegex {
        &self.regex
    }

    pub fn match_str(&self, str: &str) -> bool {
        self.regex().is_match(str)
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(flags) = &self.flags {
            write!(f, "Pattern: /{}/{}", self.pattern(), flags)
        } else {
            write!(f, "Pattern: /{}/", self.pattern())
        }
    }
}
