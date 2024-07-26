use std::collections::HashSet;

use indoc::formatdoc;
use oxigraph::{model::Term, store::Store};
use srdf::lang::Lang;

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    helper::sparql::ask,
    validation_report::report::ValidationReport,
};

/// sh:minLength specifies the minimum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MinLengthConstraintComponent
pub(crate) struct MinLengthConstraintComponent {
    min_length: isize,
}

impl MinLengthConstraintComponent {
    pub fn new(min_length: isize) -> Self {
        MinLengthConstraintComponent { min_length }
    }
}

impl Evaluate for MinLengthConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if node.is_blank_node() || node.is_triple() {
                self.make_validation_result(Some(node), report);
            } else {
                let query = formatdoc! {
                    " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                    node, self.min_length
                };
                if !ask(store, query)? {
                    self.make_validation_result(Some(node), report);
                }
            }
        }
        Ok(())
    }
}

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
pub(crate) struct MaxLengthConstraintComponent {
    max_length: isize,
}

impl MaxLengthConstraintComponent {
    pub fn new(max_length: isize) -> Self {
        MaxLengthConstraintComponent { max_length }
    }
}

impl Evaluate for MaxLengthConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if node.is_blank_node() || node.is_triple() {
                self.make_validation_result(Some(node), report);
            } else {
                let query = formatdoc! {
                    " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                    node, self.max_length
                };
                if !ask(store, query)? {
                    self.make_validation_result(Some(node), report);
                }
            }
        }
        Ok(())
    }
}

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
pub(crate) struct PatternConstraintComponent {
    pattern: String,
    flags: Option<String>,
}

impl PatternConstraintComponent {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        PatternConstraintComponent { pattern, flags }
    }
}

impl Evaluate for PatternConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if node.is_blank_node() || node.is_triple() {
                self.make_validation_result(Some(node), report);
            } else {
                let query = match &self.flags {
                    Some(flags) => formatdoc! {
                        "ASK {{
                            FILTER (regex(str({}), {}, {}))
                        }}",
                        node, self.pattern, flags
                    },
                    None => formatdoc! {
                        "ASK {{
                            FILTER (regex(str({}), {}))
                        }}",
                        node, self.pattern
                    },
                };
                if !ask(store, query)? {
                    self.make_validation_result(Some(node), report);
                }
            }
        }
        Ok(())
    }
}

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
pub(crate) struct LanguageInConstraintComponent {
    langs: Vec<Lang>,
}

impl LanguageInConstraintComponent {
    pub fn new(langs: Vec<Lang>) -> Self {
        LanguageInConstraintComponent { langs }
    }
}

impl Evaluate for LanguageInConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let literal = match node {
                Term::Literal(literal) => literal,
                _ => {
                    self.make_validation_result(Some(node), report);
                    break;
                }
            };
            match literal.language() {
                Some(language) => {
                    if !self.langs.contains(&Lang::new(language)) {
                        self.make_validation_result(Some(node), report);
                    }
                }
                _ => {
                    self.make_validation_result(Some(node), report);
                }
            }
        }
        Ok(())
    }
}

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
pub(crate) struct UniqueLangConstraintComponent {
    unique_lang: bool,
}

impl UniqueLangConstraintComponent {
    pub fn new(unique_lang: bool) -> Self {
        UniqueLangConstraintComponent { unique_lang }
    }
}

impl Evaluate for UniqueLangConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        if self.unique_lang {
            let mut langs = HashSet::new();
            for node in &value_nodes {
                if let Term::Literal(literal) = node {
                    if let Some(lang) = literal.language() {
                        if langs.contains(lang) {
                            self.make_validation_result(Some(node), report)
                        } else {
                            langs.insert(lang);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
