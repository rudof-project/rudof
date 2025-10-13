use super::dependency_graph::{DependencyGraph, PosNeg};
use super::shape_expr::ShapeExpr;
use super::shape_label::ShapeLabel;
use crate::ir::inheritance_graph::InheritanceGraph;
use crate::ir::shape::Shape;
use crate::ir::shape_expr_info::ShapeExprInfo;
use crate::ir::source_idx::SourceIdx;
use crate::{
    CResult, SchemaIRError, ShapeExprLabel, ShapeLabelIdx, ast::Schema as SchemaJson,
    ir::ast2ir::AST2IR,
};
use crate::{Expr, Pred, ResolveMethod};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use tracing::trace;

type Result<A> = std::result::Result<A, Box<SchemaIRError>>;

#[derive(Debug, Default, Clone)]
pub struct SchemaIR {
    shape_labels_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    shapes: HashMap<ShapeLabelIdx, ShapeExprInfo>,
    shape_label_counter: usize,
    sources_map: HashMap<IriS, SourceIdx>,
    sources: HashMap<SourceIdx, IriS>,
    sources_counter: usize,
    prefixmap: PrefixMap,
    local_shapes_counter: usize,
    total_shapes_counter: usize,
    imported_schemas: Vec<IriS>,
    dependency_graph: DependencyGraph,
    inheritance_graph: InheritanceGraph,
    abstract_shapes: HashSet<ShapeLabelIdx>,
}

impl SchemaIR {
    pub fn new() -> SchemaIR {
        SchemaIR {
            shape_labels_map: HashMap::new(),
            shape_label_counter: 0,
            sources_map: HashMap::new(),
            sources: HashMap::new(),
            sources_counter: 0,
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
            total_shapes_counter: 0,
            local_shapes_counter: 0,
            imported_schemas: Vec::new(),
            dependency_graph: DependencyGraph::new(),
            inheritance_graph: InheritanceGraph::new(),
            abstract_shapes: HashSet::new(),
        }
    }

    pub fn set_prefixmap(&mut self, prefixmap: Option<PrefixMap>) {
        self.prefixmap = prefixmap.clone().unwrap_or_default();
    }

