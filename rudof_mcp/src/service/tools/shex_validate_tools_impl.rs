use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use rudof_lib::formats::{ResultShExValidationFormat, ShExFormat, ShExValidationSortByMode};
use rudof_lib::types::{ResultShapeMap, ShExNode, ShExShapeLabel, ShExValidationStatus};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

use super::helpers::*;
use super::params::{ResultShExValidationFormatParam, ShExFormatParam, ShExSortByParam};

fn normalize_iri_str(s: &str, strict: bool) -> String {
    let trimmed = s.trim();
    if strict {
        return trimmed.to_string();
    }
    let is_bare_iri =
        !trimmed.starts_with('<') && !trimmed.starts_with('_') && !trimmed.starts_with('{') && trimmed.contains("://");
    if is_bare_iri {
        format!("<{}>", trimmed)
    } else {
        trimmed.to_string()
    }
}

/// Request parameters for ShEx validation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexRequest {
    /// ShEx schema to validate against. Accepts inline ShExC/ShExJ text, a URL, or a local file path.
    pub schema: String,

    /// Format of the input schema. Default: shexc.
    pub schema_format: Option<ShExFormatParam>,

    /// Base IRI for resolving relative IRIs in the schema. Example: "http://example.org/"
    pub base_schema: Option<String>,

    /// Base IRI for resolving relative IRIs in node identifiers.
    pub base_data: Option<String>,

    /// Node IRI to validate. When set without `shapemap`, auto-generates a compact ShapeMap
    /// (e.g. "<node>@<shape>" or "<node>@START"). Mutually exclusive with `shapemap`.
    pub maybe_node: Option<String>,

    /// Shape IRI or prefixed name to validate `maybe_node` against.
    /// Only used when `maybe_node` is set. When omitted, uses the START shape.
    pub maybe_shape: Option<String>,

    /// Explicit ShapeMap mapping nodes to shapes. Example: ":alice@:Person, :bob@:Employee".
    /// Overrides `maybe_node`/`maybe_shape` when provided. Compact syntax only.
    pub shapemap: Option<String>,

    /// Output format for validation results. Default: details.
    pub result_format: Option<ResultShExValidationFormatParam>,

    /// Sort order for results. Default: node.
    pub sort_by: Option<ShExSortByParam>,

    /// IRI parsing strictness for `maybe_node` and `maybe_shape`.
    /// false (default): bare "http://..." IRIs are auto-wrapped in "<>".
    /// true: bare IRIs cause a parse error — use "<http://...>" explicitly.
    pub strict_iris: Option<bool>,

    /// External-shape resolver specs (applied in order, before schema loading).
    /// Each entry follows the grammar `<kind>[:<arg>]`. Built-in kinds:
    /// "reject-all" (rejects any unhandled EXTERNAL) and "schema:<path>"
    /// (substitutes EXTERNAL declarations from a ShEx file). Read the
    /// resource at `rudof://shex/external-resolvers` to enumerate.
    pub external_resolvers: Option<Vec<String>>,
}

/// Response containing ShEx validation results.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexResponse {
    /// Whether every entry of the shapemap conforms to the shape it was mapped to.
    pub conforms: bool,
    /// How many entries reached each status.
    pub counts: ShExStatusCounts,
    /// One entry per `(node, shape)` pair of the shapemap.
    pub entries: Vec<ShExEntry>,
    /// The results as rendered in `result_format`.
    pub report: String,
    /// Result format used
    pub result_format: ResultShExValidationFormatParam,
    /// Sort order applied
    pub sort_by: ShExSortByParam,
    /// Size of the rendered results in bytes
    pub result_size_bytes: usize,
}

/// How many shapemap entries reached each validation status.
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ShExStatusCounts {
    /// Entries whose node conforms to its shape.
    pub conformant: usize,
    /// Entries whose node does not conform to its shape.
    pub non_conformant: usize,
    /// Entries the validator did not decide.
    pub pending: usize,
    /// Entries the validator found both conformant and non-conformant.
    pub inconsistent: usize,
}

/// The outcome of validating one node against one shape.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShExEntry {
    /// The node validated.
    pub node: ShExNode,
    /// The shape it was validated against.
    pub shape: ShExShapeLabel,
    /// The status it reached, with the validator's reason for it.
    pub status: ShExValidationStatus,
}

/// Reads the shapemap the validator left behind into the entries and their counts,
/// in the same order the rendered report lists them.
fn shex_entries(results: &ResultShapeMap, sort_by: &ShExValidationSortByMode) -> (Vec<ShExEntry>, ShExStatusCounts) {
    let mut counts = ShExStatusCounts::default();
    let entries = results
        .iter_sorted(&(*sort_by).into())
        .map(|(node, shape, status)| {
            match status {
                ShExValidationStatus::Conformant(_) => counts.conformant += 1,
                ShExValidationStatus::NonConformant(_) => counts.non_conformant += 1,
                ShExValidationStatus::Pending => counts.pending += 1,
                ShExValidationStatus::Inconsistent(_, _) => counts.inconsistent += 1,
            }
            ShExEntry {
                node: node.clone(),
                shape: shape.clone(),
                status: status.clone(),
            }
        })
        .collect();
    (entries, counts)
}

