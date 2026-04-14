mod error;
mod core;

use std::fmt::Debug;
use std::marker::PhantomData;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use crate::ir::components::{And, Closed, Datatype, Deactivated, HasValue, In, LanguageIn, MaxCount, MinCount, Node, Not, Or, QualifiedValueShape, UniqueLang, Xone};
pub(crate) use crate::validation::constraints::error::ConstraintError;
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::report::ValidationResult;
use crate::validation::value_nodes::ValueNodes;

// TODO - Move to crate::validator
pub(crate) trait Validator<RDF: NeighsRDF + Debug> {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &RDF, engine: &mut dyn Engine<RDF>, value_nodes: &ValueNodes<RDF>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError>;
}
// TODO - Move to crate::validator
pub(crate) trait NativeValidator<RDF: NeighsRDF> {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &RDF, engine: &mut dyn Engine<RDF>, value_nodes: &ValueNodes<RDF>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError>;
}
// TODO - Move to crate::validator
pub(crate) trait SparqlValidator<RDF: QueryRDF + Debug> {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &RDF, value_nodes: &ValueNodes<RDF>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError>;
}

macro_rules! impl_validators_via_validate {
    ($ty:ty) => {
        impl<S> crate::validation::constraints::NativeValidator<S> for $ty
        where
            S: rudof_rdf::rdf_core::NeighsRDF + std::fmt::Debug + 'static
        {
            fn validate_native(
                &self,
                component: &crate::ir::IRComponent,
                shape: &crate::ir::IRShape,
                store: &S,
                engine: &mut dyn crate::validation::engine::Engine<S>,
                value_nodes: &crate::validation::value_nodes::ValueNodes<S>,
                source_shape: Option<&crate::ir::IRShape>,
                maybe_path: Option<&rudof_rdf::rdf_core::SHACLPath>,
                shapes_graph: &crate::ir::IRSchema
            ) -> Result<Vec<crate::validation::report::ValidationResult>, crate::validation::constraints::error::ConstraintError> {
                self.validate(
                    component,
                    shape,
                    store,
                    engine,
                    value_nodes,
                    source_shape,
                    maybe_path,
                    shapes_graph
                )
            }
        }

        impl<S> crate::validation::constraints::SparqlValidator<S> for $ty
        where
            S: rudof_rdf::rdf_core::query::QueryRDF +
                rudof_rdf::rdf_core::NeighsRDF +
                std::fmt::Debug +
                'static
        {
            fn validate_sparql(
                &self,
                component: &crate::ir::IRComponent,
                shape: &crate::ir::IRShape,
                store: &S,
                value_nodes: &crate::validation::value_nodes::ValueNodes<S>,
                source_shape: Option<&crate::ir::IRShape>,
                maybe_path: Option<&rudof_rdf::rdf_core::SHACLPath>,
                shapes_graph: &crate::ir::IRSchema
            ) -> Result<Vec<crate::validation::report::ValidationResult>, crate::validation::constraints::error::ConstraintError> {
                self.validate(
                    component,
                    shape,
                    store,
                    &mut crate::validation::engine::SparqlEngine::new(),
                    value_nodes,
                    source_shape,
                    maybe_path,
                    shapes_graph
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

// TODO - Move to crate::deref
pub(crate) trait NativeDeref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

// TODO - move to crate::shacl_component
pub(crate) struct ShaclComponent<'a, S> {
    component: &'a IRComponent,
    _marker: PhantomData<S>
}

// TODO - move to crate::shacl_component
impl<'a, S> ShaclComponent<'a, S> {
    pub fn new(component: &'a IRComponent) -> Self {
        Self {
            component,
            _marker: PhantomData
        }
    }

    pub fn component(&self) -> &IRComponent {
        self.component
    }
}

// TODO - move to crate::shacl_component
impl<S: NeighsRDF + Debug + 'static> NativeDeref for ShaclComponent<'_, S> {
    type Target = dyn NativeValidator<S>;

    fn deref(&self) -> &Self::Target {
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

// TODO - Move to crate::deref
pub(crate) trait SparqlDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlDeref for ShaclComponent<'_, S> {
    type Target = dyn SparqlValidator<S>;

    fn deref(&self) -> &Self::Target {
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

pub(crate) fn get_shape_from_idx(shapes_graph: &IRSchema, shape_idx: &ShapeLabelIdx) -> Result<IRShape, ConstraintError> {
    shapes_graph
        .get_shape_from_idx(shape_idx)
        .ok_or_else(|| ConstraintError::Internal {
            err: format!("Shape idx {} not found in shapes graph", shape_idx),
        })
        .cloned()
}
