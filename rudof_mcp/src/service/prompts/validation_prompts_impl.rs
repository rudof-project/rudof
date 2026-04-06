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
    pub technology: ValidationTechnology,

    /// Optional: specific node to validate (IRI or prefixed name)
    pub node: Option<String>,

    /// Optional: specific shape to validate against
    pub shape: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ValidationTechnology {
    Shex,
    Shacl,
}

/// Guide users through validating RDF data against schemas.
pub async fn validation_guide_prompt_impl(
    Parameters(args): Parameters<ValidationGuidePromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let ValidationGuidePromptArgs {
        technology,
        node,
        shape,
    } = args;

    let node_display = node.as_deref().unwrap_or("<not specified>");
    let shape_display = shape.as_deref().unwrap_or("<not specified>");

    let (technology_key, technology_label, tool_name, schema_formats, example_schema, result_formats) =
        match technology {
            ValidationTechnology::Shex => (
                "shex",
                "SHEX",
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
            ValidationTechnology::Shacl => (
                "shacl",
                "SHACL",
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
                technology_label,
                node_display,
                shape_display
            ),
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "# {} Validation Guide\n\n\
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
                technology_label,
                technology_label,
                technology_key,
                if technology_key == "shex" { "shexc" } else { "turtle" },
                example_schema,
                tool_name,
                if technology_key == "shex" { "shexc" } else { "turtle" },
                if let Some(n) = node.as_deref() {
                    format!(",\n  \"maybe_node\": \"{n}\"")
                } else {
                    String::new()
                },
                if let Some(s) = shape.as_deref() {
                    format!(",\n  \"maybe_shape\": \"{s}\",\n",)
                } else {
                    ",\n".to_string()
                },
                schema_formats,
                result_formats,
            ),
        ),
    ];

    Ok(GetPromptResult::new(messages).with_description(format!(
        "{} validation guide{}{}",
        technology_label,
        if let Some(n) = node.as_deref() {
            format!(" for node {n}")
        } else {
            String::new()
        },
        if let Some(s) = shape.as_deref() {
            format!(" against shape {s}")
        } else {
            String::new()
        },
    )))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum StandardSparqlQueryType {
    Select,
    Construct,
    Ask,
    Describe,
}