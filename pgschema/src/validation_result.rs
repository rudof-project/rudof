use std::fmt::Display;

use either::Either;

use crate::{evidence::Evidence, pgs_error::PgsError};

#[derive(Debug)]
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
            self.is_valid = self.is_valid && true;
        } else {
            self.is_valid = false;
        }
        self.associations.push(association);
    }

    pub fn is_empty(&self) -> bool {
        self.associations.is_empty()
    }
}

#[derive(Debug)]
pub struct ResultAssociation {
    pub node_id: String,
    pub type_name: String,
    pub conforms: bool,
    pub details: Either<Vec<PgsError>, Vec<Evidence>>,
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

impl Display for ResultAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {} {})",
            self.node_id,
            self.type_name,
            self.conforms,
            show_details(&self.details)
        )
    }
}

fn show_details(details: &Either<Vec<PgsError>, Vec<Evidence>>) -> String {
    let details_str = match details {
        Either::Left(errors) => errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n   "),
        Either::Right(evidences) => evidences
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n   "),
    };
    format!(" Details:   {}", details_str)
}
