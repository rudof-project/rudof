use crate::ast::{ASTSchema, ASTShape};
use crate::ir::dg::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::rdf::ShaclParser;
use crate::rdf::error::ShaclWriterError;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfVocabulary, ShaclVocab, XsdVocab};
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Read};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct IRSchema {
    // imports: Vec<IriS>
    // entailments: Vec<IriS>
    labels_idx_map: HashMap<Object, ShapeLabelIdx>,

    shapes: HashMap<ShapeLabelIdx, IRShape>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
    dependency_graph: DependencyGraph,
    shape_label_counter: usize,
}

impl IRSchema {
    pub fn new(prefixmap: PrefixMap) -> Self {
        Self {
            labels_idx_map: HashMap::new(),
            shapes: HashMap::new(),
            prefixmap,
            base: None,
            dependency_graph: DependencyGraph::new(),
            shape_label_counter: 0,
        }
    }

    pub fn from_reader<R: Read>(
        reader: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<Self, IRError> {
        let mut graph = InMemoryGraph::new();
        graph.merge_from_reader(reader, source_name, format, base, reader_mode)?;
        let ast = ShaclParser::new(graph).parse()?;

        ast.try_into()
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<Self, IRError> {
        Self::from_reader(&mut Cursor::new(data), "String", format, base, reader_mode)
    }

    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    pub fn prefix_map(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }

    pub fn get_shape_from_idx(&self, shape_idx: &ShapeLabelIdx) -> Option<&IRShape> {
        self.shapes.get(shape_idx)
    }

    pub fn get_shape(&self, sref: &Object) -> Option<&IRShape> {
        let idx = self.labels_idx_map.get(sref)?;
        self.shapes.get(idx)
    }

    /// Returns the `ShapeLabelIdx` for the given shape reference `Object`, if it exists.
    pub fn get_idx(&self, sref: &Object) -> Option<&ShapeLabelIdx> {
        self.labels_idx_map.get(sref)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Object, &IRShape)> {
        self.labels_idx_map.iter().map(move |(node, label_idx)| {
            let shape = self.shapes.get(label_idx).unwrap_or_else(|| {
                panic!(
                    "Internal error: Shape label index {label_idx} for node {node} not found in shapes map: {:?}",
                    self.shapes
                )
            });
            (node, shape)
        })
    }

    /// Iterate over all shapes that have at least one target.
    pub fn iter_with_targets(&self) -> impl Iterator<Item = (&Object, &IRShape)> {
        self.iter().filter(|(_, shape)| !shape.targets().is_empty())
    }

    /// Returns the indices of shapes with targets, grouped into topological levels.
    ///
    /// Shapes in the same level have no dependency relationship with each other and
    /// can therefore be validated in parallel. Dependencies are placed at lower
    /// levels than the shapes that reference them, so iterating from level 0 upward
    /// ensures that a shape's sub-shapes are validated (and cached) before the
    /// shape itself runs.
    ///
    /// Shapes that do not appear in the dependency graph at all (isolated shapes
    /// with no `sh:node`/`sh:and`/etc. references) are treated as level-0 shapes.
    pub(crate) fn shapes_with_targets_by_level(&self) -> Vec<Vec<ShapeLabelIdx>> {
        let graph_levels = self.dependency_graph.topological_levels();

        // Track which indices appear in the dependency graph
        let in_graph: HashSet<ShapeLabelIdx> = graph_levels.iter().flatten().copied().collect();

        // Shapes not in the graph are fully independent -> level 0
        let mut level0: Vec<ShapeLabelIdx> = self
            .labels_idx_map
            .values()
            .copied()
            .filter(|idx| !in_graph.contains(idx))
            .filter(|idx| self.shapes.get(idx).is_some_and(|s| !s.targets().is_empty()))
            .collect();
        level0.sort_unstable();

        // Add graph level-0 shapes that have targets
        if let Some(graph_l0) = graph_levels.first() {
            level0.extend(
                graph_l0
                    .iter()
                    .copied()
                    .filter(|idx| self.shapes.get(idx).is_some_and(|s| !s.targets().is_empty())),
            );
        }

        let mut result: Vec<Vec<ShapeLabelIdx>> = Vec::new();
        if !level0.is_empty() {
            result.push(level0);
        }

        // Remaining levels (skip index 0, already handled above)
        for graph_level in graph_levels.iter().skip(1) {
            let level_with_targets: Vec<ShapeLabelIdx> = graph_level
                .iter()
                .copied()
                .filter(|idx| self.shapes.get(idx).is_some_and(|s| !s.targets().is_empty()))
                .collect();
            if !level_with_targets.is_empty() {
                result.push(level_with_targets);
            }
        }

        result
    }
}

impl IRSchema {
    fn get_next_idx(&mut self) -> usize {
        let out = self.shape_label_counter;
        self.shape_label_counter += 1;
        out
    }

    pub fn register_shape(
        &mut self,
        id: &Object,
        shape: Option<&ASTShape>,
        ast: &ASTSchema,
    ) -> Result<ShapeLabelIdx, IRError> {
        let shape = match shape {
            None => ast.get_shape(id).ok_or(IRError::ShapeNotFound {
                shape: Box::new(id.clone()),
            })?,
            Some(shape) => shape,
        };

        match self.labels_idx_map.get(id) {
            None => {
                let label_idx = ShapeLabelIdx::new(self.get_next_idx());
                self.labels_idx_map.insert(id.clone(), label_idx);
                let compiled = IRShape::compile(shape, ast, self)?;
                self.shapes.insert(label_idx, compiled);
                Ok(label_idx)
            },
            Some(idx) => Ok(*idx),
        }
    }

    pub fn register_shapes(&mut self, ids: Vec<Object>, ast: &ASTSchema) -> Result<Vec<ShapeLabelIdx>, IRError> {
        ids.into_iter().map(|id| self.register_shape(&id, None, ast)).collect()
    }

    pub fn compile(ast: &ASTSchema) -> Result<Self, IRError> {
        let mut schema_ir = Self::new(ast.prefixmap().clone()).with_base(ast.base().cloned());

        for (id, shape) in ast.iter() {
            schema_ir.register_shape(id, Some(shape), ast)?;
        }

        schema_ir.build_dependency_graph();

        if schema_ir.dependency_graph.has_cycles() {
            warn!(
                "The dependency graph has cycles. This is known as a recursive schema and the SHACL semantics for these schemas is implementation dependent"
            );
            warn!(
                "More information about recursive schemas can be found at https://www.w3.org/TR/shacl/#shapes-recursion"
            );
        }

        if schema_ir.dependency_graph.has_neg_cycle() {
            warn!(
                "Warning: The dependency graph has negative cycles. This may lead to unexpected behavior in SHACL validation due to non-stratified negation"
            );
        }

        Ok(schema_ir)
    }

    pub(crate) fn build_dependency_graph(&mut self) {
        let mut dg = DependencyGraph::new();
        let mut cache = HashSet::new();

        for (idx, shape) in self.shapes.iter() {
            // Add edges, we start by positive edges, but the direction can change when there is some negation
            shape.add_edges(*idx, &mut dg, PosNeg::Pos, self, &mut cache);
        }

        self.dependency_graph = dg;
    }
}

impl TryFrom<ASTSchema> for IRSchema {
    type Error = IRError;

    fn try_from(value: ASTSchema) -> Result<Self, Self::Error> {
        IRSchema::compile(&value)
    }
}

impl TryFrom<&ASTSchema> for IRSchema {
    type Error = IRError;

    fn try_from(value: &ASTSchema) -> Result<Self, Self::Error> {
        IRSchema::compile(value)
    }
}

impl IRSchema {
    // TODO - Maybe change error type to IRerror
    pub fn build_graph<RDF: BuildRDF>(&self) -> Result<RDF, ShaclWriterError> {
        let mut graph = RDF::empty();

        graph.set_prefix_map(self.prefixmap.clone());
        graph.add_prefix("rdf", RdfVocab::base_iri());
        graph.add_prefix("xsd", XsdVocab::base_iri());
        graph.add_prefix("sh", ShaclVocab::base_iri());

        graph.add_base(&self.base().cloned());

        self.labels_idx_map.iter().try_for_each(|(id, idx)| {
            let shape = self.shapes.get(idx).ok_or(ShaclWriterError::Write {
                msg: format!("Shape with index {idx} not found for id {id}"),
            })?;

            shape
                .register(&mut graph, &self.shapes)
                .map_err(|err| ShaclWriterError::Write {
                    msg: format!("Error registering shape with id {id} and index {idx}: {err}"),
                })
        })?;

        Ok(graph)
    }
}

impl Display for IRSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SHACL shapes graph IR")?;

        for (node, shape) in self.shapes.iter() {
            writeln!(f, "[{node}] -> {shape}")?;
        }
        writeln!(f, "Dependency graph: {}", self.dependency_graph)
    }
}
