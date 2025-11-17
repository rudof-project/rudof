use crate::QuerySolution;
use crate::Rdf;

pub trait VariableSolutionIndex<S: Rdf> {
    fn index(self, solution: &QuerySolution<S>) -> Option<usize>;
}

impl<S: Rdf> VariableSolutionIndex<S> for usize {
    #[inline]
    fn index(self, _: &QuerySolution<S>) -> Option<usize> {
        Some(self)
    }
}

impl<S: Rdf> VariableSolutionIndex<S> for &str {
    #[inline]
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables_iter().position(|v| v.as_str() == self)
    }
}
