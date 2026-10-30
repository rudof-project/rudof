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

    /// See [`ValidatorError::without_nested_references`].
    pub(crate) fn without_nested_references(&self) -> ValidatorErrors {
        ValidatorErrors::new(self.errs.iter().map(|e| e.without_nested_references()).collect())
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
