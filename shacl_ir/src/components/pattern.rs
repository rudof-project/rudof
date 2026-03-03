use crate::compiled_shacl_error::CompiledShaclError;
use rudof_rdf::rdf_core::utils::RDFRegex;
use std::fmt::Display;

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
#[derive(Debug, Clone)]
pub struct Pattern {
    pattern: String,
    flags: Option<String>,
    regex: RDFRegex,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Result<Self, Box<CompiledShaclError>> {
        let regex = RDFRegex::new(&pattern, flags.as_deref()).map_err(|e| CompiledShaclError::InvalidRegex {
            pattern: pattern.clone(),
            flags: flags.clone(),
            error: Box::new(e),
        })?;
        Ok(Pattern { pattern, flags, regex })
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn flags(&self) -> &Option<String> {
        &self.flags
    }

    pub fn regex(&self) -> &RDFRegex {
        &self.regex
    }

    pub fn match_str(&self, str: &str) -> bool {
        self.regex().is_match(str)
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(flags) = &self.flags {
            write!(f, "Pattern: /{}/{}", self.pattern(), flags)
        } else {
            write!(f, "Pattern: /{}/", self.pattern())
        }
    }
}
