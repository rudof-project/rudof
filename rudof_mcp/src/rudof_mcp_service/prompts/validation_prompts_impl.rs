use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
            format!(
                "Explain these validation errors:\n{}",
                args.validation_result
            ),
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
            Which error would you like to tackle first?",
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
