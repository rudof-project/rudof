use crate::ast::{serde_string_or_struct::*, SchemaJsonError};
use crate::{Ref, ShapeLabel};
use iri_s::IriS;
use log::debug;
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

use super::{Iri, SemAct, ShapeDecl, ShapeExpr};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Schema {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    imports: Option<Vec<Iri>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_string_or_struct",
        deserialize_with = "deserialize_opt_string_or_struct"
    )]
    start: Option<ShapeExpr>,

    #[serde(default, rename = "startActs", skip_serializing_if = "Option::is_none")]
    start_acts: Option<Vec<SemAct>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shapes: Option<Vec<ShapeDecl>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefixmap: Option<PrefixMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<IriS>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            context: "http://www.w3.org/ns/shex.jsonld".to_string(),
            type_: "Schema".to_string(),
            imports: None,
            start: None,
            start_acts: None,
            shapes: None,
            prefixmap: None,
            base: None,
        }
    }

    pub fn with_import(mut self, i: Iri) -> Self {
        match self.imports {
            None => self.imports = Some(vec![i]),
            Some(ref mut imports) => imports.push(i),
        }
        self
    }

    pub fn add_prefix(&mut self, alias: &str, iri: &IriS) {
        match self.prefixmap {
            None => {
                let mut pm = PrefixMap::new();
                pm.insert(alias, iri);
                self.prefixmap = Some(pm);
            }
            Some(ref mut pm) => pm.insert(alias, iri),
        }
    }

    pub fn with_prefixmap(mut self, prefixmap: Option<PrefixMap>) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    pub fn add_shape(&mut self, shape_label: Ref, shape_expr: ShapeExpr) {
        let sd: ShapeDecl = ShapeDecl::new(shape_label, shape_expr);
        match self.shapes {
            None => {
                let mut ses = Vec::new();
                ses.push(sd);
                self.shapes = Some(ses);
            }
            Some(ref mut ses) => ses.push(sd),
        }
    }

    pub fn parse_schema_buf(path_buf: &PathBuf) -> Result<Schema, SchemaJsonError> {
        let schema = {
            let schema_str = fs::read_to_string(&path_buf.as_path()).map_err(|e| {
                SchemaJsonError::ReadingPathError {
                    path_name: path_buf.display().to_string(),
                    error: e,
                }
            })?;
            serde_json::from_str::<Schema>(&schema_str).map_err(|e| SchemaJsonError::JsonError {
                path_name: path_buf.display().to_string(),
                error: e,
            })?
        };
        debug!("SchemaJson parsed: {:?}", schema);
        Ok(schema)
    }

    pub fn parse_schema_name(schema_name: &String, base: &Path) -> Result<Schema, SchemaJsonError> {
        let json_path = Path::new(&schema_name);
        let mut attempt = PathBuf::from(base);
        attempt.push(json_path);
        Self::parse_schema_buf(&attempt)
    }

    pub fn base(&self) -> Option<IriS> {
        self.base.clone()
    }

    pub fn prefixmap(&self) -> Option<PrefixMap> {
        self.prefixmap.clone()
    }

    pub fn shapes(&self) -> Option<Vec<ShapeDecl>> {
        self.shapes.clone()
    }

    pub fn get_type(&self) -> String {
        self.type_.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deser_user() {
        let str = r#"
        {
            "type": "Schema",
            "shapes": [
              {
                "type": "ShapeDecl",
                "id": "http://example.org/User",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                        "type": "TripleConstraint",
                        "predicate": "http://schema.org/name",
                        "valueExpr": {
                          "type": "NodeConstraint",
                          "datatype": "http://www.w3.org/2001/XMLSchema#string",
                          "length": 3
                        }
                  }
                }
              }
            ],
            "@context": "http://www.w3.org/ns/shex.jsonld"
          }
        "#;

        let schema: Schema = serde_json::from_str(&str).unwrap();
        let serialized = serde_json::to_string_pretty(&schema).unwrap();
        println!("{}", serialized);
        let schema_after_serialization = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schema, schema_after_serialization);
    }
}