    pub fn add_abstract_shape(&mut self, idx: ShapeLabelIdx) {
        self.abstract_shapes.insert(idx);
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn is_abstract(&self, idx: &ShapeLabelIdx) -> bool {
        self.abstract_shapes.contains(idx)
    }

    pub fn set_local_shapes_counter(&mut self, counter: usize) {
        self.local_shapes_counter = counter;
    }

    pub fn set_imported_schemas(&mut self, imported_schemas: Vec<IriS>) {
        self.imported_schemas = imported_schemas
    }

    pub fn increment_total_shapes(&mut self, new_counter: usize) {
        self.total_shapes_counter += new_counter
    }

    pub fn shapes_counter(&self) -> usize {
        self.shape_label_counter
    }

    pub fn get_source(&self, source_idx: &SourceIdx) -> Option<&IriS> {
        self.sources.get(source_idx)
    }

    pub fn total_shapes_count(&self) -> usize {
        self.total_shapes_counter
    }

    pub fn add_shape(
        &mut self,
        shape_label: ShapeLabel,
        se: ShapeExpr,
        source_iri: &IriS,
    ) -> ShapeLabelIdx {
        let idx = ShapeLabelIdx::from(self.shape_label_counter);
        self.shape_labels_map.insert(shape_label.clone(), idx);
        let source_idx = self.new_source_idx(source_iri);
        self.shapes.insert(
            idx,
            ShapeExprInfo::new(Some(shape_label.clone()), se, source_idx),
        );
        self.shape_label_counter += 1;
        idx
    }

    pub fn get_shape_expr(&self, shape_label: &ShapeLabel) -> Option<&ShapeExpr> {
        if let Some(idx) = self.find_shape_label_idx(shape_label) {
            self.shapes.get(idx).map(|info| info.expr())
        } else {
            None
        }
    }

    pub fn local_shapes_count(&self) -> usize {
        self.local_shapes_counter
    }

    pub fn imported_schemas(&self) -> &Vec<IriS> {
        &self.imported_schemas
    }

    pub fn parents(&self, idx: &ShapeLabelIdx) -> Vec<ShapeLabelIdx> {
        self.inheritance_graph.parents(idx)
    }

    pub fn descendants(&self, idx: &ShapeLabelIdx) -> Vec<ShapeLabelIdx> {
        self.inheritance_graph.descendants(idx)
    }

    pub fn get_triple_exprs(
        &self,
        idx: &ShapeLabelIdx,
    ) -> Option<HashMap<Option<ShapeLabelIdx>, Vec<Expr>>> {
        if let Some(info) = self.find_shape_idx(idx) {
            let mut result = HashMap::new();
            let current_exprs = info.expr().get_triple_exprs(self);
            result.insert(None, current_exprs);
            trace!("Checking parents of {idx}: {:?}", self.parents(idx));
            for e in &self.parents(idx) {
                let shape_expr = self.find_shape_idx(e).unwrap();
                let exprs = shape_expr.expr().get_triple_exprs(self);
                result.insert(Some(*e), exprs);
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn get_preds_extends(&self, idx: &ShapeLabelIdx) -> HashSet<Pred> {
        let mut preds = HashSet::new();
        if let Some(info) = self.find_shape_idx(idx) {
            preds.extend(info.expr().preds(self));
            for e in &self.parents(idx) {
                if let Some(parent_info) = self.find_shape_idx(e) {
                    preds.extend(parent_info.expr().preds(self));
                }
            }
        }
        preds
    }

    pub fn count_extends(&self) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        for (_, _, shape_expr) in self.shapes() {
            let extends_counter = match shape_expr {
                ShapeExpr::Shape(shape) => Some(shape.extends().len()),
                _ => None,
            };

            if let Some(ec) = extends_counter {
                match result.entry(ec) {
                    Entry::Occupied(mut v) => {
                        let r = v.get_mut();
                        *r += 1;
                    }
                    Entry::Vacant(vac) => {
                        vac.insert(1);
                    }
                }
            }
        }
        result
    }

    pub fn from_schema_json(
        &mut self,
        schema_json: &SchemaJson,
        resolve_method: &ResolveMethod,
        base: &Option<IriS>,
    ) -> Result<()> {
        let mut compiler = AST2IR::new(resolve_method);
        compiler.compile(schema_json, &schema_json.source_iri(), base, self)?;
        Ok(())
    }

    pub fn find_ref(&self, se_ref: &ShapeExprLabel) -> CResult<ShapeLabelIdx> {
        let shape_label = match se_ref {
            ShapeExprLabel::IriRef { value } => match value {
                IriRef::Iri(iri) => {
                    let label = ShapeLabel::iri(iri.clone());
                    Ok::<ShapeLabel, SchemaIRError>(label)
                }
                IriRef::Prefixed { prefix, local } => {
                    let iri =
                        self.prefixmap
                            .resolve_prefix_local(prefix, local)
                            .map_err(|err| SchemaIRError::PrefixedNotFound {
                                prefix: prefix.clone(),
                                local: local.clone(),
                                err: Box::new(err),
                            })?;
                    Ok::<ShapeLabel, SchemaIRError>(ShapeLabel::iri(iri))
                }
            },
            ShapeExprLabel::BNode { value } => {
                let label = ShapeLabel::from_bnode((*value).clone());
                Ok(label)
            }
            ShapeExprLabel::Start => Ok(ShapeLabel::Start),
        }?;
        match self.shape_labels_map.get(&shape_label) {
            Some(idx) => Ok(*idx),
            None => Err(Box::new(SchemaIRError::LabelNotFound { shape_label })),
        }
    }

    pub fn find_label(&self, label: &ShapeLabel) -> Option<(&ShapeLabelIdx, &ShapeExpr)> {
        self.find_shape_label_idx(label)
            .and_then(|idx| self.shapes.get(idx).map(|info| (idx, info.expr())))
    }

    pub fn find_shape_label_idx(&self, label: &ShapeLabel) -> Option<&ShapeLabelIdx> {
        self.shape_labels_map.get(label)
    }

    pub fn find_shape_idx(&self, idx: &ShapeLabelIdx) -> Option<&ShapeExprInfo> {
        self.shapes.get(idx)
    }

    pub fn shape_label_from_idx(&self, idx: &ShapeLabelIdx) -> Option<&ShapeLabel> {
        self.shapes.get(idx).and_then(|info| info.label()).or(None)
    }

    pub fn new_index(&mut self, source_iri: &IriS) -> ShapeLabelIdx {
        let idx = ShapeLabelIdx::from(self.shape_label_counter);
        self.shape_label_counter += 1;
        let source_idx = self.new_source_idx(source_iri);
        self.shapes
            .insert(idx, ShapeExprInfo::new(None, ShapeExpr::Empty, source_idx));
        idx
    }

    fn new_source_idx(&mut self, source_iri: &IriS) -> SourceIdx {
        let source_idx = self
            .sources_map
            .entry(source_iri.clone())
            .or_insert_with(|| {
                let idx = SourceIdx::new(self.sources_counter);
                self.sources.insert(idx, source_iri.clone());
                self.sources_counter += 1;
                idx
            });
        *source_idx
    }

    pub fn existing_labels(&self) -> Vec<&ShapeLabel> {
        self.shape_labels_map.keys().collect()
    }

    pub fn shapes(&self) -> impl Iterator<Item = (&ShapeLabel, &IriS, &ShapeExpr)> {
        self.shapes.iter().filter_map(|(_, info)| {
            info.label().map(|label| {
                let source = self.get_source(info.source_idx()).unwrap();
                (label, source, info.expr())
            })
        })
    }

    // Returns a map of predicates to shape label indices that reference the given index
    pub fn references(&self, idx: &ShapeLabelIdx) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        let visited = HashSet::new();
        self.references_visited(idx, visited)
    }

    //
    pub fn references_visited(
        &self,
        idx: &ShapeLabelIdx,
        mut visited: HashSet<ShapeLabelIdx>,
    ) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        if let Some(info) = self.find_shape_idx(idx) {
            match info.expr() {
                ShapeExpr::Ref { idx } => {
                    if visited.contains(idx) {
                        // If we have already visited this index, we return an empty map to avoid infinite recursion
                        return HashMap::new();
                    }
                    visited.insert(*idx);
                    self.references_visited(idx, visited)
                }
                _ => info.expr().references(self),
            }
        } else {
            HashMap::new()
        }
    }

    #[allow(dead_code)]
    fn cnv_closed(closed: &Option<bool>) -> bool {
        match closed {
            None => false,
            Some(closed) => *closed,
        }
    }

    #[allow(dead_code)]
    fn cnv_extra(&self, extra: &Option<Vec<IriRef>>) -> CResult<Vec<IriS>> {
        extra
            .as_ref()
            .map(|extra| {
                extra
                    .iter()
                    .map(|iri| self.cnv_iri_ref(iri))
                    .collect::<CResult<Vec<_>>>()
            })
            .unwrap_or(Ok(vec![]))
    }

    fn cnv_iri_ref(&self, iri_ref: &IriRef) -> Result<IriS> {
        let iri_s = (*iri_ref).clone().into();
        Ok(iri_s)
    }

    pub fn get_shape_label_idx(&self, shape_label: &ShapeLabel) -> Result<ShapeLabelIdx> {
        match self.shape_labels_map.get(shape_label) {
            Some(shape_label_idx) => Ok(*shape_label_idx),
            None => Err(Box::new(SchemaIRError::ShapeLabelNotFound {
                shape_label: shape_label.clone(),
            })),
        }
    }

    pub fn replace_shape(&mut self, idx: &ShapeLabelIdx, se: ShapeExpr) {
        self.shapes.entry(*idx).and_modify(|info| info.set_expr(se));
    }

    pub fn show_label(&self, label: &ShapeLabel) -> String {
        match label {
            ShapeLabel::Iri(iri) => self.prefixmap.qualify(iri),
            ShapeLabel::BNode(bnode) => format!("{bnode}"),
            ShapeLabel::Start => "START".to_string(),
        }
    }

    pub fn neg_cycles(&self) -> Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>> {
        let dep_graph = self.dependency_graph();
        dep_graph.neg_cycles()
    }

    /// This is used to detect cycles that involve negations in the schema
    /// A well formed schema should not have any cyclic reference that involve a negation
    pub fn has_neg_cycle(&self) -> bool {
        let dep_graph = self.dependency_graph();
        trace!("Dependency graph: {dep_graph}");
        dep_graph.has_neg_cycle()
    }

    /// Returns the dependency graph.
    pub fn dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }

