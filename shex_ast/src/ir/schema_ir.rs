use crate::Pred;
use crate::{
    CResult, SchemaIRError, ShapeExprLabel, ShapeLabelIdx, ast::Schema as SchemaJson,
    ir::ast2ir::AST2IR,
};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use tracing::trace;

use super::dependency_graph::{DependencyGraph, PosNeg};
use super::shape_expr::ShapeExpr;
use super::shape_label::ShapeLabel;

type Result<A> = std::result::Result<A, Box<SchemaIRError>>;

#[derive(Debug, Default, Clone)]
pub struct SchemaIR {
    shape_labels_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    shapes: HashMap<ShapeLabelIdx, (Option<ShapeLabel>, ShapeExpr)>,
    shape_label_counter: usize,
    prefixmap: PrefixMap,
}

impl SchemaIR {
    pub fn new() -> SchemaIR {
        SchemaIR {
            shape_labels_map: HashMap::new(),
            shape_label_counter: 0,
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
        }
    }

    pub fn set_prefixmap(&mut self, prefixmap: Option<PrefixMap>) {
        self.prefixmap = prefixmap.clone().unwrap_or_default();
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn shapes_counter(&self) -> usize {
        self.shape_label_counter
    }

    pub fn add_shape(&mut self, shape_label: ShapeLabel, se: ShapeExpr) -> ShapeLabelIdx {
        let idx = ShapeLabelIdx::from(self.shape_label_counter);
        self.shape_labels_map.insert(shape_label.clone(), idx);
        self.shapes.insert(idx, (Some(shape_label.clone()), se));
        self.shape_label_counter += 1;
        idx
    }

    pub fn get_shape_expr(&self, shape_label: &ShapeLabel) -> Option<&ShapeExpr> {
        if let Some(idx) = self.find_shape_label_idx(shape_label) {
            self.shapes.get(idx).map(|(_label, se)| se)
        } else {
            None
        }
    }

    pub fn from_schema_json(&mut self, schema_json: &SchemaJson) -> Result<()> {
        let mut schema_json_compiler = AST2IR::new(schema_json.source_iri());
        schema_json_compiler.compile(schema_json, self)?;
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
            .and_then(|idx| self.shapes.get(idx).map(|(_label, se)| (idx, se)))
    }

    pub fn find_shape_label_idx(&self, label: &ShapeLabel) -> Option<&ShapeLabelIdx> {
        self.shape_labels_map.get(label)
    }

    pub fn find_shape_idx(&self, idx: &ShapeLabelIdx) -> Option<&(Option<ShapeLabel>, ShapeExpr)> {
        self.shapes.get(idx)
    }

    pub fn shape_label_from_idx(&self, idx: &ShapeLabelIdx) -> Option<&ShapeLabel> {
        self.shapes
            .get(idx)
            .and_then(|(label, _se)| label.as_ref())
            .or(None)
    }

    pub fn new_index(&mut self) -> ShapeLabelIdx {
        let idx = ShapeLabelIdx::from(self.shape_label_counter);
        self.shape_label_counter += 1;
        self.shapes.insert(idx, (None, ShapeExpr::Empty));
        idx
    }

    pub fn existing_labels(&self) -> Vec<&ShapeLabel> {
        self.shape_labels_map.keys().collect()
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(Option<ShapeLabel>, ShapeExpr)> {
        self.shapes.values()
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
        if let Some((_label, shape_expr)) = self.find_shape_idx(idx) {
            match shape_expr {
                ShapeExpr::Ref { idx } => {
                    if visited.contains(idx) {
                        // If we have already visited this index, we return an empty map to avoid infinite recursion
                        return HashMap::new();
                    }
                    visited.insert(*idx);
                    self.references_visited(idx, visited)
                }
                _ => shape_expr.references(self),
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
        self.shapes.entry(*idx).and_modify(|(_label, s)| *s = se);
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
        trace!("Dependency graph: {dep_graph:?}");
        dep_graph.has_neg_cycle()
    }

    pub(crate) fn dependency_graph(&self) -> DependencyGraph {
        let mut dep_graph = DependencyGraph::new();
        let mut visited = Vec::new();
        for (idx, (_label, se)) in self.shapes.iter() {
            se.add_edges(*idx, &mut dep_graph, PosNeg::pos(), self, &mut visited);
        }
        dep_graph
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
        println!("Dependencies: {deps:?}");
        deps
    }
}

impl Display for SchemaIR {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(dest, "SchemaIR with {} shapes", self.shape_label_counter)?;
        writeln!(dest, "Labels to indexes:")?;
        for (label, idx) in self.shape_labels_map.iter() {
            let label = self.show_label(label);
            writeln!(dest, "{label} -> {idx}")?;
        }
        writeln!(dest, "Indexes to Shape Expressions:")?;
        for (idx, (maybe_label, se)) in self.shapes.iter() {
            let label_str = match maybe_label {
                None => "".to_string(),
                Some(label) => format!("{} = ", self.show_label(label)),
            };
            writeln!(dest, "{idx} -> {label_str}{se}")?;
        }
        writeln!(dest, "---end of schema IR")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use iri_s::iri;

    use super::SchemaIR;
    use crate::{Pred, ShapeLabelIdx, ast::Schema as SchemaJson, ir::shape_label::ShapeLabel};

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
        ir.from_schema_json(&schema_json).unwrap();
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
        ir.from_schema_json(&schema).unwrap();
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
        ir.from_schema_json(&schema).unwrap();
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

    /*#[test]
    fn validation_convert() {
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
        let mut compiled_schema = SchemaIR::new();
        compiled_schema.from_schema_json(schema_json).unwrap();
        let s1 = ShapeLabel::Iri(IriS::new("http://a.example/S1").unwrap());
        let p1 = IriS::new("http://a.example/p1").unwrap();
        let se1 = ShapeExpr::Shape {
            closed: false,
            extra: Vec::new(),
            expression: Some(TripleExpr::TripleConstraint {
                id: None,
                inverse: false,
                predicate: p1,
                value_expr: None,
                min: Min::from(1),
                max: Max::from(1),
                sem_acts: Vec::new(),
                annotations: Vec::new(),
            }),
            sem_acts: Vec::new(),
            annotations: Vec::new(),
        };
        assert_eq!(compiled_schema.find_label(&s1), Some(&se1));
    }*/
}
