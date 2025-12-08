use crate::{pgs_error::PgsError, result_association::ResultAssociation};
use colored::*;
use csv::Writer;
use serde::Serialize;
use std::{fmt::Display, io::Write};

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub associations: Vec<ResultAssociation>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            is_valid: true,
            associations: Vec::new(),
        }
    }

    pub fn add_association(&mut self, association: ResultAssociation) {
        if association.conforms {
            // self.is_valid = self.is_valid && true;
        } else {
            self.is_valid = false;
        }
        self.associations.push(association);
    }

    pub fn is_empty(&self) -> bool {
        self.associations.is_empty()
    }

    pub fn as_json<W: Write>(&self, writer: W) -> Result<(), PgsError> {
        serde_json::to_writer_pretty(writer, self).map_err(|e| PgsError::SerializationError {
            error: e.to_string(),
        })
    }

    pub fn as_csv<W: Write>(&self, writer: W, with_colors: bool) -> Result<(), PgsError> {
        let mut wtr = Writer::from_writer(writer);
        wtr.write_record(["node_id", "type_name", "conforms", "details"])
            .map_err(|e| PgsError::WritingCSVHeader {
                error: e.to_string(),
            })?;
        for assoc in &self.associations {
            let details = match &assoc.details {
                either::Either::Left(errors) => errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; "),
                either::Either::Right(evidences) => evidences
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; "),
            };
            let conforms_str = assoc.conforms.to_string();
            let colored_conforms = if with_colors {
                if assoc.conforms {
                    conforms_str.color(Color::Green)
                } else {
                    conforms_str.color(Color::Red)
                }
            } else {
                ColoredString::from(conforms_str)
            };
            wtr.write_record([
                &assoc.node_id,
                &assoc.type_name,
                &colored_conforms.to_string(),
                &details.to_string(),
            ])
            .map_err(|e| PgsError::WritingCSVRecord {
                error: e.to_string(),
            })?;
        }
        wtr.flush().map_err(|e| PgsError::FlushingCSVWriter {
            error: e.to_string(),
        })?;
        Ok(())
    }
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Result - valid?: {}: ", self.is_valid)?;
        for association in &self.associations {
            write!(f, "\n{}", association)?;
        }
        Ok(())
    }
}
