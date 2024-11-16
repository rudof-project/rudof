use shacl_ast::vocab::*;
use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

use super::validation_report_error::ResultError;
use crate::helpers::srdf::get_object_for;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult<R: Rdf> {
    focus_node: Object<R>,           // required
    path: Option<Object<R>>,         // optional
    value: Option<Object<R>>,        // optional
    source: Option<Object<R>>,       // optional
    constraint_component: Object<R>, // required
    details: Option<Vec<Object<R>>>, // optional
    message: Option<Object<R>>,      // optional
    severity: Object<R>,             // required (TODO: Replace by Severity?)
}

impl<R: Rdf> ValidationResult<R> {
    // Creates a new validation result
    pub fn new(
        focus_node: Object<R>,
        constraint_component: Object<R>,
        severity: Object<R>,
    ) -> Self {
        Self {
            focus_node,
            path: None,
            value: None,
            source: None,
            constraint_component,
            details: None,
            message: None,
            severity,
        }
    }

    pub fn with_path(mut self, path: Option<Object<R>>) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: Option<Object<R>>) -> Self {
        self.value = value;
        self
    }

    pub fn with_source(mut self, source: Option<Object<R>>) -> Self {
        self.source = source;
        self
    }

    pub fn with_details(mut self, details: Option<Vec<Object<R>>>) -> Self {
        self.details = details;
        self
    }

    pub fn with_message(mut self, message: Option<Object<R>>) -> Self {
        self.message = message;
        self
    }

    pub fn focus_node(&self) -> &Object<R> {
        &self.focus_node
    }

    pub fn component(&self) -> &Object<R> {
        &self.constraint_component
    }

    pub fn severity(&self) -> &Object<R> {
        &self.severity
    }

    pub(crate) fn parse(store: &R, validation_result: &Object<R>) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node = match get_object_for(store, validation_result, &SH_FOCUS_NODE)? {
            Some(focus_node) => focus_node,
            None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
        };
        let severity = match get_object_for(store, validation_result, &SH_RESULT_SEVERITY)? {
            Some(severity) => severity,
            None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
        };
        let constraint_component =
            match get_object_for(store, validation_result, &SH_SOURCE_CONSTRAINT_COMPONENT)? {
                Some(constraint_component) => constraint_component,
                None => {
                    return Err(ResultError::MissingRequiredField(
                        "SourceConstraintComponent".to_owned(),
                    ))
                }
            };

        // 2. Second, we must process the optional fields
        let path = get_object_for(store, validation_result, &SH_RESULT_PATH)?;
        let source = get_object_for(store, validation_result, &SH_SOURCE_SHAPE)?;
        let value = get_object_for(store, validation_result, &SH_VALUE)?;

        // 3. Lastly we build the ValidationResult<R>
        Ok(
            ValidationResult::new(focus_node, constraint_component, severity)
                .with_path(path)
                .with_source(source)
                .with_value(value),
        )
    }
}
