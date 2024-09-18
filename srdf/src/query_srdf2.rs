use std::{collections::HashMap, rc::Rc};

use crate::SRDFBasic;

/// Alternative QuerySRDF trait
pub trait QuerySRDF2: SRDFBasic {
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err>
    where
        Self: Sized;

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct VarName2 {
    str: String,
}

impl From<String> for VarName2 {
    fn from(value: String) -> Self {
        VarName2 { str: value }
    }
}

pub trait VariableSolutionIndex<S: SRDFBasic> {
    fn index(self, solution: &QuerySolution2<S>) -> Option<usize>;
}

impl<S: SRDFBasic> VariableSolutionIndex<S> for usize {
    #[inline]
    fn index(self, _: &QuerySolution2<S>) -> Option<usize> {
        Some(self)
    }
}

impl<S: SRDFBasic> VariableSolutionIndex<S> for &str {
    #[inline]
    fn index(self, solution: &QuerySolution2<S>) -> Option<usize> {
        solution.variables.iter().position(|v| v.str == self)
    }
}

impl<S: SRDFBasic> VariableSolutionIndex<S> for &VarName2 {
    #[inline]
    fn index(self, solution: &QuerySolution2<S>) -> Option<usize> {
        solution.variables.iter().position(|v| *v.str == self.str)
    }
}

pub struct QuerySolution2<S: SRDFBasic> {
    variables: Rc<Vec<VarName2>>,
    values: Vec<Option<S::Term>>,
}

impl<S: SRDFBasic> QuerySolution2<S> {
    pub fn new(variables: Rc<Vec<VarName2>>, values: Vec<Option<S::Term>>) -> QuerySolution2<S> {
        QuerySolution2 { variables, values }
    }
    pub fn find_solution(&self, index: impl VariableSolutionIndex<S>) -> Option<&S::Term> {
        match self.values.get(index.index(self)?) {
            Some(value) => value.as_ref(),
            None => None,
        }
    }
    pub fn convert<T: SRDFBasic, F>(&self, cnv_term: F) -> QuerySolution2<T>
    where
        F: Fn(&S::Term) -> T::Term,
    {
        let cnv_values: Vec<Option<T::Term>> = self
            .values
            .iter()
            .map(|s| match s {
                None => None,
                Some(s) => Some(cnv_term(s)),
            })
            .collect();
        QuerySolution2 {
            variables: self.variables.clone(),
            values: cnv_values,
        }
    }
}

impl<S: SRDFBasic, V: Into<Rc<Vec<VarName2>>>, T: Into<Vec<Option<S::Term>>>> From<(V, T)>
    for QuerySolution2<S>
{
    #[inline]
    fn from((v, s): (V, T)) -> Self {
        Self {
            variables: v.into(),
            values: s.into(),
        }
    }
}

pub struct QuerySolutions<S: SRDFBasic> {
    solutions: Vec<QuerySolution2<S>>,
}

impl<S: SRDFBasic> QuerySolutions<S> {
    pub(crate) fn empty() -> QuerySolutions<S> {
        QuerySolutions {
            solutions: Vec::new(),
        }
    }

    pub(crate) fn new(solutions: Vec<QuerySolution2<S>>) -> QuerySolutions<S> {
        QuerySolutions { solutions }
    }

    pub fn extend(&mut self, solutions: Vec<QuerySolution2<S>>) {
        self.solutions.extend(solutions)
    }

    pub fn iter(&self) -> impl Iterator<Item = &QuerySolution2<S>> {
        self.solutions.iter()
    }
}
