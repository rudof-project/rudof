use constraint_error::ConstraintError;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

pub mod constraint_error;
pub mod core;

pub trait Validator<S: SRDFBasic> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait NativeValidator<S: SRDF> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

pub trait SparqlValidator<S: QuerySRDF> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError>;
}

macro_rules! generate_deref_fn {
    ($enum_name:ident, $($variant:ident),+) => {
        fn deref(&self) -> &Self::Target {
            match self {
                $( $enum_name::$variant(inner) => inner, )+
            }
        }
    };
}

pub trait NativeDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<S: SRDF + 'static> NativeDeref for CompiledComponent<S> {
    type Target = dyn NativeValidator<S>;

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

pub trait SparqlDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<S: QuerySRDF + 'static> SparqlDeref for CompiledComponent<S> {
    type Target = dyn SparqlValidator<S>;

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
