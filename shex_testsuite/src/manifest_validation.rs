use crate::context_entry_value::ContextEntryValue;
use crate::manifest::Manifest;
use crate::manifest_error::ManifestError;
use iri_s::IriS;
use tracing::debug;
use prefixmap::IriRef;
use serde::de::{self};
use serde::{Serialize, Deserialize, Deserializer};
use shex_ast::compiled::compiled_schema::CompiledSchema;
use shex_ast::compiled::shape_label::ShapeLabel;
use shex_ast::{
    compiled::schema_json_compiler::SchemaJsonCompiler, ast::Schema as SchemaJson, Node 
};
use shex_validation::ResultValue;
use shex_validation::Validator;
use srdf::RDFFormat;
use srdf::literal::Literal;
use srdf::Object;
use srdf::srdf_graph::SRDFGraph;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use ValidationType::*;

#[derive(Deserialize, Debug)]
#[serde(from = "ManifestValidationJson")]
pub struct ManifestValidation {
    entry_names: Vec<String>,
    map: HashMap<String, ValidationEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestValidationJson {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestValidationGraph>,
}

impl<'a> From<ManifestValidationJson> for ManifestValidation {
    fn from(m: ManifestValidationJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map: HashMap<String, ValidationEntry> = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestValidation {
            entry_names: names,
            map: map,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestValidationGraph {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,

    #[serde(rename = "rdfs:comment")]
    comment: String,

    entries: Vec<ValidationEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ValidationEntry {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,
    action: Action,
    #[serde(rename = "extensionResults")]
    extension_results: Vec<ExtensionResult>,
    name: String,
    #[serde(rename = "trait")]
    trait_: Option<Vec<String>>,
    comment: String,
    status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Action {
    schema: String,
    shape: Option<String>,
    data: String,
    focus: Option<Focus>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ExtensionResult {
    extension: String,
    prints: String,
}

#[derive(Serialize, Debug, Clone)]
enum Focus {
    Single(String),
    Typed(String, String),
}

impl<'de> Deserialize<'de> for Focus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FocusVisitor;

        impl<'de> de::Visitor<'de> for FocusVisitor {
            type Value = Focus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Focus")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Focus::Single(value.to_string()))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if let Some("@value") = map.next_key()? {
                    let value: String = map.next_value()?;
                    if let Some("@type") = map.next_key()? {
                        let type_: String = map.next_value()?;
                        Ok(Focus::Typed(value, type_))
                    } else {
                        Err(de::Error::missing_field("@type"))
                    }
                } else {
                    Err(de::Error::missing_field("@value"))
                }
            }
        }
        deserializer.deserialize_any(FocusVisitor {})
    }
}

fn change_extension(name: String, old_extension: String, new_extension: String) -> String {
    if name.ends_with(&old_extension) {
        let (first, _) = name.split_at(name.len() - old_extension.len());
        format!("{}{}", first, new_extension)
    } else {
        name
    }
}

fn parse_schema(
    schema: &String,
    base: &Path,
    entry_name: &String,
) -> Result<SchemaJson, ManifestError> {
    let new_schema_name =
        change_extension(schema.to_string(), ".shex".to_string(), ".json".to_string());

    debug!("schema: {}, new_schema_name: {}", schema, new_schema_name);
    SchemaJson::parse_schema_name(&new_schema_name, base).map_err(|e| {
        ManifestError::SchemaJsonError {
            error: e,
            entry_name: entry_name.to_string(),
        }
    })
}

impl ValidationEntry {
    pub fn run(&self, base: &Path) -> Result<(), ManifestError> {
        let graph = SRDFGraph::parse_data(&self.action.data, &RDFFormat::Turtle, base)?;
        debug!("Data obtained from: {}", self.action.data);

        let schema = parse_schema(&self.action.schema, base, &self.name)?;
        debug!("Schema obtained from: {}", self.action.schema);

        let node = parse_maybe_focus(&self.action.focus, &self.name)?;
        debug!("Node: {}", node);

        let shape = parse_maybe_shape(&self.action.shape)?;
        debug!("Shape: {}", shape);

        let mut compiler = SchemaJsonCompiler::new();
        let mut compiled_schema = CompiledSchema::new();
        compiler.compile(&schema, &mut compiled_schema)?;
        let mut validator = Validator::new(compiled_schema);
        validator.validate_node_shape(&node, &shape, &graph)?;
        let type_ = parse_type(&self.type_)?;
        let result = validator.get_result(&node, &shape)?;
        match (type_, &result) {
            (Validation, ResultValue::Ok) => Ok(()),
            (Validation, _) => {
                debug!("Expected OK but failed {}", &self.name);
                Err(ManifestError::ExpectedOkButObtained {
                    value: result.clone(),
                    entry: self.name.clone(),
                })
            }
            (Failure, ResultValue::Failed) => Ok(()),
            (Failure, _) => {
                debug!("Expected Failure but passed {}", &self.name);
                Err(ManifestError::ExpectedFailureButObtained {
                    value: result.clone(),
                    entry: self.name.clone(),
                })
            }
        }
    }
}

fn parse_maybe_shape(shape: &Option<String>) -> Result<ShapeLabel, ManifestError> {
    match &shape {
        None => Ok(ShapeLabel::Start),
        Some(str) => {
            let shape = parse_shape(&str)?;
            Ok(shape)
        }
    }
}

fn parse_maybe_focus(maybe_focus: &Option<Focus>, entry: &str) -> Result<Node, ManifestError> {
    match maybe_focus {
        None => Err(ManifestError::NoFocusNode {
            entry: entry.to_string(),
        }),
        Some(focus) => {
            let node = parse_focus(focus)?;
            Ok(node)
        }
    }
}

fn parse_focus(focus: &Focus) -> Result<Node, ManifestError> {
    match focus {
        Focus::Single(str) => {
            let iri = IriS::from_str(str.as_str())?;
            Ok(iri.into())
        }
        Focus::Typed(str, str_type) => {
            let datatype = IriS::from_str(str_type.as_str())?;
            Ok(Object::Literal(Literal::datatype(str, &IriRef::Iri(datatype))).into())
        }
    }
}

fn parse_shape(str: &str) -> Result<ShapeLabel, ManifestError> {
    let shape_label = ShapeLabel::from_iri_str(str)?;
    Ok(shape_label)
}

fn parse_type(str: &str) -> Result<ValidationType, ManifestError> {
    match str {
        "sht:ValidationTest" => Ok(ValidationType::Validation),
        "sht:ValidationFailure" => Ok(ValidationType::Failure),
        _ => Err(ManifestError::ParsingValidationType {
            value: str.to_string(),
        }),
    }
}

enum ValidationType {
    Validation,
    Failure,
}

impl Manifest for ManifestValidation {
    fn len(&self) -> usize {
        self.entry_names.len()
    }

    fn entry_names(&self) -> Vec<String> {
        self.entry_names.clone() // iter().map(|n| n.clone()).collect()
    }

    fn run_entry(&self, name: &str, base: &Path) -> Result<(), ManifestError> {
        match self.map.get(name) {
            None => Err(ManifestError::NotFoundEntry {
                name: name.to_string(),
            }),
            Some(entry) => entry.run(base),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn count_validation_entries() {
        let manifest_path = Path::new("shexTest/validation/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestValidation>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.entry_names.len(), 1166);
    }
}
