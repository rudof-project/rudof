use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::MinExclusive;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MinExclusive {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        _store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_exclusive = |node: &S::Term| match S::term_as_sliteral(node) {
            Ok(lit) => lit
                .partial_cmp(self.min_exclusive())
                .map(|o| o.is_le())
                .unwrap_or(true),
            Err(_) => true,
        };
        let message = format!("MinExclusive({}) not satisfied", self.min_exclusive());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            min_exclusive,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MinExclusive {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_exclusive_value = self.min_exclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} < {}) }} ",
                value_node, min_exclusive_value
            }
        };

        let message = format!("MinExclusive({}) not satisfied", self.min_exclusive());
        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query,
            &message,
            maybe_path,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::shacl_processor::{RdfDataValidation, ShaclValidationMode};

    use crate::shacl_processor::ShaclProcessor;
    use shacl_rdf::parse_shacl_rdf;
    use sparql_service::RdfData;
    use srdf::{RDFFormat, ReaderMode};

    #[test]
    fn test_min_exclusive_native() {
        let graph = r#"
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:MinInclusive a sh:NodeShape ;
  sh:targetClass :Node ;
  sh:property [
    sh:path :p ;
    sh:datatype xsd:double ;
    sh:minInclusive "0.0"^^xsd:double ;
    sh:minCount 1
 ] .

:ok1 a :Node; :p "0"^^xsd:double .
:ok2 a :Node; :p "10.5"^^xsd:double .
:ko1 a :Node; :p "-5.3"^^xsd:double .
:ko2 a :Node; :p "other" .
:ko3 a :Node; :p "other"^^xsd:double .
"#;
        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let validator = RdfDataValidation::from_rdf_data(rdf.clone(), ShaclValidationMode::Native);
        let schema = parse_shacl_rdf(rdf).unwrap();
        let schema_ir = schema.try_into().unwrap();
        let report = validator.validate(&schema_ir).unwrap();
        if report.results().len() != 5 {
            println!("Report results should be 5:\n{report}");
        }
        assert_eq!(report.results().len(), 5);
    }
}
