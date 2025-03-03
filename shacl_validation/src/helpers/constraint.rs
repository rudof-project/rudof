use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Object;
use srdf::Rdf;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::IterationStrategy;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

fn apply<R: Rdf, I: IterationStrategy<R>>(
    component: &CompiledComponent<R>,
    shape: &CompiledShape<R>,
    value_nodes: &ValueNodes<R>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> Result<bool, ConstraintError>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            if let Ok(condition) = evaluator(item) {
                if condition {
                    let focus = focus_node.clone().into();
                    let component = Object::iri(component.into());
                    let severity = shape.severity().clone().into();
                    let source = Some(shape.id().clone().into());
                    return Some(
                        ValidationResult::new(focus, component, severity).with_source(source),
                    );
                }
            }
            None
        })
        .collect(); // TODO: could this be removed?

    Ok(results)
}

pub fn validate_with<R: Rdf, I: IterationStrategy<R>>(
    component: &CompiledComponent<R>,
    shape: &CompiledShape<R>,
    value_nodes: &ValueNodes<R>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> bool,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        iteration_strategy,
        |item: &I::Item| Ok(evaluator(item)),
    )
}

pub fn validate_ask_with<S: Sparql>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
    store: &S,
    value_nodes: &ValueNodes<S>,
    eval_query: impl Fn(&S::Term) -> String,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        ValueNodeIteration,
        |value_node| match store.query_ask(&eval_query(value_node)) {
            Ok(ask) => Ok(!ask),
            Err(err) => Err(ConstraintError::Query(format!("ASK query failed: {}", err))),
        },
    )
}
