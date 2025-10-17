use hashlink::LinkedHashMap;
use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use yaml_rust2::Yaml;

/// MIE: Metadata Interoperable Exchange Format
/// This is a custom format to capture metadata about RDF datasets and their schemas,
/// including shape expressions, example RDF data, SPARQL queries, and cross references.
/// It is designed to be extensible and human-readable, facilitating sharing and analysis of RDF schema information.
/// The main goal is to be used for MCP servers
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Mie {
    /// Basic information about the schema and endpoint
    schema_info: SchemaInfo,

    /// Prefixes defined in the endpoint
    prefixes: HashMap<String, IriS>,

    /// Shape expressions defined in the schema
    shape_expressions: HashMap<String, ShapeExpression>,

    /// Example of RDF
    sample_rdf_entries: HashMap<String, RdfExample>,

    /// SPARQL queries as examples
    sparql_query_examples: HashMap<String, SparqlQueryExample>,

    // SPARQL queries employed for cross references
    cross_references: HashMap<String, CrossReference>,

    /// Statistics about the data
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    data_statistics: HashMap<String, DataStatistics>,
}

/// Statistics about the data
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataStatistics {
    /// Number of classes
    #[serde(skip_serializing_if = "Option::is_none")]
    classes: Option<isize>,

    /// Number of properties
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<isize>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    class_partitions: HashMap<String, isize>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    property_partitions: HashMap<String, isize>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    cross_references: HashMap<String, Option<isize>>,
}

/// Basic information about the schema and endpoint
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SchemaInfo {
    /// Title of the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    /// Description of the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// SPARQL endpoint URL
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<String>,

    /// Base URI for the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    base_uri: Option<String>,

    /// Named graphs used in the endpoint
    #[serde(skip_serializing_if = "Vec::is_empty")]
    graphs: Vec<IriS>,
}

/// Shape expressions defined in the schema
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShapeExpression {
    /// Description of the Shape Expression
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Shape expressions content
    #[serde(skip_serializing_if = "String::is_empty")]
    shape_expr: String,
}

/// RDF examples
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RdfExample {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    rdf: String,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    other_fields: HashMap<String, String>,
}

/// SPARQL queries as examples
/// - description: Optional description of the query
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SparqlQueryExample {
    description: Option<String>,
    sparql: String,
    other_fields: HashMap<String, String>,
}

/// SPARQL queries used for cross-references
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CrossReference {
    description: Option<String>,
    sparql: String,
    other_fields: HashMap<String, String>,
}

impl Mie {
    pub fn new(
        schema_info: SchemaInfo,
        prefixes: HashMap<String, IriS>,
        shape_expressions: HashMap<String, ShapeExpression>,
        sample_rdf_entries: HashMap<String, RdfExample>,
        sparql_query_examples: HashMap<String, SparqlQueryExample>,
        cross_references: HashMap<String, CrossReference>,
        data_statistics: HashMap<String, DataStatistics>,
    ) -> Self {
        Mie {
            schema_info,
            prefixes,
            shape_expressions,
            sample_rdf_entries,
            sparql_query_examples,
            cross_references,
            data_statistics,
        }
    }

    pub fn add_endpoint(&mut self, endpoint: Option<&str>) {
        self.schema_info.endpoint = endpoint.map(|e| e.to_string());
    }

    pub fn add_title(&mut self, title: &str) {
        self.schema_info.title = Some(title.to_string());
    }

    pub fn add_graphs<I: Iterator<Item = IriS>>(&mut self, iter: I) {
        self.schema_info.graphs = iter.collect()
    }

    pub fn add_prefixes(&mut self, prefixes: HashMap<String, IriS>) {
        self.prefixes = prefixes;
    }

    pub fn to_yaml(&self) -> Yaml {
        let mut result = LinkedHashMap::new();
        result.insert(
            Yaml::String("schema_info".to_string()),
            self.schema_info.to_yaml(),
        );
        if !self.prefixes.is_empty() {
            let mut prefixes_yaml = LinkedHashMap::new();
            for (k, v) in &self.prefixes {
                prefixes_yaml.insert(
                    Yaml::String(k.clone()),
                    Yaml::String(v.as_str().to_string()),
                );
            }
            result.insert(
                Yaml::String("prefixes".to_string()),
                Yaml::Hash(prefixes_yaml),
            );
        }
        if !self.shape_expressions.is_empty() {
            let mut shapes_yaml = LinkedHashMap::new();
            for (k, v) in &self.shape_expressions {
                shapes_yaml.insert(Yaml::String(k.clone()), v.to_yaml());
            }
            result.insert(
                Yaml::String("shape_expressions".to_string()),
                Yaml::Hash(shapes_yaml),
            );
        }
        Yaml::Hash(result)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_yaml_str(&self) -> String {
        let yaml = self.to_yaml();
        let mut str = String::new();
        let mut emitter = yaml_rust2::YamlEmitter::new(&mut str);
        emitter.dump(&yaml).unwrap();
        str
    }
}

impl SchemaInfo {
    pub fn to_yaml(&self) -> Yaml {
        let mut result = LinkedHashMap::new();
        if let Some(title) = &self.title {
            result.insert(
                Yaml::String("title".to_string()),
                Yaml::String(title.clone()),
            );
        }
        if let Some(desc) = &self.description {
            result.insert(
                Yaml::String("description".to_string()),
                Yaml::String(desc.clone()),
            );
        }
        if let Some(endpoint) = &self.endpoint {
            result.insert(
                Yaml::String("endpoint".to_string()),
                Yaml::String(endpoint.clone()),
            );
        }
        if let Some(base_uri) = &self.base_uri {
            result.insert(
                Yaml::String("base_uri".to_string()),
                Yaml::String(base_uri.clone()),
            );
        }
        /*if !self.scope.is_empty() {
            let scope_yaml: Vec<Yaml> =
                self.scope.iter().map(|s| Yaml::String(s.clone())).collect();
            result.insert(Yaml::String("scope".to_string()), Yaml::Array(scope_yaml));
        }*/
        Yaml::Hash(result)
    }
}

impl RdfExample {
    pub fn new() -> Self {
        RdfExample {
            description: None,
            rdf: "".to_string(),
            other_fields: HashMap::new(),
        }
    }

