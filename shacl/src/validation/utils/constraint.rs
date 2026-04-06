use rudof_rdf::rdf_core::{Rdf, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::{IRComponent, IRShape};
use crate::validation::constraints::ConstraintError;
use crate::validation::iteration::{IterationStrategy, ValueNodeIteration};
use crate::validation::report::ValidationResult;
use crate::validation::value_nodes::ValueNodes;

fn apply<S: Rdf, I: IterationStrategy<S>>(
    component: &IRComponent,
    shape: &IRShape,
    value_nodes: &ValueNodes<S>,
    strategy: I,
    evaluator: impl Fn(&I::Item) -> Result<bool, ConstraintError>,
    msg: &str,
    maybe_path: Option<&SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    let results = strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            let focus = S::term_as_object(focus_node).ok()?;
            let component = Object::iri(component.into());
            let shape_id = shape.id();
            let source = Some(shape_id);
            let value = strategy.to_object(item);
            if let Ok(condition) = evaluator(item) && condition {
                return Some(
                    ValidationResult::new(focus, component, shape.severity())
                        .with_source(source.cloned())
                        .with_message(Some(msg.to_string()))
                        .with_path(maybe_path.cloned())
                        .with_value(value)
                )
            }
            None
        })
        .collect();
    Ok(results)
}

// TODO - Extract common logic with above fn?
fn apply_with_focus<S: Rdf, I: IterationStrategy<S>>(
    component: &IRComponent,
    shape: &IRShape,
    value_nodes: &ValueNodes<S>,
    strategy: I,
    evaluator: impl Fn(&S::Term, &I::Item) -> Result<bool, ConstraintError>,
    msg: &str,
    maybe_path: Option<&SHACLPath>
) -> Result<Vec<ValidationResult>, ConstraintError> {
    let results = strategy
        .iterate(value_nodes)
        .flat_map(|(focus_node, item)| {
            let focus = S::term_as_object(focus_node).ok()?;
            let component = Object::iri(component.into());
            let shape_id = shape.id();
            let source = Some(shape_id);
            let value = strategy.to_object(item);
            match evaluator(focus_node, item) {
                Ok(true) => Some(
                    ValidationResult::new(focus, component, shape.severity())
                        .with_source(source.cloned())
                        .with_message(Some(msg.to_string()))
                        .with_path(maybe_path.cloned())
                        .with_value(value)
                ),
                Ok(false) => None,
                Err(_) => None,
            }
        })
        .collect();

    Ok(results)
}

/// Validate with a boolean evaluator. If the evaluator returns true, it means there is a violation
pub(crate) fn validate_with<S: Rdf, I: IterationStrategy<S>>(
    component: &IRComponent,
    shape: &IRShape,
    value_nodes: &ValueNodes<S>,
    strategy: I,
    evaluator: impl Fn(&I::Item) -> bool,
    msg: &str,
    maybe_path: Option<&SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        strategy,
        |item| Ok(evaluator(item)),
        msg,
        maybe_path
    )
}

/// Validate with a boolean evaluator. If the evaluator returns true, it means that there is a violation
pub(crate) fn validate_with_focus<S: Rdf, I: IterationStrategy<S>>(
    component: &IRComponent,
    shape: &IRShape,
    value_nodes: &ValueNodes<S>,
    strategy: I,
    evaluator: impl Fn(&S::Term, &I::Item) -> bool,
    msg: &str,
    maybe_path: Option<&SHACLPath>
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply_with_focus(
        component,
        shape,
        value_nodes,
        strategy,
        |f, i| Ok(evaluator(f, i)),
        msg,
        maybe_path
    )
}

pub(crate) fn validate_ask_with<S: QueryRDF>(
    component: &IRComponent,
    shape: &IRShape,
    store: &S,
    value_nodes: &ValueNodes<S>,
    eval_query: impl Fn(&S::Term) -> String,
    msg: &str,
    maybe_path: Option<&SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply(
        component,
        shape,
        value_nodes,
        ValueNodeIteration,
        |vn| match store.query_ask(&eval_query(vn)) {
            Ok(ask) => Ok(!ask),
            Err(err) => Err(ConstraintError::Query {
                err: format!("ASK query failed: {err}")
            })
        },
        msg,
        maybe_path
    )
}