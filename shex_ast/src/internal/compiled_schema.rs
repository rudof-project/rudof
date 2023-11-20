use crate::ast::schema_json_compiler::SchemaJsonCompiler;
use crate::{
    ast, ast::IriRef, ast::Ref, ast::Schema as SchemaJson, internal::ObjectValue,
    internal::ValueSetValue, CResult, CompiledSchemaError, Cond, Node, ShapeLabel, ShapeLabelIdx,
};
use iri_s::IriS;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
// use std::str::FromStr;
use crate::Pred;
use log::debug;
use rbe::{MatchCond, RbeTable};
use std::fmt::Display;

type Result<A> = std::result::Result<A, CompiledSchemaError>;

#[derive(Debug)]
pub struct CompiledSchema {
    shape_labels_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    shapes: HashMap<ShapeLabelIdx, (ShapeLabel, ShapeExpr)>,
    shape_label_counter: ShapeLabelIdx,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub struct TripleExprIdx(usize);
impl TripleExprIdx {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum NodeKind {
    Iri,
    BNode,
    NonLiteral,
    Literal,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ShapeExpr {
    ShapeOr {
        exprs: Vec<ShapeExpr>,
    },
    ShapeAnd {
        exprs: Vec<ShapeExpr>,
    },
    ShapeNot {
        expr: Box<ShapeExpr>,
    },
    NodeConstraint {
        node_kind: Option<NodeKind>,
        datatype: Option<IriS>,
        xs_facet: Option<Vec<XsFacet>>,
        values: Option<Vec<ValueSetValue>>,
        cond: Cond,
    },
    Shape {
        closed: bool,
        extra: Vec<IriS>,
        rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    External {},
    Ref {
        idx: ShapeLabelIdx,
    },
    Empty,
}

impl ShapeExpr {
    pub fn mk_ref(idx: ShapeLabelIdx) -> ShapeExpr {
        ShapeExpr::Ref { idx }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum XsFacet {
    StringFacet,
    NumericFacet,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemAct {
    name: IriS,
    code: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Annotation {
    predicate: IriS,
    object: ObjectValue,
}

impl CompiledSchema {
    pub fn new() -> CompiledSchema {
        CompiledSchema {
            shape_labels_map: HashMap::new(),
            shape_label_counter: ShapeLabelIdx::default(),
            shapes: HashMap::new(),
        }
    }

    pub fn add_shape(&mut self, shape_label: ShapeLabel, se: ShapeExpr) {
        let idx = self.shape_label_counter;
        self.shape_labels_map.insert(shape_label.clone(), idx);
        self.shapes.insert(idx, (shape_label.clone(), se));
        self.shape_label_counter.incr()
    }

    pub fn get_shape_expr(&self, shape_label: &ShapeLabel) -> Option<&ShapeExpr> {
        if let Some(idx) = self.find_shape_label_idx(shape_label) {
            self.shapes.get(idx).map(|(label, se)| se)
        } else {
            None
        }
    }

    pub fn from_schema_json<'a>(&mut self, schema_json: &SchemaJson) -> Result<()> {
        let mut schema_json_compiler = SchemaJsonCompiler::new();
        schema_json_compiler.compile(schema_json, self)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn shape_decl_to_shape_expr<'a>(&mut self, sd: &ast::ShapeDecl) -> CResult<ShapeExpr> {
        self.cnv_shape_expr(&sd.shape_expr)
    }

    fn cnv_shape_expr<'a>(&mut self, se: &ast::ShapeExpr) -> CResult<ShapeExpr> {
        match se {
            ast::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for sew in ses {
                    let se = self.cnv_shape_expr(&sew.se)?;
                    cnv.push(se);
                }
                Ok(ShapeExpr::ShapeOr { exprs: cnv })
            }
            ast::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for sew in ses {
                    let se = self.cnv_shape_expr(&sew.se)?;
                    cnv.push(se);
                }
                Ok(ShapeExpr::ShapeAnd { exprs: cnv })
            }
            ast::ShapeExpr::ShapeNot { shape_expr: sew } => {
                let se = self.cnv_shape_expr(&sew.se)?;
                Ok(ShapeExpr::ShapeNot { expr: Box::new(se) })
            }
            /*            schema_json::ShapeExpr::Shape {
                closed,
                extra,
                expression,
                sem_acts,
                annotations,
            } => {
                let new_extra = self.cnv_extra(extra)?;
                let expression = match expression {
                    Some(ref e) => {
                      let te = self.cnv_triple_expr(e)?;
                      Some(te)
                    },
                    None => None
                };
                Ok(ShapeExpr::Shape {
                    closed: Self::cnv_closed(closed),
                    extra: new_extra,
                    expression: expression,
                    sem_acts: Self::cnv_sem_acts(&sem_acts),
                    annotations: Self::cnv_annotations(&annotations),
                })
            }
            schema_json::ShapeExpr::Ref(se_ref) => {
                let idx = self.find_ref(se_ref)?;
                Ok(ShapeExpr::Ref{idx})
            } */
            _ => todo!(),
        }
    }

    pub fn find_ref(&mut self, se_ref: &Ref) -> CResult<ShapeLabelIdx> {
        let shape_label = match se_ref {
            Ref::IriRef { value } => {
                let iri_s: IriS = (*value).clone().into();
                let label = ShapeLabel::iri(iri_s);
                Ok::<ShapeLabel, CompiledSchemaError>(label)
            }
            Ref::BNode { value } => {
                let label = ShapeLabel::from_bnode((*value).clone());
                Ok(label)
            }
        }?;
        match self.shape_labels_map.get(&shape_label) {
            Some(idx) => Ok(*idx),
            None => {
                todo!()
            }
        }
    }

    pub fn find_label(&self, label: &ShapeLabel) -> Option<(&ShapeLabelIdx, &ShapeExpr)> {
        self.find_shape_label_idx(label).and_then(|idx| {
            self.shapes
                .get(idx)
                .and_then(|(_label, se)| Some((idx, se)))
        })
    }

    pub fn find_shape_label_idx(&self, label: &ShapeLabel) -> Option<&ShapeLabelIdx> {
        self.shape_labels_map.get(label)
    }

    pub fn find_shape_idx(&self, idx: &ShapeLabelIdx) -> Option<&(ShapeLabel, ShapeExpr)> {
        self.shapes.get(idx)
    }

    pub fn existing_labels(&self) -> Vec<&ShapeLabel> {
        self.shape_labels_map.keys().collect()
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(ShapeLabel, ShapeExpr)> {
        /*self.shape_labels_map
        .iter()
        .map(|(label, idx)| match self.shapes.get(idx) {
            Some(se) => (label, se),
            None => {
                panic!("CompiledSchema: Internal Error obtaining shapes. Unknown idx: {idx:?}")
            }
        })*/
        self.shapes.iter().map(|(idx, pair)| pair)
    }

    #[allow(dead_code)]
    fn cnv_closed(closed: &Option<bool>) -> bool {
        match closed {
            None => false,
            Some(closed) => *closed,
        }
    }

    #[allow(dead_code)]
    fn cnv_extra<'a>(&self, extra: &Option<Vec<IriRef>>) -> CResult<Vec<IriS>> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = self.cnv_iri_ref(&iri)?;
                vs.push(nm);
            }
            Ok(vs)
        } else {
            Ok(Vec::new())
        }
    }

