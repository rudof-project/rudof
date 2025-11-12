use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SuggestShexSchemaPromptArgs {
    /// Optional domain or focus area for the schema
    pub domain: Option<String>,
}

pub async fn suggest_shex_schema_prompt_impl(
    Parameters(args): Parameters<SuggestShexSchemaPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let domain_context = args.domain.unwrap_or_else(|| "the loaded data".to_string());
    
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!("Suggest a ShEx schema for {}", domain_context)
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "I'll help you create a ShEx schema for {}.\n\n\
                **Step-by-step approach:**\n\n\
                1. **Analyze the data structure:**\n\
                   - Use `execute_sparql_query` to find all classes:\n\
                     `SELECT DISTINCT ?class WHERE {{ ?s a ?class }}`\n\
                   - Identify properties for each class:\n\
                     `SELECT DISTINCT ?p WHERE {{ ?s a <ClassURI> ; ?p ?o }}`\n\n\
                2. **Create shape definitions:**\n\
                   - Define a shape for each class\n\
                   - Specify required vs optional properties\n\
                   - Add value constraints (datatype, nodeKind, pattern)\n\n\
                3. **Example ShEx structure:**\n\
                ```shex\n\
                PREFIX ex: <http://example.org/>\n\
                PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n\n\
                ex:PersonShape {{\n\
                  a [ex:Person] ;\n\
                  ex:name xsd:string ;\n\
                  ex:age xsd:integer ? ;\n\
                  ex:email xsd:string {{1,5}} ;\n\
                  ex:knows @ex:PersonShape *\n\
                }}\n\
                ```\n\n\
                Would you like me to:\n\
                1. Analyze your data to suggest specific shapes?\n\
                2. Explain ShEx syntax and constraints?\n\
                3. Validate existing data against a schema?",
                domain_context
            )
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!("ShEx schema suggestions for {}", domain_context)),
        messages,
    })
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExplainValidationErrorsPromptArgs {
    /// Validation result to analyze (JSON format)
    pub validation_result: String,
}

pub async fn explain_validation_errors_prompt_impl(
    Parameters(args): Parameters<ExplainValidationErrorsPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!("Explain these validation errors:\n{}", args.validation_result)
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            "I'll help you understand and fix these validation errors.\n\n\
            **Common ShEx validation error types:**\n\n\
            1. **Missing required property:**\n\
               - Error: Node doesn't have a required property\n\
               - Fix: Add the missing triple to your data\n\n\
            2. **Wrong datatype:**\n\
               - Error: Value doesn't match expected type (e.g., string vs integer)\n\
               - Fix: Convert the value to the correct type\n\n\
            3. **Cardinality violation:**\n\
               - Error: Too many or too few values (e.g., requires exactly 1, found 0 or 2)\n\
               - Fix: Adjust the number of values to match the constraint\n\n\
            4. **Pattern mismatch:**\n\
               - Error: String doesn't match required regex pattern\n\
               - Fix: Update the value to match the pattern or adjust the schema\n\n\
            5. **Shape mismatch:**\n\
               - Error: Referenced node doesn't conform to expected shape\n\
               - Fix: Ensure referenced nodes validate against their shapes\n\n\
            **To fix your specific errors:**\n\
            1. Review the validation result details\n\
            2. Use `node_info` to inspect problematic nodes\n\
            3. Update your RDF data or adjust the ShEx schema\n\
            4. Re-validate using `validate_with_shex`\n\n\
            Which error would you like to tackle first?"
        ),
    ];

    Ok(GetPromptResult {
        description: Some("Detailed explanation of validation errors".to_string()),
        messages,
    })
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DebugShexSchemaPromptArgs {
    /// ShEx schema to debug
    pub schema: String,
    /// Optional error message or issue description
    pub error_message: Option<String>,
}

pub async fn debug_shex_schema_prompt_impl(
    Parameters(args): Parameters<DebugShexSchemaPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let error_context = args.error_message
        .map(|e| format!("\nError: {}", e))
        .unwrap_or_default();
    
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!("Debug this ShEx schema:\n```shex\n{}\n```{}", args.schema, error_context)
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            "I'll help you debug this ShEx schema.\n\n\
            **Common ShEx Schema Issues:**\n\n\
            1. **Syntax errors:**\n\
               - Missing semicolons between properties\n\
               - Incorrect prefix declarations\n\
               - Mismatched braces or brackets\n\n\
            2. **Reference errors:**\n\
               - Undefined shape references (@ShapeName)\n\
               - Circular shape dependencies\n\
               - Wrong namespace prefixes\n\n\
            3. **Cardinality mistakes:**\n\
               - `?` = 0 or 1\n\
               - `*` = 0 or more\n\
               - `+` = 1 or more\n\
               - `{m,n}` = between m and n\n\n\
            4. **Value constraint issues:**\n\
               - Wrong datatype (use xsd: prefix)\n\
               - Invalid regex patterns\n\
               - NodeKind mismatch (IRI, BNODE, LITERAL)\n\n\
            5. **Logical errors:**\n\
               - Overly restrictive constraints\n\
               - Ambiguous shape definitions\n\
               - Missing required properties\n\n\
            **Debugging steps:**\n\
            1. Load the schema using `load_shex_from_sources`\n\
            2. Try validating with sample data using `validate_with_shex`\n\
            3. Simplify the schema to isolate the problem\n\
            4. Check the ShEx specification for correct syntax\n\n\
            What specific issue are you encountering?"
        ),
    ];

    Ok(GetPromptResult {
        description: Some("ShEx schema debugging assistance".to_string()),
        messages,
    })
}
