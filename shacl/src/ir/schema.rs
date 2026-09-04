use crate::ast::{ASTSchema, ASTShape};
use crate::error::ASTError;
use crate::ir::RecursionSemantics;
use crate::ir::dg::{DependencyGraph, PosNeg, ShapeRecursionKind};
use crate::ir::error::IRError;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::rdf::ShaclParser;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfVocabulary, ShaclVocab, XsdVocab};
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
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
    // This map is used to get the label from the index, but it is not used in the IRSchema itself. It is used in the IRShape to get the label of a shape from its index.
    idx_labels_map: HashMap<ShapeLabelIdx, Object>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
    dependency_graph: DependencyGraph,
    shape_label_counter: usize,
}

impl IRSchema {
    pub fn new(prefixmap: PrefixMap) -> Self {
        Self {
            labels_idx_map: HashMap::new(),
            idx_labels_map: HashMap::new(),
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
        let mut graph = OxigraphInMemory::new();
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

    pub fn get_shape_from_idx_e(&self, shape_idx: &ShapeLabelIdx) -> Result<&IRShape, IRError> {
        self.get_shape_from_idx(shape_idx)
            .ok_or(IRError::ShapeNotFound(*shape_idx))
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

    /// Classifies every shape by how it participates in recursion (not
    /// recursive, positive recursive, stratified recursive, or
    /// non-stratified recursive). See [`ShapeRecursionKind`].
    ///
    /// A schema that compiled successfully never contains a
    /// non-stratified shape — [`Self::compile_with_recursion`] rejects
    /// those outright — but the classification is still exposed here so
    /// callers (e.g. the `shacl` command) can report each shape's kind.
    pub fn recursion_kinds(&self) -> HashMap<&Object, ShapeRecursionKind> {
        self.dependency_graph
            .shape_recursion_kinds()
            .into_iter()
            .filter_map(|(idx, kind)| self.idx_labels_map.get(&idx).map(|label| (label, kind)))
            .collect()
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
            None => ast.get_shape(id).ok_or::<ASTError>(id.clone().into())?,
            Some(shape) => shape,
        };

        match self.labels_idx_map.get(id) {
            None => {
                let label_idx = ShapeLabelIdx::new(self.get_next_idx());
                self.labels_idx_map.insert(id.clone(), label_idx);
                self.idx_labels_map.insert(label_idx, id.clone());
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

    /// Compiles with [`RecursionSemantics::default`] — i.e. a shapes graph
    /// with a cyclic shape reference is rejected. Use
    /// [`Self::compile_with_recursion`] to opt into accepting one.
    pub fn compile(ast: &ASTSchema) -> Result<Self, IRError> {
        Self::compile_with_recursion(ast, RecursionSemantics::default())
    }

    /// Compiles a shapes graph, controlling whether a cyclic shape
    /// reference is accepted and, if so, how it's resolved at validation
    /// time. See [`RecursionSemantics`].
    pub fn compile_with_recursion(ast: &ASTSchema, recursion_semantics: RecursionSemantics) -> Result<Self, IRError> {
        let mut schema_ir = Self::new(ast.prefixmap().clone()).with_base(ast.base().cloned());

        for (id, shape) in ast.iter() {
            schema_ir.register_shape(id, Some(shape), ast)?;
        }

        schema_ir.build_dependency_graph();

        if schema_ir.dependency_graph.has_cycles() {
            warn!(
                "The dependency graph has cycles. This is known as a recursive schema and the SHACL semantics for these schemas is implementation dependent"
            );

            // A cycle is safe to accept (subject to `recursion_semantics`)
            // as long as it's not `NonStratified`: either it's built purely
            // from monotonic constraints, or every negating constraint it
            // carries targets a shape that doesn't itself depend on any
            // recursion — see `ShapeRecursionKind` for the full condition.
            let is_stratified = schema_ir.dependency_graph.is_stratified();

            if !is_stratified || !recursion_semantics.allows_recursion() {
                let cycles: Vec<Vec<Object>> = schema_ir
                    .dependency_graph
                    .cycles()
                    .into_iter()
                    .map(|cycle| {
                        cycle
                            .into_iter()
                            .map(|idx| {
                                schema_ir.idx_labels_map.get(&idx).cloned().unwrap_or_else(|| {
                                    panic!(
                                        "Internal error: Shape label index {idx} not found in idx_labels_map: {:?}",
                                        schema_ir.idx_labels_map
                                    )
                                })
                            })
                            .collect()
                    })
                    .collect();
                if !is_stratified {
                    warn!(
                        "Warning: The dependency graph has non-stratified negation: a negating constraint reaches back into a recursive shape. This may lead to unexpected behavior in SHACL validation"
                    );
                    return Err(IRError::DependencyGraphHasNegativeCycles { cycles });
                }
                warn!(
                    "Recursive shapes are disabled (recursion_semantics = none): rejecting the cyclic schema. Set recursion_semantics to \"cautious\" or \"brave\" to allow it."
                );
                return Err(IRError::DependencyGraphHasCycles { cycles });
            }
            warn!(
                "The cycle is stratified (any negation in it targets recursion-free shapes): validation will handle it via the configured recursion semantics."
            );
        }

        Ok(schema_ir)
    }

    pub(crate) fn build_dependency_graph(&mut self) {
        let mut dg = DependencyGraph::new();
        let mut cache = HashSet::new();

        // `self.shapes` is a `HashMap`, whose iteration order is randomized per process.
        // The graph-building traversal below is order-sensitive (a shape's edges can be
        // (re)visited with different polarity depending on which caller reaches it first),
        // so we iterate over a deterministic, sorted order of shape indices to make the
        // resulting dependency graph reproducible across runs.
        let mut indices: Vec<ShapeLabelIdx> = self.shapes.keys().copied().collect();
        indices.sort_unstable();

        for idx in indices {
            // Add edges, we start by positive edges, but the direction can change when there is some negation
            let shape = self.shapes.get(&idx).expect("index collected from self.shapes.keys()");
            shape.add_edges(idx, &mut dg, PosNeg::Pos, self, &mut cache);
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
    pub fn build_graph<RDF: BuildRDF>(&self) -> Result<RDF, IRError> {
        let mut graph = RDF::empty();

        graph.set_prefix_map(self.prefixmap.clone());
        graph.add_prefix("rdf", RdfVocab::base_iri());
        graph.add_prefix("xsd", XsdVocab::base_iri());
        graph.add_prefix("sh", ShaclVocab::base_iri());

        graph.add_base(&self.base().cloned());

        self.labels_idx_map.iter().try_for_each(|(_, idx)| {
            let shape = self.shapes.get(idx).ok_or(IRError::ShapeNotFound(*idx))?;

            shape.register(&mut graph, &self.shapes)
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
        writeln!(f, "Dependency graph: {}", self.dependency_graph)?;

        let mut recursive_shapes: Vec<(&Object, ShapeRecursionKind)> = self
            .recursion_kinds()
            .into_iter()
            .filter(|(_, kind)| *kind != ShapeRecursionKind::NonRecursive)
            .collect();
        if recursive_shapes.is_empty() {
            writeln!(f, "Recursive shapes: none")
        } else {
            recursive_shapes.sort_by_key(|(label, _)| label.to_string());
            writeln!(f, "Recursive shapes:")?;
            for (label, kind) in recursive_shapes {
                writeln!(f, "  {label}: {kind}")?;
            }
            Ok(())
        }
    }
}
