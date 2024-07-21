use iri_s::IriS;
use oxrdf::{Subject, Term};
use shacl_ast::SH_SOURCE_CONSTRAINT;
use shacl_ast::{
    SH_FOCUS_NODE, SH_RESULT_PATH, SH_RESULT_SEVERITY, SH_SOURCE_CONSTRAINT_COMPONENT,
    SH_SOURCE_SHAPE, SH_VALUE,
};
use srdf::{SRDFGraph, SRDF};

use super::validation_report_error::ValidationResultError;

pub struct ValidationResult {
    focus_node: Option<Term>,
    result_severity: Option<Term>,
    result_path: Option<Term>,
    source_constraint: Option<Term>,
    source_constraint_component: Option<Term>,
    source_shape: Option<Term>,
    value: Option<Term>,
}

impl ValidationResult {
    pub fn new(
        focus_node: Option<Term>,
        result_severity: Option<Term>,
        result_path: Option<Term>,
        source_constraint: Option<Term>,
        source_constraint_component: Option<Term>,
        source_shape: Option<Term>,
        value: Option<Term>,
    ) -> Self {
        ValidationResult {
            focus_node,
            result_severity,
            result_path,
            source_constraint,
            source_constraint_component,
            source_shape,
            value,
        }
    }

    fn get_object_for(
        graph: &SRDFGraph,
        subject: &Subject,
        predicate: &IriS,
    ) -> Result<Option<Term>, ValidationResultError> {
        match graph.objects_for_subject_predicate(subject, predicate.as_named_node()) {
            Ok(triples) => match triples.into_iter().nth(0) {
                Some(triple) => Ok(Some(triple)),
                None => Ok(None),
            },
            Err(_) => todo!(),
        }
    }

    pub fn parse(
        graph: &SRDFGraph,
        subject: &Subject,
    ) -> Result<ValidationResult, ValidationResultError> {
        let focus_node = match Self::get_object_for(graph, subject, &SH_FOCUS_NODE) {
            Ok(focus_node) => focus_node,
            Err(_) => todo!(),
        };
        let result_severity = match Self::get_object_for(graph, subject, &SH_RESULT_SEVERITY) {
            Ok(result_severity) => result_severity,
            Err(_) => todo!(),
        };
        let result_path = match Self::get_object_for(graph, subject, &SH_RESULT_PATH) {
            Ok(result_path) => result_path,
            Err(_) => todo!(),
        };
        let source_constraint = match Self::get_object_for(graph, subject, &SH_SOURCE_CONSTRAINT) {
            Ok(source_constraint) => source_constraint,
            Err(_) => todo!(),
        };
        let source_constraint_component =
            match Self::get_object_for(graph, subject, &SH_SOURCE_CONSTRAINT_COMPONENT) {
                Ok(source_constraint_component) => source_constraint_component,
                Err(_) => todo!(),
            };
        let source_shape = match Self::get_object_for(graph, subject, &SH_SOURCE_SHAPE) {
            Ok(source_shape) => source_shape,
            Err(_) => todo!(),
        };
        let value = match Self::get_object_for(graph, subject, &SH_VALUE) {
            Ok(value) => value,
            Err(_) => todo!(),
        };

        Ok(ValidationResult::new(
            focus_node,
            result_severity,
            result_path,
            source_constraint,
            source_constraint_component,
            source_shape,
            value,
        ))
    }

    pub fn focus_node(&self) -> Option<Term> {
        self.focus_node.to_owned()
    }

    pub fn result_severity(&self) -> Option<Term> {
        self.result_severity.to_owned()
    }

    pub fn result_path(&self) -> Option<Term> {
        self.result_path.to_owned()
    }

    pub fn source_constraint(&self) -> Option<Term> {
        self.source_constraint.to_owned()
    }

    pub fn source_constraint_component(&self) -> Option<Term> {
        self.source_constraint_component.to_owned()
    }

    pub fn source_shape(&self) -> Option<Term> {
        self.source_shape.to_owned()
    }

    pub fn value(&self) -> Option<Term> {
        self.value.to_owned()
    }
}
