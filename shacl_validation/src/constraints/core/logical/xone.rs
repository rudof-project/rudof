use shacl_ast::compiled::component::Xone;
use srdf::Rdf;

use crate::constraints::Evaluator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::value_nodes::ValueNodeIteration;

impl<R: Rdf> Evaluator<R, ValueNodeIteration> for Xone<R> {
    // TODO: this implementation could be beautified
    fn evaluate_with<E: Engine<R>>(&self, item: R::Term, store: &R) -> Result<bool, ValidateError> {
        let count = self
            .shapes()
            .iter()
            .filter(|&xone_shape| {
                let focus_nodes = FocusNodes::new(std::iter::once(item.clone()));
                match Validate::<R>::validate::<E>(xone_shape, store, Some(&focus_nodes)) {
                    Ok(results) => results.is_empty(),
                    Err(_) => false, // TODO: return an error here
                }
            })
            .count();

        Ok(count != 1usize)
    }
}

// impl<R: Rdf, E: Engine<R>> Validator<R, E> for Xone<R> {
//     fn validate(
//         &self,
//         component: &CompiledComponent<R>,
//         shape: &CompiledShape<R>,
//         store: &R,
//         value_nodes: &ValueNodes<R>,
//     ) -> Result<Vec<ValidationResult>, ValidateError> {
//         let xone = |value_node: &R::Term| {
//             let valid_count = self
//                 .shapes()
//                 .iter()
//                 .filter(|&xone_shape| {
//                     let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
//                     match Validate::<R>::validate::<E>(xone_shape, store, Some(&focus_nodes)) {
//                         Ok(results) => results.is_empty(),
//                         Err(_) => false, // TODO: return an error here
//                     }
//                 })
//                 .count();

//             Ok(valid_count != 1usize)
//         };

//         validate_with(component, shape, value_nodes, ValueNodeIteration, xone)
//     }
// }
