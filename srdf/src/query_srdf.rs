use std::rc::Rc;

use crate::SRDFBasic;

pub trait QuerySRDF: SRDFBasic {
    fn query_select(&self, query: &str) -> Result<QuerySolutionIter<Self>, Self::Err>
    where
        Self: Sized;

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct VarName {
    str: String,
}

impl From<String> for VarName {
    fn from(value: String) -> Self {
        VarName { str: value }
    }
}

pub trait VariableSolutionIndex<S: SRDFBasic> {
    fn index(self, solution: &QuerySolution<S>) -> Option<usize>;
}

impl<S: SRDFBasic> VariableSolutionIndex<S> for usize {
    #[inline]
    fn index(self, _: &QuerySolution<S>) -> Option<usize> {
        Some(self)
    }
}

impl<S: SRDFBasic> VariableSolutionIndex<S> for &str {
    #[inline]
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables.iter().position(|v| v.str == self)
    }
}

impl<S: SRDFBasic> VariableSolutionIndex<S> for &VarName {
    #[inline]
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables.iter().position(|v| *v.str == self.str)
    }
}

pub struct QuerySolution<S: SRDFBasic> {
    variables: Rc<Vec<VarName>>,
    values: Vec<Option<S::Term>>,
}

impl<S: SRDFBasic> QuerySolution<S> {
    pub fn find_solution(&self, index: impl VariableSolutionIndex<S>) -> Option<&S::Term> {
        match self.values.get(index.index(self)?) {
            Some(value) => value.as_ref(),
            None => None,
        }
    }
}

impl<S: SRDFBasic, V: Into<Rc<Vec<VarName>>>, T: Into<Vec<Option<S::Term>>>> From<(V, T)>
    for QuerySolution<S>
{
    #[inline]
    fn from((v, s): (V, T)) -> Self {
        Self {
            variables: v.into(),
            values: s.into(),
        }
    }
}

pub struct QuerySolutionIter<S: SRDFBasic> {
    iter: Box<dyn Iterator<Item = Result<QuerySolution<S>, S::Err>>>,
}

impl<S: SRDFBasic> QuerySolutionIter<S> {
    pub(crate) fn new(
        variables: Rc<Vec<VarName>>,
        iter: impl Iterator<Item = Result<Vec<Option<S::Term>>, S::Err>> + 'static,
    ) -> Self {
        Self {
            iter: Box::new(
                iter.map(move |t| t.map(|values| (Rc::clone(&variables), values).into())),
            ),
        }
    }
}

impl<S: SRDFBasic> Iterator for QuerySolutionIter<S> {
    type Item = Result<QuerySolution<S>, S::Err>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
