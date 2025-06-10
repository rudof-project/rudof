use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Object;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::IterationStrategy;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

fn apply<S: Rdf, I: IterationStrategy<S>>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
    value_nodes: &ValueNodes<S>,
    iteration_strategy: I,
    evaluator: impl Fn(&I::Item) -> Result<bool, ConstraintError>,
    message: &str,
    maybe_path: Option<SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    let results = iteration_strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            if let Ok(condition) = evaluator(item) {
                if condition {
                    let focus = S::term_as_object(&focus_node)?;
                    let component = Object::iri(S::term_as_object(component)?);
                    let severity = S::term_as_object(&shape.severity())?;
                    let source = 
                       Some(S::term_as_object(shape.id().as_ref()).ok()?);
                    let value = iteration_strategy.to_object(item);
                    return Some(
                        ValidationResult::new(focus, component, severity)
                            .with_source(source)
                            .with_message(message)
                            .with_path(maybe_path.clone())
                            .with_value(value),
                    );
                }
            }
            None
        })
        .collect();

    Ok(results)
}

pub fn validate_with<S: Rdf, I: IterationStrategy<S>>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
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

pub fn validate_ask_with<S: Sparql>(
    component: &CompiledComponent<S>,
    shape: &CompiledShape<S>,
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
            Err(err) => Err(ConstraintError::Query(format!("ASK query failed: {}", err))),
        },
        message,
        maybe_path,
    )
}
