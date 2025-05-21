use super::validation_report_error::{ReportError, ResultError};
use crate::helpers::srdf::get_object_for;
use shacl_ast::*;
use srdf::{Object, Query, RDFNode, SRDFBuilder};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    focus_node: RDFNode,           // required
    path: Option<RDFNode>,         // optional
    value: Option<RDFNode>,        // optional
    source: Option<RDFNode>,       // optional
    constraint_component: RDFNode, // required
    details: Option<Vec<RDFNode>>, // optional
    message: Option<RDFNode>,      // optional
    severity: RDFNode,             // required (TODO: Replace by Severity?)
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

    pub fn value(&self) -> Option<&Object> {
        self.value.as_ref()
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
    pub(crate) fn parse<S: Query>(
        store: &S,
        validation_result: &S::Term,
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

    pub fn to_rdf<RDF>(
        &self,
        rdf_writer: &mut RDF,
        report_node: RDF::Subject,
    ) -> Result<(), ReportError>
    where
        RDF: SRDFBuilder + Sized,
    {
        rdf_writer
            .add_type(report_node.clone(), SH_VALIDATION_RESULT.clone())
            .map_err(|e| ReportError::ValidationReportError { msg: e.to_string() })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                SH_FOCUS_NODE.clone(),
                self.focus_node.clone(),
            )
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding focus node to validation result: {e}"),
            })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                SH_SOURCE_CONSTRAINT_COMPONENT.clone(),
                self.constraint_component.clone(),
            )
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding source constraint component to validation result: {e}"),
            })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                SH_RESULT_SEVERITY.clone(),
                self.severity.clone(),
            )
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding severity to validation result: {e}"),
            })?;
        let message = match self.message {
            Some(ref message) => message.clone(),
            None => Object::str("No message"),
        };
        rdf_writer
            .add_triple(report_node.clone(), SH_RESULT_MESSAGE.clone(), message)
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error result message to validation result: {e}"),
            })?;
        if let Some(source) = &self.source {
            let source_term: RDF::Term = source.clone().into();
            rdf_writer
                .add_triple(report_node.clone(), SH_SOURCE_SHAPE.clone(), source_term)
                .map_err(|e| ReportError::ValidationReportError {
                    msg: format!("Error adding source to validation result: {e}"),
                })?;
        }

        Ok(())
    }
}
