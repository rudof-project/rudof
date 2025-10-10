use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::Object;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use tracing::debug;

use crate::constraints::constraint_error::ConstraintError;
use crate::iteration_strategy::IterationStrategy;
use crate::iteration_strategy::ValueNodeIteration;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

fn apply<S: Rdf, I: IterationStrategy<S>>(
    component: &ComponentIR,
    shape: &ShapeIR,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> Result<bool, ConstraintError>,
    message: &str,
    maybe_path: Option<SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            let focus = S::term_as_object(focus_node).ok()?;
            let component = Object::iri(component.into());
            let shape_id = shape.id();
            let source = Some(shape_id);
            let value = iteration_strategy.to_object(item);
            if let Ok(condition) = evaluator(item)
                && condition
            {
                return Some(
                    ValidationResult::new(focus, component, shape.severity())
                        .with_source(source.cloned())
                        .with_message(message)
                        .with_path(maybe_path.clone())
                        .with_value(value),
                );
            }
            None
        })
        .collect();

    Ok(results)
}

fn apply_with_focus<S: Rdf, I: IterationStrategy<S>>(
    component: &ComponentIR,
    shape: &ShapeIR,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&S::Term, &I::Item) -> Result<bool, ConstraintError>,
    message: &str,
    maybe_path: Option<SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            let focus = S::term_as_object(focus_node).ok()?;
            let component = Object::iri(component.into());
            let shape_id = shape.id();
            let source = Some(shape_id);
            let value = iteration_strategy.to_object(item);
            match evaluator(focus_node, item) {
                Ok(true) => Some(
                    ValidationResult::new(focus, component, shape.severity())
                        .with_source(source.cloned())
                        .with_message(message)
                        .with_path(maybe_path.clone())
                        .with_value(value),
                ),
                Ok(false) => None,
                Err(err) => {
                    debug!(
                        "LessThan.validate_native with focus: {:?}, err: {err}",
                        focus
                    );
                    None
                }
            }
        })
        .collect();

    Ok(results)
}

/// Validate with a boolean evaluator. If the evaluator returns true, it means that there is a violation
pub fn validate_with<S: Rdf, I: IterationStrategy<S>>(
    component: &ComponentIR,
    shape: &ShapeIR,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> bool,
    message: &str,
    maybe_path: Option<SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        iteration_strategy,
        |item: &I::Item| Ok(evaluator(item)),
        message,
        maybe_path,
    )
}

/// Validate with a boolean evaluator. If the evaluator returns true, it means that there is a violation
pub fn validate_with_focus<S: Rdf, I: IterationStrategy<S>>(
    component: &ComponentIR,
    shape: &ShapeIR,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&S::Term, &I::Item) -> bool,
    message: &str,
    maybe_path: Option<SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply_with_focus(
        component,
        shape,
        value_nodes,
        iteration_strategy,
        |focus: &S::Term, item: &I::Item| Ok(evaluator(focus, item)),
        message,
        maybe_path,
    )
}

pub fn validate_ask_with<S: QueryRDF>(
    component: &ComponentIR,
    shape: &ShapeIR,
    store: &S,
    value_nodes: &ValueNodes<S>,
    eval_query: impl Fn(&S::Term) -> String,
    message: &str,
    maybe_path: Option<SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        ValueNodeIteration,
        |value_node| match store.query_ask(&eval_query(value_node)) {
            Ok(ask) => Ok(!ask),
            Err(err) => Err(ConstraintError::Query(format!("ASK query failed: {err}"))),
        },
        message,
        maybe_path,
    )
}
