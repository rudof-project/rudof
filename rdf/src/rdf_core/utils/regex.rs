use std::{borrow::Cow, fmt::Display};

use regex::{Regex, RegexBuilder};
use thiserror::Error;

/// Maximum size limit for compiled regular expressions.
const REGEX_SIZE_LIMIT: usize = 1_000_000;

/// A regular expression for RDF and SPARQL pattern matching.
///
/// This type wraps the Rust [`Regex`] type with SPARQL/XPath-compatible flag
/// handling. It stores both the compiled regex and the original pattern with
/// flags for display and debugging purposes.
#[derive(Debug, Clone)]
pub struct RDFRegex {
    /// The compiled regular expression.
    regex: Regex,
    /// The source pattern string (after flag processing).
    source: String,
    /// Optional flags that were applied to this regex.
    flags: Option<String>,
}

impl RDFRegex {
    /// Creates a new regular expression with SPARQL-compatible flags.
    ///
    /// This constructor builds a regex following the XPath/SPARQL flag semantics,
    /// which differ slightly from standard Rust regex flags. The implementation
    /// is inspired by the Oxigraph SPARQL engine (https://github.com/oxigraph/oxigraph/blob/main/lib/spareval/src/eval.rs).
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern string
    /// * `flags` - Optional string containing flag characters (see below)
    ///
    /// # Supported Flags
    ///
    /// Following the [XPath specification](https://www.w3.org/TR/xpath-functions/#flags):
    ///
    /// - **`s`** (dot-all): Makes `.` match any character including newlines
    /// - **`m`** (multi-line): Makes `^` and `$` match line boundaries, not just string boundaries
    /// - **`i`** (case-insensitive): Performs case-insensitive matching
    /// - **`x`** (ignore whitespace): Ignores unescaped whitespace and enables comments with `#`
    /// - **`q`** (quote/literal): Treats the entire pattern as a literal string (escapes special characters)
    ///
    /// Flags can be combined, e.g., `"im"` for case-insensitive multi-line matching.
    /// 
    /// # Size Limits
    ///
    /// The compiled regex is limited to [`REGEX_SIZE_LIMIT`] bytes. Patterns
    /// exceeding this limit will fail with an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The pattern syntax is invalid (e.g., unbalanced parentheses, invalid escape sequences)
    /// - An unsupported flag character is provided
    /// - The compiled regex exceeds the size limit
    pub fn new(pattern: &str, flags: Option<&str>) -> Result<Self, RDFRegexError> {
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
                'q' => (),                                             
                _ => return Err(RDFRegexError::InvalidFlagOption(flag)), 
            }
        }
        let regex = regex_builder.build()?;
        Ok(RDFRegex {
            regex,
            source: pattern.into_owned(),
            flags: if flags.is_empty() {
                None
            } else {
                Some(flags.to_string())
            },
        })
    }

    /// Tests whether the regex matches the given text.
    ///
    /// Returns `true` if the pattern matches any part of the input string,
    /// following standard regex semantics (not anchored by default).
    ///
    /// # Arguments
    ///
    /// * `text` - The string to test against the pattern
    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    /// Returns the flags used to create this regex, if any.
    pub fn flags(&self) -> Option<&str> {
        self.flags.as_deref()
    }

    /// Returns the source pattern string.
    ///
    /// For patterns created with the `q` (quote) flag, this returns the
    /// escaped version of the original pattern.
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Errors that can occur when creating or using RDF regular expressions.
#[derive(Error, Debug)]
pub enum RDFRegexError {
    /// The regex pattern has invalid syntax.
    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(#[from] regex::Error),

    /// An unsupported flag character was provided.
    ///
    /// Valid flags are: `s`, `m`, `i`, `x`, `q`. Any other character
    /// in the flags string will trigger this error.
    #[error("Invalid regex flag option: {0}")]
    InvalidFlagOption(char),
}

impl Display for RDFRegex {
    /// Formats the regex in a compact notation showing pattern and flags.
    /// 
    /// # Output format 
    /// - `/pattern/flags`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}/{}",
            self.regex.as_str(),
            self.flags.as_deref().unwrap_or("")
        )
    }
}
