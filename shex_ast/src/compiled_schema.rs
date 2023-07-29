use crate::{schema_json, IriRef, SchemaJson, CompiledSchemaError, Ref, ShapeLabel, TripleExprLabel};
use iri_s::{IriError, IriS, IriSError};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::fmt::Display;
use rbe::{Min, Max};


type CResult<T> = Result<T, CompiledSchemaError>;

#[derive(Debug)]
pub struct CompiledSchema {
    shape_labels_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    shape_label_counter: ShapeLabelIdx,
    shapes: HashMap<ShapeLabelIdx, ShapeExpr>,

    triple_expr_labels_map: HashMap<TripleExprLabel, TripleExprIdx>,
    triple_exprs: HashMap<TripleExprIdx, TripleExpr>,
    triple_expr_label_counter: TripleExprIdx
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub struct ShapeLabelIdx(usize);
impl ShapeLabelIdx {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub struct TripleExprIdx(usize);
impl TripleExprIdx {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Iri,
    BNode,
    NonLiteral,
    Literal,
}

#[derive(Debug, PartialEq)]
pub enum ShapeExpr {
    ShapeOr {
        exprs: Vec<Box<ShapeExpr>>,
    },
    ShapeAnd {
        exprs: Vec<Box<ShapeExpr>>,
    },
    ShapeNot {
        expr: Box<ShapeExpr>,
    },
    NodeConstraint {
        node_kind: Option<NodeKind>,
        datatype: Option<IriS>,
        xs_facet: Vec<XsFacet>,
        values: Vec<ValueSetValue>,
    },
    Shape {
        closed: bool,
        extra: Vec<IriS>,
        expression: Option<TripleExpr>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    ShapeExternal {},
    Ref{ idx: ShapeLabelIdx },
}

#[derive(Debug, PartialEq)]
pub enum TripleExpr {
    EachOf {
        expressions: Vec<Box<TripleExpr>>,
        min: Option<i32>,
        max: Option<i32>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    OneOf {
        id: Option<TripleExprLabel>,
        expressions: Vec<Box<TripleExpr>>,
        min: Option<i32>,
        max: Option<i32>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    TripleConstraint {
        id: Option<TripleExprLabel>,
        inverse: bool,
        predicate: IriS,
        value_expr: Option<Box<ShapeExpr>>,
        min: Min,
        max: Max,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
    },
    TripleExprRef(TripleExprLabel),
}

#[derive(Debug, PartialEq)]
pub enum XsFacet {
    StringFacet,
    NumericFacet,
}

#[derive(Debug, PartialEq)]
pub enum ValueSetValue {
    IriStem {
        type_: String,
        stem: IriS,
    },
    IriStemRange {
        type_: String,
        stem: IriRefOrWildcard,
        exclusions: Option<Vec<StringOrIriStem>>,
    },
    LiteralStem {
        type_: String,
        stem: String,
    },
    LiteralStemRange {
        type_: String,
        stem: StringOrWildcard,
        exclusions: Option<Vec<StringOrLiteralStem>>,
    },
    Language {
        type_: String,
        language_tag: String,
    },
    LanguageStem,
    LanguageStemRange,
    ObjectValue(ObjectValue),
}

#[derive(PartialEq, Debug)]
pub enum StringOrLiteralStem {
    String(String),
    LiteralStem { stem: String },
}

#[derive(PartialEq, Debug)]
pub enum IriRefOrWildcard {
    IriRef(IriS),
    Wildcard { type_: String },
}

#[derive(PartialEq, Debug)]
pub enum StringOrWildcard {
    String(String),
    Wildcard { type_: String },
}

#[derive(Debug, PartialEq)]
pub enum StringOrIriStem {
    String(String),
    IriStem { stem: String },
}

#[derive(PartialEq, Debug)]
pub enum ObjectValue {
    IriRef(IriS),
    ObjectLiteral {
        value: String,
        language: Option<String>,
        type_: Option<String>,
    },
}

#[derive(Debug, PartialEq)]
pub struct SemAct {
    name: IriS,
    code: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Annotation {
    predicate: IriS,
    object: ObjectValue,
}

impl CompiledSchema
{
    pub fn new() -> CompiledSchema {
        CompiledSchema {
           shape_labels_map: HashMap::new(),
           shape_label_counter: ShapeLabelIdx::default(),
           shapes: HashMap::new(),

           triple_expr_labels_map: HashMap::new(),
           triple_exprs: HashMap::new(),
           triple_expr_label_counter: TripleExprIdx::default()
        }
    }

    pub fn add_shape(&mut self, shape_label: ShapeLabel, se: ShapeExpr) {
        let idx = self.shape_label_counter;
        self.shape_labels_map.insert(shape_label, idx);
        self.shape_label_counter.incr();
        self.shapes.insert(idx, se); 
    }

    pub fn from_schema_json<'a>(
        &mut self, 
        schema_json: SchemaJson,
    ) -> Result<(), CompiledSchemaError> {
        if let Some(shape_decls) = schema_json.shapes {
            for sd in shape_decls {
                let label = self.id_to_shape_label(sd.id.clone())?;
                let se = self.shape_decl_to_shape_expr(sd)?;
                self.add_shape(label, se);
            }
        }
        Ok(())
    }

    fn id_to_shape_label<'a>(&self, id: String) -> CResult<ShapeLabel> {
        let label = ShapeLabel::from_iri_str(id)?;
        Ok(label)
    }

    fn shape_decl_to_shape_expr<'a>(
        &mut self,
        sd: schema_json::ShapeDecl,
    ) -> CResult<ShapeExpr> {
        self.cnv_shape_expr(sd.shape_expr)
    }

