use std::fmt::Debug;

use shacl_ast::vocab::*;
use srdf::model::matcher::Matcher;
use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;
use srdf::model::Triple;

use super::validation_report_error::ResultError;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult<R: Rdf> {
    focus_node: Object<R>,           // required
    path: Option<Object<R>>,         // optional
    value: Option<Object<R>>,        // optional
    source: Option<Object<R>>,       // optional
    constraint_component: Object<R>, // required
    details: Option<Vec<Object<R>>>, // optional
    message: Option<Object<R>>,      // optional
    severity: Object<R>,             // required (TODO: Replace by the Severity struct?)
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

    pub(crate) fn parse(store: &R, result: Object<R>) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node = store
            .triples_matching(result, SH_FOCUS_NODE.into(), Matcher::Any)
            .map_err(|_| ResultError::Srdf)?
            .map(Triple::into_object)
            .next()
            .ok_or(ResultError::MissingRequiredField("FocusNode"))?;

        let severity = store
            .triples_matching(result, SH_RESULT_SEVERITY.into(), Matcher::Any)
            .map_err(|_| ResultError::Srdf)?
            .map(Triple::into_object)
            .next()
            .ok_or(ResultError::MissingRequiredField("Severity"))?;

        let constraint_component = store
            .triples_matching(result, SH_SOURCE_CONSTRAINT_COMPONENT.into(), Matcher::Any)
            .map_err(|_| ResultError::Srdf)?
            .map(Triple::into_object)
            .next()
            .ok_or(ResultError::MissingRequiredField("ConstraintComponent"))?;

        // 2. Second, we must process the optional fields
        let path = store
            .triples_matching(result, SH_RESULT_PATH.into(), Matcher::Any)
            .map_err(|_| ResultError::Srdf)?
            .map(Triple::into_object)
            .next();

        let source = store
            .triples_matching(result, SH_SOURCE_SHAPE.into(), Matcher::Any)
            .map_err(|_| ResultError::Srdf)?
            .map(Triple::into_object)
            .next();

        let value = store
            .triples_matching(result, SH_VALUE.into(), Matcher::Any)
            .map_err(|_| ResultError::Srdf)?
            .map(Triple::into_object)
            .next();

        // 3. Lastly we build the ValidationResult<R>
        Ok(
            ValidationResult::new(focus_node, constraint_component, severity)
                .with_path(path)
                .with_source(source)
                .with_value(value),
        )
    }
}
