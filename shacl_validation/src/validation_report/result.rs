use std::fmt::Debug;

use shacl_ast::*;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::helper::srdf::get_object_for;

use super::validation_report_error::ResultError;

pub struct ValidationResult<S: SRDFBasic> {
    focus_node: S::Term,                   // required
    path: Option<S::Term>,                 // optional
    value: Option<S::Term>,                // optional
    source: Option<S::Term>,               // optional
    constraint_component: Option<S::Term>, // TODO: this is required
    details: Option<Vec<S::Term>>,         // optional
    message: Option<S::Term>,              // optional
    severity: Option<S::Term>,             // TODO: this is required
}

impl<S: SRDFBasic> ValidationResult<S> {
    pub fn new(focus_node: S::Term) -> Self {
        Self {
            focus_node,
            path: Default::default(),
            value: Default::default(),
            source: Default::default(),
            constraint_component: Default::default(),
            details: Default::default(),
            message: Default::default(),
            severity: Default::default(),
        }
    }
}

impl<S: SRDFBasic> Debug for ValidationResult<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationResult")
            .field("focus_node", &self.focus_node)
            .field("path", &self.path)
            .field("value", &self.value)
            .field("source", &self.source)
            .field("constraint_component", &self.constraint_component)
            .field("details", &self.details)
            .field("message", &self.message)
            .field("severity", &self.severity)
            .finish()
    }
}

impl<S: SRDF> ValidationResult<S> {
    pub(crate) fn parse(store: &S, validation_result: &S::Term) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node =
            match get_object_for(store, validation_result, &S::iri_s2iri(&SH_FOCUS_NODE))? {
                Some(focus_node) => focus_node,
                None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
            };
        let severity =
            match get_object_for(store, validation_result, &S::iri_s2iri(&SH_RESULT_SEVERITY))? {
                Some(severity) => severity,
                None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
            };
        let constraint_component = match get_object_for(
            store,
            validation_result,
            &S::iri_s2iri(&SH_SOURCE_CONSTRAINT_COMPONENT),
        )? {
            Some(constraint_component) => constraint_component,
            None => {
                return Err(ResultError::MissingRequiredField(
                    "SourceConstraintComponent".to_owned(),
                ))
            }
        };

        // 2. Second, we must process the optional fields
        let path = get_object_for(store, validation_result, &S::iri_s2iri(&SH_RESULT_PATH))?;
        let source = get_object_for(store, validation_result, &S::iri_s2iri(&SH_SOURCE_SHAPE))?;
        let value = get_object_for(store, validation_result, &S::iri_s2iri(&SH_VALUE))?;

        // 3. Lastly we build the ValidationResult
        Ok(ValidationResult {
            focus_node,
            path,
            value,
            source,
            constraint_component: Some(constraint_component),
            details: None,
            message: None,
            severity: Some(severity),
        })
    }
}
