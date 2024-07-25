use indoc::formatdoc;
use oxigraph::{model::Term, sparql::QueryResults, store::Store};

use super::validation_report_error::ValidationResultError;

#[derive(Default)]
pub struct ValidationResultBuilder {
    focus_node: Option<Term>,
    result_severity: Option<Term>,
    result_path: Option<Term>,
    source_constraint: Option<Term>,
    source_constraint_component: Option<Term>,
    source_shape: Option<Term>,
    value: Option<Term>,
}

impl ValidationResultBuilder {
    pub fn focus_node(&mut self, focus_node: Term) {
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

    pub fn build(self) -> ValidationResult {
        ValidationResult {
            focus_node: self.focus_node,
            result_severity: self.result_severity,
            result_path: self.result_path,
            source_constraint: self.source_constraint,
            source_constraint_component: self.source_constraint_component,
            source_shape: self.source_shape,
            value: self.value,
        }
    }
}

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
    pub fn parse(store: &Store, subject: &Term) -> Result<Self, ValidationResultError> {
        let query = formatdoc! {
            "
                SELECT ?focus_node ?result_severity ?result_path
                    ?source_constraint ?source_constraint_component
                    ?source_shape ?value
                WHERE {{
                    OPTIONAL {{
                        {} {} ?focus_node .
                        {} {} ?result_severity .
                        {} {} ?result_path .
                        {} {} ?source_constraint .
                        {} {} ?source_constraint_component .
                        {} {} ?source_shape .
                        {} {} ?value .
                    }}
                }}
            ", 
            subject, shacl_ast::SH_FOCUS_NODE.as_named_node(),
            subject, shacl_ast::SH_RESULT_SEVERITY.as_named_node(),
            subject, shacl_ast::SH_RESULT_PATH.as_named_node(),
            subject, shacl_ast::SH_SOURCE_CONSTRAINT.as_named_node(),
            subject, shacl_ast::SH_SOURCE_CONSTRAINT_COMPONENT.as_named_node(),
            subject, shacl_ast::SH_SOURCE_SHAPE.as_named_node(),
            subject, shacl_ast::SH_VALUE.as_named_node()
        };

        let mut builder = ValidationResultBuilder::default();

        match store.query(&query) {
            Ok(QueryResults::Solutions(solutions)) => match solutions.into_iter().nth(0) {
                Some(Ok(solution)) => {
                    if let Some(term) = solution.get("focus_node") {
                        builder.focus_node(term.to_owned());
                    }
                    if let Some(term) = solution.get("result_severity") {
                        builder.result_severity(term.to_owned());
                    }
                    if let Some(term) = solution.get("result_path") {
                        builder.result_path(term.to_owned());
                    }
                    if let Some(term) = solution.get("source_constraint") {
                        builder.source_constraint(term.to_owned());
                    }
                    if let Some(term) = solution.get("source_constraint_component") {
                        builder.source_constraint_component(term.to_owned());
                    }
                    if let Some(term) = solution.get("source_shape") {
                        builder.source_shape(term.to_owned());
                    }
                    if let Some(term) = solution.get("focus_node") {
                        builder.focus_node(term.to_owned());
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        };

        Ok(builder.build())
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
