//! Validation-related prompt implementations.
//!
//! These prompts guide users through validating RDF data against ShEx or SHACL schemas.

use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Arguments for the validation guide prompt.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidationGuidePromptArgs {
    /// Validation technology to use: 'shex' or 'shacl'
    pub technology: String,

    /// Optional: specific node to validate (IRI or prefixed name)
    pub node: Option<String>,

    /// Optional: specific shape to validate against
    pub shape: Option<String>,
}

/// Guide users through validating RDF data against schemas.
pub async fn validation_guide_prompt_impl(
    Parameters(args): Parameters<ValidationGuidePromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let technology = args.technology.to_lowercase();
    let node_display = args.node.as_deref().unwrap_or("<not specified>");
    let shape_display = args.shape.as_deref().unwrap_or("<not specified>");

    let (tool_name, schema_formats, example_schema, result_formats) = match technology.as_str() {
        "shex" => (
            "validate_shex",
            "shexc, shexj, turtle",
            r#"prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string ;
  :age xsd:integer ? ;
  :knows @:Person *
}"#,
            "compact, details, json, csv",
        ),
        "shacl" | _ => (
            "validate_shacl",
            "turtle, ntriples, rdfxml, jsonld",
            r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix : <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:property [
    sh:path :name ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
  ] ;
  sh:property [
    sh:path :age ;
    sh:datatype xsd:integer ;
    sh:maxCount 1 ;
  ] ."#,
            "compact, details, minimal, json, csv, turtle",
        ),
    };

    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Guide me through {} validation.\nNode: {}\nShape: {}",
                technology.to_uppercase(),
                node_display,
                shape_display
            ),
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "# ✅ {} Validation Guide\n\n\
                I'll help you validate your RDF data using **{}**.\n\n\
                ## Prerequisites\n\n\
                1. **Load your RDF data first** using `load_rdf_data_from_sources`\n\
                2. **Prepare your schema** (inline or from file/URL)\n\n\
                ## Example Schema\n\n\
                Here's a sample {} schema:\n\
                ```{}\n\
                {}\n\
                ```\n\n\
                ## Running Validation\n\n\
                Use the `{}` tool with these parameters:\n\n\
                ```json\n\
                {{\n\
                  \"schema\": \"<your schema here>\",\n\
                  \"schema_format\": \"{}\"{}{}
                  \"result_format\": \"details\"\n\
                }}\n\
                ```\n\n\
                ## Schema Format Options\n\
                Supported formats: `{}`\n\n\
                ## Result Format Options\n\
                Supported formats: `{}`\n\n\
                ## Interpreting Results\n\n\
                - **Conformant**: Node matches the shape constraints\n\
                - **Non-conformant**: Node violates one or more constraints\n\
                - **Pending**: Validation is incomplete (e.g., missing data)\n\n\
                ## Tips\n\n\
                - Start with `result_format: \"details\"` for verbose output\n\
                - Use `compact` for machine-readable summaries\n\
                - Check specific nodes with the `node` parameter\n\n\
                Would you like me to help you write a schema or run the validation?",
                technology.to_uppercase(),
                technology.to_uppercase(),
                technology,
                if technology == "shex" {
                    "shexc"
                } else {
                    "turtle"
                },
                example_schema,
                tool_name,
                if technology == "shex" {
                    "shexc"
                } else {
                    "turtle"
                },
                if args.node.is_some() {
                    format!(",\n  \"maybe_node\": \"{}\"", args.node.as_ref().unwrap())
                } else {
                    String::new()
                },
                if args.shape.is_some() {
                    format!(
                        ",\n  \"maybe_shape\": \"{}\",\n",
                        args.shape.as_ref().unwrap()
                    )
                } else {
                    ",\n".to_string()
                },
                schema_formats,
                result_formats,
            ),
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!(
            "{} validation guide{}{}",
            technology.to_uppercase(),
            if args.node.is_some() {
                format!(" for node {}", args.node.unwrap())
            } else {
                String::new()
            },
            if args.shape.is_some() {
                format!(" against shape {}", args.shape.unwrap())
            } else {
                String::new()
            },
        )),
        messages,
    })
}