/// Validate RDF data against a ShEx schema.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - No shapemap is given, directly or through `maybe_node`
/// - The schema or the shapemap cannot be resolved, parsed or validated
///
/// Returns a Protocol Error for internal validation failures.
pub async fn validate_shex_impl(
    service: &RudofMcpService,
    Parameters(ValidateShexRequest {
        schema,
        schema_format,
        base_schema,
        base_data,
        maybe_node,
        maybe_shape,
        shapemap,
        result_format,
        sort_by,
        strict_iris,
        external_resolvers,
    }): Parameters<ValidateShexRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;
    let session_dir = service.session_dir.read().await.clone();

    let parsed_schema = match resolve_spec(&session_dir, &schema, "schema") {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_schema_format: ShExFormat = schema_format.unwrap_or(ShExFormatParam::ShExC).into();

    let strict = strict_iris.unwrap_or(false);

    // Build effective ShapeMap: explicit shapemap wins; fall back to auto-generating
    // a compact ShapeMap from maybe_node + maybe_shape when only those are provided.
    let effective_shapemap = match (shapemap, maybe_node.as_deref(), maybe_shape.as_deref()) {
        (Some(sm), _, _) => sm,
        (None, Some(node), Some(shape)) => format!(
            "{}@{}",
            normalize_iri_str(node, strict),
            normalize_iri_str(shape, strict)
        ),
        (None, Some(node), None) => format!("{}@START", normalize_iri_str(node, strict)),
        (None, None, _) => {
            return Ok(ToolExecutionError::with_hint(
                "No shapemap provided",
                "Supply 'shapemap' (e.g. ':alice@:Person'), or 'maybe_node' with an optional 'maybe_shape'",
            )
            .into_call_tool_result());
        },
    };

    let parsed_shapemap = match resolve_spec(&session_dir, &effective_shapemap, "shapemap") {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let result_format = result_format.unwrap_or(ResultShExValidationFormatParam::Details);
    let parsed_result_format: ResultShExValidationFormat = result_format.into();
    let sort_by = sort_by.unwrap_or(ShExSortByParam::Node);
    let parsed_sort_by: ShExValidationSortByMode = sort_by.into();

    // Register external-shape resolver specs before loading the schema so the
    // compiler can apply them during AST→IR. Bad specs are reported as a tool
    // execution error with a hint pointing to the resource that enumerates kinds.
    if let Some(specs) = external_resolvers.as_ref() {
        for spec in specs {
            if let Err(e) = rudof.add_external_resolver(spec) {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid external resolver spec '{}': {}", spec, e),
                    "Read the rudof://shex/external-resolvers resource to enumerate valid kinds",
                )
                .into_call_tool_result());
            }
        }
    }

    let mut shex_schema_loading = rudof
        .load_shex_schema(&parsed_schema)
        .with_shex_schema_format(&parsed_schema_format);
    if let Some(base_schema) = base_schema.as_deref() {
        shex_schema_loading = shex_schema_loading.with_base(base_schema);
    }
    if let Err(e) = shex_schema_loading.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("Failed to load ShEx schema: {}", e),
            "Check the schema content and schema_format parameter",
        )
        .into_call_tool_result());
    }

    let mut shapemap_loading = rudof.load_shapemap(&parsed_shapemap);
    if let Some(base_data) = base_data.as_deref() {
        shapemap_loading = shapemap_loading.with_base_nodes(base_data);
    }
    if let Some(base_schema) = base_schema.as_deref() {
        shapemap_loading = shapemap_loading.with_base_shapes(base_schema);
    }
    if let Err(e) = shapemap_loading.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("Failed to load ShapeMap: {}", e),
            "Check shapemap syntax (e.g. ':alice@:Person') or the auto-generated ShapeMap from maybe_node/maybe_shape",
        )
        .into_call_tool_result());
    }

    if let Err(e) = rudof.validate_shex().execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("ShEx validation failed: {}", e),
            "Ensure the RDF data is loaded and the schema/shapemap are correct",
        )
        .into_call_tool_result());
    }

    let shapemap = rudof.shex_validation_results().ok_or_else(|| {
        internal_error(
            "Failed to read validation results",
            "Validation reported success but left no shapemap behind",
            Some(json!({"operation":"validate_shex_impl", "phase":"read_validation_results"})),
        )
    })?;
    let (entries, counts) = shex_entries(shapemap, &parsed_sort_by);
    let conforms = counts.non_conformant == 0 && counts.inconsistent == 0;

    let mut output_buffer = Cursor::new(Vec::new());
    let serialization = rudof
        .serialize_shex_validation_results(&mut output_buffer)
        .with_shex_validation_sort_order_mode(&parsed_sort_by)
        .with_result_shex_validation_format(&parsed_result_format);
    serialization.execute().map_err(|e| {
        internal_error(
            "Failed to serialize validation results",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"serialize_results"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"utf8_conversion"})),
        )
    })?;
    // Calculate metadata
    let result_size_bytes = output_str.len();

    let response = ValidateShexResponse {
        conforms,
        counts,
        entries,
        report: output_str.clone(),
        result_format,
        sort_by,
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "validate_shex_impl")?;

    let mut summary = format!(
        "ShEx validation completed.\nConforms: {}\nEntries: {} (conformant: {}, non-conformant: {}, pending: {}, inconsistent: {})\nResult format: {}\nSort by: {}\nResult size: {} bytes",
        response.conforms,
        response.entries.len(),
        response.counts.conformant,
        response.counts.non_conformant,
        response.counts.pending,
        response.counts.inconsistent,
        result_format,
        sort_by,
        result_size_bytes,
    );

    // Add ShapeMap source info
    if maybe_node.is_some() || maybe_shape.is_some() {
        summary.push_str(&format!("\nShapeMap (auto-generated): {}", effective_shapemap));
    }

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);

    Ok(result)
}