    fn cnv_shape_expr<'a>(&mut self, se: schema_json::ShapeExpr) -> CResult<ShapeExpr> {
        match se {
            schema_json::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for se in ses {
                    let unboxed = (*se).se;
                    let se = self.cnv_shape_expr(unboxed)?;
                    cnv.push(Box::new(se));
                }
                Ok(ShapeExpr::ShapeOr { exprs: cnv })
            }
            schema_json::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for se in ses {
                    let unboxed = (*se).se;
                    let se = self.cnv_shape_expr(unboxed)?;
                    cnv.push(Box::new(se));
                }
                Ok(ShapeExpr::ShapeAnd { exprs: cnv })
            }
            schema_json::ShapeExpr::ShapeNot { shape_expr: se } => {
                let unboxed = (*se).se;
                let se = self.cnv_shape_expr(unboxed)?;
                Ok(ShapeExpr::ShapeNot { expr: Box::new(se) })
            }
            schema_json::ShapeExpr::Shape {
                closed,
                extra,
                expression,
                sem_acts,
                annotations,
            } => {
                let new_extra = self.cnv_extra(extra)?;
                let expression = match expression {
                    Some(e) => {
                      let te = self.cnv_triple_expr(e)?;
                      Some(te)
                    },
                    None => None
                };
                Ok(ShapeExpr::Shape {
                    closed: Self::cnv_closed(closed),
                    extra: new_extra,
                    expression: expression,
                    sem_acts: Self::cnv_sem_acts(sem_acts),
                    annotations: Self::cnv_annotations(annotations),
                })
            }
            schema_json::ShapeExpr::Ref(se_ref) => {
                let idx = self.find_ref(se_ref)?;
                Ok(ShapeExpr::Ref{idx})
            }
            _ => todo!(),
        }
    }