    pub(crate) fn build_dependency_graph(&mut self) {
        let mut dep_graph = DependencyGraph::new();
        let mut visited = Vec::new();
        for (idx, info) in self.shapes.iter() {
            info.expr()
                .add_edges(*idx, &mut dep_graph, PosNeg::pos(), self, &mut visited);
        }
        self.dependency_graph = dep_graph
    }

    pub(crate) fn build_inheritance_graph(&mut self) {
        let mut inheritance_graph = InheritanceGraph::new();
        for (idx, info) in self.shapes.iter() {
            match info.expr() {
                ShapeExpr::Shape(shape) => {
                    for e in shape.extends() {
                        inheritance_graph.add_edge(*idx, *e);
                    }
                }
                _ => continue,
            }
        }
        self.inheritance_graph = inheritance_graph
    }

    pub fn dependencies(&self) -> Vec<(ShapeLabel, PosNeg, ShapeLabel)> {
        let mut deps = Vec::new();
        for (source, posneg, target) in self.dependency_graph().all_edges() {
            match (
                self.shape_label_from_idx(&source),
                self.shape_label_from_idx(&target),
            ) {
                (Some(source_label), Some(target_label)) => {
                    deps.push((source_label.clone(), posneg, target_label.clone()));
                }
                _ => {
                    // We ignore dependencies between shapes that have no labels
                }
            }
        }
        deps
    }

