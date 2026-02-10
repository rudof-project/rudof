use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Datatype;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use srdf::Literal as _;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use srdf::SLiteral;
use std::fmt::Debug;
use tracing::trace;

impl<R: NeighsRDF + Debug> Validator<R> for Datatype {
    fn validate(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        _: &R,
        _: &mut dyn Engine<R>,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let check = |value_node: &R::Term| {
            trace!("sh:datatype: Checking {value_node} as datatype {}", self.datatype());
            if let Ok(literal) = R::term_as_literal(value_node) {
                match TryInto::<SLiteral>::try_into(literal.clone()) {
                    Ok(SLiteral::WrongDatatype {
                        lexical_form,
                        datatype,
                        error,
                    }) => {
                        trace!(
                            "Wrong datatype for value node: {value_node}. Expected datatype: {datatype}, found: {lexical_form}. Error: {error}"
                        );
                        true
                    },
                    Ok(_slit) => literal.datatype() != self.datatype().as_str(),
                    Err(_) => {
                        trace!("Failed to convert literal to SLiteral: {literal}");
                        true
                    },
                }
            } else {
                true
            }
        };

        let message = format!(
            "Expected datatype: {}",
            shapes_graph.prefix_map().qualify(self.datatype())
        );
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check,
            &message,
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Datatype {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            engine,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Datatype {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}
