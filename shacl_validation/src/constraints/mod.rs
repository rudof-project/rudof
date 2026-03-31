use constraint_error::ConstraintError;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF};
use shacl::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::shacl_engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

pub mod constraint_error;
pub mod core;

pub trait Validator<S: NeighsRDF + Debug> {
    #[allow(clippy::too_many_arguments)]
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait NativeValidator<S: NeighsRDF> {
    #[allow(clippy::too_many_arguments)]
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait SparqlValidator<S: QueryRDF + Debug> {
    #[allow(clippy::too_many_arguments)]
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

/*
macro_rules! generate_deref_fn {
    ($enum_name:ident, $($variant:ident),+) => {
        fn deref(&self) -> &Self::Target {
            match self.component() {
                $( $enum_name::$variant(inner) => inner, )+
            }
        }
    };
}*/

pub trait NativeDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

pub struct ShaclComponent<'a, S> {
    component: &'a IRComponent,
    _marker: PhantomData<S>,
}

impl<'a, S> ShaclComponent<'a, S> {
    pub fn new(component: &'a IRComponent) -> Self {
        ShaclComponent {
            component,
            _marker: PhantomData,
        }
    }

    pub fn component(&self) -> &IRComponent {
        self.component
    }
}

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

/*generate_deref_fn!(
    ComponentIR,
    Class,
    Datatype,
    NodeKind,
    MinCount,
    MaxCount,
    MinExclusive,
    MaxExclusive,
    MinInclusive,
    MaxInclusive,
    MinLength,
    MaxLength,
    Pattern,
    UniqueLang,
    LanguageIn,
    Equals,
    Disjoint,
    LessThan,
    LessThanOrEquals,
    Or,
    And,
    Not,
    Xone,
    Closed,
    Node,
    HasValue,
    In,
    QualifiedValueShape
);*/

pub trait SparqlDeref {
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

    /*   generate_deref_fn!(
        ComponentIR,
        Class,
        Datatype,
        NodeKind,
        MinCount,
        MaxCount,
        MinExclusive,
        MaxExclusive,
        MinInclusive,
        MaxInclusive,
        MinLength,
        MaxLength,
        Pattern,
        UniqueLang,
        LanguageIn,
        Equals,
        Disjoint,
        LessThan,
        LessThanOrEquals,
        Or,
        And,
        Not,
        Xone,
        Closed,
        Node,
        HasValue,
        In,
        QualifiedValueShape
    ); */
}

pub fn get_shape_from_idx(shapes_graph: &IRSchema, shape_idx: &ShapeLabelIdx) -> Result<IRShape, ConstraintError> {
    shapes_graph
        .get_shape_from_idx(shape_idx)
        .ok_or_else(|| ConstraintError::InternalError {
            msg: format!("Shape idx {} not found in shapes graph", shape_idx),
        })
        .cloned()
}
