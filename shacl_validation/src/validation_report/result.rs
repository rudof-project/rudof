use std::fmt::Debug;

use shacl_ast::vocab::*;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;
use srdf::model::Iri as _;

use crate::helpers::srdf::get_object_for;

use super::validation_report_error::ResultError;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult<R: Rdf> {
    focus_node: TObject<R>,           // required
    path: Option<TObject<R>>,         // optional
    value: Option<TObject<R>>,        // optional
    source: Option<TObject<R>>,       // optional
    constraint_component: TObject<R>, // required
    details: Option<Vec<TObject<R>>>, // optional
    message: Option<TObject<R>>,      // optional
    severity: TObject<R>,             // required (TODO: Replace by Severity?)
}

impl<R: Rdf> ValidationResult<R> {
    // Creates a new validation result
    pub fn new(
        focus_node: TObject<R>,
        constraint_component: TObject<R>,
        severity: TObject<R>,
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

    pub fn with_path(mut self, path: Option<TObject<R>>) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: Option<TObject<R>>) -> Self {
        self.value = value;
        self
    }

    pub fn with_source(mut self, source: Option<TObject<R>>) -> Self {
        self.source = source;
        self
    }

    pub fn with_details(mut self, details: Option<Vec<TObject<R>>>) -> Self {
        self.details = details;
        self
    }

    pub fn with_message(mut self, message: Option<TObject<R>>) -> Self {
        self.message = message;
        self
    }

    pub fn focus_node(&self) -> &TObject<R> {
        &self.focus_node
    }

    pub fn component(&self) -> &TObject<R> {
        &self.constraint_component
    }

    pub fn severity(&self) -> &TObject<R> {
        &self.severity
    }

    pub(crate) fn parse(store: &R, validation_result: &TObject<R>) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node = match get_object_for(
            store,
            validation_result,
            &TPredicate::<R>::new(SH_FOCUS_NODE.as_str()),
        )? {
            Some(focus_node) => focus_node,
            None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
        };

        let severity = match get_object_for(
            store,
            validation_result,
            &TPredicate::<R>::new(SH_RESULT_SEVERITY.as_str()),
        )? {
            Some(severity) => severity,
            None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
        };

        let constraint_component = match get_object_for(
            store,
            validation_result,
            &TPredicate::<R>::new(SH_SOURCE_CONSTRAINT_COMPONENT.as_str()),
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
            &TPredicate::<R>::new(SH_RESULT_PATH.as_str()),
        )?;
        let source = get_object_for(
            store,
            validation_result,
            &TPredicate::<R>::new(SH_SOURCE_SHAPE.as_str()),
        )?;
        let value = get_object_for(
            store,
            validation_result,
            &TPredicate::<R>::new(SH_VALUE.as_str()),
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
