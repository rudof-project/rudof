use std::{borrow::Cow, fmt::Display};

/// Regex utilities
///
use regex::{Regex, RegexBuilder};
use thiserror::Error;

const REGEX_SIZE_LIMIT: usize = 1_000_000;

#[derive(Debug, Clone)]
pub struct SRegex {
    regex: Regex,
    source: String,
    flags: Option<String>,
}

impl SRegex {
    /// Create a new regex with the given pattern and flags.
    /// The possible flags are defined in SPARQL, which are based on XPath:
    /// https://www.w3.org/TR/xpath-functions/#flags
    pub fn new(pattern: &str, flags: Option<&str>) -> Result<Self, SRegexError> {
        // Parts of this code have been inspired by
        // https://github.com/oxigraph/oxigraph/blob/main/lib/spareval/src/eval.rs
        let mut pattern = Cow::Borrowed(pattern);
        let flags = flags.unwrap_or_default();
        if flags.contains('q') {
            pattern = regex::escape(&pattern).into();
        }
        let mut regex_builder = RegexBuilder::new(&pattern);
        regex_builder.size_limit(REGEX_SIZE_LIMIT);
        for flag in flags.chars() {
            match flag {
                's' => {
                    regex_builder.dot_matches_new_line(true);
                }
                'm' => {
                    regex_builder.multi_line(true);
                }
                'i' => {
                    regex_builder.case_insensitive(true);
                }
                'x' => {
                    regex_builder.ignore_whitespace(true);
                }
                'q' => (),                                             // Already supported
                _ => return Err(SRegexError::InvalidFlagOption(flag)), // invalid option
            }
        }
        let regex = regex_builder.build()?;
        Ok(SRegex {
            regex,
            source: pattern.into_owned(),
            flags: flags
                .is_empty()
                .then(|| None)
                .unwrap_or(Some(flags.to_string())),
        })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    pub fn flags(&self) -> Option<&str> {
        self.flags.as_deref()
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Error, Debug)]
pub enum SRegexError {
    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(#[from] regex::Error),

    #[error("Invalid regex flag option: {0}")]
    InvalidFlagOption(char),
}

impl Display for SRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}/{}",
            self.regex.as_str(),
            self.flags.as_deref().unwrap_or("")
        )
    }
}
