use std::fmt::Debug;

use shacl_ast::*;
use srdf::{Object, Query, RDFNode};

use crate::helpers::srdf::get_object_for;

use super::validation_report_error::ResultError;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    focus_node: RDFNode,           // required
    path: Option<RDFNode>,         // optional
    value: Option<RDFNode>,        // optional
    source: Option<RDFNode>,       // optional
    constraint_component: RDFNode, // required
    details: Option<Vec<RDFNode>>, // optional
    message: Option<RDFNode>,      // optional
    severity: RDFNode,             // required
}

impl ValidationResult {
    // Creates a new validation result
    pub fn new(focus_node: Object, constraint_component: Object, severity: Object) -> Self {
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

    pub fn with_path(mut self, path: Option<Object>) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: Option<Object>) -> Self {
        self.value = value;
        self
    }

    pub fn with_source(mut self, source: Option<Object>) -> Self {
        self.source = source;
        self
    }

    pub fn with_details(mut self, details: Option<Vec<Object>>) -> Self {
        self.details = details;
        self
    }

    pub fn with_message(mut self, message: Option<Object>) -> Self {
        self.message = message;
        self
    }

    pub fn source(&self) -> Option<&Object> {
        self.source.as_ref()
    }

    pub fn focus_node(&self) -> &Object {
        &self.focus_node
    }

    pub fn component(&self) -> &Object {
        &self.constraint_component
    }

    pub fn severity(&self) -> &Object {
        &self.severity
    }
}

impl ValidationResult {
    pub(crate) fn parse<Q: Query>(
        store: &Q,
        validation_result: &Q::Term,
    ) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node =
            match get_object_for(store, validation_result, &SH_FOCUS_NODE.clone().into())? {
                Some(focus_node) => focus_node,
                None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
            };
        let severity =
            match get_object_for(store, validation_result, &SH_RESULT_SEVERITY.clone().into())? {
                Some(severity) => severity,
                None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
            };
        let constraint_component = match get_object_for(
            store,
            validation_result,
            &SH_SOURCE_CONSTRAINT_COMPONENT.clone().into(),
        )? {
            Some(constraint_component) => constraint_component,
            None => {
                return Err(ResultError::MissingRequiredField(
                    "SourceConstraintComponent".to_owned(),
                ))
            }
        };

        // 2. Second, we must process the optional fields
        let path = get_object_for(store, validation_result, &SH_RESULT_PATH.clone().into())?;
        let source = get_object_for(store, validation_result, &SH_SOURCE_SHAPE.clone().into())?;
        let value = get_object_for(store, validation_result, &SH_VALUE.clone().into())?;

        // 3. Lastly we build the ValidationResult
        Ok(
            ValidationResult::new(focus_node, constraint_component, severity)
                .with_path(path)
                .with_source(source)
                .with_value(value),
        )
    }
}

impl PartialEq for ValidationResult {
    fn eq(&self, other: &Self) -> bool {
        // we check for the equality of the required fields
        self.focus_node == other.focus_node
            && self.constraint_component == other.constraint_component
            && self.severity == other.severity
    }
}
