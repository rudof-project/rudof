use shacl_ast::compiled::component::MaxCount;
use srdf::Rdf;

use crate::constraints::Evaluator;
use crate::focus_nodes::FocusNodes;
use crate::value_nodes::FocusNodeIteration;

impl<R: Rdf> Evaluator<R, FocusNodeIteration> for MaxCount {
    fn evaluate(&self, item: FocusNodes<R>) -> bool {
        item.len() > self.max_count()
    }
}

// impl<R: Rdf, E: Engine<R>> Validator<R, E> for MaxCount {
//     fn validate(
//         &self,
//         shape: &CompiledShape<R>,
//         _store: &R,
//         value_nodes: &ValueNodes<R>,
//     ) -> Result<Vec<ValidationResult>, ValidateError> {
//         let max_count = |targets: &FocusNodes<R>| Ok(targets.len() > self.max_count());
//         validate_with(shape, value_nodes, FocusNodeIteration, max_count)
//     }
// }
