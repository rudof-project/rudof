use crate::rudof_mcp_service::{errors::*, service::RudofMcpService};
use iri_s::IriS;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RudofConfig, result_shex_validation_format::ResultShExValidationFormat,
    shapemap_format::ShapeMapFormat, shex::validate_shex, shex_format::ShExFormat,
    sort_by_result_shape_map::SortByResultShapeMap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use srdf::ReaderMode;
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexRequest {
    /// ShEx schema to validate against.
    pub schema: String,

    /// ShEx Schema format [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads].
    pub schema_format: Option<String>,

    /// Base Schema (used to resolve relative IRIs in Schema). If not set, falls back to configuration or current working directory.
    pub base_schema: Option<String>,

    /// RDF Reader mode [default: strict] [possible values: lax, strict].
    pub reader_mode: Option<String>,

    /// Node selector (IRI or blank node) to validate.
    pub maybe_node: Option<String>,

    /// Shape label to validate the node against (default = START).
    pub maybe_shape: Option<String>,

    /// ShapeMap inline content mapping nodes to shapes.
    pub shapemap: Option<String>,

    /// ShapeMap format [default: compact] [possible values: compact, internal].
    pub shapemap_format: Option<String>,

    /// Ouput result format [default: compact] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, compact, json].
    pub result_format: Option<String>,

    /// Sorting mode for the output result table [default: node] [possible values: node, shape, status, details].
    pub sort_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexResponse {
    /// Validation result output.
    pub results: String,
}

pub async fn validate_shex_impl(
    service: &RudofMcpService,
    Parameters(ValidateShexRequest {
        schema,
        schema_format,
        base_schema,
        reader_mode,
        maybe_node,
        maybe_shape,
        shapemap,
        shapemap_format,
        result_format,
        sort_by,
    }): Parameters<ValidateShexRequest>,
) -> Result<CallToolResult, McpError> {
    let shcema_spec = Some(InputSpec::Str(schema.clone()));

    let parsed_schema_format: Option<ShExFormat> = match schema_format {
        Some(s) => Some(ShExFormat::from_str(&s).map_err(|e| {
            invalid_request(
                error_messages::INVALID_SCHEMA_FORMAT,
                Some(json!({ "error": e.to_string()})),
            )
        })?),
        None => None,
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request(
                error_messages::INVALID_BASE_IRI,
                Some(json!({ "error": e.to_string()})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request(
                error_messages::INVALID_READER_MODE,
                Some(json!({ "error": e.to_string()})),
            )
        })?,
        None => ReaderMode::Strict,
    };

    let shapemap_spec: Option<InputSpec> = match shapemap {
        Some(s) => Some(InputSpec::Str(s.clone())),
        None => None,
    };

    let parsed_shapemap_format: ShapeMapFormat = match shapemap_format {
        Some(s) => ShapeMapFormat::from_str(&s).map_err(|e| {
            invalid_request(
                error_messages::INVALID_SHAPEMAP_FORMAT,
                Some(json!({ "error": e.to_string()})),
            )
        })?,
        None => ShapeMapFormat::Compact,
    };

    let parsed_result_format: ResultShExValidationFormat = match result_format {
        Some(s) => ResultShExValidationFormat::from_str(&s).map_err(|e| {
            invalid_request(
                error_messages::INVALID_RESULT_SHEX_VALIDARION_FORMAT,
                Some(json!({ "error": e.to_string()})),
            )
        })?,
        None => ResultShExValidationFormat::Compact,
    };

    let parsed_sort_by: SortByResultShapeMap = match sort_by {
        Some(s) => SortByResultShapeMap::from_str(&s).map_err(|e| {
            invalid_request(
                error_messages::INVALID_RESULT_SHEX_VALIDARION_FORMAT,
                Some(json!({ "error": e.to_string()})),
            )
        })?,
        None => SortByResultShapeMap::Node,
    };

    let rudof_config = RudofConfig::new().unwrap();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    validate_shex(
        &mut rudof,
        &shcema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &maybe_node,
        &maybe_shape,
        &shapemap_spec,
        &parsed_shapemap_format,
        &parsed_result_format,
        &parsed_sort_by,
        &rudof_config,
        &mut output_buffer,
    )
    .map_err(|e| {
        internal_error(
            error_messages::QUERY_EXECUTION_ERROR,
            Some(json!({"error": e.to_string(),})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            error_messages::CONVERSION_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let response = ValidateShexResponse {
        results: output_str.to_string(),
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let text_output = format!(
        "ShEx validation executed successfully\n
        Results:\n{}",
        output_str
    );

    let mut result = CallToolResult::success(vec![Content::text(text_output)]);
    result.structured_content = Some(structured);

    Ok(result)
}
