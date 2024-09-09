use crate::{
    ast::Schema as SchemaJson, compiled::schema_json_compiler::SchemaJsonCompiler, CResult,
    CompiledSchemaError, ShapeExprLabel, ShapeLabelIdx,
};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use std::collections::HashMap;
use std::fmt::Display;

use super::shape_expr::ShapeExpr;
use super::shape_label::ShapeLabel;

type Result<A> = std::result::Result<A, CompiledSchemaError>;

#[derive(Debug, Default)]
pub struct CompiledSchema {
    shape_labels_map: HashMap<ShapeLabel, ShapeLabelIdx>,
    shapes: HashMap<ShapeLabelIdx, (ShapeLabel, ShapeExpr)>,
    shape_label_counter: ShapeLabelIdx,
    prefixmap: PrefixMap,
}

impl CompiledSchema {
    pub fn new() -> CompiledSchema {
        CompiledSchema {
            shape_labels_map: HashMap::new(),
            shape_label_counter: ShapeLabelIdx::default(),
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
        }
    }

    pub fn set_prefixmap(&mut self, prefixmap: Option<PrefixMap>) {
        self.prefixmap = prefixmap.clone().unwrap_or_default();
    }

    pub fn add_shape(&mut self, shape_label: ShapeLabel, se: ShapeExpr) {
        let idx = self.shape_label_counter;
        self.shape_labels_map.insert(shape_label.clone(), idx);
        self.shapes.insert(idx, (shape_label.clone(), se));
        self.shape_label_counter.incr()
    }

    pub fn get_shape_expr(&self, shape_label: &ShapeLabel) -> Option<&ShapeExpr> {
        if let Some(idx) = self.find_shape_label_idx(shape_label) {
            self.shapes.get(idx).map(|(_label, se)| se)
        } else {
            None
        }
    }

    pub fn from_schema_json(&mut self, schema_json: &SchemaJson) -> Result<()> {
        let mut schema_json_compiler = SchemaJsonCompiler::new();
        schema_json_compiler.compile(schema_json, self)?;
        Ok(())
    }

    pub fn find_ref(&mut self, se_ref: &ShapeExprLabel) -> CResult<ShapeLabelIdx> {
        let shape_label = match se_ref {
            ShapeExprLabel::IriRef { value } => match value {
                IriRef::Iri(iri) => {
                    let label = ShapeLabel::iri(iri.clone());
                    Ok::<ShapeLabel, CompiledSchemaError>(label)
                }
                IriRef::Prefixed { prefix, local } => {
                    let iri =
                        self.prefixmap
                            .resolve_prefix_local(prefix, local)
                            .map_err(|err| CompiledSchemaError::PrefixedNotFound {
                                prefix: prefix.clone(),
                                local: local.clone(),
                                err: Box::new(err),
                            })?;
                    Ok::<ShapeLabel, CompiledSchemaError>(ShapeLabel::iri(iri))
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
            None => Err(CompiledSchemaError::LabelNotFound { shape_label }),
        }
    }

    pub fn find_label(&self, label: &ShapeLabel) -> Option<(&ShapeLabelIdx, &ShapeExpr)> {
        self.find_shape_label_idx(label)
            .and_then(|idx| self.shapes.get(idx).map(|(_label, se)| (idx, se)))
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
        self.shapes.values()
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
            None => Err(CompiledSchemaError::ShapeLabelNotFound {
                shape_label: shape_label.clone(),
            }),
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
    use super::CompiledSchema;
    use crate::ast::Schema as SchemaJson;

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
