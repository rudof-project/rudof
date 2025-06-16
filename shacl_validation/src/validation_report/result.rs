use super::validation_report_error::{ReportError, ResultError};
use crate::helpers::srdf::*;
use shacl_ast::shacl_vocab::{sh_value, sh_result_path, sh_source_shape, sh_source_constraint_component, sh_focus_node, sh_result_severity, sh_result_message, sh_validation_result};
use srdf::{Object, NeighsRDF, RDFNode, SHACLPath, BuildRDF};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    focus_node: RDFNode,           // required
    path: Option<SHACLPath>,       // optional
    value: Option<RDFNode>,        // optional
    source: Option<RDFNode>,       // optional
    constraint_component: RDFNode, // required
    details: Option<Vec<RDFNode>>, // optional
    message: Option<String>,       // optional
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

    pub fn with_path(mut self, path: Option<SHACLPath>) -> Self {
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

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn source(&self) -> Option<&Object> {
        self.source.as_ref()
    }

    pub fn value(&self) -> Option<&Object> {
        self.value.as_ref()
    }

    pub fn path(&self) -> Option<&SHACLPath> {
        self.path.as_ref()
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
    pub(crate) fn parse<S: NeighsRDF>(
        store: &S,
        validation_result: &S::Term,
    ) -> Result<Self, ResultError> {
        // 1. First, we must start processing the required fields. In case some
        //    don't appear, an error message must be raised
        let focus_node =
            match get_object_for(store, validation_result, &sh_focus_node().clone().into())? {
                Some(focus_node) => focus_node,
                None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
            };
        let severity =
            match get_object_for(store, validation_result, &sh_result_severity().clone().into())? {
                Some(severity) => severity,
                None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
            };
        let constraint_component = match get_object_for(
            store,
            validation_result,
            &sh_source_constraint_component().clone().into(),
        )? {
            Some(constraint_component) => constraint_component,
            None => {
                return Err(ResultError::MissingRequiredField(
                    "SourceConstraintComponent".to_owned(),
                ))
            }
        };

        // 2. Second, we must process the optional fields
        let sh_result_path_iri: S::IRI = sh_result_path().clone().into();
        let path = get_path_for(store, validation_result, &sh_result_path_iri)?;

        let sh_source_shape_iri: S::IRI = sh_source_shape().clone().into();
        let source = get_object_for(store, validation_result, &sh_source_shape_iri)?;
        let sh_value_iri: S::IRI = sh_value().clone().into(); 
        let value = get_object_for(store, validation_result, &sh_value_iri)?;

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
        RDF: BuildRDF + Sized,
    {
        rdf_writer
            .add_type(report_node.clone(), sh_validation_result().clone())
            .map_err(|e| ReportError::ValidationReportError { msg: e.to_string() })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                sh_focus_node().clone(),
                self.focus_node.clone(),
            )
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding focus node to validation result: {e}"),
            })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                sh_source_constraint_component().clone(),
                self.constraint_component.clone(),
            )
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding source constraint component to validation result: {e}"),
            })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                sh_result_severity().clone(),
                self.severity.clone(),
            )
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding severity to validation result: {e}"),
            })?;
        let message = match self.message {
            Some(ref message) => Object::str(message),
            None => Object::str("No message"),
        };
        rdf_writer
            .add_triple(report_node.clone(), sh_result_message().clone(), message)
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error result message to validation result: {e}"),
            })?;
        if let Some(source) = &self.source {
            let source_term: RDF::Term = source.clone().into();
            rdf_writer
                .add_triple(report_node.clone(), sh_source_shape().clone(), source_term)
                .map_err(|e| ReportError::ValidationReportError {
                    msg: format!("Error adding source to validation result: {e}"),
                })?;
        }
        if let Some(path) = &self.path {
            let result_path: RDF::Term = path_to_rdf::<RDF>(path);
            rdf_writer
                .add_triple(report_node.clone(), sh_result_path().clone(), result_path)
                .map_err(|e| ReportError::ValidationReportError {
                    msg: format!("Error adding result path to validation result: {e}"),
                })?;
        }
        if let Some(value) = &self.value {
            let value_term: RDF::Term = value.clone().into();
            rdf_writer
                .add_triple(report_node.clone(), sh_value().clone(), value_term)
                .map_err(|e| ReportError::ValidationReportError {
                    msg: format!("Error adding value to validation result: {e}"),
                })?;
        }
        Ok(())
    }
}

fn path_to_rdf<RDF>(path: &SHACLPath) -> RDF::Term
where
    RDF: NeighsRDF,
{
    match path {
        SHACLPath::Predicate { pred } => pred.clone().into(),
        SHACLPath::Alternative { paths: _ } => todo!(),
        SHACLPath::Sequence { paths: _ } => todo!(),
        SHACLPath::Inverse { path: _ } => todo!(),
        SHACLPath::ZeroOrMore { path: _ } => todo!(),
        SHACLPath::OneOrMore { path: _ } => todo!(),
        SHACLPath::ZeroOrOne { path: _ } => todo!(),
    }
}
