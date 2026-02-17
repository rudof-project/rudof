use crate::rdf_core::{Rdf, query::QuerySolution};

/// Trait for types that can index into a SPARQL query solution.
///
/// This trait abstracts over different ways to access variables in a
/// [`QuerySolution`]. It allows the same lookup API to work with numeric
/// indices, string variable names, or wrapped variable name types.
///
/// # Type Parameters
///
/// * `S` - The RDF graph type implementing [`Rdf`]
pub trait VariableSolutionIndex<S: Rdf> {
    /// Returns the position index of a variable in the query solution.
    ///
    /// This method attempts to locate the variable in the solution and returns
    /// its zero-based position index if found.
    ///
    /// # Arguments
    ///
    /// * `solution` - The query solution to search within
    fn index(self, solution: &QuerySolution<S>) -> Option<usize>;
}

impl<S: Rdf> VariableSolutionIndex<S> for usize {
    /// Returns the numeric index directly without validation.
    ///
    /// This implementation provides O(1) direct positional access to variables
    /// in a query solution. It does not validate that the index is within bounds;
    /// validation should be performed by the caller when accessing the value.
    ///
    /// # Arguments
    ///
    /// * `_` - The query solution (unused, as the index is already known)
    fn index(self, _: &QuerySolution<S>) -> Option<usize> {
        Some(self)
    }
}

impl<S: Rdf> VariableSolutionIndex<S> for &str {
    /// Finds the index of a variable by name string.
    ///
    /// This implementation performs a linear search through the solution's
    /// variables to find one matching the provided name string. The search
    /// compares against the bare variable name (without `?` or `$` prefix).
    ///
    /// # Arguments
    ///
    /// * `solution` - The query solution to search
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables_iter().position(|v| v.as_str() == self)
    }
}
