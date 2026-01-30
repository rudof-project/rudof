use crate::rdf_core::{Rdf, query::{QuerySolution, VariableSolutionIndex}};
use serde::Serialize;
use std::fmt::Display;

/// Represents a SPARQL variable name without the `?` or `$` prefix.
///
/// In SPARQL queries, variables are identifiers that begin with `?` or `$` (e.g.,
/// `?person`, `$email`). This type stores the variable name without the prefix,
/// but displays it with `?` when formatted.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize)]
pub struct VarName {
    /// The variable name string without prefix.
    str: String,
}

impl VarName {
    /// Creates a new variable name from a string.
    ///
    /// # Arguments
    ///
    /// * `str` - The variable name without prefix
    pub fn new(str: &str) -> VarName {
        VarName {
            str: str.to_string(),
        }
    }

    /// Returns the variable name as a string slice without prefix.
    pub fn as_str(&self) -> &str {
        &self.str
    }
}

impl Display for VarName {
    /// Formats the variable name with the SPARQL `?` prefix.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.str)
    }
}

impl From<String> for VarName {
    /// Converts a `String` into a `VarName`.
    ///
    /// This conversion takes ownership of the string and wraps it as a variable
    /// name. The input should be the variable name without the `?` or `$` prefix.
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert (without prefix)
    fn from(value: String) -> Self {
        VarName { str: value }
    }
}

impl<S: Rdf> VariableSolutionIndex<S> for &VarName {
    /// Finds the index of this variable in a query solution.
    ///
    /// This implementation allows `&VarName` to be used for looking up variable
    /// positions within a [`QuerySolution`]. The variable is matched by comparing
    /// the stored name string with variable names in the solution.
    ///
    /// # Arguments
    ///
    /// * `solution` - The query solution to search for this variable
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables_iter().position(|v| *v.str == self.str)
    }
}
