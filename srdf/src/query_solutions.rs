use crate::Object;
use crate::QuerySolution;
use crate::RDFError;
use crate::Rdf;
use prefixmap::{PrefixMap, PrefixMapError};
use serde::Serialize;
use std::fmt::Display;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};

/// Represent a list of query solutions
#[derive(Debug, Clone, Serialize)]
pub struct QuerySolutions<S: Rdf> {
    solutions: Vec<QuerySolution<S>>,
    prefixmap: PrefixMap,
}

impl<S: Rdf> QuerySolutions<S> {
    pub fn empty() -> QuerySolutions<S> {
        QuerySolutions {
            solutions: Vec::new(),
            prefixmap: PrefixMap::new(),
        }
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn new(solutions: Vec<QuerySolution<S>>, prefixmap: PrefixMap) -> QuerySolutions<S> {
        QuerySolutions {
            solutions,
            prefixmap: prefixmap.clone(),
        }
    }

    pub fn extend(
        &mut self,
        solutions: Vec<QuerySolution<S>>,
        prefixmap: PrefixMap,
    ) -> Result<(), PrefixMapError> {
        self.solutions.extend(solutions);
        self.prefixmap.merge(prefixmap)?;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &QuerySolution<S>> {
        self.solutions.iter()
    }

    pub fn count(&self) -> usize {
        self.solutions.len()
    }

    pub fn write_table(&self, writer: &mut dyn Write) -> Result<(), RDFError> {
        let mut results_iter = self.iter().peekable();
        if let Some(first) = results_iter.peek() {
            let mut builder = Builder::default();
            let mut variables = Vec::new();
            variables.push("".to_string()); // First column = index
            variables.extend(
                first
                    .variables_iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>(),
            );
            builder.push_record(variables);
            for (idx, result) in results_iter.enumerate() {
                let mut record = Vec::new();
                record.push(format!("{}", idx + 1)); // First column = index
                for (idx, _variable) in result.variables_iter().enumerate() {
                    let str = match result.find_solution(idx) {
                        Some(term) => {
                            let object = S::term_as_object(term)?;
                            match object {
                                Object::Iri(iri) => self.prefixmap.qualify(&iri),
                                Object::BlankNode(blank_node) => format!("_:{}", blank_node),
                                Object::Literal(literal) => literal.to_string(),
                                Object::Triple {
                                    subject,
                                    predicate,
                                    object,
                                } => format!(
                                    "<<{} {} {}>>",
                                    subject.show_qualified(&self.prefixmap),
                                    self.prefixmap.qualify(&predicate),
                                    object.show_qualified(&self.prefixmap)
                                ),
                            }
                        }
                        None => String::new(),
                    };
                    record.push(str);
                }
                builder.push_record(record);
            }

            let mut table = builder.build();
            table.with(Style::modern_rounded());
            writeln!(writer, "{table}").map_err(|e| RDFError::WritingTableError {
                error: format!("{e}"),
            })?;
        } else {
            write!(writer, "No results").map_err(|e| RDFError::WritingTableError {
                error: format!("{e}"),
            })?;
        }
        Ok(())
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
            writeln!(f, "{}", solution.show())?;
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
