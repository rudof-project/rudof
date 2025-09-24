use serde::Serialize;
use std::fmt::Display;

use crate::{QueryResultFormat, Rdf};

/// Represents RDF that supports SPARQL-like queries
pub trait QueryRDF: Rdf {
    /// SPARQL SELECT query
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err>
    where
        Self: Sized;

    /// SPARQL CONSTRUCT query
    fn query_construct(
        &self,
        query: &str,
        result_format: &QueryResultFormat,
    ) -> Result<String, Self::Err>
    where
        Self: Sized;

    /// SPARQL ASK query    
    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize)]
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
        solution.variables.iter().position(|v| v.str == self)
    }
}

impl<S: Rdf> VariableSolutionIndex<S> for &VarName {
    #[inline]
    fn index(self, solution: &QuerySolution<S>) -> Option<usize> {
        solution.variables.iter().position(|v| *v.str == self.str)
    }
}

/// Represents one query solution
#[derive(Debug, Clone)]
pub struct QuerySolution<S: Rdf> {
    variables: Vec<VarName>,
    values: Vec<Option<S::Term>>,
}

impl<S: Rdf> Serialize for QuerySolution<S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.variables.len()))?;
        for (i, var) in self.variables.iter().enumerate() {
            if let Some(value) = &self.values[i] {
                let str = format!("{}", value);
                map.serialize_entry(&var.str, &str)?;
            }
        }
        map.end()
    }
}

impl<S: Rdf> QuerySolution<S> {
    pub fn new(variables: Vec<VarName>, values: Vec<Option<S::Term>>) -> QuerySolution<S> {
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

    pub fn convert<T: Rdf, F>(&self, cnv_term: F) -> QuerySolution<T>
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

    pub fn show(&self) -> String {
        let mut result = String::new();
        for var in self.variables.iter() {
            let value = match self.find_solution(var) {
                None => "()".to_string(),
                Some(v) => format!("{v}"),
            };
            result.push_str(format!("{var} -> {value}\n").as_str())
        }
        result
    }
}

impl<S: Rdf, V: Into<Vec<VarName>>, T: Into<Vec<Option<S::Term>>>> From<(V, T)>
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
#[derive(Debug, Clone, Serialize)]
pub struct QuerySolutions<S: Rdf> {
    solutions: Vec<QuerySolution<S>>,
}

impl<S: Rdf> QuerySolutions<S> {
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

    pub fn count(&self) -> usize {
        self.solutions.len()
    }
}

impl<S: Rdf + serde::Serialize> QuerySolutions<S> {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|_| "[]".to_string())
    }
}

impl<S: Rdf> Display for QuerySolutions<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for solution in &self.solutions {
            write!(f, "{}\n", solution.show())?;
        }
        Ok(())
    }
}

impl<S: Rdf> IntoIterator for QuerySolutions<S> {
    type Item = QuerySolution<S>;
    type IntoIter = std::vec::IntoIter<QuerySolution<S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.solutions.into_iter()
    }
}
