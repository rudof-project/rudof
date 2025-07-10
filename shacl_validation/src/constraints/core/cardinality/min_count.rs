use shacl_ast::compiled::component::MinCount;
use srdf::Rdf;

use crate::constraints::Evaluator;
use crate::focus_nodes::FocusNodes;
use crate::value_nodes::FocusNodeIteration;

impl<R: Rdf> Evaluator<R, FocusNodeIteration> for MinCount {
    fn evaluate(&self, item: FocusNodes<R>) -> bool {
        if self.min_count() == 0 {
            return true;
        }
        item.len() < self.min_count()
    }
}

// impl<R: Rdf, E: Engine<R>> Validator<R, E> for MinCount {
//     fn validate(
//         &self,
//         component: &CompiledComponent<R>,
//         shape: &CompiledShape<R>,
//         _store: &R,
//         value_nodes: &ValueNodes<R>,
//     ) -> Result<Vec<ValidationResult>, ValidateError> {
//         // If min_count is 0, then it always passes
//         if self.min_count() == 0 {
//             return Ok(Default::default());
//         }

//         let min_count = |targets: &FocusNodes<R>| Ok(targets.len() < self.min_count());
//         validate_with(component, shape, value_nodes, FocusNodeIteration, min_count)
//     }
// }
