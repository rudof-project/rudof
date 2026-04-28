mod core;
pub(crate) mod error;
mod test;

use crate::error::ConstraintError;
use crate::ir::components::{
    And, Closed, Datatype, Deactivated, HasValue, In, LanguageIn, MaxCount, MinCount, Node, Not, Or,
    QualifiedValueShape, UniqueLang, Xone,
};
use crate::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use crate::types::MessageMap;
use crate::validator::engine::Engine;
use crate::validator::iteration::{IterationStrategy, ValueNodeIteration};
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath};
use std::fmt::Debug;
use std::marker::PhantomData;

// TODO - Move to crate::validator
pub trait Validator<RDF: NeighsRDF + Debug> {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &RDF,
        engine: &mut dyn Engine<RDF>,
        value_nodes: &ValueNodes<RDF>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}
// TODO - Move to crate::validator
pub trait NativeValidator<RDF: NeighsRDF> {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &RDF,
        engine: &mut dyn Engine<RDF>,
        value_nodes: &ValueNodes<RDF>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}
// TODO - Move to crate::validator
#[cfg(feature = "sparql")]
pub trait SparqlValidator<RDF: QueryRDF + Debug> {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &RDF,
        value_nodes: &ValueNodes<RDF>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

macro_rules! impl_validators_via_validate {
    ($ty:ty) => {
        impl<S> crate::validator::constraints::NativeValidator<S> for $ty
        where
            S: rudof_rdf::rdf_core::NeighsRDF + std::fmt::Debug + 'static,
        {
            fn validate_native(
                &self,
                component: &crate::ir::IRComponent,
                shape: &crate::ir::IRShape,
                store: &S,
                engine: &mut dyn crate::validator::engine::Engine<S>,
                value_nodes: &crate::validator::nodes::ValueNodes<S>,
                source_shape: Option<&crate::ir::IRShape>,
                maybe_path: Option<&rudof_rdf::rdf_core::SHACLPath>,
                shapes_graph: &crate::ir::IRSchema,
            ) -> Result<Vec<crate::validator::report::ValidationResult>, crate::validator::error::ConstraintError> {
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

        #[cfg(feature = "sparql")]
        impl<S> crate::validator::constraints::SparqlValidator<S> for $ty
        where
            S: rudof_rdf::rdf_core::query::QueryRDF + rudof_rdf::rdf_core::NeighsRDF + std::fmt::Debug + 'static,
        {
            fn validate_sparql(
                &self,
                component: &crate::ir::IRComponent,
                shape: &crate::ir::IRShape,
                store: &S,
                value_nodes: &crate::validator::nodes::ValueNodes<S>,
                source_shape: Option<&crate::ir::IRShape>,
                maybe_path: Option<&rudof_rdf::rdf_core::SHACLPath>,
                shapes_graph: &crate::ir::IRSchema,
            ) -> Result<Vec<crate::validator::report::ValidationResult>, crate::validator::error::ConstraintError> {
                self.validate(
                    component,
                    shape,
                    store,
                    &mut crate::validator::engine::SparqlEngine::new(),
                    value_nodes,
                    source_shape,
                    maybe_path,
                    shapes_graph,
                )
            }
        }
    };
}

// TODO - Maybe this can be replaced with blanket implementation
// TODO - If the remaining constraint are equal for native and sparql
impl_validators_via_validate!(MinCount);
impl_validators_via_validate!(MaxCount);
impl_validators_via_validate!(Or);
impl_validators_via_validate!(And);
impl_validators_via_validate!(Not);
impl_validators_via_validate!(Xone);
impl_validators_via_validate!(Deactivated);
impl_validators_via_validate!(Closed);
impl_validators_via_validate!(In);
impl_validators_via_validate!(HasValue);
impl_validators_via_validate!(Node);
impl_validators_via_validate!(QualifiedValueShape);
impl_validators_via_validate!(LanguageIn);
impl_validators_via_validate!(UniqueLang);
impl_validators_via_validate!(Datatype);

// TODO - move to crate::shacl_component
pub(crate) struct ShaclComponent<'a, S> {
    component: &'a IRComponent,
    _marker: PhantomData<S>,
}

// TODO - move to crate::shacl_component
impl<'a, S> ShaclComponent<'a, S> {
    pub fn new(component: &'a IRComponent) -> Self {
        Self {
            component,
            _marker: PhantomData,
        }
    }

    pub fn component(&self) -> &'a IRComponent {
        self.component
    }
}

// TODO - Move to crate::deref
pub(crate) trait ValidatorDeref<'a, V: ?Sized + 'a> {
    fn deref(&self) -> &'a V;
}

impl<'a, S: NeighsRDF + Debug + 'static> ValidatorDeref<'a, dyn NativeValidator<S> + 'a> for ShaclComponent<'a, S> {
    fn deref(&self) -> &'a dyn NativeValidator<S> {
        match self.component() {
            IRComponent::Class(inner) => inner,
            IRComponent::Datatype(inner) => inner,
            IRComponent::NodeKind(inner) => inner,
            IRComponent::MinCount(inner) => inner,
            IRComponent::MaxCount(inner) => inner,
            IRComponent::MinExclusive(inner) => inner,
            IRComponent::MaxExclusive(inner) => inner,
            IRComponent::MinInclusive(inner) => inner,
            IRComponent::MaxInclusive(inner) => inner,
            IRComponent::MinLength(inner) => inner,
            IRComponent::MaxLength(inner) => inner,
            IRComponent::Pattern(inner) => inner,
            IRComponent::UniqueLang(inner) => inner,
            IRComponent::LanguageIn(inner) => inner,
            IRComponent::Equals(inner) => inner,
            IRComponent::Disjoint(inner) => inner,
            IRComponent::LessThan(inner) => inner,
            IRComponent::LessThanOrEquals(inner) => inner,
            IRComponent::Or(inner) => inner,
            IRComponent::And(inner) => inner,
            IRComponent::Not(inner) => inner,
            IRComponent::Xone(inner) => inner,
            IRComponent::Node(inner) => inner,
            IRComponent::HasValue(inner) => inner,
            IRComponent::In(inner) => inner,
            IRComponent::QualifiedValueShape(inner) => inner,
            IRComponent::Closed(inner) => inner,
            IRComponent::Deactivated(inner) => inner,
        }
    }
}

