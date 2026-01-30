use crate::rdf_core::{Rdf, query::{VarName, VariableSolutionIndex}};
use serde::Serialize;

/// Represents a single solution from a SPARQL SELECT query.
///
/// A query solution is analogous to a row in a SQL result set. It contains
/// bindings for SPARQL variables, mapping each variable to an RDF term (or
/// indicating the variable is unbound).
///
/// Each solution maintains:
/// - An ordered list of variable names (the "columns")
/// - A corresponding list of optional term values (a value of `None` indicates that the variable is unbound in this solution,
/// which can occur with OPTIONAL patterns or UNION queries).
///
/// # Type Parameters
///
/// * `S` - The RDF graph type implementing [`Rdf`]
#[derive(Debug, Clone)]
pub struct QuerySolution<S: Rdf> {
    /// The ordered list of variable names in this solution.
    variables: Vec<VarName>,
    /// The term values bound to each variable.
    values: Vec<Option<S::Term>>,
}

impl<S: Rdf> QuerySolution<S> {
    /// Creates a new query solution from variables and values.
    ///
    /// The variables and values vectors must have the same length, with each
    /// value corresponding to the variable at the same index.
    ///
    /// # Arguments
    ///
    /// * `variables` - The ordered list of variable names
    /// * `values` - The ordered list of optional term values
    pub fn new(variables: Vec<VarName>, values: Vec<Option<S::Term>>) -> QuerySolution<S> {
        QuerySolution { variables, values }
    }

    /// Finds and returns the term bound to a variable in this solution.
    ///
    /// This method accepts any type implementing [`VariableSolutionIndex`],
    /// allowing lookup by variable name, position, or custom index types.
    ///
    /// # Arguments
    ///
    /// * `index` - The variable index (name string, position, or VarName reference)
    pub fn find_solution(&self, index: impl VariableSolutionIndex<S>) -> Option<&S::Term> {
        match self.values.get(index.index(self)?) {
            Some(value) => value.as_ref(),
            None => None,
        }
    }

    /// Returns an iterator over the variable names in this solution.
    ///
    /// The iterator yields references to [`VarName`] instances in the order
    /// they appear in the solution (matching the SELECT clause order).
    pub fn variables_iter(&self) -> impl Iterator<Item = &VarName> {
        self.variables.iter()
    }

    /// Returns a reference to the vector of variable names.
    pub fn variables(&self) -> &Vec<VarName> {
        &self.variables
    }

    /// Converts this solution to use a different RDF type.
    ///
    /// This method transforms a solution from one RDF implementation to another
    /// by applying a conversion function to each bound term. The variable names
    /// are preserved, and unbound variables remain unbound.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target RDF type
    /// * `F` - The term conversion function type
    ///
    /// # Arguments
    ///
    /// * `cnv_term` - A function that converts terms from `S::Term` to `T::Term`
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

    /// Returns a human-readable string representation of this solution.
    ///
    /// The output shows each variable with its bound value (or "()" for unbound
    /// variables), one per line in the format: `?variable -> value`
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
    /// Constructs a query solution from a tuple of variables and values.
    ///
    /// This convenience implementation allows creating solutions using tuple
    /// syntax, automatically converting compatible types into the required
    /// vector types.
    ///
    /// # Arguments
    ///
    /// * Tuple of (variables, values) where both elements are convertible
    ///   to their respective vector types
    fn from((v, s): (V, T)) -> Self {
        Self {
            variables: v.into(),
            values: s.into(),
        }
    }
}

impl<S: Rdf> Serialize for QuerySolution<S> {
    /// Serializes the query solution as a map of variable names to term strings.
    ///
    /// Unbound variables (None values) are **omitted** from the serialized output
    /// rather than being represented as null. This matches common SPARQL JSON
    /// result format conventions.
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.variables.len()))?;
        for (var, value) in self.variables.iter().zip(self.values.iter()) {
            if let Some(term) = value {
                let str = format!("{term}");
                map.serialize_entry(&var.as_str(), &str)?;
            }
        }
        map.end()
    }
}
