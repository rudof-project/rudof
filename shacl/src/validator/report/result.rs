use crate::error::ValidationError;
use crate::types::{MessageMap, Severity};
use crate::validator::report::error_mapper;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShaclVocab};
use rudof_rdf::rdf_core::{BuildRDF, FocusRDF, SHACLPath};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
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
    message: MessageMap,
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
            message: Default::default(),
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
    pub fn with_message(mut self, message: MessageMap) -> Self {
        self.message = message;
        self
    }

    pub fn details(&self) -> Option<&Vec<Object>> {
        self.details.as_ref()
    }

    pub fn constraint_component(&self) -> &Object {
        &self.constraint_component
    }

    pub fn message(&self) -> &MessageMap {
        &self.message
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
    pub(crate) fn parse<S: FocusRDF>(store: &mut S, validation_result: &S::Term) -> Result<Self, ValidationError> {
        // Start processing the required fields
        let focus_node = store
            .object_for(validation_result, &ShaclVocab::sh_focus_node().into())?
            .ok_or(ValidationError::MissingRequiredField(
                ShaclVocab::SH_FOCUS_NODE.to_string(),
            ))?;

        let severity = match store.object_for(validation_result, &ShaclVocab::sh_result_severity().into())? {
            Some(Object::Iri(severity)) => (&severity).into(),
            Some(other) => {
                return Err(ValidationError::InvalidIriValue {
                    field: ShaclVocab::SH_SEVERITY.to_string(),
                    value: other.to_string(),
                });
            },
            None => {
                return Err(ValidationError::MissingRequiredField(
                    ShaclVocab::SH_SEVERITY.to_string(),
                ));
            },
        };

        let constraint_component = store
            .object_for(validation_result, &ShaclVocab::sh_source_constraint_component().into())?
            .ok_or(ValidationError::MissingRequiredField(
                ShaclVocab::SH_SOURCE_CONSTRAINT_COMPONENT.to_string(),
            ))?;

        // Process the optional fields
        let path = store.get_path_for(validation_result, &ShaclVocab::sh_result_path().into())?;

        let source = store.object_for(validation_result, &ShaclVocab::sh_source_shape().into())?;

        let value = store.object_for(validation_result, &ShaclVocab::sh_value().into())?;

        Ok(ValidationResult::new(focus_node, constraint_component, severity)
            .with_path(path)
            .with_source(source)
            .with_value(value))
    }

    pub fn to_rdf<RDF: BuildRDF + Sized>(
        &self,
        writer: &mut RDF,
        report_node: RDF::Subject,
    ) -> Result<(), ValidationError> {
        writer
            .add_type(report_node.clone(), ShaclVocab::sh_validation_result())
            .map_err(error_mapper::<RDF>(""))?;
        writer
            .add_triple(
                report_node.clone(),
                ShaclVocab::sh_focus_node(),
                self.focus_node.clone(),
            )
            .map_err(error_mapper::<RDF>("Error adding focus node to validation result"))?;
        writer
            .add_triple(
                report_node.clone(),
                ShaclVocab::sh_source_constraint_component(),
                self.constraint_component.clone(),
            )
            .map_err(error_mapper::<RDF>(
                "Error adding source constraint component to validation result",
            ))?;

        let severity: RDF::Term = <&Severity as Into<IriS>>::into(&self.severity).into();
        writer
            .add_triple(report_node.clone(), ShaclVocab::sh_result_severity(), severity)
            .map_err(error_mapper::<RDF>("Error adding severity to validation result"))?;

        for lit in self.message.iter_literals() {
            writer
                .add_triple::<_, _, RDF::Literal>(report_node.clone(), ShaclVocab::sh_result_message(), lit.into())
                .map_err(error_mapper::<RDF>("Error result message to validation result"))?;
        }

        if let Some(source) = &self.source {
            let term: RDF::Term = source.clone().into();
            writer
                .add_triple(report_node.clone(), ShaclVocab::sh_source_shape(), term)
                .map_err(error_mapper::<RDF>("Error adding source to validation result"))?;
        }

        if let Some(path) = &self.path {
            let result_path = path_to_rdf(writer, path)?;
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

/// Serializes a SHACL path returning the term that identifies the path.
/// Complex paths are encoded as blank node structures
fn path_to_rdf<RDF: BuildRDF>(writer: &mut RDF, path: &SHACLPath) -> Result<RDF::Term, ValidationError> {
    match path {
        SHACLPath::Predicate { pred } => Ok(pred.clone().into()),
        SHACLPath::Alternative { paths } => {
            let list = path_list_to_rdf(writer, paths)?;
            path_with_predicate_to_rdf(writer, ShaclVocab::sh_alternative_path(), list)
        },
        SHACLPath::Sequence { paths } => path_list_to_rdf(writer, paths),
        SHACLPath::Inverse { path } => {
            let sub_path = path_to_rdf(writer, path)?;
            path_with_predicate_to_rdf(writer, ShaclVocab::sh_inverse_path(), sub_path)
        },
        SHACLPath::ZeroOrMore { path } => {
            let sub_path = path_to_rdf(writer, path)?;
            path_with_predicate_to_rdf(writer, ShaclVocab::sh_zero_or_more_path(), sub_path)
        },
        SHACLPath::OneOrMore { path } => {
            let sub_path = path_to_rdf(writer, path)?;
            path_with_predicate_to_rdf(writer, ShaclVocab::sh_one_or_more_path(), sub_path)
        },
        SHACLPath::ZeroOrOne { path } => {
            let sub_path = path_to_rdf(writer, path)?;
            path_with_predicate_to_rdf(writer, ShaclVocab::sh_zero_or_one_path(), sub_path)
        },
    }
}

/// Creates a blank node linked to the given object through the given predicate,
/// as used by alternative, inverse and quantified paths.
fn path_with_predicate_to_rdf<RDF: BuildRDF>(
    writer: &mut RDF,
    predicate: IriS,
    object: RDF::Term,
) -> Result<RDF::Term, ValidationError> {
    let node: RDF::Subject = writer
        .add_bnode()
        .map_err(error_mapper::<RDF>("Error creating bnode for path"))?
        .into();
    writer
        .add_triple(node.clone(), predicate, object)
        .map_err(error_mapper::<RDF>("Error adding path to bnode"))?;
    Ok(node.into())
}

/// Serializes a list of SHACL paths as an RDF collection, returning the head of the list.
fn path_list_to_rdf<RDF: BuildRDF>(writer: &mut RDF, paths: &[SHACLPath]) -> Result<RDF::Term, ValidationError> {
    let mut rest: RDF::Term = RdfVocab::rdf_nil().into();
    for path in paths.iter().rev() {
        let element = path_to_rdf(writer, path)?;
        let node: RDF::Subject = writer
            .add_bnode()
            .map_err(error_mapper::<RDF>("Error creating bnode for path list"))?
            .into();
        writer
            .add_triple(node.clone(), RdfVocab::rdf_first(), element)
            .map_err(error_mapper::<RDF>("Error adding rdf:first to path list"))?;
        writer
            .add_triple(node.clone(), RdfVocab::rdf_rest(), rest)
            .map_err(error_mapper::<RDF>("Error adding rdf:rest to path list"))?;
        rest = node.into();
    }
    Ok(rest)
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
        self.focus_node == other.focus_node
            && self.constraint_component == other.constraint_component
            && self.severity == other.severity
            && self.path == other.path
            && self.value == other.value
            && self.source == other.source
            && self.details == other.details
    }
}

impl Hash for ValidationResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.focus_node.hash(state);
        self.constraint_component.hash(state);
        self.severity.hash(state);
        self.path.hash(state);
        self.value.hash(state);
        self.source.hash(state);
        self.details.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_rdf::rdf_core::Rdf;
    use rudof_rdf::rdf_impl::OxigraphInMemory;

    fn assert_path_round_trip(path: SHACLPath) {
        let mut graph = OxigraphInMemory::empty();
        let node = graph.add_bnode().unwrap();
        let term = path_to_rdf(&mut graph, &path).unwrap();
        graph
            .add_triple(node.clone(), ShaclVocab::sh_result_path(), term)
            .unwrap();

        let subject: <OxigraphInMemory as Rdf>::Term = node.into();
        let predicate: <OxigraphInMemory as Rdf>::IRI = ShaclVocab::sh_result_path().into();
        let parsed = graph.get_path_for(&subject, &predicate).unwrap().unwrap();
        assert_eq!(parsed, path);
    }

    fn pred(local: &str) -> SHACLPath {
        SHACLPath::iri(IriS::new_unchecked(&format!("http://example.org/{local}")))
    }

    #[test]
    fn path_to_rdf_predicate() {
        assert_path_round_trip(pred("knows"));
    }

    #[test]
    fn path_to_rdf_inverse() {
        assert_path_round_trip(SHACLPath::inverse(pred("knows")));
    }

    #[test]
    fn path_to_rdf_zero_or_more() {
        assert_path_round_trip(SHACLPath::zero_or_more(pred("knows")));
    }

    #[test]
    fn path_to_rdf_one_or_more() {
        assert_path_round_trip(SHACLPath::one_or_more(pred("knows")));
    }

    #[test]
    fn path_to_rdf_zero_or_one() {
        assert_path_round_trip(SHACLPath::zero_or_one(pred("knows")));
    }

    #[test]
    fn path_to_rdf_sequence() {
        assert_path_round_trip(SHACLPath::sequence(vec![pred("knows"), pred("name")]));
    }

    #[test]
    fn path_to_rdf_alternative() {
        assert_path_round_trip(SHACLPath::alternative(vec![pred("knows"), pred("name")]));
    }

    #[test]
    fn path_to_rdf_nested() {
        assert_path_round_trip(SHACLPath::sequence(vec![
            SHACLPath::alternative(vec![pred("knows"), SHACLPath::inverse(pred("name"))]),
            SHACLPath::zero_or_more(pred("friend")),
        ]));
    }
}
