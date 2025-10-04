use crate::context_entry_value::ContextEntryValue;
use crate::manifest::Manifest;
use crate::manifest_error::ManifestError;
use crate::manifest_map::ManifestMap;
use ValidationType::*;
use iri_s::IriS;
use prefixmap::IriRef;
use serde::de::{self};
use serde::{Deserialize, Deserializer, Serialize};
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::shapemap::ValidationStatus;
use shex_ast::{Node, ast::Schema as SchemaJson, ir::ast2ir::AST2IR};
use shex_validation::Validator;
use shex_validation::ValidatorConfig;
use srdf::Object;
use srdf::RDFFormat;
use srdf::SLiteral;
use srdf::srdf_graph::SRDFGraph;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::path::Path;
use std::str::FromStr;
use tracing::{debug, trace};

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

impl From<ManifestValidationJson> for ManifestValidation {
    fn from(m: ManifestValidationJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map: HashMap<String, ValidationEntry> = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestValidation {
            entry_names: names,
            map,
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
    map: Option<String>,
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

impl Display for Focus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Focus::Single(str) => write!(f, "{}", str),
            Focus::Typed(str, str_type) => write!(f, "{}^^{}", str, str_type),
        }
    }
}

fn change_extension(name: String, old_extension: String, new_extension: String) -> String {
    if name.ends_with(&old_extension) {
        let (first, _) = name.split_at(name.len() - old_extension.len());
        format!("{first}{new_extension}")
    } else {
        name
    }
}

fn parse_schema(
    schema: &String,
    folder: &Path,
    _base: Option<&str>,
    entry_name: &String,
) -> Result<SchemaJson, Box<ManifestError>> {
    let new_schema_name =
        change_extension(schema.to_string(), ".shex".to_string(), ".json".to_string());

    debug!("schema: {}, new_schema_name: {}", schema, new_schema_name);
    SchemaJson::parse_schema_name(&new_schema_name, folder).map_err(|e| {
        Box::new(ManifestError::SchemaJsonError {
            error: Box::new(e),
            entry_name: entry_name.to_string(),
        })
    })
}

impl ValidationEntry {
    pub fn run(&self, folder: &Path) -> Result<(), Box<ManifestError>> {
        let base = Some("base:://");
        let graph = SRDFGraph::parse_data(
            &self.action.data,
            &RDFFormat::Turtle,
            folder,
            base,
            &srdf::ReaderMode::Strict,
        )
        .map_err(|e| Box::new(ManifestError::SRDFError(e)))?;
        trace!("Data obtained from: {}", self.action.data);

        let schema = parse_schema(&self.action.schema, folder, base, &self.name)?;
        trace!("Schema obtained from: {}", self.action.schema);

        trace!("Entry action: {:?}", self.action);

        trace!("Compiling schema...");
        let mut compiler = AST2IR::new();
        let mut compiled_schema = SchemaIR::new();
        compiler
            .compile(
                &schema,
                &IriS::from_path(folder).unwrap(),
                &mut compiled_schema,
            )
            .map_err(|e| Box::new(ManifestError::SchemaIRError(e)))?;
        let schema = compiled_schema.clone();
        let mut validator = Validator::new(compiled_schema, &ValidatorConfig::default())
            .map_err(ManifestError::ValidationError)?;
        let expected_type = parse_type(&self.type_)?;
        debug!("Schema compiled...expected type: {:?}", expected_type);
        trace!("Schema: {}", schema);

        let mut failed_status: Vec<ValidationStatus> = Vec::new();
        let mut passed_status: Vec<ValidationStatus> = Vec::new();
        if let Some(map) = &self.action.map {
            let map_path = folder.join(map);
            let str =
                std::fs::read_to_string(&map_path).map_err(|e| ManifestError::ReadingShapeMap {
                    error: e.to_string(),
                    entry: self.name.clone(),
                    map: map_path.clone(),
                })?;
            let manifest_map = serde_json::from_str::<ManifestMap>(&str).map_err(|e| {
                ManifestError::ParsingManifestMap {
                    error: e.to_string(),
                    entry: self.name.clone(),
                }
            })?;
            for entry in manifest_map.entries() {
                let node = parse_node(entry.node(), base)?;
                let shape = parse_shape(entry.shape())?;
                let result = validator
                    .validate_node_shape(&node, &shape, &graph, &schema, &None, &None)
                    .map_err(|e| Box::new(ManifestError::ValidationError(e)))?;

                let partial_status = result.get_info(&node, &shape).unwrap();
                if partial_status.is_conformant() {
                    passed_status.push(partial_status);
                } else {
                    failed_status.push(partial_status);
                }
            }
        }
        if let Some(focus) = &self.action.focus {
            trace!("Focus: {}", focus);
            let node = parse_focus(focus, base)?;
            let shape = parse_maybe_shape(&self.action.shape)?;
            trace!("Focus node: {}, shape: {}", node, shape);

            let result = validator
                .validate_node_shape(&node, &shape, &graph, &schema, &None, &None)
                .map_err(|e| Box::new(ManifestError::ValidationError(e)))?;
            let validation_status = result.get_info(&node, &shape).unwrap();
            if validation_status.is_conformant() {
                passed_status.push(validation_status);
            } else {
                failed_status.push(validation_status);
            }
        }
        match (expected_type, failed_status.is_empty()) {
            (Validation, true) => Ok(()),
            (Validation, _) => {
                debug!("Expected OK but failed {}", &self.name);
                Err(Box::new(ManifestError::ExpectedOkButObtained {
                    failed_status,
                    passed_status,
                    entry: Box::new(self.name.clone()),
                }))
            }
            (Failure, false) => Ok(()),
            (Failure, _) => {
                debug!("Expected Failure but passed {}", &self.name);
                Err(Box::new(ManifestError::ExpectedFailureButObtained {
                    failed_status,
                    passed_status,
                    entry: self.name.clone(),
                }))
            }
        }
    }
}

