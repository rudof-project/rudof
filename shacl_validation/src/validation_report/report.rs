use std::fmt;

use srdf::SRDFBasic;
use srdf::SRDF;

use crate::context::Context;
use crate::helper::srdf::get_objects_for;

use super::result::ValidationResult;
use super::result::ValidationResultBuilder;
use super::validation_report_error::ReportError;

pub struct ValidationReport<S: SRDFBasic> {
    conforms: bool,
    results: Vec<ValidationResult<S>>,
}

impl<S: SRDFBasic> ValidationReport<S> {
    pub(crate) fn is_conformant(&self) -> bool {
        self.results.is_empty()
    }

    pub(crate) fn add_result(&mut self, result: ValidationResult<S>) {
        if self.conforms {
            self.conforms = false; // we add a result --> make the Report non-conformant
        }
        self.results.push(result)
    }

    pub(crate) fn make_validation_result(
        &mut self,
        focus_node: &S::Term,
        context: &Context,
        value_node: Option<&S::Term>,
    ) {
        let mut builder = ValidationResultBuilder::default();

        builder.focus_node(focus_node.to_owned());
        builder.source_constraint_component(context.source_constraint_component::<S>());

        if let Some(result_severity) = context.result_severity::<S>() {
            builder.result_severity(result_severity);
        }
        if let Some(source_shape) = context.source_shape::<S>() {
            builder.source_shape(source_shape);
        }
        if let Some(value) = value_node {
            builder.value(value.to_owned());
        }

        self.add_result(builder.build());
    }
}

impl<S: SRDF> ValidationReport<S> {
    pub fn parse(store: &S, subject: S::Term) -> Result<Self, ReportError> {
        let mut report = ValidationReport::<S>::default();
        let predicate = S::iri_s2iri(&shacl_ast::SH_RESULT);
        for result in get_objects_for(store, &subject, &predicate)? {
            report.add_result(ValidationResult::parse(store, &result)?);
        }
        Ok(report)
    }
}

impl<S: SRDFBasic> Default for ValidationReport<S> {
    fn default() -> Self {
        ValidationReport {
            conforms: true,
            results: Vec::new(),
        }
    }
}

impl<S: SRDFBasic> PartialEq for ValidationReport<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.conforms != other.conforms {
            return false;
        }
        if self.results.len() != other.results.len() {
            return false;
        }
        true
    }
}

impl<S: SRDFBasic> fmt::Display for ValidationReport<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Validation Report: [")?;
        writeln!(f, "\tconforms: {},", self.conforms)?;
        writeln!(f, "\tresult:")?;
        for result in &self.results {
            writeln!(f, "\t\t[")?;
            if let Some(term) = &result.focus_node() {
                writeln!(f, "\t\t\tfocus_node: {},", term)?;
            }
            if let Some(term) = &result.result_path() {
                writeln!(f, "\t\t\tresult_path: {},", term)?;
            }
            if let Some(term) = &result.result_severity() {
                writeln!(f, "\t\t\tresult_severity: {},", term)?;
            }
            if let Some(term) = &result.source_constraint_component() {
                writeln!(f, "\t\t\tsource_constraint_component: {},", term)?;
            }
            if let Some(term) = &result.source_shape() {
                writeln!(f, "\t\t\tsource_shape: {},", term)?;
            }
            if let Some(term) = &result.value() {
                writeln!(f, "\t\t\tvalue: {},", term)?;
            }
            writeln!(f, "\t\t],")?;
        }
        writeln!(f, "]")
    }
}
