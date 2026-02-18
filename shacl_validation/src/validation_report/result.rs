use super::validation_report_error::{ReportError, ResultError};
use rudof_rdf::rdf_core::{BuildRDF, FocusRDF, NeighsRDF, SHACLPath, term::Object};
use shacl_ast::ShaclVocab;
use shacl_ir::severity::CompiledSeverity;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidationResult {
    focus_node: Object,           // required
    path: Option<SHACLPath>,      // optional
    value: Option<Object>,        // optional
    source: Option<Object>,       // optional
    constraint_component: Object, // required
    details: Option<Vec<Object>>, // optional
    message: Option<String>,      // optional
    severity: CompiledSeverity,   // required
}

impl ValidationResult {
    // Creates a new validation result
    pub fn new(focus_node: Object, constraint_component: Object, severity: CompiledSeverity) -> Self {
        Self {
            focus_node,
            path: None,
            value: None,
            source: None,
            constraint_component,
            details: None,
            message: None,
            severity,
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

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
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

    pub fn component(&self) -> &Object {
        &self.constraint_component
    }

    pub fn severity(&self) -> &CompiledSeverity {
        &self.severity
    }
}

impl ValidationResult {
    pub(crate) fn parse<S: FocusRDF>(store: &mut S, validation_result: &S::Term) -> Result<Self, ResultError> {
        // Start processing the required fields.
        let focus_node = match store
            .object_for(validation_result, &ShaclVocab::sh_focus_node().clone().into())
            .map_err(|e| ResultError::ObjectFor {
                subject: validation_result.to_string(),
                predicate: ShaclVocab::sh_focus_node().to_string(),
                error: e.to_string(),
            })? {
            Some(focus_node) => focus_node,
            None => return Err(ResultError::MissingRequiredField("FocusNode".to_owned())),
        };
        let severity = match store
            .object_for(validation_result, &ShaclVocab::sh_result_severity().clone().into())
            .map_err(|e| ResultError::ObjectFor {
                subject: validation_result.to_string(),
                predicate: ShaclVocab::sh_result_severity().to_string(),
                error: e.to_string(),
            })? {
            Some(Object::Iri(severity)) => {
                CompiledSeverity::from_iri(&severity).ok_or_else(|| ResultError::WrongIRIForSeverity {
                    field: "Severity".to_owned(),
                    value: format!("{severity}"),
                })?
            },
            Some(other) => {
                return Err(ResultError::WrongNodeForSeverity {
                    field: "Severity".to_owned(),
                    value: format!("{other}"),
                });
            },
            None => return Err(ResultError::MissingRequiredField("Severity".to_owned())),
        };
        let constraint_component = match store
            .object_for(
                validation_result,
                &ShaclVocab::sh_source_constraint_component().clone().into(),
            )
            .map_err(|e| ResultError::ObjectFor {
                subject: validation_result.to_string(),
                predicate: ShaclVocab::sh_source_constraint_component().to_string(),
                error: e.to_string(),
            })? {
            Some(constraint_component) => constraint_component,
            None => {
                return Err(ResultError::MissingRequiredField(
                    "SourceConstraintComponent".to_owned(),
                ));
            },
        };

        // Process the optional fields
        let sh_result_path_iri: S::IRI = ShaclVocab::sh_result_path().clone().into();
        let path = store
            .get_path_for(validation_result, &sh_result_path_iri)
            .map_err(|e| ResultError::PathFor {
                subject: validation_result.to_string(),
                path: sh_result_path_iri.to_string(),
                error: e.to_string(),
            })?;

        let sh_source_shape_iri: S::IRI = ShaclVocab::sh_source_shape().clone().into();
        let source = store
            .object_for(validation_result, &sh_source_shape_iri)
            .map_err(|e| ResultError::ObjectFor {
                subject: validation_result.to_string(),
                predicate: sh_source_shape_iri.to_string(),
                error: e.to_string(),
            })?;
        let sh_value_iri: S::IRI = ShaclVocab::sh_value().clone().into();
        let value = store
            .object_for(validation_result, &sh_value_iri)
            .map_err(|e| ResultError::ObjectFor {
                subject: validation_result.to_string(),
                predicate: sh_value_iri.to_string(),
                error: e.to_string(),
            })?;

        // 3. Lastly we build the ValidationResult
        Ok(ValidationResult::new(focus_node, constraint_component, severity)
            .with_path(path)
            .with_source(source)
            .with_value(value))
    }

    pub fn to_rdf<RDF>(&self, rdf_writer: &mut RDF, report_node: RDF::Subject) -> Result<(), ReportError>
    where
        RDF: BuildRDF + Sized,
    {
        rdf_writer
            .add_type(report_node.clone(), ShaclVocab::sh_validation_result().clone())
            .map_err(|e| ReportError::ValidationError { msg: e.to_string() })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                ShaclVocab::sh_focus_node().clone(),
                self.focus_node.clone(),
            )
            .map_err(|e| ReportError::ValidationError {
                msg: format!("Error adding focus node to validation result: {e}"),
            })?;
        rdf_writer
            .add_triple(
                report_node.clone(),
                ShaclVocab::sh_source_constraint_component().clone(),
                self.constraint_component.clone(),
            )
            .map_err(|e| ReportError::ValidationError {
                msg: format!("Error adding source constraint component to validation result: {e}"),
            })?;
        let severity: RDF::Term = self.severity().to_iri().into();
        rdf_writer
            .add_triple(report_node.clone(), ShaclVocab::sh_result_severity().clone(), severity)
            .map_err(|e| ReportError::ValidationError {
                msg: format!("Error adding severity to validation result: {e}"),
            })?;
        let message = match self.message {
            Some(ref message) => Object::str(message),
            None => Object::str("No message"),
        };
        rdf_writer
            .add_triple(report_node.clone(), ShaclVocab::sh_result_message().clone(), message)
            .map_err(|e| ReportError::ValidationError {
                msg: format!("Error result message to validation result: {e}"),
            })?;
        if let Some(source) = &self.source {
            let source_term: RDF::Term = source.clone().into();
            rdf_writer
                .add_triple(report_node.clone(), ShaclVocab::sh_source_shape().clone(), source_term)
                .map_err(|e| ReportError::ValidationError {
                    msg: format!("Error adding source to validation result: {e}"),
                })?;
        }
        if let Some(path) = &self.path {
            let result_path: RDF::Term = path_to_rdf::<RDF>(path);
            rdf_writer
                .add_triple(report_node.clone(), ShaclVocab::sh_result_path().clone(), result_path)
                .map_err(|e| ReportError::ValidationError {
                    msg: format!("Error adding result path to validation result: {e}"),
                })?;
        }
        if let Some(value) = &self.value {
            let value_term: RDF::Term = value.clone().into();
            rdf_writer
                .add_triple(report_node.clone(), ShaclVocab::sh_value().clone(), value_term)
                .map_err(|e| ReportError::ValidationError {
                    msg: format!("Error adding value to validation result: {e}"),
                })?;
        }
        Ok(())
    }
}

fn path_to_rdf<RDF>(path: &SHACLPath) -> RDF::Term
where
    RDF: NeighsRDF,
{
    match path {
        SHACLPath::Predicate { pred } => pred.clone().into(),
        _ => todo!(),
    }
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValidationResult(focus_node: {}, constraint_component: {}, severity: {:?}, message: {:?}, path: {:?}, value: {:?}, source: {:?}, details: {:?})",
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