/// Arguments for the SPARQL query builder prompt.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SparqlBuilderPromptArgs {
    /// What kind of query to build: 'select', 'construct', 'ask', 'describe', or describe your goal
    pub query_type: String,

    /// Optional: natural language description of what you want to query
    pub description: Option<String>,
}

/// Guide users through building SPARQL queries.
pub async fn sparql_builder_prompt_impl(
    Parameters(args): Parameters<SparqlBuilderPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let query_type = args.query_type.to_lowercase();
    let description = args
        .description
        .unwrap_or_else(|| "explore the data".to_string());

    let (query_template, explanation) = match query_type.as_str() {
        "select" => (
            r#"SELECT ?subject ?predicate ?object
WHERE {
  ?subject ?predicate ?object .
  # Add filters here
  # FILTER(?predicate = <http://example.org/name>)
}
LIMIT 100"#,
            "SELECT queries return tabular results (variable bindings).",
        ),
        "construct" => (
            r#"CONSTRUCT {
  ?s ?p ?o .
}
WHERE {
  ?s ?p ?o .
  # Add patterns to match
}
LIMIT 100"#,
            "CONSTRUCT queries return new RDF triples.",
        ),
        "ask" => (
            r#"ASK {
  ?s a <http://example.org/Person> .
}"#,
            "ASK queries return true/false based on pattern existence.",
        ),
        "describe" => (
            r#"DESCRIBE <http://example.org/resource>"#,
            "DESCRIBE queries return all triples about a resource.",
        ),
        _ => (
            r#"# Based on your description, here are some useful patterns:

# Find all types
SELECT DISTINCT ?type WHERE { ?s a ?type }

# Find instances of a type
SELECT ?instance WHERE { ?instance a <Type> }

# Find properties of a subject
SELECT ?p ?o WHERE { <Subject> ?p ?o }

# Count triples
SELECT (COUNT(*) AS ?count) WHERE { ?s ?p ?o }"#,
            "Here are common query patterns you can adapt.",
        ),
    };

    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Help me build a SPARQL query.\nType: {}\nGoal: {}",
                query_type, description
            ),
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "# 🔍 SPARQL Query Builder\n\n\
                I'll help you build a **{}** query to **{}**.\n\n\
                ## Query Template\n\n\
                {}\n\n\
                ```sparql\n\
                {}\n\
                ```\n\n\
                ## How to Execute\n\n\
                Use the `execute_sparql_query` tool:\n\
                ```json\n\
                {{\n\
                  \"query\": \"<your SPARQL query>\",\n\
                  \"result_format\": \"internal\"\n\
                }}\n\
                ```\n\n\
                Or describe your query in natural language:\n\
                ```json\n\
                {{\n\
                  \"query_natural_language\": \"{}\"\n\
                }}\n\
                ```\n\n\
                ## Result Format Options\n\
                - `internal`: Human-readable format (default)\n\
                - `csv`: Comma-separated values\n\
                - `json-ld`: JSON-LD format\n\
                - `turtle`: Turtle format (for CONSTRUCT)\n\n\
                ## Common SPARQL Patterns\n\n\
                | Pattern | Description |\n\
                |---------|-------------|\n\
                | `?s a ?type` | Find types |\n\
                | `?s ?p ?o` | Match any triple |\n\
                | `FILTER(...)` | Add conditions |\n\
                | `OPTIONAL {{...}}` | Optional patterns |\n\
                | `UNION {{...}}` | Alternative patterns |\n\
                | `GROUP BY` | Aggregate results |\n\n\
                Would you like me to refine this query or help with a specific pattern?",
                query_type.to_uppercase(),
                description,
                explanation,
                query_template,
                description,
            ),
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!(
            "SPARQL {} query builder: {}",
            query_type, description
        )),
        messages,
    })
}
