use hashlink::LinkedHashMap;
use std::collections::HashMap;
use yaml_rust2::Yaml;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Mie {
    schema_info: SchemaInfo,
    prefixes: HashMap<String, String>,
    shape_expressions: HashMap<String, ShapeExpression>,
    sample_rdf_entries: HashMap<String, RdfExample>,
    sparql_query_examples: HashMap<String, SparqlQueryExample>,
    cross_references: HashMap<String, CrossReference>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SchemaInfo {
    title: Option<String>,
    description: Option<String>,
    endpoint: Option<String>,
    base_uri: Option<String>,
    date_analyzed: Option<String>,
    scope: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeExpression {
    description: Option<String>,
    target_class: Option<String>,
    properties: HashMap<String, ValueDescription>,
}

#[derive(Clone, Debug, PartialEq)]
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
}

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
    reviewed: Option<bool>,
    cross_references: Option<String>,
    rdf: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SparqlQueryExample {
    description: Option<String>,
    tested: Option<bool>,
    returns: Option<isize>,
    sparql: String,
    other_fields: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CrossReference {
    id: String,
    description: Option<String>,
    url: String,
}

impl Mie {
    pub fn new(
        schema_info: SchemaInfo,
        prefixes: HashMap<String, String>,
        shape_expressions: HashMap<String, ShapeExpression>,
        sample_rdf_entries: HashMap<String, RdfExample>,
        sparql_query_examples: HashMap<String, SparqlQueryExample>,
        cross_references: HashMap<String, CrossReference>,
    ) -> Self {
        Mie {
            schema_info,
            prefixes,
            shape_expressions,
            sample_rdf_entries,
            sparql_query_examples,
            cross_references,
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
        if let Some(date_analyzed) = &self.date_analyzed {
            result.insert(
                Yaml::String("date_analyzed".to_string()),
                Yaml::String(date_analyzed.clone()),
            );
        }
        if let Some(scope) = &self.scope {
            result.insert(
                Yaml::String("scope".to_string()),
                Yaml::String(scope.clone()),
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
            reviewed: None,
            cross_references: None,
            rdf: None,
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
        if let Some(reviewed) = &self.reviewed {
            result.insert(
                Yaml::String("reviewed".to_string()),
                Yaml::Boolean(*reviewed),
            );
        }
        if let Some(cross_references) = &self.cross_references {
            result.insert(
                Yaml::String("cross_references".to_string()),
                Yaml::String(cross_references.clone()),
            );
        }
        if let Some(rdf) = &self.rdf {
            result.insert(Yaml::String("rdf".to_string()), Yaml::String(rdf.clone()));
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
        if let Some(tc) = &self.target_class {
            result.insert(
                Yaml::String("description".to_string()),
                Yaml::String(tc.clone()),
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
        let mut protein_properties = HashMap::new();
        protein_properties.insert(
            "mnemonic".to_string(),
            ValueDescription {
                _type: Some("xsd:string".to_string()),
                required: Some(true),
                description: Some("Unique protein identifier".to_string()),
                pattern: None,
                example: Some("KAPCA_HUMAN".to_string()),
                note: None,
                values: vec![],
                cardinality: None,
                path: None,
                cross_reference_pattern: None,
                subtypes: Vec::new(),
                classification_types: HashMap::new(),
            },
        );

        let mut shape_expressions = HashMap::new();
        shape_expressions.insert(
            "Protein".to_string(),
            ShapeExpression {
                description: Some("A protein entity".to_string()),
                target_class: Some("ex:Protein".to_string()),
                properties: protein_properties,
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
                date_analyzed: Some("2024-10-01".to_string()),
                scope: Some("Protein structure, function taxonomy...".to_string()),
            },
            prefixes: prefixes,
            shape_expressions,
            sample_rdf_entries,
            sparql_query_examples,
            cross_references,
        };
        let mut str = String::new();
        let mut emitter = YamlEmitter::new(&mut str);
        emitter.dump(&mie.to_yaml()).unwrap();
        println!("YAML Output:\n{}", str);
        assert_eq!(mie.schema_info.title.unwrap(), "Example Schema");
    }
}
