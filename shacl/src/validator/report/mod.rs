use crate::types::Severity;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{BuildRDF, FocusRDF, Rdf};
use std::fmt::{Display, Formatter};

mod evidence;
mod outcome;
mod result;
mod sorting;

use crate::error::ValidationError;
pub use evidence::Evidence;
pub use outcome::ValidationOutcome;
pub use result::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
pub use sorting::ValidationReportSorting;

#[derive(Debug, Clone)]
pub struct ValidationReport {
    conforms: bool,
    results: Vec<ValidationResult>,
    evidences: Vec<Evidence>,
    nodes_pm: PrefixMap,
    shapes_pm: PrefixMap,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            conforms: true,
            results: Vec::new(),
            evidences: Vec::new(),
            nodes_pm: PrefixMap::new(),
            shapes_pm: PrefixMap::new(),
        }
    }

    /// Sets whether the validated data conforms, independently of whether
    /// the underlying violations are retained in [`Self::results`]. Needed
    /// because "no errors" mode intentionally leaves `results` empty while
    /// still needing to report `conforms` correctly.
    pub fn with_conforms(mut self, conforms: bool) -> Self {
        self.conforms = conforms;
        self
    }

    pub fn with_results(mut self, results: Vec<ValidationResult>) -> Self {
        self.results = results;
        self
    }

    pub fn with_evidences(mut self, evidences: Vec<Evidence>) -> Self {
        self.evidences = evidences;
        self
    }

    /// Sets the same prefixmap for nodes and shapes
    pub fn with_prefixmap(mut self, pm: PrefixMap) -> Self {
        self.shapes_pm = pm.clone();
        self.nodes_pm = pm;
        self
    }

    /// Sets the prefixmap for nodes
    pub fn with_nodes_prefixmap(mut self, pm: PrefixMap) -> Self {
        self.nodes_pm = pm;
        self
    }

    /// Sets the prefixmap for shapes
    pub fn with_shapes_prefixmap(mut self, pm: PrefixMap) -> Self {
        self.shapes_pm = pm;
        self
    }

    pub fn results(&self) -> &Vec<ValidationResult> {
        &self.results
    }

    pub fn evidences(&self) -> &Vec<Evidence> {
        &self.evidences
    }

    pub fn nodes_prefixmap(&self) -> &PrefixMap {
        &self.nodes_pm
    }

    pub fn shapes_prefixmap(&self) -> &PrefixMap {
        &self.shapes_pm
    }

    /// Whether the validated data conforms. Set explicitly by the
    /// validator (via [`Self::with_conforms`]) rather than derived from
    /// [`Self::results`], since "no errors" mode intentionally leaves
    /// `results` empty even when the data doesn't conform.
    pub fn conforms(&self) -> bool {
        self.conforms
    }

    pub fn get_count_of(&self, severity: &Severity) -> usize {
        self.results.iter().filter(|r| r.severity() == severity).count()
    }
}

impl ValidationReport {
    pub fn parse<S: FocusRDF>(store: &mut S, subject: S::Term) -> Result<Self, ValidationError> {
        let mut results = Vec::new();

        for result in store.objects_for(&subject, &ShaclVocab::sh_result().into())? {
            results.push(ValidationResult::parse(store, &result)?);
        }

        let mut report = Self::new().with_conforms(results.is_empty()).with_results(results);

        if let Some(pm) = store.prefixmap() {
            report = report.with_prefixmap(pm);
        }

        Ok(report)
    }

