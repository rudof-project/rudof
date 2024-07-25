use std::collections::HashSet;

use indoc::formatdoc;
use oxigraph::{model::Term, store::Store};
use srdf::literal::Literal;

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    helper::sparql::ask,
    validation_report::report::ValidationReport,
};

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
pub(crate) struct MinExclusiveConstraintComponent {
    literal: Literal,
}

impl MinExclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MinExclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MinExclusiveConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} < {}) }} ",
                node, self.literal
            };
            println!("{}", query);
            if !ask(store, query)? {
                let result = self.make_validation_result(Some(node));
                report.add_result(result);
            }
        }
        Ok(())
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
pub(crate) struct MinInclusiveConstraintComponent {
    literal: Literal,
}

impl MinInclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MinInclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MinInclusiveConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} <= {}) }} ",
                node, self.literal
            };
            if !ask(store, query)? {
                let result = self.make_validation_result(Some(node));
                report.add_result(result);
            }
        }
        Ok(())
    }
}

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
pub(crate) struct MaxExclusiveConstraintComponent {
    literal: Literal,
}

impl MaxExclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MaxExclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MaxExclusiveConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} > {}) }} ",
                node, self.literal
            };
            if !ask(store, query)? {
                let result = self.make_validation_result(Some(&node));
                report.add_result(result);
            }
        }
        Ok(())
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
pub(crate) struct MaxInclusiveConstraintComponent {
    literal: Literal,
}

impl MaxInclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MaxInclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MaxInclusiveConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} >= {}) }} ",
                node, self.literal
            };
            if !ask(store, query)? {
                let result = self.make_validation_result(Some(&node));
                report.add_result(result);
            }
        }
        Ok(())
    }
}
