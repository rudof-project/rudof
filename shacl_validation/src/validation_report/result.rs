use shacl_ast::*;
use srdf::{SRDFBasic, SRDF};

use crate::helper::srdf::get_object_for;

use super::validation_report_error::ResultError;

pub struct ValidationResultBuilder<S: SRDFBasic> {
    focus_node: Option<S::Term>,
    result_severity: Option<S::Term>,
    result_path: Option<S::Term>,
    source_constraint_component: Option<S::Term>,
    source_shape: Option<S::Term>,
    value: Option<S::Term>,
}

impl<S: SRDFBasic> ValidationResultBuilder<S> {
    pub fn focus_node(&mut self, focus_node: S::Term) {
        self.focus_node = Some(focus_node);
    }

    pub fn result_severity(&mut self, result_severity: S::Term) {
        self.result_severity = Some(result_severity);
    }

    pub fn result_path(&mut self, result_path: S::Term) {
        self.result_path = Some(result_path);
    }

    pub fn source_constraint_component(&mut self, source_constraint_component: S::Term) {
        self.source_constraint_component = Some(source_constraint_component);
    }

    pub fn source_shape(&mut self, source_shape: S::Term) {
        self.source_shape = Some(source_shape);
    }

    pub fn value(&mut self, value: S::Term) {
        self.value = Some(value);
    }

    pub fn build(self) -> ValidationResult<S> {
        ValidationResult::new(
            self.focus_node,
            self.result_severity,
            self.result_path,
            self.source_constraint_component,
            self.source_shape,
            self.value,
        )
    }
}

impl<S: SRDFBasic> Default for ValidationResultBuilder<S> {
    fn default() -> Self {
        ValidationResultBuilder {
            focus_node: None,
            result_severity: None,
            result_path: None,
            source_constraint_component: None,
            source_shape: None,
            value: None,
        }
    }
}

pub struct ValidationResult<S: SRDFBasic> {
    focus_node: Option<S::Term>,
    result_severity: Option<S::Term>,
    result_path: Option<S::Term>,
    source_constraint_component: Option<S::Term>,
    source_shape: Option<S::Term>,
    value: Option<S::Term>,
}

impl<S: SRDFBasic> ValidationResult<S> {
    pub(crate) fn new(
        focus_node: Option<S::Term>,
        result_severity: Option<S::Term>,
        result_path: Option<S::Term>,
        source_constraint_component: Option<S::Term>,
        source_shape: Option<S::Term>,
        value: Option<S::Term>,
    ) -> Self {
        ValidationResult {
            focus_node,
            result_severity,
            result_path,
            source_constraint_component,
            source_shape,
            value,
        }
    }

    pub(crate) fn focus_node(&self) -> Option<S::Term> {
        self.focus_node.to_owned()
    }

    pub(crate) fn result_severity(&self) -> Option<S::Term> {
        self.result_severity.to_owned()
    }

    pub(crate) fn result_path(&self) -> Option<S::Term> {
        self.result_path.to_owned()
    }

    pub(crate) fn source_constraint_component(&self) -> Option<S::Term> {
        self.source_constraint_component.to_owned()
    }

    pub(crate) fn source_shape(&self) -> Option<S::Term> {
        self.source_shape.to_owned()
    }

    pub(crate) fn value(&self) -> Option<S::Term> {
        self.value.to_owned()
    }
}

impl<S: SRDF> ValidationResult<S> {
    pub(crate) fn parse(store: &S, subject: &S::Term) -> Result<Self, ResultError> {
        let mut builder = ValidationResultBuilder::default();

        if let Some(term) = get_object_for(store, subject, &S::iri_s2iri(&SH_FOCUS_NODE))? {
            builder.focus_node(term)
        };
        if let Some(term) = get_object_for(store, subject, &S::iri_s2iri(&SH_RESULT_SEVERITY))? {
            builder.result_severity(term)
        };
        if let Some(term) = get_object_for(store, subject, &S::iri_s2iri(&SH_RESULT_PATH))? {
            builder.result_path(term)
        };
        if let Some(term) = get_object_for(
            store,
            subject,
            &S::iri_s2iri(&SH_SOURCE_CONSTRAINT_COMPONENT),
        )? {
            builder.source_constraint_component(term)
        };
        if let Some(term) = get_object_for(store, subject, &S::iri_s2iri(&SH_SOURCE_SHAPE))? {
            builder.source_shape(term)
        };
        if let Some(term) = get_object_for(store, subject, &S::iri_s2iri(&SH_VALUE))? {
            builder.value(term)
        };

        Ok(builder.build())
    }
}
