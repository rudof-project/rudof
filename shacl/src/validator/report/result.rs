use std::fmt::{Display, Formatter};
use iri_s::IriS;
use rudof_rdf::rdf_core::{BuildRDF, FocusRDF, NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use crate::error::{ReportError, ResultError};
use crate::types::Severity;
use crate::validator::report::error_mapper;

#[derive(Debug, Clone, Eq, Hash)]
pub struct ValidationResult {
    // Required
    focus_node: Object,
    constraint_component: Object,
    severity: Severity,

    // Optional
    path: Option<SHACLPath>,
    value: Option<Object>,
    source: Option<Object>,
    details: Option<Vec<Object>>,
    message: Option<String>,
}

impl ValidationResult {
    /// Creates a new validation result
    pub fn new(focus_node: Object, constraint_component: Object, severity: Severity) -> Self {
        Self {
            focus_node,
            constraint_component,
            severity,
            path: None,
            value: None,
            source: None,
            details: None,
            message: None,
        }
    }

    pub fn with_path(mut self, path: Option<SHACLPath>) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: Option<Object>) -> Self {
        self.value = value;
        self
    }
    pub fn with_source(mut self, source: Option<Object>) -> Self {
        self.source = source;
        self
    }
    pub fn with_details(mut self, details: Option<Vec<Object>>) -> Self {
        self.details = details;
        self
    }
    pub fn with_message(mut self, message: Option<String>) -> Self {
        self.message = message;
        self
    }

    pub fn details(&self) -> Option<&Vec<Object>> {
        self.details.as_ref()
    }

    pub fn constraint_component(&self) -> &Object {
        &self.constraint_component
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn source(&self) -> Option<&Object> {
        self.source.as_ref()
    }

    pub fn value(&self) -> Option<&Object> {
        self.value.as_ref()
    }

    pub fn path(&self) -> Option<&SHACLPath> {
        self.path.as_ref()
    }

    pub fn focus_node(&self) -> &Object {
        &self.focus_node
    }

    pub fn severity(&self) -> &Severity {
        &self.severity
    }
}

impl ValidationResult {
    pub(crate) fn parse<S: FocusRDF>(store: &mut S, validation_result: &S::Term) -> Result<Self, ResultError> {
        // Start processing the required fields
        let focus_node = match store
            .object_for(validation_result, &ShaclVocab::sh_focus_node().into())
            .map_err(|e| ResultError::ObjectFor {
                error: e.to_string(),
                subject: validation_result.to_string(),
                predicate: ShaclVocab::SH_FOCUS_NODE.to_string(),
            })? {
            Some(fnode) => fnode,
            None => return Err(ResultError::MissingRequiredField {
                field: "FocusNode".to_string()
            }),
        };

        let severity = match store
            .object_for(validation_result, &ShaclVocab::sh_result_severity().into())
            .map_err(|e| ResultError::ObjectFor {
                error: e.to_string(),
                subject: validation_result.to_string(),
                predicate: ShaclVocab::SH_RESULT_SEVERITY.to_string(),
            })? {
            Some(Object::Iri(severity)) => (&severity).into(),
            Some(other) => return Err(ResultError::WrongNodeForSeverity {
                field: "Severity".to_string(),
                value: other.to_string(),
            }),
            None => return Err(ResultError::MissingRequiredField {
                field: "Severity".to_string(),
            }),
        };

        let constraint_component = match store
            .object_for(validation_result, &ShaclVocab::sh_source_constraint_component().into())
            .map_err(|e| ResultError::ObjectFor {
                error: e.to_string(),
                subject: validation_result.to_string(),
                predicate: ShaclVocab::SH_SOURCE_CONSTRAINT_COMPONENT.to_string(),
            })? {
            Some(component) => component,
            None => return Err(ResultError::MissingRequiredField {
                field: "SourceConstraintComponent".to_string(),
            })
        };

        // Process the optional fields
        let path = store
            .get_path_for(validation_result, &ShaclVocab::sh_result_path().into())
            .map_err(|e| ResultError::PathFor {
                subject: validation_result.to_string(),
                error: e.to_string(),
            })?;

        let source = store
            .object_for(validation_result, &ShaclVocab::sh_source_shape().into())
            .map_err(|e| ResultError::ObjectFor {
                error: e.to_string(),
                subject: validation_result.to_string(),
                predicate: ShaclVocab::SH_SOURCE_SHAPE.to_string(),
            })?;

        let value = store
            .object_for(validation_result, &ShaclVocab::sh_value().into())
            .map_err(|e| ResultError::ObjectFor {
                error: e.to_string(),
                subject: validation_result.to_string(),
                predicate: ShaclVocab::SH_VALUE.to_string(),
            })?;

        Ok(ValidationResult::new(focus_node, constraint_component, severity)
            .with_path(path)
            .with_source(source)
            .with_value(value))
    }

    pub fn to_rdf<RDF: BuildRDF + Sized>(&self, writer: &mut RDF, report_node: RDF::Subject) -> Result<(), ReportError> {
        writer
            .add_type(report_node.clone(), ShaclVocab::sh_validation_result())
            .map_err(error_mapper::<RDF>(""))?;
        writer
            .add_triple(report_node.clone(), ShaclVocab::sh_focus_node(), self.focus_node.clone())
            .map_err(error_mapper::<RDF>("Error adding focus node to validation result"))?;
        writer
            .add_triple(report_node.clone(), ShaclVocab::sh_source_constraint_component(), self.constraint_component.clone())
            .map_err(error_mapper::<RDF>("Error adding source constraint component to validation result"))?;

        let severity: RDF::Term = <&Severity as Into<IriS>>::into(&self.severity).into();
        writer
            .add_triple(report_node.clone(), ShaclVocab::sh_result_severity(), severity)
            .map_err(error_mapper::<RDF>("Error adding severity to validation result"))?;

        let msg = match self.message {
            None => Object::str("No message"),
            Some(ref msg) => Object::str(msg),
        };
        writer
            .add_triple(report_node.clone(), ShaclVocab::sh_result_message(), msg)
            .map_err(error_mapper::<RDF>("Error result message to validation result"))?;

        if let Some(source) = &self.source {
            let term: RDF::Term = source.clone().into();
            writer
                .add_triple(report_node.clone(), ShaclVocab::sh_source_shape(), term)
                .map_err(error_mapper::<RDF>("Error adding source to validation result"))?;
        }

        if let Some(path) = &self.path {
            let result_path = path_to_rdf::<RDF>(path);
            writer
                .add_triple(report_node.clone(), ShaclVocab::sh_result_path(), result_path)
                .map_err(error_mapper::<RDF>("Error adding result path to validation result"))?;
        }

        if let Some(value) = &self.value {
            let term: RDF::Term = value.clone().into();
            writer
                .add_triple(report_node.clone(), ShaclVocab::sh_value(), term)
                .map_err(error_mapper::<RDF>("Error adding value to validation result"))?;
        }

        Ok(())
    }
}

fn path_to_rdf<RDF: NeighsRDF>(path: &SHACLPath) -> RDF::Term {
    match path {
        SHACLPath::Predicate { pred } => pred.clone().into(),
        SHACLPath::Alternative { .. } => todo!(),
        SHACLPath::Sequence { .. } => todo!(),
        SHACLPath::Inverse { .. } => todo!(),
        SHACLPath::ZeroOrMore { .. } => todo!(),
        SHACLPath::OneOrMore { .. } => todo!(),
        SHACLPath::ZeroOrOne { .. } => todo!(),
    }
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValidationResult(focus_node: {}, constraint_component: {}, severity: {}, message: {:?}, path: {:?}, value: {:?}, source: {:?}, details: {:?})",
            self.focus_node,
            self.constraint_component,
            self.severity,
            self.message,
            self.path,
            self.value,
            self.source,
            self.details
        )
    }
}

impl PartialEq for ValidationResult {
    fn eq(&self, other: &Self) -> bool {
        self.focus_node == other.focus_node &&
            self.constraint_component == other.constraint_component &&
            self.severity == other.severity &&
            self.path == other.path &&
            self.value == other.value &&
            self.source == other.source &&
            self.details == other.details
    }
}