fn parse_maybe_shape(shape: &Option<String>) -> Result<ShapeLabel, Box<ManifestError>> {
    match &shape {
        None => Ok(ShapeLabel::Start),
        Some(str) => {
            let shape = parse_shape(str)?;
            Ok(shape)
        }
    }
}

/*
fn parse_maybe_focus(
    maybe_focus: &Option<Focus>,
    entry: &str,
    base: Option<&str>,
) -> Result<Node, ManifestError> {
    match maybe_focus {
        None => Err(ManifestError::NoFocusNode {
            entry: entry.to_string(),
        }),
        Some(focus) => {
            let node = parse_focus(focus, base)?;
            Ok(node)
        }
    }
}
*/

fn parse_focus(focus: &Focus, base: Option<&str>) -> Result<Node, Box<ManifestError>> {
    match focus {
        Focus::Single(str) => {
            trace!("Parsing focus node: {str}");
            let node = parse_node(str, base)?;
            trace!("Parsed focus node: {node}");
            Ok(node)
        }
        Focus::Typed(str, str_type) => {
            let datatype = IriS::from_str(str_type.as_str())
                .map_err(|e| Box::new(ManifestError::IriError(e)))?;
            Ok(Object::Literal(SLiteral::lit_datatype(str, &IriRef::Iri(datatype))).into())
        }
    }
}

fn parse_node(str: &str, base: Option<&str>) -> Result<Node, Box<ManifestError>> {
    Node::parse(str, base).map_err(|e| {
        Box::new(ManifestError::ParsingFocusNode {
            value: str.to_string(),
            error: Box::new(e),
        })
    })
}

fn parse_shape(str: &str) -> Result<ShapeLabel, Box<ManifestError>> {
    let node = Node::parse(str, None).map_err(|e| {
        Box::new(ManifestError::ParsingShapeLabel {
            value: str.to_string(),
            error: e.to_string(),
        })
    })?;
    let shape_label = ShapeLabel::from_object(node.as_object()).map_err(|e| {
        ManifestError::ParsingShapeLabel {
            value: str.to_string(),
            error: e.to_string(),
        }
    })?;
    Ok(shape_label)
}

fn parse_type(str: &str) -> Result<ValidationType, Box<ManifestError>> {
    match str {
        "sht:ValidationTest" => Ok(ValidationType::Validation),
        "sht:ValidationFailure" => Ok(ValidationType::Failure),
        _ => Err(Box::new(ManifestError::ParsingValidationType {
            value: str.to_string(),
        })),
    }
}

#[derive(Debug, PartialEq)]
enum ValidationType {
    Validation,
    Failure,
}

impl Manifest for ManifestValidation {
    fn len(&self) -> usize {
        self.entry_names.len()
    }

    fn is_empty(&self) -> bool {
        self.entry_names.is_empty()
    }

    fn entry_names(&self) -> Vec<String> {
        self.entry_names.clone() // iter().map(|n| n.clone()).collect()
    }

    fn run_entry(&self, name: &str, base: &Path) -> Result<(), Box<ManifestError>> {
        match self.map.get(name) {
            None => Err(Box::new(ManifestError::NotFoundEntry {
                name: name.to_string(),
            })),
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
            let manifest_str = fs::read_to_string(manifest_path).unwrap();
            serde_json::from_str::<ManifestValidation>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.entry_names.len(), 1166);
    }
}
