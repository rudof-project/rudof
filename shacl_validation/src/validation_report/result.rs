use shacl_ast::*;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::helper::srdf::get_object_for;
use crate::helper::term::Term;

use super::validation_report_error::ValidationResultError;

#[derive(Default)]
pub struct ValidationResultBuilder<'a> {
    focus_node: Option<&'a Term>,
    result_severity: Option<Term>,
    result_path: Option<Term>,
    source_constraint: Option<Term>,
    source_constraint_component: Option<Term>,
    source_shape: Option<Term>,
    value: Option<Term>,
}

impl<'a> ValidationResultBuilder<'a> {
    pub fn default() -> Self {
        ValidationResultBuilder {
            focus_node: None,
            result_severity: None,
            result_path: None,
            source_constraint: None,
            source_constraint_component: None,
            source_shape: None,
            value: None,
        }
    }

    pub fn focus_node(&mut self, focus_node: &Term) {
        self.focus_node = Some(focus_node);
    }

    pub fn result_severity(&mut self, result_severity: Term) {
        self.result_severity = Some(result_severity);
    }

    pub fn result_path(&mut self, result_path: Term) {
        self.result_path = Some(result_path);
    }

    pub fn source_constraint(&mut self, source_constraint: Term) {
        self.source_constraint = Some(source_constraint);
    }

    pub fn source_constraint_component(&mut self, source_constraint_component: Term) {
        self.source_constraint_component = Some(source_constraint_component);
    }

    pub fn source_shape(&mut self, source_shape: Term) {
        self.source_shape = Some(source_shape);
    }

    pub fn value(&mut self, value: Term) {
        self.value = Some(value);
    }

    pub fn build(self) -> ValidationResult<'a> {
        ValidationResult::new(
            self.focus_node,
            self.result_severity,
            self.result_path,
            self.source_constraint,
            self.source_constraint_component,
            self.source_shape,
            self.value,
        )
    }
}

pub struct ValidationResult<'a> {
    focus_node: Option<&'a Term>,
    result_severity: Option<Term>,
    result_path: Option<Term>,
    source_constraint: Option<Term>,
    source_constraint_component: Option<Term>,
    source_shape: Option<Term>,
    value: Option<Term>,
}

impl<'a> ValidationResult<'a> {
    pub(crate) fn new(
        focus_node: Option<&'a Term>,
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

    pub(crate) fn parse<S: SRDF + SRDFBasic>(
        store: &S,
        subject: &Term,
    ) -> Result<Self, ValidationResultError> {
        let mut builder = ValidationResultBuilder::default();

        let subject = match subject {
            Term::IRI(_) => subject,
            Term::BlankNode(_) => subject,
            Term::Literal(_) => return Err(ValidationResultError::LiteralToSubject),
        };

        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&SH_FOCUS_NODE))? {
            builder.focus_node(&term)
        };
        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&SH_RESULT_SEVERITY))? {
            builder.result_severity(term)
        };
        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&SH_RESULT_PATH))? {
            builder.result_path(term)
        };
        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&SH_SOURCE_CONSTRAINT))? {
            builder.source_constraint(term)
        };
        if let Some(term) = get_object_for(
            store,
            &subject,
            &S::iri_s2iri(&SH_SOURCE_CONSTRAINT_COMPONENT),
        )? {
            builder.source_constraint_component(term)
        };
        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&SH_SOURCE_SHAPE))? {
            builder.source_shape(term)
        };
        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&SH_VALUE))? {
            builder.value(term)
        };

        Ok(builder.build())
    }

    pub(crate) fn focus_node(&self) -> Option<&Term> {
        self.focus_node
    }

    pub(crate) fn result_severity(&self) -> Option<Term> {
        self.result_severity
    }

    pub(crate) fn result_path(&self) -> Option<Term> {
        self.result_path
    }

    pub(crate) fn source_constraint(&self) -> Option<Term> {
        self.source_constraint
    }

    pub(crate) fn source_constraint_component(&self) -> Option<Term> {
        self.source_constraint_component
    }

    pub(crate) fn source_shape(&self) -> Option<Term> {
        self.source_shape
    }

    pub(crate) fn value(&self) -> Option<Term> {
        self.value
    }
}
