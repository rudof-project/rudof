use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::*;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Rdf;

use crate::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::IterationStrategy;
use crate::value_nodes::ValueNodes;

pub mod core;

/// The evaluation of Constraint has three posible working scenarios:
/// 1. The constraint does not raise any error and the expected outcome is a
///    boolean value. That is the case for example of the MinCount constraint,
///    which evaluates to true if the number of value nodes is greater than or
///    equal to the minimum cardinality.
/// 2. The constraint raises an error, but the expected outcome is a boolean
///    value. That may happen for example if the evaluated constraint tries to
///    access the Store, and during the query execution an error is raised.
/// 3. The constraint raises an error, and the expected outcome is a list of
///    validation results.
pub trait Evaluator<R: Rdf, I: IterationStrategy> {
    fn evaluate(&self, item: I::Item<'_, R>) -> bool {
        unimplemented!()
    }

    fn evaluate_with<E: Engine<R>>(
        &self,
        item: I::Item<'_, R>,
        store: &R,
    ) -> Result<bool, ValidateError> {
        Ok(self.evaluate(item))
    }

    // TODO: rename this method
    fn evaluate_validation_results<E: Engine<R>>(
        &self,
        item: I::Item<'_, R>,
        store: &R,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let result = match self.evaluate_with::<E>(item, store)? {
            true => Default::default(),
            false => todo!(), // TODO: create a validation result
        };
        Ok(result)
    }
}

pub trait Validator<R: Rdf, E: Engine<R>> {
    fn validate(
        &self,
        shape: &CompiledShape<R>,
        store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError>;
}

// Implement Validator for CompiledComponent by delegating to the variant's implementation
impl<R: Rdf, E: Engine<R>> Validator<R, E> for CompiledComponent<R>
where
    Class<R>: Validator<R, E>,
    Datatype<R>: Validator<R, E>,
    Nodekind: Validator<R, E>,
    MinCount: Validator<R, E>,
    MaxCount: Validator<R, E>,
    MinExclusive<R>: Validator<R, E>,
    MaxExclusive<R>: Validator<R, E>,
    MinInclusive<R>: Validator<R, E>,
    MaxInclusive<R>: Validator<R, E>,
    MinLength: Validator<R, E>,
    MaxLength: Validator<R, E>,
    Pattern: Validator<R, E>,
    UniqueLang: Validator<R, E>,
    LanguageIn: Validator<R, E>,
    Equals<R>: Validator<R, E>,
    Disjoint<R>: Validator<R, E>,
    LessThan<R>: Validator<R, E>,
    LessThanOrEquals<R>: Validator<R, E>,
    Or<R>: Validator<R, E>,
    And<R>: Validator<R, E>,
    Not<R>: Validator<R, E>,
    Xone<R>: Validator<R, E>,
    Closed<R>: Validator<R, E>,
    Node<R>: Validator<R, E>,
    HasValue<R>: Validator<R, E>,
    In<R>: Validator<R, E>,
    QualifiedValueShape<R>: Validator<R, E>,
{
    fn validate(
        &self,
        shape: &CompiledShape<R>,
        store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        match self {
            CompiledComponent::Class(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Datatype(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::NodeKind(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MinCount(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MaxCount(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MinExclusive(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MaxExclusive(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MinInclusive(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MaxInclusive(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MinLength(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::MaxLength(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Pattern(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::UniqueLang(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::LanguageIn(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Equals(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Disjoint(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::LessThan(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::LessThanOrEquals(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Or(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::And(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Not(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Xone(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Closed(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::Node(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::HasValue(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::In(v) => v.validate(shape, store, value_nodes),
            CompiledComponent::QualifiedValueShape(v) => v.validate(shape, store, value_nodes),
        }
    }
}
