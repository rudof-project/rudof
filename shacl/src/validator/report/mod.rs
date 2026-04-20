use std::fmt::{Display, Formatter};
use prefixmap::PrefixMap;
use rudof_rdf::rdf_core::{BuildRDF, FocusRDF, SHACLPath};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use crate::types::Severity;

mod result;
pub(crate) mod error;
mod sorting;

pub use result::ValidationResult;
pub use sorting::ValidationReportSorting;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use crate::error::ReportError;

#[derive(Debug, Clone)]
pub struct ValidationReport {
    results: Vec<ValidationResult>,
    nodes_pm: PrefixMap,
    shapes_pm: PrefixMap,

    // TODO - This should be handled by the CLI crate
    // TODO - This could also be wrapped in the CLI with some severity wrapper for validation results
    // ok_color: Option<Color>, // Green
    // info_color: Option<Color>, // Blue
    // warning_color: Option<Color>, // Yellow
    // debug_color: Option<Color>, // Magenta
    // trace_color: Option<Color>, // Cyan
    // fail_color: Option<Color>, // Red
    // display_with_colors: Option<Color>, // true
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            nodes_pm: PrefixMap::new(),
            shapes_pm: PrefixMap::new(),
        }
    }

    pub fn with_results(mut self, results: Vec<ValidationResult>) -> Self {
        self.results = results;
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

    pub fn nodes_prefixmap(&self) -> &PrefixMap {
        &self.nodes_pm
    }

    pub fn shapes_prefixmap(&self) -> &PrefixMap {
        &self.shapes_pm
    }

    pub fn conforms(&self) -> bool {
        self.results.is_empty()
    }

    pub fn get_count_of(&self, severity: &Severity) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity() == severity)
            .count()
    }
}

impl ValidationReport {
    pub fn parse<S: FocusRDF>(store: &mut S, subject: S::Term) -> Result<Self, ReportError> {
        let mut results = Vec::new();

        for result in store.objects_for(&subject, &ShaclVocab::sh_result().into())
            .map_err(|e| ReportError::ObjectsFor {
                subject: subject.to_string(),
                predicate: ShaclVocab::SH_RESULT.to_string(),
                error: e.to_string(),
            })? {
            results.push(ValidationResult::parse(store, &result)?);
        }

        let mut report = Self::new()
            .with_results(results);

        if let Some(pm) = store.prefixmap() {
            report = report.with_prefixmap(pm);
        }

        Ok(report)
    }

    pub fn to_rdf<RDF: BuildRDF + Sized>(&self, writer: &mut RDF) -> Result<(), ReportError> {
        writer
            .add_prefix("sh", ShaclVocab::sh_ref())
            .map_err(error_mapper::<RDF>("Error adding prefix to writer"))?;

        let report_node: RDF::Subject = writer.add_bnode()
            .map_err(error_mapper::<RDF>("Error creating bnode"))?
            .into();
        writer
            .add_type(report_node.clone(), ShaclVocab::sh_validation_report())
            .map_err(error_mapper::<RDF>("Error type ValidationReport to bnode"))?;

        let conforms: RDF::IRI = ShaclVocab::sh_conforms().into();
        let result: RDF::IRI = ShaclVocab::sh_result().into();

        if self.results.is_empty() {
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
                    .map_err(|_| ReportError::ValidationError {
                        msg: "Cannot convert term to subject".to_string(),
                    })?;
                vr.to_rdf(writer, result_node_subject)?;
            }
        }

        Ok(())
    }
}

