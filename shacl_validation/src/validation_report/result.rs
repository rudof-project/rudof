use std::fmt::Debug;

use shacl_ast::vocab::*;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObjectRef;
use srdf::model::rdf::TPredicateRef;
use srdf::model::Iri as _;

use crate::helpers::srdf::get_object_for;

use super::validation_report_error::ResultError;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult<R: Rdf> {
    focus_node: TObjectRef<R>,           // required
    path: Option<TObjectRef<R>>,         // optional
    value: Option<TObjectRef<R>>,        // optional
    source: Option<TObjectRef<R>>,       // optional
    constraint_component: TObjectRef<R>, // required
    details: Option<Vec<TObjectRef<R>>>, // optional
    message: Option<TObjectRef<R>>,      // optional
    severity: TObjectRef<R>,             // required (TODO: Replace by Severity?)
}

impl<R: Rdf> ValidationResult<R> {
    // Creates a new validation result
    pub fn new(
        focus_node: TObjectRef<R>,
        constraint_component: TObjectRef<R>,
        severity: TObjectRef<R>,
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

    pub fn with_path(mut self, path: Option<TObjectRef<R>>) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: Option<TObjectRef<R>>) -> Self {
        self.value = value;
        self
    }

    pub fn with_source(mut self, source: Option<TObjectRef<R>>) -> Self {
        self.source = source;
        self
    }

    pub fn with_details(mut self, details: Option<Vec<TObjectRef<R>>>) -> Self {
        self.details = details;
        self
    }

    pub fn with_message(mut self, message: Option<TObjectRef<R>>) -> Self {
        self.message = message;
        self
    }

    pub fn focus_node(&self) -> &TObjectRef<R> {
        &self.focus_node
    }

    pub fn component(&self) -> &TObjectRef<R> {
        &self.constraint_component
    }

    pub fn severity(&self) -> &TObjectRef<R> {
        &self.severity
    }

    pub(crate) fn parse(store: &R, validation_result: &TObjectRef<R>) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node = match get_object_for(
            store,
            validation_result,
            &TPredicateRef::<R>::new(SH_FOCUS_NODE.as_str()),
        )? {
            Some(focus_node) => focus_node,
            None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
        };

        let severity = match get_object_for(
            store,
            validation_result,
            &TPredicateRef::<R>::new(SH_RESULT_SEVERITY.as_str()),
        )? {
            Some(severity) => severity,
            None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
        };

        let constraint_component = match get_object_for(
            store,
            validation_result,
            &TPredicateRef::<R>::new(SH_SOURCE_CONSTRAINT_COMPONENT.as_str()),
        )? {
            Some(constraint_component) => constraint_component,
            None => {
                return Err(ResultError::MissingRequiredField(
                    "SourceConstraintComponent".to_owned(),
                ))
            }
        };

        // 2. Second, we must process the optional fields
        let path = get_object_for(
            store,
            validation_result,
            &TPredicateRef::<R>::new(SH_RESULT_PATH.as_str()),
        )?;
        let source = get_object_for(
            store,
            validation_result,
            &TPredicateRef::<R>::new(SH_SOURCE_SHAPE.as_str()),
        )?;
        let value = get_object_for(
            store,
            validation_result,
            &TPredicateRef::<R>::new(SH_VALUE.as_str()),
        )?;

        // 3. Lastly we build the ValidationResult<R>
        Ok(
            ValidationResult::new(focus_node, constraint_component, severity)
                .with_path(path)
                .with_source(source)
                .with_value(value),
        )
    }
}
