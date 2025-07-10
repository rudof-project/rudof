use shacl_ast::compiled::component::Not;
use srdf::Rdf;

use crate::constraints::Evaluator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::value_nodes::ValueNodeIteration;

impl<R: Rdf> Evaluator<R, ValueNodeIteration> for Not<R> {
    // TODO: this implementation could be beautified
    fn evaluate_with<E: Engine<R>>(&self, item: R::Term, store: &R) -> Result<bool, ValidateError> {
        let focus_nodes = FocusNodes::new(std::iter::once(item.clone()));
        match Validate::<R>::validate::<E>(self.shape(), store, Some(&focus_nodes)) {
            Ok(results) => Ok(results.is_empty()),
            Err(error) => Err(error),
        }
    }
}

// impl<R: Rdf, E: Engine<R>> Validator<R, E> for Not<R> {
//     fn validate(
//         &self,
//         component: &CompiledComponent<R>,
//         shape: &CompiledShape<R>,
//         store: &R,
//         value_nodes: &ValueNodes<R>,
//     ) -> Result<Vec<ValidationResult>, ValidateError> {
//         let not = |value_node: &R::Term| {
//             let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
//             match Validate::<R>::validate::<E>(self.shape(), store, Some(&focus_nodes)) {
//                 Ok(results) => Ok(results.is_empty()),
//                 Err(error) => Err(error),
//             }
//         };

//         validate_with(component, shape, value_nodes, ValueNodeIteration, not)
//     }
// }
