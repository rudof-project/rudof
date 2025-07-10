use shacl_ast::compiled::component::Or;
use srdf::Rdf;

use crate::constraints::Evaluator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::value_nodes::ValueNodeIteration;

impl<R: Rdf> Evaluator<R, ValueNodeIteration> for Or<R> {
    // TODO: this implementation could be beautified
    fn evaluate_with<E: Engine<R>>(&self, item: R::Term, store: &R) -> Result<bool, ValidateError> {
        let result = self.shapes().iter().any(|or_shape| {
            let focus_nodes = FocusNodes::new(std::iter::once(item.clone()));
            match Validate::<R>::validate::<E>(or_shape, store, Some(&focus_nodes)) {
                Ok(validation_results) => validation_results.is_empty(),
                Err(_) => false, // TODO: return an error here
            }
        });
        Ok(result)
    }
}

// impl<R: Rdf, E: Engine<R>> Validator<R, E> for Or<R> {
//     fn validate(
//         &self,
//         component: &CompiledComponent<R>,
//         shape: &CompiledShape<R>,
//         store: &R,
//         value_nodes: &ValueNodes<R>,
//     ) -> Result<Vec<ValidationResult>, ValidateError> {
//         let or = |value_node: &R::Term| {
//             let is_some_valid = self.shapes().iter().any(|or_shape| {
//                 let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
//                 match Validate::<R>::validate::<E>(or_shape, store, Some(&focus_nodes)) {
//                     Ok(validation_results) => validation_results.is_empty(),
//                     Err(_) => false, // TODO: return an error here
//                 }
//             });
//             Ok(!is_some_valid)
//         };

//         validate_with(component, shape, value_nodes, ValueNodeIteration, or)
//     }
// }
