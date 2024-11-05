use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;

use crate::constraints::constraint_error::ConstraintError;
use crate::store::Store;
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
) -> Result<Vec<ValidationResult>, ConstraintError> {
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
                    let focus = S::term_as_object(focus_node);
                    let component = RDFNode::iri(component.into());
                    let severity = S::term_as_object(&shape.severity());
                    let source = Some(S::term_as_object(&shape.id().to_owned()));
                    return Some(
                        ValidationResult::new(focus, component, severity).with_source(source),
                    );
                }
                // if the condition is not met, the target passes :D
                else if !condition {
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
) -> Result<Vec<ValidationResult>, ConstraintError> {
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
    store: &Store<S>,
    value_nodes: &ValueNodes<S>,
    eval_query: impl Fn(&S::Term) -> String,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        ValueNodeIteration,
        |value_node| match store.inner_store().query_ask(&eval_query(value_node)) {
            Ok(ask) => Ok(!ask),
            Err(err) => Err(ConstraintError::Query(format!("ASK query failed: {}", err))),
        },
    )
}
