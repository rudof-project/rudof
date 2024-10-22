use crate::ast::{serde_string_or_struct::*, SchemaJsonError};
use crate::ShapeExprLabel;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap, PrefixMapError};
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};
use tracing::debug;

use super::{IriOrStr, SemAct, ShapeDecl, ShapeExpr};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Schema {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    imports: Option<Vec<IriOrStr>>,

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

    /// Source IRI from which this Schema has been obtained
    #[serde(skip)]
    source_iri: IriS,
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
            source_iri: IriS::new_unchecked("http://default/"),
        }
    }

    /// Returns the list of import declared in the Schema
    pub fn imports(&self) -> Vec<IriOrStr> {
        if let Some(imports) = &self.imports {
            imports.to_vec()
        } else {
            let is: Vec<IriOrStr> = Vec::new();
            is
        }
    }

    pub fn source_iri(&self) -> IriS {
        self.source_iri.clone()
    }

    pub fn with_source_iri(&mut self, source_iri: &IriS) {
        self.source_iri = source_iri.clone();
    }

    /// Obtain a Schema from an IRI
    pub fn from_iri(iri: &IriS) -> Result<Schema, SchemaJsonError> {
        let body =
            iri.dereference(&Some(iri.clone()))
                .map_err(|e| SchemaJsonError::DereferencingIri {
                    iri: iri.clone(),
                    error: e,
                })?;
        let mut schema = Schema::from_reader(body.as_bytes())?;
        schema.with_source_iri(iri);
        Ok(schema)
    }

    pub fn with_import(mut self, i: IriOrStr) -> Self {
        match self.imports {
            None => self.imports = Some(vec![i]),
            Some(ref mut imports) => imports.push(i),
        }
        self
    }

    pub fn with_start_actions(mut self, start_actions: Option<Vec<SemAct>>) -> Self {
        self.start_acts = start_actions;
        self
    }

    pub fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), PrefixMapError> {
        match self.prefixmap {
            None => {
                let mut pm = PrefixMap::new();
                pm.insert(alias, iri)?;
                self.prefixmap = Some(pm);
            }
            Some(ref mut pm) => pm.insert(alias, iri)?,
        };
        Ok(())
    }

    pub fn with_prefixmap(mut self, prefixmap: Option<PrefixMap>) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    pub fn with_start(mut self, start: Option<ShapeExpr>) -> Self {
        self.start = start;
        self
    }

    pub fn with_shapes(mut self, shapes: Option<Vec<ShapeDecl>>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn resolve_iriref(&self, iri_ref: &IriRef) -> IriS {
        match &self.prefixmap {
            Some(pm) => match pm.resolve_iriref(iri_ref) {
                Err(_) => todo!(),
                Ok(iri) => iri.clone(),
            },
            None => match iri_ref {
                IriRef::Iri(iri) => iri.clone(),
                IriRef::Prefixed { prefix, local } => {
                    let str = format!("{prefix}{local}");
                    IriS::new_unchecked(str.as_str())
                }
            },
        }
    }

    pub fn add_shape(
        &mut self,
        shape_label: ShapeExprLabel,
        shape_expr: ShapeExpr,
        is_abstract: bool,
    ) {
        let sd: ShapeDecl = ShapeDecl::new(shape_label, shape_expr, is_abstract);
        match self.shapes {
            None => {
                self.shapes = Some(vec![sd]);
            }
            Some(ref mut ses) => ses.push(sd),
        }
    }

    pub fn add_shape_decl(&mut self, shape_decl: &ShapeDecl) {
        match self.shapes {
            None => self.shapes = Some(vec![shape_decl.clone()]),
            Some(ref mut ses) => ses.push(shape_decl.clone()),
        }
    }

    pub fn parse_schema_buf(path: &Path) -> Result<Schema, SchemaJsonError> {
        let schema = {
            let schema_str =
                fs::read_to_string(path).map_err(|e| SchemaJsonError::ReadingPathError {
                    path_name: path.display().to_string(),
                    error: e.to_string(),
                })?;
            serde_json::from_str::<Schema>(&schema_str).map_err(|e| SchemaJsonError::JsonError {
                path_name: path.display().to_string(),
                error: e.to_string(),
            })?
        };
        debug!("SchemaJson parsed: {:?}", schema);
        Ok(schema)
    }

    pub fn from_reader<R: io::Read>(rdr: R) -> Result<Schema, SchemaJsonError> {
        let schema = serde_json::from_reader::<R, Schema>(rdr).map_err(|e| {
            SchemaJsonError::JsonErrorFromReader {
                error: e.to_string(),
            }
        })?;
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

    pub fn start_actions(&self) -> Option<Vec<SemAct>> {
        self.start_acts.clone()
    }

    pub fn start(&self) -> Option<ShapeExpr> {
        self.start.clone()
    }

    pub fn shapes(&self) -> Option<Vec<ShapeDecl>> {
        self.shapes.clone()
    }

    pub fn get_type(&self) -> String {
        self.type_.clone()
    }

    pub fn find_shape_by_label(
        &self,
        label: &ShapeExprLabel,
    ) -> Result<Option<ShapeExpr>, SchemaJsonError> {
        match label {
            ShapeExprLabel::IriRef { value } => self.find_shape_by_iri_ref(value),
            ShapeExprLabel::BNode { value: _ } => todo!(),
            ShapeExprLabel::Start => todo!(),
        }
    }

    pub fn find_shape_by_iri_ref(
        &self,
        shape: &IriRef,
    ) -> Result<Option<ShapeExpr>, SchemaJsonError> {
        let prefixmap = match &self.prefixmap {
            None => PrefixMap::default(),
            Some(pm) => {
                // TODO: This should not be necessary
                pm.clone()
            }
        };
        if let Some(shapes) = self.shapes() {
            let expected_iri = prefixmap.resolve_iriref(shape)?;
            for shape_decl in shapes {
                if let ShapeExprLabel::IriRef { value } = shape_decl.id {
                    let iri = prefixmap.resolve_iriref(&value)?;
                    if iri == expected_iri {
                        return Ok(Some(shape_decl.shape_expr));
                    }
                }
            }
            // Not found in shapes
            Ok(None)
        } else {
            // No shapes
            Ok(None)
        }
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
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

        let schema: Schema = serde_json::from_str(str).unwrap();
        let serialized = serde_json::to_string_pretty(&schema).unwrap();
        let schema_after_serialization = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schema, schema_after_serialization);
    }
}