    pub fn show_shape_idx(&self, idx: &ShapeLabelIdx) -> String {
        let mut result = String::new();
        if let Some(info) = self.find_shape_idx(idx) {
            match info.label() {
                Some(label) => {
                    result.push_str(
                        format!(
                            "{} = {}",
                            self.show_label(label),
                            self.show_shape_expr(info.expr())
                        )
                        .as_str(),
                    );
                }
                None => {
                    result.push_str(format!("{}", self.show_shape_expr(info.expr())).as_str());
                }
            }
        } else {
            result.push_str(format!("ShapeLabelIdx {idx} not found").as_str());
        }
        result
    }

    fn show_shape_expr(&self, se: &ShapeExpr) -> String {
        match se {
            ShapeExpr::ShapeOr { exprs } => format!(
                "({})",
                exprs
                    .iter()
                    .map(|e| self.show_shape_idx(e))
                    .collect::<Vec<_>>()
                    .join(" OR ")
            ),
            ShapeExpr::ShapeAnd { exprs } => format!(
                "({})",
                exprs
                    .iter()
                    .map(|e| self.show_shape_idx(e))
                    .collect::<Vec<_>>()
                    .join(" AND ")
            ),
            ShapeExpr::ShapeNot { expr } => format!("NOT ({})", self.show_shape_idx(expr)),
            ShapeExpr::NodeConstraint(nc) => format!("{nc}"),
            ShapeExpr::Shape(shape) => self.show_shape(shape),
            ShapeExpr::External {} => "EXTERNAL".to_string(),
            ShapeExpr::Ref { idx } => format!("@{}", idx),
            ShapeExpr::Empty => "{}".to_string(),
        }
    }

