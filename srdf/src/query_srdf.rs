use std::{fmt::Display, rc::Rc};

use crate::SRDFBasic;

pub trait QuerySRDF: SRDFBasic {
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err>
    where
        Self: Sized;

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct VarName {
    str: String,
}

impl Display for VarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.str)
    }
}

impl VarName {
    pub fn new(str: &str) -> VarName {
        VarName {
            str: str.to_string(),
        }
    }
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

/// Represents one query solution
pub struct QuerySolution<S: SRDFBasic> {
    variables: Rc<Vec<VarName>>,
    values: Vec<Option<S::Term>>,
}

impl<S: SRDFBasic> QuerySolution<S> {
    pub fn new(variables: Rc<Vec<VarName>>, values: Vec<Option<S::Term>>) -> QuerySolution<S> {
        QuerySolution { variables, values }
    }
    pub fn find_solution(&self, index: impl VariableSolutionIndex<S>) -> Option<&S::Term> {
        match self.values.get(index.index(self)?) {
            Some(value) => value.as_ref(),
            None => None,
        }
    }

    pub fn variables(&self) -> impl Iterator<Item = &VarName> {
        self.variables.iter()
    }

    pub fn convert<T: SRDFBasic, F>(&self, cnv_term: F) -> QuerySolution<T>
    where
        F: Fn(&S::Term) -> T::Term,
    {
        let cnv_values: Vec<Option<T::Term>> = self
            .values
            .iter()
            .map(|s| s.as_ref().map(&cnv_term))
            .collect();
        QuerySolution {
            variables: self.variables.clone(),
            values: cnv_values,
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

/// Represent a list of query solutions
pub struct QuerySolutions<S: SRDFBasic> {
    solutions: Vec<QuerySolution<S>>,
}

impl<S: SRDFBasic> QuerySolutions<S> {
    pub fn empty() -> QuerySolutions<S> {
        QuerySolutions {
            solutions: Vec::new(),
        }
    }

    pub fn new(solutions: Vec<QuerySolution<S>>) -> QuerySolutions<S> {
        QuerySolutions { solutions }
    }

    pub fn extend(&mut self, solutions: Vec<QuerySolution<S>>) {
        self.solutions.extend(solutions)
    }

    pub fn iter(&self) -> impl Iterator<Item = &QuerySolution<S>> {
        self.solutions.iter()
    }
}