    pub fn to_rdf<RDF: BuildRDF + Sized>(&self, writer: &mut RDF) -> Result<(), ValidationError> {
        writer.add_prefix("sh", ShaclVocab::sh_ref());

        let report_node: RDF::Subject = writer
            .add_bnode()
            .map_err(error_mapper::<RDF>("Error creating bnode"))?
            .into();
        writer
            .add_type(report_node.clone(), ShaclVocab::sh_validation_report())
            .map_err(error_mapper::<RDF>("Error type ValidationReport to bnode"))?;

        let conforms: RDF::IRI = ShaclVocab::sh_conforms().into();
        let result: RDF::IRI = ShaclVocab::sh_result().into();

        if self.conforms {
            let true_term: RDF::Term = Object::boolean(true).into();
            writer
                .add_triple(report_node.clone(), conforms, true_term)
                .map_err(error_mapper::<RDF>("Error adding conforms to bnode"))?;
        } else {
            let false_term: RDF::Term = Object::boolean(false).into();
            writer
                .add_triple(report_node.clone(), conforms, false_term)
                .map_err(error_mapper::<RDF>("Error adding conforms to bnode"))?;

            for vr in self.results.iter() {
                let result_node = writer
                    .add_bnode()
                    .map_err(error_mapper::<RDF>("Error creating bnode"))?;
                let result_node_term: RDF::Term = result_node.into();
                writer
                    .add_triple(report_node.clone(), result.clone(), result_node_term.clone())
                    .map_err(error_mapper::<RDF>("Error adding result to bnode"))?;

                let result_node_subject: RDF::Subject = RDF::Subject::try_from(result_node_term)
                    .map_err(|_| ValidationError::CastError("term".to_string(), "subject".to_string()))?;
                vr.to_rdf(writer, result_node_subject)?;
            }
        }

        Ok(())
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for ValidationReport {
    fn eq(&self, other: &Self) -> bool {
        self.conforms == other.conforms
            && self.results.len() == other.results.len()
            && self.results.iter().all(|r| other.results.contains(r))
    }
}

impl Display for ValidationReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.results.is_empty() {
            write!(f, "No Errors found")
        } else {
            writeln!(f, "{} errors found", self.results.len())?;

            for result in self.results.iter() {
                writeln!(f, "{}", self.shapes_pm.qualify(&result.severity().into()))?;

                writeln!(
                    f,
                    " node: {} {}\n{}{}{}{}",
                    self.nodes_pm.show(result.focus_node()),
                    self.shapes_pm.show(result.constraint_component()),
                    result.message(),
                    self.shapes_pm.show(&result.path()),
                    self.shapes_pm.show(&result.source()),
                    self.nodes_pm.show(&result.value()),
                )?;
            }

            Ok(())
        }
    }
}

fn error_mapper<RDF: Rdf>(msg: &str) -> impl FnOnce(RDF::Err) -> ValidationError {
    move |e| ValidationError::new_graph_error_ctx::<RDF>(e, msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_rdf::rdf_core::NeighsRDF;
    use rudof_rdf::rdf_core::term::Triple;
    use rudof_rdf::rdf_core::vocabs::ShaclVocab;
    use rudof_rdf::rdf_impl::OxigraphInMemory;

    #[test]
    fn conforms_is_independent_of_whether_results_are_retained() {
        // "no errors" mode: conforms=false but results deliberately empty.
        let report = ValidationReport::new().with_conforms(false);
        assert!(!report.conforms());
        assert!(report.results().is_empty());
    }

    #[test]
    fn to_rdf_serializes_conforms_field_not_results_emptiness() {
        let report = ValidationReport::new().with_conforms(false);
        let mut graph = OxigraphInMemory::empty();
        report.to_rdf(&mut graph).unwrap();

        let conforms_pred: <OxigraphInMemory as rudof_rdf::rdf_core::Rdf>::IRI = ShaclVocab::sh_conforms().into();
        let false_term: <OxigraphInMemory as rudof_rdf::rdf_core::Rdf>::Term = Object::boolean(false).into();
        let found = graph
            .triples_with_predicate(&conforms_pred)
            .unwrap()
            .any(|t| t.obj() == &false_term);
        assert!(found, "sh:conforms should be false even though results() is empty");
    }

    #[test]
    fn default_report_conforms() {
        assert!(ValidationReport::new().conforms());
    }
}
