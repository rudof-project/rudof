use crate::ast::shape::ASTShape;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct ASTSchema {
    // imports: Vec<IriS>
    // entailments: Vec<IriS>
    shapes: HashMap<Object, ASTShape>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
}

impl ASTSchema {
    pub fn new() -> Self {
        ASTSchema {
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
            base: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_shapes(mut self, shapes: HashMap<Object, ASTShape>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Object, &ASTShape)> {
        // `self.shapes` is a `HashMap`, whose iteration order is randomized per process.
        // Shape registration order feeds into dependency-graph construction (see
        // `IRSchema::compile`), so we sort by a canonical string form here to make that
        // order reproducible across runs.
        let mut entries: Vec<(&Object, &ASTShape)> = self.shapes.iter().collect();
        entries.sort_by_key(|(a, _)| a.to_string());
        entries.into_iter()
    }

    pub fn get_shape(&self, sref: &Object) -> Option<&ASTShape> {
        self.shapes.get(sref)
    }
}

impl IntoIterator for ASTSchema {
    type Item = (Object, ASTShape);
    type IntoIter = IntoIter<Object, ASTShape>;

    fn into_iter(self) -> Self::IntoIter {
        self.shapes.into_iter()
    }
}

impl Display for ASTSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, shape) in self.shapes.iter() {
            writeln!(f, "{id} -> {shape}")?;
        }
        Ok(())
    }
}
