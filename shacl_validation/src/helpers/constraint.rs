use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDFBasic;

use crate::constraints::constraint_error::ConstraintError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::IterationStrategy;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

fn apply<S: SRDFBasic, I: IterationStrategy<S>>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> Result<bool, ConstraintError>,
) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            if let Ok(condition) = evaluator(item) {
                if condition {
                    return Some(ValidationResult::new(
                        focus_node.to_owned(),
                        None, // TODO: path
                        None, // TODO: item
                        Some(shape.id().to_owned()),
                        S::iri_s2term(&component.into()),
                        None, // TODO: details
                        None, // TODO: message
                        shape.severity(),
                    ));
                }
            }
            None
        })
        .collect();

    Ok(results)
}

pub fn validate_with<S: SRDFBasic, I: IterationStrategy<S>>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> bool,
) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        iteration_strategy,
        |item: &I::Item| Ok(evaluator(item)),
    )
}

pub fn validate_ask_with<S: QuerySRDF>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
    store: &S,
    value_nodes: &ValueNodes<S>,
    eval_query: impl Fn(&S::Term) -> String,
) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
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