#[cfg(feature = "sparql")]
impl<'a, S: QueryRDF + NeighsRDF + Debug + 'static> ValidatorDeref<'a, dyn SparqlValidator<S> + 'a>
    for ShaclComponent<'a, S>
{
    fn deref(&self) -> &'a dyn SparqlValidator<S> {
        match self.component() {
            IRComponent::Class(inner) => inner,
            IRComponent::Datatype(inner) => inner,
            IRComponent::NodeKind(inner) => inner,
            IRComponent::MinCount(inner) => inner,
            IRComponent::MaxCount(inner) => inner,
            IRComponent::MinExclusive(inner) => inner,
            IRComponent::MaxExclusive(inner) => inner,
            IRComponent::MinInclusive(inner) => inner,
            IRComponent::MaxInclusive(inner) => inner,
            IRComponent::MinLength(inner) => inner,
            IRComponent::MaxLength(inner) => inner,
            IRComponent::Pattern(inner) => inner,
            IRComponent::UniqueLang(inner) => inner,
            IRComponent::LanguageIn(inner) => inner,
            IRComponent::Equals(inner) => inner,
            IRComponent::Disjoint(inner) => inner,
            IRComponent::LessThan(inner) => inner,
            IRComponent::LessThanOrEquals(inner) => inner,
            IRComponent::Or(inner) => inner,
            IRComponent::And(inner) => inner,
            IRComponent::Not(inner) => inner,
            IRComponent::Xone(inner) => inner,
            IRComponent::Node(inner) => inner,
            IRComponent::HasValue(inner) => inner,
            IRComponent::In(inner) => inner,
            IRComponent::QualifiedValueShape(inner) => inner,
            IRComponent::Closed(inner) => inner,
            IRComponent::Deactivated(inner) => inner,
        }
    }
}

pub(crate) fn get_shape_from_idx<'a>(
    shapes_graph: &'a IRSchema,
    shape_idx: &'a ShapeLabelIdx,
) -> Result<&'a IRShape, ConstraintError> {
    shapes_graph
        .get_shape_from_idx(shape_idx)
        .ok_or_else(|| ConstraintError::Internal {
            err: format!("Shape idx {} not found in shapes graph", shape_idx),
        })
}

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
            let mut msg = MessageMap::from(msg);
            if let Some(m) = shape.message() {
                msg = msg.merge(m.to_owned(), true);
            }
            if let Ok(condition) = evaluator(item)
                && condition
            {
                return Some(
                    ValidationResult::new(focus, component, shape.severity())
                        .with_source(source.cloned())
                        .with_message(msg)
                        .with_path(maybe_path.cloned())
                        .with_value(value),
                );
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
            match evaluator(focus_node, item) {
                Ok(true) => Some(
                    ValidationResult::new(focus, component, shape.severity())
                        .with_source(source.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_path(maybe_path.cloned())
                        .with_value(value),
                ),
                Ok(false) => None,
                Err(_) => None,
            }
        })
        .collect();

    Ok(results)
}

/// Validate with a boolean evaluator. If the evaluator returns true, it means there is a violation
fn validate_with<S: Rdf, I: IterationStrategy<S>>(
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
        maybe_path,
    )
}

/// Validate with a boolean evaluator. If the evaluator returns true, it means that there is a violation
fn validate_with_focus<S: Rdf, I: IterationStrategy<S>>(
    component: &IRComponent,
    shape: &IRShape,
    value_nodes: &ValueNodes<S>,
    strategy: I,
    evaluator: impl Fn(&S::Term, &I::Item) -> bool,
    msg: &str,
    maybe_path: Option<&SHACLPath>,
) -> Result<Vec<ValidationResult>, ConstraintError> {
    apply_with_focus(
        component,
        shape,
        value_nodes,
        strategy,
        |f, i| Ok(evaluator(f, i)),
        msg,
        maybe_path,
    )
}

fn validate_ask_with<S: QueryRDF>(
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
                err: format!("ASK query failed: {err}"),
            }),
        },
        msg,
        maybe_path,
    )
}
