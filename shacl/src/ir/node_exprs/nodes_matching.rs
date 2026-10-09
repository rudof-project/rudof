use crate::error::IRError;
use crate::ir::{IRSchema, ShapeLabelIdx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Triple;
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
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        _scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let mut all_nodes: HashSet<Rdf::Term> = HashSet::new();
        for triple in focus_graph
            .triples()
            .map_err(|e| IRError::from_rdf_err::<Rdf>("enumerate triples", e))?
        {
            all_nodes.insert(Rdf::Term::from(triple.subj().clone()));
            all_nodes.insert(triple.obj().clone());
        }

        let targets = FocusNodes::from_iter(all_nodes);
        let shape = ir.get_shape_from_idx_e(&self.shape)?;
        let outcome = shape.validate(focus_graph, runner, Some(&targets), None, ir)?;

        let non_conforming: HashSet<Rdf::Term> = outcome
            .violations()
            .iter()
            .map(|v| Rdf::Term::from(v.focus_node().clone()))
            .collect();

        Ok(targets
            .iter()
            .filter(|n| !non_conforming.contains(*n))
            .cloned()
            .collect())
    }
}

impl Display for NodesMatching {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodesMatching({})", self.shape())
    }
}
