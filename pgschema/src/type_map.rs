use std::fmt::Display;

use either::Either::{Left, Right};

use crate::{
    evidence::Evidence,
    // formal_graph_type::FormalGraphType,
    pg::PropertyGraph,
    pgs::PropertyGraphSchema,
    pgs_error::PgsError,
    validation_result::{ResultAssociation, ValidationResult},
};

/// Defines associations between node IDs and type names that can be used to trigger validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeMap {
    // TODO: Improve the performance of this representation using a HashMap
    associations: Vec<Association>,
}

impl Default for TypeMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeMap {
    pub fn new() -> Self {
        TypeMap {
            associations: Vec::new(),
        }
    }

    pub fn find_association(&self, node_id: &str, type_name: &str) -> Option<&Association> {
        self.associations
            .iter()
            .find(|ass| ass.node_id() == node_id && ass.type_name() == type_name)
    }

    pub fn add_association(&mut self, association: Association) {
        self.associations.push(association);
    }

    /* TODO: It gives an error because MapBuilder is not in scope
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, PgsError> {
        let content =
            std::fs::read_to_string(&path).map_err(|e| PgsError::TypeMapFileReadError {
                path: path.as_ref().to_str().unwrap().to_string(),
                error: e.to_string(),
            })?;
        MapBuilder::new().parse_map(content.as_str())
    }*/

    pub fn validate(
        &self,
        schema: &PropertyGraphSchema,
        graph: &PropertyGraph,
    ) -> Result<ValidationResult, PgsError> {
        let mut result = ValidationResult::new();
        for association in &self.associations {
            let node_id = association.node_id();
            let type_name = association.type_name();
            let either_node_edge = graph.get_node_edge_by_label(node_id).map_err(|_| {
                PgsError::MissingNodeEdgeLabel {
                    label: node_id.to_string(),
                }
            })?;
            let conforms_result = match either_node_edge {
                Left(node) => schema.conforms_node(type_name, node),
                Right(edge) => schema.conforms_edge(type_name, edge),
            };
            // TODO: Handle when should_conform is false
            result.add_association(ResultAssociation {
                node_id: node_id.clone(),
                type_name: type_name.clone(),
                conforms: conforms_result.is_right(),
                details: conforms_result,
            });
        }
        Ok(result) // Assuming validation passes for now
    }

    pub fn compare_with_result(
        &self,
        result: &ValidationResult,
    ) -> Result<Vec<FailedAssociation>, PgsError> {
        let mut failed_associations = Vec::new();
        for result_association in &result.associations {
            if let Some(expected_association) =
                self.find_association(&result_association.node_id, &result_association.type_name)
            {
                match (
                    expected_association.should_conform,
                    &result_association.details,
                ) {
                    (true, Right(_)) => continue,
                    (true, Left(errors)) => {
                        failed_associations.push(FailedAssociation {
                            node_id: result_association.node_id.clone(),
                            type_name: result_association.type_name.clone(),
                            status: FailedAssociationStatus::FailedResult_ShouldConform {
                                errors: errors.clone(),
                            },
                        });
                    }
                    (false, Right(evidences)) => {
                        failed_associations.push(FailedAssociation {
                            node_id: result_association.node_id.clone(),
                            type_name: result_association.type_name.clone(),
                            status: FailedAssociationStatus::PassedResult_ShouldNotConform {
                                evidences: evidences.clone(),
                            },
                        });
                    }
                    (false, Left(_)) => continue,
                }
            } else {
                return Err(PgsError::MissingAssociation {
                    node: result_association.node_id.clone(),
                    type_name: result_association.type_name.clone(),
                });
            }
        }
        Ok(failed_associations)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FailedAssociation {
    node_id: String,
    type_name: String,
    status: FailedAssociationStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FailedAssociationStatus {
    FailedResult_ShouldConform { errors: Vec<PgsError> },
    PassedResult_ShouldNotConform { evidences: Vec<Evidence> },
}

impl Display for FailedAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.status {
            FailedAssociationStatus::FailedResult_ShouldConform { errors } => write!(
                f,
                "{}:{} should conform, but result failed: Errors: {}",
                self.node_id,
                self.type_name,
                show_errors(errors)
            ),
            FailedAssociationStatus::PassedResult_ShouldNotConform { evidences } => write!(
                f,
                "{}:{} should fail but result passed with evidences: {:?}",
                self.node_id,
                self.type_name,
                show_evidences(evidences)
            ),
        }
    }
}

fn show_evidences(evidences: &[Evidence]) -> String {
    evidences
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("\n ")
}

fn show_errors(errors: &[PgsError]) -> String {
    errors
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("\n ")
}

impl Display for TypeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ass in &self.associations {
            writeln!(f, "  {}", ass)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Association {
    node_id: String,
    type_name: String,
    should_conform: bool,
}

impl Association {
    pub fn new(node_id: String, type_name: String) -> Self {
        Association {
            node_id,
            type_name,
            should_conform: true,
        }
    }

    pub fn with_no_conform(mut self) -> Self {
        self.should_conform = false;
        self
    }

    pub fn node_id(&self) -> &String {
        &self.node_id
    }

    pub fn type_name(&self) -> &String {
        &self.type_name
    }
}

impl Display for Association {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{},", self.node_id, self.type_name)
    }
}