    pub fn find_ref(&mut self, se_ref: Ref) -> CResult<ShapeLabelIdx> {
        let shape_label = match se_ref {
            Ref::IriRef { value } => { 
                let label = ShapeLabel::from_iri_str(value)?;
                Ok::<ShapeLabel, CompiledSchemaError>(label)
            },
            Ref::BNode { value } => {
                let label = ShapeLabel::from_bnode_str(value);
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

    pub fn find_label(&self, label: &ShapeLabel) -> Option<&ShapeExpr> {
        self.shape_labels_map.get(label).and_then(|idx| self.shapes.get(idx))
    }

    pub fn existing_labels(&self) -> Vec<&ShapeLabel> {
        self.shape_labels_map.keys().collect()
    }

    pub fn shapes(&self) -> impl Iterator<Item = (&ShapeLabel, &ShapeExpr)> {
        self.shape_labels_map.iter().map(|(label,idx)| {
            match self.shapes.get(idx) {
                Some(se) => (label, se),
                None => panic!("CompiledSchema: Internal Error obtaining shapes. Unknown idx: {idx:?}")
            }
        })
    }

    fn cnv_closed(closed: Option<bool>) -> bool {
        if let Some(closed) = closed {
            return closed;
        } else {
            false
        }
    }

    fn cnv_extra<'a>(&self, extra: Option<Vec<IriRef>>) -> CResult<Vec<IriS>> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = self.cnv_iri_ref(iri)?;
                vs.push(nm);
            }
            Ok(vs)
        } else {
            Ok(Vec::new())
        }
    }

    fn cnv_iri_ref<'a>(&self, iri: IriRef) -> Result<IriS, CompiledSchemaError> {
        let iri = IriS::new(&iri.value.as_str())?;
        Ok(iri)
    }

    fn handle_triple_expr_id(&mut self, id: Option<TripleExprLabel>, te: TripleExpr) -> CResult<()> {
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
    }

    /*fn cnv_shape_exprs(&mut self, ses: Vec<Box<schema_json::ShapeExpr>>) -> CResult<Vec<Box<ShapeExpr>>> {
        let rs: Vec<CResult<Box<ShapeExpr>>> = ses.iter().map(|se| {
            let nse = self.cnv_shape_expr(**se)?;
            Ok(Box::new(nse))
        }).collect();
        rs.into_iter().collect()
    }

    fn cnv_triple_exprs(&mut self, ses: Vec<Box<schema_json::TripleExprWrapper>>) -> CResult<Vec<Box<TripleExpr>>> {
        let rs: Vec<CResult<Box<TripleExpr>>> = ses.iter().map(|te| {
            let te = *te;
            let te = self.cnv_triple_expr(&te)?;
            Ok(Box::new(te))
        }).collect();
        rs.into_iter().collect()
    } */


    fn cnv_triple_expr<'a>(
        &mut self,
        triple_expr_wrapper: schema_json::TripleExprWrapper,
    ) -> CResult<TripleExpr> {
        match triple_expr_wrapper.te {
                schema_json::TripleExpr::EachOf {
                    id,
                    expressions,
                    min,
                    max,
                    sem_acts,
                    annotations,
                } => {
                    // let ses = self.cnv_triple_exprs(*expressions)?;
                    // let min = self.cnv_min(*min)?;
                    todo!()

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
/*                    let es = self.cnv_shape_exprs(expressions);
                    let te = TripleExpr::EachOf { 
                        expressions: (), 
                        min: (), 
                        max: (), 
                        sem_acts: (), 
                        annotations: () 
                    }
*/
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
                            let se = self.cnv_shape_expr(*se)?;
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

    fn cnv_min(&self, min: Option<i32>) -> CResult<Min> {
        todo!()
    }

    fn cnv_max(&self, max: Option<i32>) -> CResult<Max> {
        todo!()
    }

    fn cnv_id(id: Option<schema_json::TripleExprLabel>) -> Option<TripleExprLabel> {
        match id {
            None => None,
            Some(l) => {
                // TODO
                None
            }
        }
    }

    fn cnv_sem_acts(sem_acts: Option<Vec<schema_json::SemAct>>) -> Vec<SemAct> {
        if let Some(vs) = sem_acts {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn cnv_annotations(annotations: Option<Vec<schema_json::Annotation>>) -> Vec<Annotation> {
        if let Some(anns) = annotations {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }
}

impl Display for CompiledSchema {
  fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
    for (label, se) in self.shapes() {
        writeln!(dest, "{label:?} {se:?}")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use iri_s::*;

    #[test]
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
    }
}