// TODO - This needs to be moved probably to rudof_cli or some kind of I/O crate
// impl ValidationReport {
    /*pub fn show_as_table<W: Write>(
        &self,
        mut writer: W,
        _sort_mode: SortModeReport,
        with_details: Option<bool>,
        terminal_width: Option<usize>,
    ) -> Result<(), Error> {
        let with_details = with_details.unwrap_or(false);
        let terminal_width = terminal_width.unwrap_or(80);

        let mut builder = Builder::default();
        if with_details {
            builder.push_record([
                "Severity",
                "Node",
                "Component",
                "Path",
                "Value",
                "Source shape",
                "Details",
            ]);
        } else {
            builder.push_record(["Severity", "node", "Component", "Path", "value", "Source shape"]);
        }
        if self.results.is_empty() {
            let str = "No Errors found";
            if self.display_with_colors {
                if let Some(ok_color) = self.ok_color {
                    write!(writer, "{}", str.color(ok_color))?;
                } else {
                    write!(writer, "{str}")?;
                }
            } else {
                write!(writer, "{str}")?;
            }
            Ok(())
        } else {
            let shacl_prefixmap = if self.display_with_colors {
                PrefixMap::basic()
            } else {
                PrefixMap::basic().with_hyperlink(true).without_default_colors()
            };
            for result in self.results.iter() {
                let severity_str = show_severity(result.severity(), &shacl_prefixmap);
                let severity = if self.display_with_colors {
                    let color = calculate_color(result.severity(), self);
                    severity_str.color(color)
                } else {
                    ColoredString::from(severity_str)
                };
                let node = show_object(result.focus_node(), &self.nodes_prefixmap);
                let component = show_object(result.component(), &shacl_prefixmap);
                let path = show_path_opt(result.path(), &self.shapes_prefixmap);
                let source = show_object_opt(result.source(), &self.shapes_prefixmap);
                let value = show_object_opt(result.value(), &self.nodes_prefixmap);
                let details = result.message().unwrap_or("").to_string();
                if with_details {
                    builder.push_record([
                        &severity.to_string(),
                        &node,
                        &component,
                        &path,
                        &value,
                        &source,
                        &details,
                    ]);
                } else {
                    builder.push_record([&severity.to_string(), &node, &component, &path, &value, &source]);
                }
            }
            let mut table = builder.build();
            table.with(Style::modern_rounded());
            table.with(Modify::new(Segment::all()).with(Width::wrap(terminal_width)));
            writeln!(writer, "{table}")?;
            Ok(())
        }
    } */
// }

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for ValidationReport {
    fn eq(&self, other: &Self) -> bool {
        self.results.len() == other.results.len() &&
            self.results.iter().all(|r| other.results.contains(r))
    }
}

impl Display for ValidationReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.results.is_empty() {
            write!(f, "No Errors found")
        } else {
            writeln!(f, "{} errors found", self.results.len())?;

            for result in self.results.iter() {
                writeln!(f, "{}", &self.shapes_pm.qualify(&result.severity().into()))?;

                writeln!(
                    f,
                    " node: {} {}\n{}{}{}{}",
                    show_object(result.focus_node(), &self.nodes_pm),
                    show_object(result.constraint_component(), &self.shapes_pm),
                    result.message().unwrap_or(""),
                    show_path_opt(result.path(), &self.shapes_pm),
                    show_object_opt(result.source(), &self.shapes_pm),
                    show_object_opt(result.value(), &self.nodes_pm),
                )?;
            }

            Ok(())
        }
    }
}

fn show_object(object: &Object, pm: &PrefixMap) -> String {
    match object {
        Object::Iri(iri) => pm.qualify(iri),
        Object::BlankNode(n) => format!("_:{n}"),
        Object::Literal(lit) => lit.to_string(),
        Object::Triple {
            subject,
            predicate,
            object
        } => format!(
            "<<{} {} {}>>",
            show_subject(subject, pm),
            pm.qualify(predicate),
            show_object(object, pm)
        )
    }
}

fn show_object_opt(object: Option<&Object>, pm: &PrefixMap) -> String {
    match object {
        None => String::new(),
        Some(o) => show_object(o, pm),
    }
}

fn show_subject(subject: &IriOrBlankNode, pm: &PrefixMap) -> String {
    match subject {
        IriOrBlankNode::BlankNode(bnode) => format!("_:{bnode}"),
        IriOrBlankNode::Iri(iri) => pm.qualify(iri),
    }
}

fn show_path_opt(object: Option<&SHACLPath>, pm: &PrefixMap) -> String {
    match object {
        None => String::new(),
        Some(SHACLPath::Predicate { pred }) => {
            let path = pm.qualify(pred);
            path.to_string()
        }
        Some(path) => path.to_string(),
    }
}

fn error_mapper<RDF: BuildRDF>(msg: &str) -> impl FnOnce(RDF::Err) -> ReportError {
    move |e| {
        ReportError::ValidationError {
            msg: format!("{}: {}", msg, e.to_string())
        }
    }
}