    pub fn to_yaml(&self) -> Yaml {
        let mut result = LinkedHashMap::new();
        if let Some(desc) = &self.description {
            result.insert(
                Yaml::String("description".to_string()),
                Yaml::String(desc.clone()),
            );
        }
        Yaml::Hash(result)
    }
}

impl ShapeExpression {
    pub fn to_yaml(&self) -> Yaml {
        let mut result = LinkedHashMap::new();
        if let Some(desc) = &self.description {
            result.insert(
                Yaml::String("description".to_string()),
                Yaml::String(desc.clone()),
            );
        }
        Yaml::Hash(result)
    }
}

impl Display for Mie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MIE")?;
        if let Some(title) = &self.schema_info.title {
            writeln!(f, " Title: {title}")?;
        }
        if let Some(desc) = &self.schema_info.description {
            writeln!(f, " Description: {desc}")?;
        }
        if let Some(endpoint) = &self.schema_info.endpoint {
            writeln!(f, " Endpoint: {endpoint}")?;
        }
        if let Some(base_uri) = &self.schema_info.base_uri {
            writeln!(f, " Base URI: {base_uri}")?;
        }
        if !self.schema_info.graphs.is_empty() {
            writeln!(f, " Graphs: {:?}", self.schema_info.graphs)?;
        }
        if !self.prefixes.is_empty() {
            writeln!(f, " Prefixes:")?;
            for (prefix, uri) in &self.prefixes {
                writeln!(f, "  - {prefix}: {uri}")?;
            }
        }
        if !self.shape_expressions.is_empty() {
            writeln!(f, " Shape Expressions:")?;
            for (name, shape_expr) in &self.shape_expressions {
                writeln!(f, "  - {}: {}", name, shape_expr.shape_expr)?;
                if let Some(desc) = &shape_expr.description {
                    writeln!(f, "      Description: {desc}")?;
                }
            }
        }
        if !self.sample_rdf_entries.is_empty() {
            writeln!(f, " Sample RDF Entries:")?;
            for (name, rdf_example) in &self.sample_rdf_entries {
                writeln!(f, "  - {name}: [RDF data omitted]")?;
                if let Some(desc) = &rdf_example.description {
                    writeln!(f, "      Description: {desc}")?;
                }
            }
        }
        if !self.sparql_query_examples.is_empty() {
            writeln!(f, " SPARQL Query Examples:")?;
            for (name, query_example) in &self.sparql_query_examples {
                writeln!(f, "  - {name}: [SPARQL query omitted]")?;
                if let Some(desc) = &query_example.description {
                    writeln!(f, "      Description: {desc}")?;
                }
            }
        }
        if !self.cross_references.is_empty() {
            writeln!(f, " Cross References:")?;
            for (name, cross_ref) in &self.cross_references {
                writeln!(f, "  - {name}: [SPARQL query omitted]")?;
                if let Some(desc) = &cross_ref.description {
                    writeln!(f, "      Description: {desc}")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use yaml_rust2::YamlEmitter;

    use super::*;
    #[test]
    fn test_mie_creation() {
        let mut prefixes = HashMap::new();
        prefixes.insert("ex".to_string(), iri!("http://example.org/"));
        prefixes.insert(
            "rdf".to_string(),
            iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        );

        let mut shape_expressions = HashMap::new();
        shape_expressions.insert(
            "Protein".to_string(),
            ShapeExpression {
                description: Some("A protein entity".to_string()),
                shape_expr: "ex:ProteinShape".to_string(),
            },
        );
        let mut sample_rdf_entries = HashMap::new();
        sample_rdf_entries.insert("human_kinase_example".to_string(), RdfExample::new());
        let sparql_query_examples = HashMap::new();
        let cross_references = HashMap::new();
        let mie = Mie {
            schema_info: SchemaInfo {
                title: Some("Example Schema".to_string()),
                description: Some("An example schema for testing".to_string()),
                endpoint: Some("http://example.org/sparql".to_string()),
                base_uri: Some("http://example.org/".to_string()),
                graphs: vec![iri!("http://example.org/graph1")],
            },
            prefixes,
            shape_expressions,
            sample_rdf_entries,
            sparql_query_examples,
            cross_references,
            data_statistics: HashMap::new(),
        };
        let mut str = String::new();
        let mut emitter = YamlEmitter::new(&mut str);
        emitter.dump(&mie.to_yaml()).unwrap();
        println!("YAML Output:\n{str}");
        assert_eq!(mie.schema_info.title.unwrap(), "Example Schema");
    }
}
