use constraint_error::ConstraintError;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
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
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait NativeValidator<S: NeighsRDF> {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait SparqlValidator<S: QueryRDF + Debug> {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
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
    component: &'a ComponentIR,
    _marker: PhantomData<S>,
}

impl<'a, S> ShaclComponent<'a, S> {
    pub fn new(component: &'a ComponentIR) -> Self {
        ShaclComponent {
            component,
            _marker: PhantomData,
        }
    }

    pub fn component(&self) -> &ComponentIR {
        self.component
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeDeref for ShaclComponent<'_, S> {
    type Target = dyn NativeValidator<S>;

    fn deref(&self) -> &Self::Target {
        match self.component() {
            ComponentIR::Class(inner) => inner,
            ComponentIR::Datatype(inner) => inner,
            ComponentIR::NodeKind(inner) => inner,
            ComponentIR::MinCount(inner) => inner,
            ComponentIR::MaxCount(inner) => inner,
            ComponentIR::MinExclusive(inner) => inner,
            ComponentIR::MaxExclusive(inner) => inner,
            ComponentIR::MinInclusive(inner) => inner,
            ComponentIR::MaxInclusive(inner) => inner,
            ComponentIR::MinLength(inner) => inner,
            ComponentIR::MaxLength(inner) => inner,
            ComponentIR::Pattern(inner) => inner,
            ComponentIR::UniqueLang(inner) => inner,
            ComponentIR::LanguageIn(inner) => inner,
            ComponentIR::Equals(inner) => inner,
            ComponentIR::Disjoint(inner) => inner,
            ComponentIR::LessThan(inner) => inner,
            ComponentIR::LessThanOrEquals(inner) => inner,
            ComponentIR::Or(inner) => inner,
            ComponentIR::And(inner) => inner,
            ComponentIR::Not(inner) => inner,
            ComponentIR::Xone(inner) => inner,
            ComponentIR::Node(inner) => inner,
            ComponentIR::HasValue(inner) => inner,
            ComponentIR::In(inner) => inner,
            ComponentIR::QualifiedValueShape(inner) => inner,
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
            ComponentIR::Class(inner) => inner,
            ComponentIR::Datatype(inner) => inner,
            ComponentIR::NodeKind(inner) => inner,
            ComponentIR::MinCount(inner) => inner,
            ComponentIR::MaxCount(inner) => inner,
            ComponentIR::MinExclusive(inner) => inner,
            ComponentIR::MaxExclusive(inner) => inner,
            ComponentIR::MinInclusive(inner) => inner,
            ComponentIR::MaxInclusive(inner) => inner,
            ComponentIR::MinLength(inner) => inner,
            ComponentIR::MaxLength(inner) => inner,
            ComponentIR::Pattern(inner) => inner,
            ComponentIR::UniqueLang(inner) => inner,
            ComponentIR::LanguageIn(inner) => inner,
            ComponentIR::Equals(inner) => inner,
            ComponentIR::Disjoint(inner) => inner,
            ComponentIR::LessThan(inner) => inner,
            ComponentIR::LessThanOrEquals(inner) => inner,
            ComponentIR::Or(inner) => inner,
            ComponentIR::And(inner) => inner,
            ComponentIR::Not(inner) => inner,
            ComponentIR::Xone(inner) => inner,
            ComponentIR::Node(inner) => inner,
            ComponentIR::HasValue(inner) => inner,
            ComponentIR::In(inner) => inner,
            ComponentIR::QualifiedValueShape(inner) => inner,
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
