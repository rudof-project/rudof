use iri_s::IriS;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;
use srdf::model::Iri;

use crate::constraints::constraint_error::ConstraintError;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::IterationStrategy;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

fn apply<R: Rdf + Clone, I: IterationStrategy<R>>(
    component: &CompiledComponent<R>,
    shape: &CompiledShape<R>,
    value_nodes: &ValueNodes<R>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> Result<bool, ConstraintError>,
    subsetting: &Subsetting,
) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, target)| {
            // we are applying the provided validator to the corresponding item;
            // that is, to the targets. If the evaluation is true, then a result
            // must be raised, it is incorporated to the resulting subset
            // otherwise
            if let Ok(condition) = evaluator(target) {
                // if the condition is met --> Result
                if condition {
                    let result = ValidationResult::new(
                        focus_node.clone(),
                        Predicate::<R>::new(IriS::from(component.clone()).as_str()).into(),
                        Predicate::<R>::new(IriS::from(shape.severity()).as_str()).into(),
                    );
                    return Some(result.with_source(Some(shape.id().clone())));
                }
                // if the condition is not met, the target passes :D
                else if *subsetting != Subsetting::None {
                    // neighborhood(focus_node, target);
                }
            }
            None
        })
        .collect();

    Ok(results)
}

pub fn validate_native_with_strategy<R: Rdf + Clone, I: IterationStrategy<R>>(
    component: &CompiledComponent<R>,
    shape: &CompiledShape<R>,
    value_nodes: &ValueNodes<R>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> bool,
    subsetting: &Subsetting,
) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        iteration_strategy,
        |item: &I::Item| Ok(evaluator(item)),
        subsetting,
    )
}

pub fn validate_sparql_ask<R: Rdf + Sparql + Clone>(
    component: &CompiledComponent<R>,
    shape: &CompiledShape<R>,
    store: &Store<R>,
    value_nodes: &ValueNodes<R>,
    query: impl Fn(&Object<R>) -> String,
    subsetting: &Subsetting,
) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        ValueNodeIteration,
        |value_node| match store.inner_store().ask(&query(value_node)) {
            Ok(ask) => Ok(!ask),
            Err(err) => Err(ConstraintError::Query(format!("ASK query failed: {}", err))),
        },
        subsetting,
    )
}
