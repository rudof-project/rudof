use constraint_error::ConstraintError;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

pub mod constraint_error;
pub mod core;

pub trait Validator<S: Rdf + Debug> {
    #[allow(clippy::too_many_arguments)]
    fn validate(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait NativeValidator<S: NeighsRDF> {
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait SparqlValidator<S: QueryRDF + Debug> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
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
    component: &'a CompiledComponent,
    _marker: PhantomData<S>,
}

impl<'a, S> ShaclComponent<'a, S> {
    pub fn new(component: &'a CompiledComponent) -> Self {
        ShaclComponent {
            component,
            _marker: PhantomData,
        }
    }

    pub fn component(&self) -> &CompiledComponent {
        self.component
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeDeref for ShaclComponent<'_, S> {
    type Target = dyn NativeValidator<S>;

    fn deref(&self) -> &Self::Target {
        match self.component() {
            CompiledComponent::Class(inner) => inner,
            CompiledComponent::Datatype(inner) => inner,
            CompiledComponent::NodeKind(inner) => inner,
            CompiledComponent::MinCount(inner) => inner,
            CompiledComponent::MaxCount(inner) => inner,
            CompiledComponent::MinExclusive(inner) => inner,
            CompiledComponent::MaxExclusive(inner) => inner,
            CompiledComponent::MinInclusive(inner) => inner,
            CompiledComponent::MaxInclusive(inner) => inner,
            CompiledComponent::MinLength(inner) => inner,
            CompiledComponent::MaxLength(inner) => inner,
            CompiledComponent::Pattern(inner) => inner,
            CompiledComponent::UniqueLang(inner) => inner,
            CompiledComponent::LanguageIn(inner) => inner,
            CompiledComponent::Equals(inner) => inner,
            CompiledComponent::Disjoint(inner) => inner,
            CompiledComponent::LessThan(inner) => inner,
            CompiledComponent::LessThanOrEquals(inner) => inner,
            CompiledComponent::Or(inner) => inner,
            CompiledComponent::And(inner) => inner,
            CompiledComponent::Not(inner) => inner,
            CompiledComponent::Xone(inner) => inner,
            CompiledComponent::Node(inner) => inner,
            CompiledComponent::HasValue(inner) => inner,
            CompiledComponent::In(inner) => inner,
            CompiledComponent::QualifiedValueShape(inner) => inner,
        }
    }

    /*generate_deref_fn!(
        CompiledComponent,
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
}

pub trait SparqlDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<S: QueryRDF + Debug + 'static> SparqlDeref for ShaclComponent<'_, S> {
    type Target = dyn SparqlValidator<S>;

    fn deref(&self) -> &Self::Target {
        match self.component() {
            CompiledComponent::Class(inner) => inner,
            CompiledComponent::Datatype(inner) => inner,
            CompiledComponent::NodeKind(inner) => inner,
            CompiledComponent::MinCount(inner) => inner,
            CompiledComponent::MaxCount(inner) => inner,
            CompiledComponent::MinExclusive(inner) => inner,
            CompiledComponent::MaxExclusive(inner) => inner,
            CompiledComponent::MinInclusive(inner) => inner,
            CompiledComponent::MaxInclusive(inner) => inner,
            CompiledComponent::MinLength(inner) => inner,
            CompiledComponent::MaxLength(inner) => inner,
            CompiledComponent::Pattern(inner) => inner,
            CompiledComponent::UniqueLang(inner) => inner,
            CompiledComponent::LanguageIn(inner) => inner,
            CompiledComponent::Equals(inner) => inner,
            CompiledComponent::Disjoint(inner) => inner,
            CompiledComponent::LessThan(inner) => inner,
            CompiledComponent::LessThanOrEquals(inner) => inner,
            CompiledComponent::Or(inner) => inner,
            CompiledComponent::And(inner) => inner,
            CompiledComponent::Not(inner) => inner,
            CompiledComponent::Xone(inner) => inner,
            CompiledComponent::Node(inner) => inner,
            CompiledComponent::HasValue(inner) => inner,
            CompiledComponent::In(inner) => inner,
            CompiledComponent::QualifiedValueShape(inner) => inner,
        }
    }

    /*   generate_deref_fn!(
        CompiledComponent,
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
