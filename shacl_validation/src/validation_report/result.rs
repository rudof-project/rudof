use std::fmt::Debug;

use shacl_ast::*;
use srdf::{matcher::Any, Object, Query, RDFNode, Triple};

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
        // 0. First, we must start by converting the validation result to a subject
        let result: Q::Subject = validation_result
            .clone()
            .try_into()
            .map_err(|_| ResultError::ExpectedSubject)?;

        // 1. Second, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node = store
            .triples_matching(result.clone(), SH_FOCUS_NODE.clone(), Any)
            .map_err(|_| ResultError::Query)?
            .next()
            .ok_or(ResultError::MissingField("FocusNode"))?
            .into_object()
            .into(); // convert to Object

        let severity = store
            .triples_matching(result.clone(), SH_RESULT_SEVERITY.clone(), Any)
            .map_err(|_| ResultError::Query)?
            .next()
            .ok_or(ResultError::MissingField("Severity"))?
            .into_object()
            .into(); // convert to Object

        let constraint_component = store
            .triples_matching(result.clone(), SH_SOURCE_CONSTRAINT_COMPONENT.clone(), Any)
            .map_err(|_| ResultError::Query)?
            .next()
            .ok_or(ResultError::MissingField("SourceConstraintComponent"))?
            .into_object()
            .into(); // convert to Object

        // 2. Third, we must process the optional fields
        let path = store
            .triples_matching(result.clone(), SH_RESULT_PATH.clone(), Any)
            .map_err(|_| ResultError::Query)?
            .next()
            .map(Triple::into_object)
            .map(Into::into); // convert to Object

        let source = store
            .triples_matching(result.clone(), SH_SOURCE_SHAPE.clone(), Any)
            .map_err(|_| ResultError::Query)?
            .next()
            .map(Triple::into_object)
            .map(Into::into); // convert to Object

        let value = store
            .triples_matching(result, SH_VALUE.clone(), Any)
            .map_err(|_| ResultError::Query)?
            .next()
            .map(Triple::into_object)
            .map(Into::into); // convert to Object

        // 3. Lastly we build the ValidationResult
        let validation_result = ValidationResult::new(focus_node, constraint_component, severity)
            .with_path(path)
            .with_source(source)
            .with_value(value);

        Ok(validation_result)
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