    fn cnv_iri_ref<'a>(&self, iri_ref: &IriRef) -> Result<IriS> {
        let iri_s = (*iri_ref).clone().into();
        Ok(iri_s)
    }

    pub fn get_shape_label_idx(&self, shape_label: &ShapeLabel) -> Result<ShapeLabelIdx> {
        match self.shape_labels_map.get(shape_label) {
            Some(shape_label_idx) => Ok(*shape_label_idx),
            None => Err(CompiledSchemaError::ShapeLabelNotFound {
                shape_label: shape_label.clone(),
            }),
        }
    }

    /*     fn handle_triple_expr_id(&mut self, id: Option<TripleExprLabel>, te: TripleExpr) -> CResult<()> {
        if let Some(label) = id {
            if let Some(found) = self.triple_expr_labels_map.get(&label) {
              return Err(CompiledSchemaError::DuplicatedTripleExprLabel {
                label: label
              })
            } else {
              let idx = self.triple_expr_label_counter;
              self.triple_expr_labels_map.insert(label, idx);
              self.triple_exprs.insert(idx, te);
              self.triple_expr_label_counter.incr();
              Ok(())
            }
        } else {
            Ok(())
        }
    } */

    /*fn cnv_shape_exprs(&mut self, ses: Vec<Box<schema_json::ShapeExpr>>) -> CResult<Vec<Box<ShapeExpr>>> {
        let rs: Vec<CResult<Box<ShapeExpr>>> = ses.iter().map(|se| {
            let nse = self.cnv_shape_expr(**se)?;
            Ok(Box::new(nse))
        }).collect();
        rs.into_iter().collect()
    } */

