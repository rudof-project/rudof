use crate::{evidence::Evidence, pgs_error::PgsError};
use either::Either;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ResultAssociation {
    pub node_id: String,
    pub type_name: String,
    pub conforms: bool,
    pub details: Either<Vec<PgsError>, Vec<Evidence>>,
}

impl Display for ResultAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {} {}",
            self.node_id,
            self.type_name,
            self.conforms,
            show_details(&self.details)
        )
    }
}

fn show_details(details: &Either<Vec<PgsError>, Vec<Evidence>>) -> String {
    let details_str = match details {
        Either::Left(errors) => errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n   "),
        Either::Right(evidences) => evidences
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n   "),
    };
    format!(" Details:   {}", details_str)
}
