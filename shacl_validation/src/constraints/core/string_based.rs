use std::collections::HashSet;

use srdf::{lang::Lang, RDFNode, SRDFGraph};

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    validation_report::{report::ValidationReport, result::ValidationResult},
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
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
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
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
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
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
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
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
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
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}
