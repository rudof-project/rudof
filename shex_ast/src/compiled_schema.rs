use crate::{schema_json, IriRef, SchemaJson, CompiledSchemaError};
use iri_s::{IriError, IriS, IriSError};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::fmt::Display;

type ShapeLabelIdx = usize;

#[derive(Debug)]
pub struct CompiledSchema {
    shape_labels_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    shape_label_counter: ShapeLabelIdx,
    shapes: HashMap<ShapeLabelIdx, ShapeExpr>,
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ShapeLabel {
    Iri(IriS),
    BNode(String),
}

impl FromStr for ShapeLabel {
    type Err = IriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShapeLabel::Iri(IriS::from_str(s)?))
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TripleExprLabel {
    Iri(IriS),
    BNode(String),
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
    Ref {
        label: ShapeLabel,
    },
}

#[derive(Debug, PartialEq)]
pub enum TripleExpr {
    EachOf {
        id: Option<TripleExprLabel>,
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
        min: Option<i32>,
        max: Option<i32>,
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
           shape_label_counter: 0,
           shapes: HashMap::new()
        }
    }

    pub fn add_shape(&mut self, shape_label: ShapeLabel, se: ShapeExpr) {
        let idx = self.shape_label_counter;
        self.shape_labels_map.insert(shape_label, idx);
        self.shape_label_counter += 1;
        self.shapes.insert(idx, se); 
    }

    pub fn from_schema_json<'a>(
        schema_json: SchemaJson,
    ) -> Result<CompiledSchema, CompiledSchemaError> {
        let mut schema = CompiledSchema::new();
        if let Some(shape_decls) = schema_json.shapes {
            for sd in shape_decls {
                let label = Self::id_to_shape_label(sd.id.clone())?;
                let se = Self::shape_decl_to_shape_expr(sd)?;
                schema.add_shape(label, se);
            }
        }
        Ok(schema)
    }

    fn id_to_shape_label<'a>(id: String) -> Result<ShapeLabel, CompiledSchemaError> {
        ShapeLabel::from_str(id.as_str()).map_err(|err| { todo!()})
        // SL::from_str(&id).map_err(|err| CompiledSchemaError::Str2IriError { str: id })
    }

    fn shape_decl_to_shape_expr<'a>(
        sd: schema_json::ShapeDecl,
    ) -> Result<ShapeExpr, CompiledSchemaError> {
        Self::cnv_shape_expr(sd.shape_expr)
    }

    fn cnv_shape_expr<'a>(se: schema_json::ShapeExpr) -> Result<ShapeExpr, CompiledSchemaError> {
        match se {
            schema_json::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for se in ses {
                    let unboxed = (*se).se;
                    let se = Self::cnv_shape_expr(unboxed)?;
                    cnv.push(Box::new(se));
                }
                Ok(ShapeExpr::ShapeOr { exprs: cnv })
            }
            schema_json::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for se in ses {
                    let unboxed = (*se).se;
                    let se = Self::cnv_shape_expr(unboxed)?;
                    cnv.push(Box::new(se));
                }
                Ok(ShapeExpr::ShapeAnd { exprs: cnv })
            }
            schema_json::ShapeExpr::ShapeNot { shape_expr: se } => {
                let unboxed = (*se).se;
                let se = Self::cnv_shape_expr(unboxed)?;
                Ok(ShapeExpr::ShapeNot { expr: Box::new(se) })
            }
            schema_json::ShapeExpr::Shape {
                closed,
                extra,
                expression,
                sem_acts,
                annotations,
            } => {
                let new_extra = Self::cnv_extra(extra)?;
                let expression = Self::cnv_triple_expr(expression)?;
                Ok(ShapeExpr::Shape {
                    closed: Self::cnv_closed(closed),
                    extra: new_extra,
                    expression: expression,
                    sem_acts: Self::cnv_sem_acts(sem_acts),
                    annotations: Self::cnv_annotations(annotations),
                })
            }
            _ => todo!(),
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
                None => panic!("CompiledSchema: Internal Error obtaining shapes. Unknown idx: {idx}")
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

    fn cnv_extra<'a>(extra: Option<Vec<IriRef>>) -> Result<Vec<IriS>, CompiledSchemaError> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = Self::cnv_iri_ref(iri)?;
                vs.push(nm);
            }
            Ok(vs)
        } else {
            Ok(Vec::new())
        }
    }

    fn cnv_iri_ref<'a>(iri: IriRef) -> Result<IriS, CompiledSchemaError> {
        let iri = IriS::new(&iri.value.as_str())?;
        Ok(iri)
    }

    fn cnv_triple_expr<'a>(
        triple_expr_wrapper: Option<schema_json::TripleExprWrapper>,
    ) -> Result<Option<TripleExpr>, CompiledSchemaError> {
        if let Some(tew) = triple_expr_wrapper {
            let te = tew.te;
            match te {
                schema_json::TripleExpr::EachOf {
                    id,
                    expressions,
                    min,
                    max,
                    sem_acts,
                    annotations,
                } => todo!(),
                schema_json::TripleExpr::OneOf {
                    id,
                    expressions,
                    min,
                    max,
                    sem_acts,
                    annotations,
                } => todo!(),
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
                    let predicate = Self::cnv_iri_ref(predicate)?;
                    let value_expr = if let Some(se) = value_expr {
                        let se = Self::cnv_shape_expr(*se)?;
                        Some(Box::new(se))
                    } else {
                        None
                    };
                    Ok(Some(TripleExpr::TripleConstraint {
                        id: id,
                        inverse: inverse.unwrap_or(false),
                        predicate: predicate,
                        value_expr: value_expr,
                        min: min,
                        max: max,
                        sem_acts: sem_acts,
                        annotations: annotations,
                    }))
                }
                schema_json::TripleExpr::TripleExprRef(_) => todo!(),
            }
        } else {
            Ok(None)
        }
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
        let compiled_schema = CompiledSchema::from_schema_json(schema_json).unwrap();
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
                min: None,
                max: None,
                sem_acts: Vec::new(),
                annotations: Vec::new(),
            }),
            sem_acts: Vec::new(),
            annotations: Vec::new(),
        };
        assert_eq!(compiled_schema.find_label(&s1), Some(&se1));
    }
}