    /*    fn cnv_triple_exprs(&mut self, ses: &Vec<schema_json::TripleExprWrapper>) -> CResult<Vec<TripleExpr>> {
            let rs: Vec<CResult<TripleExpr>> = ses.iter().map(|tew| {
                // let te = te.as_ref();
                let te = self.cnv_triple_expr(tew)?;
                Ok(te)
            }).collect();
            rs.into_iter().collect()
        }


        fn cnv_triple_expr<'a>(
            &mut self,
            triple_expr_wrapper: &schema_json::TripleExprWrapper,
        ) -> CResult<TripleExpr> {
            match &triple_expr_wrapper.te {
                    schema_json::TripleExpr::EachOf {
                        id,
                        expressions,
                        min,
                        max,
                        sem_acts,
                        annotations,
                    } => {
                        let ses = self.cnv_triple_exprs(expressions)?;
                        let min = self.cnv_min(min)?;
                        let sem_acts = Self::cnv_sem_acts(sem_acts);
                        let annotations = Self::cnv_annotations(annotations);
                        let max = self.cnv_max(max)?;
                        Ok(TripleExpr::EachOf { expressions: ses, min, max, sem_acts, annotations})

                    },
                    schema_json::TripleExpr::OneOf {
                        id,
                        expressions,
                        min,
                        max,
                        sem_acts,
                        annotations,
                    } => {
                        todo!()
    *//*                    let es = self.cnv_shape_exprs(expressions);
                        let te = TripleExpr::EachOf {
                            expressions: (),
                            min: (),
                            max: (),
                            sem_acts: (),
                            annotations: ()
                        }
    *//*
                    },
                    schema_json::TripleExpr::TripleConstraint {
                        id,
                        inverse,
                        predicate,
                        value_expr,
                        min,
                        max,
                        sem_acts,
                        annotations,
                    } => {
                        let id = Self::cnv_id(id);
                        let sem_acts = Self::cnv_sem_acts(sem_acts);
                        let annotations = Self::cnv_annotations(annotations);
                        let predicate = self.cnv_iri_ref(predicate)?;
                        let min = self.cnv_min(min)?;
                        let max = self.cnv_max(max)?;
                        let value_expr = match value_expr {
                            Some(se) => {
                                let se = self.cnv_shape_expr(se)?;
                                Some(Box::new(se))
                            },
                            None => None
                        } ;
                        Ok(TripleExpr::TripleConstraint {
                            id: id,
                            inverse: inverse.unwrap_or(false),
                            predicate: predicate,
                            value_expr: value_expr,
                            min: min,
                            max: max,
                            sem_acts: sem_acts,
                            annotations: annotations,
                        })
                    },
                    schema_json::TripleExpr::TripleExprRef(_) => todo!(),
            }
        }

        fn cnv_min(&self, min: &Option<i32>) -> CResult<Min> {
            match min {
             Some(min) if *min < 0 => Err(CompiledSchemaError::MinLessZero { min: *min }),
             Some(min) => Ok(Min::from(*min)),
             None => Ok(Min::from(1))
            }
        }

        fn cnv_max(&self, max: &Option<i32>) -> CResult<Max> {
            match *max {
                Some(-1) => Ok(Max::Unbounded),
                Some(max) if max < -1 => Err(CompiledSchemaError::MaxIncorrect { max }),
                Some(max) => Ok(Max::from(max)),
                None => Ok(Max::from(1))
               }
        }

        fn cnv_id(id: &Option<schema_json::TripleExprLabel>) -> Option<TripleExprLabel> {
            match id {
                None => None,
                Some(l) => {
                    // TODO
                    None
                }
            }
        }

        fn cnv_sem_acts(sem_acts: &Option<Vec<schema_json::SemAct>>) -> Vec<SemAct> {
            if let Some(vs) = sem_acts {
                // TODO
                Vec::new()
            } else {
                Vec::new()
            }
        }

        fn cnv_annotations(annotations: &Option<Vec<schema_json::Annotation>>) -> Vec<Annotation> {
            if let Some(anns) = annotations {
                // TODO
                Vec::new()
            } else {
                Vec::new()
            }
        }
    */
    pub fn replace_shape(&mut self, idx: &ShapeLabelIdx, se: ShapeExpr) {
        self.shapes.entry(*idx).and_modify(|(label, s)| *s = se);
    }
}

impl Display for CompiledSchema {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (label, se) in self.shapes() {
            let error_idx = ShapeLabelIdx::error();
            let idx = self.shape_labels_map.get(label).unwrap_or(&error_idx);
            writeln!(dest, "{idx}@{label} -> {se:?}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Schema as SchemaJson;
    use crate::internal::CompiledSchema;

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
        let mut compiled_schema = CompiledSchema::new();
        compiled_schema.from_schema_json(&schema_json).unwrap();
        //        let shape = compiled_schema.get
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
        let mut compiled_schema = CompiledSchema::new();
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
