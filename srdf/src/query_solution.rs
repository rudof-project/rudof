use crate::{Rdf, VarName, VariableSolutionIndex};
use serde::Serialize;

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
                let str = format!("{value}");
                map.serialize_entry(&var.as_str(), &str)?;
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

    pub fn variables_iter(&self) -> impl Iterator<Item = &VarName> {
        self.variables.iter()
    }

    pub fn variables(&self) -> &Vec<VarName> {
        &self.variables
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
