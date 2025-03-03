use constraint_error::ConstraintError;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

pub mod constraint_error;
pub mod core;

pub trait Validator<Q: Query, E: Engine<Q>> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait SparqlValidator<S: Sparql + Query>: Validator<S, SparqlEngine> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, value_nodes)
    }
}

macro_rules! generate_deref_fn {
    ($enum_name:ident, $($variant:ident),+) => {
        fn deref(&'a self) -> &'a Self::Target {
            match self {
                $( $enum_name::$variant(inner) => inner, )+
            }
        }
    };
}

pub trait NativeDeref<'a> {
    type Target: ?Sized;

    fn deref(&'a self) -> &'a Self::Target;
}

impl<'a, Q: Query> NativeDeref<'a> for CompiledComponent<Q> {
    type Target = dyn Validator<Q, NativeEngine> + 'a;

    generate_deref_fn!(
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
    );
}

pub trait SparqlDeref<'a> {
    type Target: ?Sized;

    fn deref(&'a self) -> &'a Self::Target;
}

impl<'a, S: Sparql + Query> SparqlDeref<'a> for CompiledComponent<S> {
    type Target = dyn SparqlValidator<S> + 'a;

    generate_deref_fn!(
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
    );
}
