use crate::error::ValidationError;
use crate::ir::components::SubsetOf;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{Validator, apply_with_focus};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationOutcome;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for SubsetOf {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        // The nodes reachable from the focus node through the sh:subsetOf path
        // only depend on the focus node, so they are computed once per focus node
        let other_nodes = RefCell::new(HashMap::new());

        let check_fn = |focus: &S::Term, vn: &S::Term| {
            let mut other_nodes = other_nodes.borrow_mut();
            if !other_nodes.contains_key(focus) {
                let nodes = store.objects_for_shacl_path(focus, self.path())?;
                other_nodes.insert(focus.clone(), nodes);
            }

            // A value node that is not reachable through the path is a violation
            Ok(!other_nodes[focus].contains(vn))
        };

        apply_with_focus(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check_fn,
            &format!("SubsetOf failed. Values are not a subset of path {}", self.path()),
            maybe_path,
        )
    }
}
