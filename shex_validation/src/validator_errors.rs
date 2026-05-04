use crate::ValidatorError;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize)]
pub struct ValidatorErrors {
    errs: Vec<ValidatorError>,
}

impl ValidatorErrors {
    pub fn new(errs: Vec<ValidatorError>) -> ValidatorErrors {
        ValidatorErrors { errs }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ValidatorError> {
        self.errs.iter()
    }
}

impl Display for ValidatorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in self.errs.iter() {
            writeln!(f, "  {err}")?;
        }
        Ok(())
    }
}
