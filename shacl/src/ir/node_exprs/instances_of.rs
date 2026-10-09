use crate::error::IRError;
use crate::ir::IRSchema;
use crate::validator::engine::Engine;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct InstancesOf {
    iri: IriS,
}

impl InstancesOf {
    pub fn new(iri: IriS) -> Self {
        Self { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

impl InstancesOf {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        _: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let cls: Rdf::Term = self.iri.clone().into();

        let instances = focus_graph
            .shacl_instances_of(&cls)
            .map_err(|e| IRError::from_rdf_err::<Rdf>("compute SHACL instances", e))?;

        Ok(instances.map(|subj| Rdf::Term::from(subj)).collect())
    }
}

impl Display for InstancesOf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstancesOf({})", self.iri())
    }
}
