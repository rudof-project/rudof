use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeRdfDataPromptArgs {}

pub async fn analyze_rdf_data_prompt_impl(
    Parameters(_args): Parameters<AnalyzeRdfDataPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            "Analyze the loaded RDF data"
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            "I'll help you analyze the RDF data currently loaded in Rudof.\n\n\
            **Analysis steps:**\n\
            1. **Export the data** using the `export_rdf_data` tool to see the current triples\n\
            2. **Query patterns** using `execute_sparql_query` to find:\n\
               - All unique classes: `SELECT DISTINCT ?class WHERE { ?s a ?class }`\n\
               - All unique predicates: `SELECT DISTINCT ?p WHERE { ?s ?p ?o }`\n\
               - Triple count: `SELECT (COUNT(*) as ?count) WHERE { ?s ?p ?o }`\n\
            3. **Explore specific nodes** using `node_info` to understand relationships\n\
            4. **Visualize** using `export_rdf_plantuml` or `export_rdf_image` for a graph overview\n\n\
            **What insights would you like?**\n\
            - Data statistics (classes, predicates, instance counts)\n\
            - Graph structure and connectivity\n\
            - Data quality issues\n\
            - Schema/ontology patterns\n\n\
            Let me know what aspect you'd like to explore first!"
        ),
    ];

    Ok(GetPromptResult {
        description: Some("Comprehensive analysis of loaded RDF data".to_string()),
        messages,
    })
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenerateTestDataPromptArgs {
    /// ShEx schema to generate test data for
    pub schema: String,
    /// Number of example instances to generate
    pub num_examples: Option<u32>,
}

pub async fn generate_test_data_prompt_impl(
    Parameters(args): Parameters<GenerateTestDataPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let num = args.num_examples.unwrap_or(3);
    
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!("Generate {} test data examples for this schema:\n```shex\n{}\n```", num, args.schema)
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "I'll help you generate {} conformant RDF test data examples.\n\n\
                **Test data generation approach:**\n\n\
                1. **Analyze the schema:**\n\
                   - Identify all shapes and their properties\n\
                   - Note required vs optional properties\n\
                   - Check cardinality constraints\n\
                   - Review value constraints (datatypes, patterns, nodeKinds)\n\n\
                2. **Generate valid instances:**\n\
                   - Create example IRIs/BNodes for subjects\n\
                   - Provide values matching each constraint\n\
                   - Respect cardinality (e.g., if `+`, include at least 1)\n\
                   - Use appropriate datatypes\n\n\
                3. **Example generation:**\n\
                ```turtle\n\
                @prefix ex: <http://example.org/> .\n\
                @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n\
                # Example 1 - minimum required data\n\
                ex:instance1 a ex:MyClass ;\n\
                    ex:required \"value\"^^xsd:string .\n\n\
                # Example 2 - with optional properties\n\
                ex:instance2 a ex:MyClass ;\n\
                    ex:required \"value\"^^xsd:string ;\n\
                    ex:optional 42 .\n\n\
                # Example 3 - with multiple values\n\
                ex:instance3 a ex:MyClass ;\n\
                    ex:required \"value\"^^xsd:string ;\n\
                    ex:multi \"first\", \"second\" .\n\
                ```\n\n\
                4. **Validate the generated data:**\n\
                   - Load test data with `load_rdf_data_from_sources`\n\
                   - Validate against schema with `validate_with_shex`\n\
                   - Adjust if validation fails\n\n\
                Would you like me to help create specific test cases?",
                num
            )
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!("Generate {} RDF test data examples", num)),
        messages,
    })
}
