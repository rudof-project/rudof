use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::Datatype;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::Literal as _;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::SLiteral;
use std::fmt::Debug;

impl<R: Rdf + Debug> Validator<R> for Datatype {
    fn validate(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _: &R,
        _: impl Engine<R>,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let check = |value_node: &R::Term| {
            if let Ok(literal) = R::term_as_literal(value_node) {
                match TryInto::<SLiteral>::try_into(literal.clone()) {
                    Ok(SLiteral::WrongDatatypeLiteral {
                        lexical_form,
                        datatype,
                        error,
                    }) => {
                        println!("Wrong datatype for value node: {value_node}. Expected datatype: {datatype}, found: {lexical_form}. Error: {error}");
                        true
                    }
                    Ok(_slit) => literal.datatype() != self.datatype().as_str(),
                    Err(_) => {
                        println!("Failed to convert literal to SLiteral: {literal}");
                        true
                    }
                }
            } else {
                true
            }
        };

        let message = format!(
            "Datatype constraint not satisfied. Expected datatype: {}",
            self.datatype()
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
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Datatype {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}
