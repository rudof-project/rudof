use crate::{QuerySolution, Rdf, VariableSolutionIndex};
use serde::Serialize;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize)]
pub struct VarName {
    str: String,
}

impl VarName {
    pub fn as_str(&self) -> &str {
        &self.str
    }
}

impl Display for VarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.str)
    }
}

impl VarName {
    pub fn new(str: &str) -> VarName {
        VarName { str: str.to_string() }
    }
}

impl From<String> for VarName {
    fn from(value: String) -> Self {
        VarName { str: value }
    }
}

impl<S: Rdf> VariableSolutionIndex<S> for &VarName {
    #[inline]
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables_iter().position(|v| *v.str == self.str)
    }
}
