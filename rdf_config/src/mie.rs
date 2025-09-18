use hashlink::LinkedHashMap;
use std::collections::HashMap;
use yaml_rust2::Yaml;

/// MIE: Metadata Interoperable Exchange Format
/// This is a custom format to capture metadata about RDF datasets and their schemas,
/// including shape expressions, example RDF data, SPARQL queries, and cross references.
/// It is designed to be extensible and human-readable, facilitating sharing and analysis of RDF schema information.
/// The main goal is to be used for MCP servers
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Mie {
    schema_info: SchemaInfo,
    prefixes: HashMap<String, String>,
    shape_expressions: HashMap<String, ShapeExpression>,
    // Example of RDF
    sample_rdf_entries: HashMap<String, RdfExample>,
    sparql_query_examples: HashMap<String, SparqlQueryExample>,
    // SPARQL queries employed for cross references
    cross_references: HashMap<String, CrossReference>,
    data_statistics: HashMap<String, DataStatistics>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DataStatistics {
    classes: isize,
    properties: isize,
    class_partitions: HashMap<String, isize>,
    property_partitions: HashMap<String, isize>,
    cross_references: HashMap<String, Option<isize>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SchemaInfo {
    title: Option<String>,
    description: Option<String>,
    endpoint: Option<String>,
    base_uri: Option<String>,
    // date_analyzed: Option<String>,
    // scope: Option<String>,
    graphs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeExpression {
    description: Option<String>,
    shape_expr: String,
    // target_class: Option<String>,
    // properties: HashMap<String, ValueDescription>,
}

/*#[derive(Clone, Debug, PartialEq)]
pub struct ValueDescription {
    _type: Option<String>,
    required: Option<bool>,
    description: Option<String>,
    path: Option<String>,
    pattern: Option<String>,
    cross_reference_pattern: Option<String>,
    example: Option<String>,
    note: Option<String>,
    values: Vec<String>,
    cardinality: Option<String>,
    subtypes: Vec<String>,
    classification_types: HashMap<String, ClassificationPattern>,
}*/

#[derive(Clone, Debug, PartialEq)]
pub struct ClassificationPattern {
    description: Option<String>,
    pattern: Option<String>,
    property_used: Option<String>,
    categories: HashMap<String, Category>,
    cross_reference_targets: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    String(String),
    List(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RdfExample {
    description: Option<String>,
    // reviewed: Option<bool>,
    // cross_references: Option<String>,
    rdf: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SparqlQueryExample {
    description: Option<String>,
    // tested: Option<bool>,
    // returns: Option<isize>,
    sparql: String,
    other_fields: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CrossReference {
    // id: String,
    description: Option<String>,
    sparql: String,
}

impl Mie {
    pub fn new(
        schema_info: SchemaInfo,
        prefixes: HashMap<String, String>,
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

    pub fn add_endpoint(&mut self, endpoint: &str) {
        self.schema_info.endpoint = Some(endpoint.to_string());
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
                prefixes_yaml.insert(Yaml::String(k.clone()), Yaml::String(v.clone()));
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

#[cfg(test)]
mod tests {
    use yaml_rust2::YamlEmitter;

    use super::*;
    #[test]
    fn test_mie_creation() {
        let mut prefixes = HashMap::new();
        prefixes.insert("ex".to_string(), "http://example.org/".to_string());
        prefixes.insert(
            "rdf".to_string(),
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
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
                graphs: vec!["http://example.org/graph1".to_string()],
            },
            prefixes: prefixes,
            shape_expressions,
            sample_rdf_entries,
            sparql_query_examples,
            cross_references,
            data_statistics: HashMap::new(),
        };
        let mut str = String::new();
        let mut emitter = YamlEmitter::new(&mut str);
        emitter.dump(&mie.to_yaml()).unwrap();
        println!("YAML Output:\n{}", str);
        assert_eq!(mie.schema_info.title.unwrap(), "Example Schema");
    }
}