    fn show_shape(&self, shape: &Shape) -> String {
        let extends = if shape.extends().is_empty() {
            "".to_string()
        } else {
            format!(
                " EXTENDS [{}]",
                shape
                    .extends()
                    .iter()
                    .map(|e| self.show_shape_idx(e))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        let closed = if shape.is_closed() { "CLOSED" } else { "" };
        let extra = if shape.extra().is_empty() {
            "".to_string()
        } else {
            format!(
                " EXTRA [{}]",
                shape
                    .extra()
                    .iter()
                    .map(|e| self.prefixmap.qualify(e.iri()))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        let rbe = shape.triple_expr().show_rbe_simplified();
        format!("Shape {extends}{closed}{extra}{{{rbe}}}")
    }
}

impl Display for SchemaIR {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.sources_counter > 1 {
            writeln!(dest, "Sources:")?;
            for (idx, iri) in self.sources.iter() {
                writeln!(dest, "{idx} -> {iri}")?;
            }
        } else if self.sources_counter == 1 {
            write!(dest, "Source: ")?;
            if let Some((idx, iri)) = self.sources.iter().next() {
                writeln!(dest, "{idx} -> {iri}")?;
            }
        } else {
            writeln!(dest, "No sources")?;
        }
        writeln!(dest, "SchemaIR with {} shapes", self.shape_label_counter)?;
        writeln!(dest, "Labels to indexes:")?;
        for (label, idx) in self.shape_labels_map.iter() {
            let label = self.show_label(label);
            writeln!(dest, "{label} -> {idx}")?;
        }
        writeln!(dest, "Indexes to Shape Expressions:")?;
        for (idx, info) in self.shapes.iter() {
            let label_str = match info.label() {
                None => "".to_string(),
                Some(label) => format!("{} = ", self.show_label(label)),
            };
            writeln!(
                dest,
                "{idx}{} -> {label_str}{}",
                if self.sources_counter > 1 {
                    format!(" (source: {})", info.source_idx())
                } else {
                    "".to_string()
                },
                info.expr()
            )?;
        }
        writeln!(dest, "Dependency graph: {}", self.dependency_graph())?;
        writeln!(dest, "Inheritance graph: {}", self.inheritance_graph)?;
        writeln!(dest, "---end of schema IR")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use iri_s::iri;

    use super::SchemaIR;
    use crate::{
        Pred, ResolveMethod, ShapeLabelIdx, ast::Schema as SchemaJson, ir::shape_label::ShapeLabel,
    };

    #[test]
    fn test_find_component() {
        let str = r#"{
            "@context": "http://www.w3.org/ns/shex.jsonld",
            "type": "Schema",
            "shapes": [
                {
                    "type": "ShapeDecl",
                    "id": "http://a.example/S1",
                    "shapeExpr": {
                        "type": "Shape",
                        "expression": {
                            "type": "TripleConstraint",
                            "predicate": "http://a.example/p1"
                        }
                    }
                }
            ]
        }"#;
        let schema_json: SchemaJson = serde_json::from_str::<SchemaJson>(str).unwrap();
        let mut ir = SchemaIR::new();
        ir.from_schema_json(&schema_json, &ResolveMethod::default(), &None)
            .unwrap();
        println!("Schema IR: {ir}");
        let s1_label: ShapeLabel = ShapeLabel::iri(iri!("http://a.example/S1"));
        let s1 = ir
            .shape_label_from_idx(&ir.get_shape_label_idx(&s1_label).unwrap())
            .unwrap();
        assert_eq!(s1, &s1_label);
    }

    #[test]
    fn test_ir_references() {
        let str = r#"{ "type": "Schema",
            "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
            "type": "Shape",
            "expression": {
            "type": "EachOf",
            "expressions": [{
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/T"
            },
            {
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/U"
            }
          ]
        }
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/T",
      "shapeExpr": {
        "type": "Shape"
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/U",
      "shapeExpr": {
        "type": "Shape"
      }
    }
  ],
  "@context": "http://www.w3.org/ns/shex.jsonld"
}"#;
        let schema: SchemaJson = serde_json::from_str(str).unwrap();
        let mut ir = SchemaIR::new();
        ir.from_schema_json(&schema, &ResolveMethod::default(), &None)
            .unwrap();
        println!("Schema IR: {ir}");
        let s: ShapeLabel = ShapeLabel::iri(iri!("http://example.org/S"));
        let idx = ir.get_shape_label_idx(&s).unwrap();
        let references = ir.references(&idx);
        let expected: HashMap<Pred, Vec<ShapeLabelIdx>> = vec![(
            Pred::new_unchecked("http://example.org/p"),
            vec![
                ShapeLabelIdx::from(1), // T
                ShapeLabelIdx::from(2), // U
            ],
        )]
        .into_iter()
        .collect();
        assert_eq!(references, expected);
    }

    #[test]
    fn test_ir_references_and() {
        let str = r#"{
  "type": "Schema",
  "shapes": [
    {
      "type": "ShapeDecl",
      "id": "http://example.org/S",
      "shapeExpr": {
        "type": "ShapeAnd",
        "shapeExprs": [
          {
            "type": "Shape",
            "expression": {
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/T"
            }
          },
          {
            "type": "Shape",
            "expression": {
              "type": "TripleConstraint",
              "predicate": "http://example.org/p",
              "valueExpr": "http://example.org/U"
            }
          }
        ]
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/T",
      "shapeExpr": {
        "type": "Shape"
      }
    },
    {
      "type": "ShapeDecl",
      "id": "http://example.org/U",
      "shapeExpr": {
        "type": "Shape"
      }
    }
  ],
  "@context": "http://www.w3.org/ns/shex.jsonld"
}"#;
        let schema: SchemaJson = serde_json::from_str(str).unwrap();
        let mut ir = SchemaIR::new();
        ir.from_schema_json(&schema, &ResolveMethod::default(), &None)
            .unwrap();
        let s: ShapeLabel = ShapeLabel::iri(iri!("http://example.org/S"));
        let idx = ir.get_shape_label_idx(&s).unwrap();
        println!("Schema IR: {ir}");
        println!("Idx: {idx}");
        let references = ir.references(&idx);
        let expected: HashMap<Pred, Vec<ShapeLabelIdx>> = vec![(
            Pred::new_unchecked("http://example.org/p"),
            vec![
                ShapeLabelIdx::from(1), // T
                ShapeLabelIdx::from(2), // U
            ],
        )]
        .into_iter()
        .collect();
        assert_eq!(references, expected);
    }
}
