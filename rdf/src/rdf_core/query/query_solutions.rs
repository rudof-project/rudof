use crate::rdf_core::{RDFError, Rdf, query::QuerySolution, term::Object};
use prefixmap::{PrefixMap, PrefixMapError};
use serde::Serialize;
use std::fmt::Display;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};

/// Represents a collection of query solutions from a SPARQL SELECT query.
///
/// This type holds the complete result set from a SPARQL query execution,
/// including all solution rows and a prefix map for qualifying IRIs in output.
///
/// # Type Parameters
///
/// * `S` - The RDF graph type implementing [`Rdf`]
#[derive(Debug, Clone, Serialize)]
pub struct QuerySolutions<S: Rdf> {
    /// The collection of query solutions (result rows).
    solutions: Vec<QuerySolution<S>>,
    /// Prefix map for qualifying IRIs in output.
    prefixmap: PrefixMap,
}

impl<S: Rdf> QuerySolutions<S> {
    /// Creates a new query solutions collection with the given data.
    ///
    /// # Arguments
    ///
    /// * `solutions` - Vector of query solution rows
    /// * `prefixmap` - Prefix map for IRI qualification in output
    pub fn new(solutions: Vec<QuerySolution<S>>, prefixmap: PrefixMap) -> QuerySolutions<S> {
        QuerySolutions {
            solutions,
            prefixmap: prefixmap,
        }
    }

    /// Creates an empty query solutions collection.
    ///
    /// Returns a new instance with no solutions and an empty prefix map.
    pub fn empty() -> QuerySolutions<S> {
        QuerySolutions {
            solutions: Vec::new(),
            prefixmap: PrefixMap::new(),
        }
    }

    /// Returns a reference to the prefix map.
    ///
    /// The prefix map contains namespace bindings used for qualifying IRIs
    /// when displaying or formatting results.
    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    /// Extends this collection with additional solutions and prefix mappings.
    ///
    /// # Arguments
    ///
    /// * `solutions` - Additional solutions to append
    /// * `prefixmap` - Prefix map to merge (new prefixes or updated IRIs)
    ///
    /// # Errors
    ///
    /// Returns an error if the prefix maps have conflicting definitions for
    /// the same prefix (same prefix bound to different namespaces).
    pub fn extend(
        &mut self,
        solutions: Vec<QuerySolution<S>>,
        prefixmap: PrefixMap,
    ) -> Result<(), PrefixMapError> {
        self.solutions.extend(solutions);
        self.prefixmap.merge(prefixmap)?;
        Ok(())
    }

    /// Returns an iterator over the query solutions.
    pub fn iter(&self) -> impl Iterator<Item = &QuerySolution<S>> {
        self.solutions.iter()
    }

    /// Returns the number of solutions in this collection.
    pub fn count(&self) -> usize {
        self.solutions.len()
    }

    /// Writes the query solutions as a formatted ASCII table.
    ///
    /// Produces a human-readable table with modern rounded borders, row numbers,
    /// and qualified IRIs using the prefix map. The table includes:
    ///
    /// - Row numbers in the first column (1-indexed)
    /// - Variable names as column headers (with `?` prefix)
    /// - Qualified IRIs using registered prefixes
    /// - Blank nodes displayed as `_:identifier`
    /// - Literals shown with their datatype/language tags
    /// - RDF-star quoted triples in `<< s p o >>` syntax
    /// - Empty cells for unbound variables
    ///
    /// # Arguments
    ///
    /// * `writer` - Output destination (stdout, file, buffer, etc.)
    ///
    /// Returns an error if:
    /// - Writing to the output fails
    /// - Term conversion fails when processing solutions
    pub fn write_table(&self, writer: &mut dyn Write) -> Result<(), RDFError> {
        if self.solutions.is_empty() {
            return write!(writer, "No results").map_err(|e| RDFError::WritingTableError {
                error: format!("{e}"),
            });
        }

        let first = &self.solutions[0];
        let mut builder = Builder::default();

        // Build header row with pre-allocated capacity
        let variable_count = first.variables_iter().count();
        let mut variables = Vec::with_capacity(variable_count + 1);
        variables.push(String::new()); // First column = index
        variables.extend(first.variables_iter().map(|v| v.to_string()));
        builder.push_record(variables);

        // Build data rows
        for (idx, result) in self.solutions.iter().enumerate() {
            let mut record = Vec::with_capacity(variable_count + 1);
            record.push((idx + 1).to_string()); // First column = index

            for (var_idx, _variable) in result.variables_iter().enumerate() {
                let str = match result.find_solution(var_idx) {
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

        Ok(())
    }
}

impl<S: Rdf + serde::Serialize> QuerySolutions<S> {
    /// Serializes the query solutions as pretty-printed JSON.
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|_| "[]".to_string())
    }
}

impl<S: Rdf> Display for QuerySolutions<S> {
    /// Formats the query solutions as plain text, one solution per line.
    ///
    /// Each solution is displayed using its `show()` method, which presents
    /// variable bindings in the format `?variable -> value`. Solutions are
    /// separated by blank lines for readability.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for solution in &self.solutions {
            writeln!(f, "{}", solution.show())?;
        }
        Ok(())
    }
}

impl<S: Rdf> IntoIterator for QuerySolutions<S> {
    /// The type of items yielded by the iterator.
    type Item = QuerySolution<S>;
    /// The iterator type for consuming iteration.
    type IntoIter = std::vec::IntoIter<QuerySolution<S>>;

    /// Converts the query solutions into an iterator, consuming the collection.
    ///
    /// This allows using `QuerySolutions` directly in for loops and other
    /// iterator contexts. The collection is consumed, yielding owned
    /// `QuerySolution` instances.
    fn into_iter(self) -> Self::IntoIter {
        self.solutions.into_iter()
    }
}
