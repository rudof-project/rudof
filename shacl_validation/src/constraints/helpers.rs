use srdf::QuerySRDF;
use srdf::SRDFBasic;

use crate::validation_report::result::ValidationResult;
use crate::value_nodes::IterationStrategy;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

use super::constraint_error::ConstraintError;

fn apply<S: SRDFBasic, T>(
    value_nodes: &ValueNodes<S>,
    iteration_strategy: &dyn IterationStrategy<S, Item = T>,
    evaluator: impl Fn(&T) -> Result<bool, ConstraintError>,
) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            if let Ok(condition) = evaluator(item) {
                if condition {
                    return Some(ValidationResult::new(focus_node.to_owned()));
                }
            }
            None
        })
        .collect();

    Ok(results)
}

pub fn validate_with<S: SRDFBasic, T>(
    value_nodes: &ValueNodes<S>,
    iteration_strategy: &dyn IterationStrategy<S, Item = T>,
    evaluator: impl Fn(&T) -> bool,
) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
    apply(value_nodes, iteration_strategy, |item: &T| {
        Ok(evaluator(item))
    })
}

pub fn validate_ask_with<S: QuerySRDF + 'static>(
    store: &S,
    value_nodes: &ValueNodes<S>,
    eval_query: impl Fn(&S::Term) -> String,
) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
    apply(value_nodes, &ValueNodeIteration, |value_node| {
        match store.query_ask(&eval_query(value_node)) {
            Ok(ask) => Ok(!ask),
            Err(_) => todo!(),
        }
    })
}
