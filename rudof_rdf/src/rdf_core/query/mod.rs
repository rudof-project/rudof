mod query_rdf;
mod query_result_format;
mod query_solution;
mod query_solutions;
#[cfg(feature = "sparql")]
mod sparql_query;
mod var_name;
mod variable_solution_index;

pub use query_rdf::QueryRDF;
pub use query_result_format::QueryResultFormat;
pub use query_solution::QuerySolution;
pub use query_solutions::QuerySolutions;
#[cfg(feature = "sparql")]
pub use sparql_query::SparqlQuery;
pub use var_name::VarName;
pub use variable_solution_index::VariableSolutionIndex;

/// Represents the output format for displaying query results in a tabular form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableFormat {
    #[default]
    Ascii,
    Csv,
    Markdown,
}

/// Represents options for displaying query results in a tabular form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableOptions {
    pub show_index: bool,
    pub style: Option<TableStyle>,
}

impl Default for TableOptions {
    fn default() -> Self {
        TableOptions {
            show_index: false,
            style: Some(TableStyle::default()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableStyle {
    #[default]
    ModernRounded,
    Ascii,
    Markdown,
}
