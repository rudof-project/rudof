use crate::error::{IRError, NodeExprEvalError};
use crate::ir::{IRSchema, ShapeLabelIdx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::Triple;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct NodesMatching {
    shape: ShapeLabelIdx,
}

impl NodesMatching {
    pub fn new(shape: ShapeLabelIdx) -> Self {
        Self { shape }
    }

    pub fn shape(&self) -> &ShapeLabelIdx {
        &self.shape
    }
}

impl NodesMatching {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        store: &Rdf,
        _focus: &Rdf::Term,
        _scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let mut all_nodes: HashSet<Rdf::Term> = HashSet::new();
        for triple in store
            .triples()
            .map_err(|e| IRError::from_rdf_err::<Rdf>("enumerate triples", e))?
        {
            all_nodes.insert(Rdf::Term::from(triple.subj().clone()));
            all_nodes.insert(triple.obj().clone());
        }

        let targets = FocusNodes::from_iter(all_nodes);
        let shape = ir.get_shape_from_idx_e(&self.shape)?;

        // The nodes that fail the shape do not depend on the focus node, so they are found once per
        // run, and asking for them again while finding them means the shape deactivates itself
        if let Entry::Vacant(entry) = runner.node_expr_state().matching.entry(self.shape) {
            entry.insert(None);
            let outcome = shape.validate(store, runner, Some(&targets), None, ir);
            let matching = &mut runner.node_expr_state().matching;
            match outcome {
                Ok(outcome) => {
                    let non_conforming = outcome.violations().iter().map(|v| v.focus_node().clone()).collect();
                    matching.insert(self.shape, Some(non_conforming))
                },
                // A caller such as sh:not can go on after the error, and then must not see a cycle
                Err(e) => {
                    matching.remove(&self.shape);
                    return Err(e.into());
                },
            };
        }
        let Some(Some(non_conforming)) = runner.node_expr_state().matching.get(&self.shape) else {
            return Err(NodeExprEvalError::CyclicNodesMatching(self.to_string()).into());
        };

        Ok(targets
            .iter()
            .filter(|n| !Rdf::term_as_object(n).is_ok_and(|o| non_conforming.contains(&o)))
            .cloned()
            .collect())
    }
}

impl Display for NodesMatching {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodesMatching({})", self.shape())
    }
}
