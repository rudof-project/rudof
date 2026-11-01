//! `generate_data` tool: synthetic data from a ShEx, SHACL or PG-Schema schema, with
//! optional controlled violations and their ground-truth report.

use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use rudof_generate::config::{OutputFormat, RdfReading, ViolationOperator, ViolationsConfig};
use rudof_generate::{DataGenerator, GeneratorConfig};
use rudof_lib::formats::{DataFormat, InputSpec, ResultDataFormat};
use rudof_lib::{Rudof, RudofConfig};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use super::helpers::*;
use crate::service::{errors::*, mcp_service::RudofMcpService};

/// Entities generated when neither `entities` nor `config` sets a number.
const DEFAULT_ENTITIES: usize = 20;

/// Schema language of the input schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum GenerationSchemaFormatParam {
    #[serde(rename = "shex")]
    ShEx,
    #[serde(rename = "shacl")]
    Shacl,
    #[serde(rename = "pgschema")]
    PgSchema,
}

impl GenerationSchemaFormatParam {
    fn extension(self) -> &'static str {
        match self {
            Self::ShEx => "shex",
            Self::Shacl => "ttl",
            Self::PgSchema => "pgs",
        }
    }
}

impl Display for GenerationSchemaFormatParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ShEx => "shex",
            Self::Shacl => "shacl",
            Self::PgSchema => "pgschema",
        })
    }
}

/// Controlled violations: a share of the entities is broken on purpose, and the
/// report lists every node (and, for a property graph, edge) a correct validator
/// must flag.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ViolationsRequest {
    /// Probability that an entity receives one violation, from 0.0 to 1.0.
    pub rate: f64,

    /// Operator names, e.g. ["drop-required", "dangling-reference"]. All operators
    /// when omitted.
    pub operators: Option<Vec<String>>,

    /// PG-Schema only: whether the RDF reading of the graph keeps PG-Schema's closed
    /// records. Default: true. A ShEx or SHACL modeller would usually leave shapes open.
    pub closed: Option<bool>,

    /// PG-Schema only: whether keys without OPTIONAL stay required in the RDF reading.
    /// Default: true. A SHACL modeller would usually leave them optional.
    pub keys_required: Option<bool>,
}

/// Request parameters for data generation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenerateDataRequest {
    /// Schema to generate from: inline text or a local file path.
    pub schema: String,

    /// Language of the schema: shex, shacl or pgschema.
    pub schema_format: GenerationSchemaFormatParam,

    /// Number of entities to generate. Default: 20, or the one in `config`.
    pub entities: Option<usize>,

    /// Random seed; the same schema, configuration and seed give the same data.
    pub seed: Option<u64>,

    /// Probability that an optional property is generated, from 0.0 to 1.0.
    pub property_fill: Option<f64>,

    /// Generator configuration as TOML text (the format of `rudof generate --generator-config` (format v2)).
    /// The other parameters override it.
    pub config: Option<String>,

    /// Break part of the generated data on purpose and return what each language must flag.
    pub violations: Option<ViolationsRequest>,

    /// Load the generated RDF into the session, so that validate_shex, validate_shacl
    /// and execute_sparql_query run on it. Loading merges with data already loaded.
    /// Default: true.
    pub load: Option<bool>,
}

/// Response containing the generated data.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GenerateDataResponse {
    /// Language of the schema the data was generated from.
    pub schema_format: GenerationSchemaFormatParam,
    /// Number of triples generated.
    pub triple_count: usize,
    /// The generated graph as RDF (Turtle). From a PG-Schema, the RDF encoding of the
    /// property graph.
    pub data: String,
    /// ShapeMap (compact syntax) associating every node with the shape that generated it,
    /// ready for the `shapemap` argument of validate_shex.
    pub shapemap: String,
    /// PG-Schema only: the graph in rudof's property graph format.
    pub property_graph: Option<String>,
    /// PG-Schema only: the type map assigning every node and edge its type.
    pub type_map: Option<String>,
    /// With violations: the report of what a correct validator of the schema's language
    /// must flag, one object per node or edge.
    pub violations: Option<Vec<Value>>,
    /// PG-Schema with violations: the report for the same graph read as RDF, where a
    /// reference failure fails the node that holds the reference, transitively.
    pub rdf_violations: Option<Vec<Value>>,
    /// Whether the RDF data was loaded into the session.
    pub loaded: bool,
}

/// Generate synthetic data from a schema.
///
/// # Errors
///
/// Returns a Tool Execution Error when the schema cannot be read, the configuration or
/// the violations are invalid, or the generator fails on the schema.
pub async fn generate_data_impl(
    service: &RudofMcpService,
    Parameters(request): Parameters<GenerateDataRequest>,
) -> Result<CallToolResult, McpError> {
    let dir = tempfile::tempdir().map_err(|e| {
        internal_error(
            "Temporary directory error",
            e.to_string(),
            Some(json!({"operation":"generate_data_impl","phase":"tempdir"})),
        )
    })?;
    let generated = match generate(&request, dir.path()).await {
        Ok(generated) => generated,
        Err(error) => return Ok(error.into_call_tool_result()),
    };

    let triple_count = RudofMcpService::count_triples_in_ntriples(&to_ntriples(&generated.data)?);
    let load = request.load.unwrap_or(true);
    if load {
        let mut rudof = service.rudof.lock().await;
        rudof
            .load_data()
            .with_data(&[InputSpec::str(&generated.data)])
            .with_data_format(&DataFormat::Turtle)
            .execute()
            .map_err(|e| {
                internal_error(
                    "RDF load error",
                    e.to_string(),
                    Some(json!({"operation":"generate_data_impl","phase":"load_into_session"})),
                )
            })?;
        drop(rudof);
        if let Err(e) = service.persist_state_with(None).await {
            tracing::warn!("Failed to persist state after generating data: {}", e);
        }
    }

    let response = GenerateDataResponse {
        schema_format: request.schema_format,
        triple_count,
        data: generated.data,
        shapemap: generated.shapemap,
        property_graph: generated.property_graph,
        type_map: generated.type_map,
        violations: generated.violations,
        rdf_violations: generated.rdf_violations,
        loaded: load,
    };
    let structured = serialize_structured(&response, "generate_data_impl")?;

    let mut summary = format!(
        "Generated {} triples from a {} schema.",
        triple_count, response.schema_format
    );
    if let Some(labels) = &response.violations {
        summary.push_str(&format!(" The report lists {} nodes or edges to flag.", labels.len()));
    }
    if load {
        summary.push_str(" The data is loaded: pass `shapemap` to validate_shex to check it.");
    }
    summary.push_str("\n\n");
    summary.push_str(&code_block_preview(
        "turtle",
        &response.data,
        DEFAULT_CONTENT_PREVIEW_CHARS,
    ));

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);
    Ok(result)
}

/// The files of one generator run, read back.
struct Generated {
    data: String,
    shapemap: String,
    property_graph: Option<String>,
    type_map: Option<String>,
    violations: Option<Vec<Value>>,
    rdf_violations: Option<Vec<Value>>,
}

async fn generate(request: &GenerateDataRequest, dir: &Path) -> ToolResult<Generated> {
    let pg = request.schema_format == GenerationSchemaFormatParam::PgSchema;
    let schema_path = schema_path(&request.schema, request.schema_format, dir)?;
    let config = config(request, dir)?;

    let mut generator = DataGenerator::new(config.clone()).map_err(generation_error)?;
    match request.schema_format {
        GenerationSchemaFormatParam::ShEx => generator.load_shex_schema(&schema_path).await,
        GenerationSchemaFormatParam::Shacl => generator.load_shacl_schema(&schema_path).await,
        GenerationSchemaFormatParam::PgSchema => generator.load_pgschema_schema(&schema_path).await,
    }
    .map_err(|e| {
        ToolExecutionError::with_hint(
            format!("Could not read the {} schema: {e}", request.schema_format),
            "Check that `schema_format` matches the schema, and that the schema parses",
        )
    })?;
    generator.generate().await.map_err(generation_error)?;

    let read = |path: &Path| std::fs::read_to_string(path).map_err(generation_error);
    let violations = config.generation.violations.as_ref();
    Ok(Generated {
        data: read(&dir.join("data.ttl"))?,
        shapemap: read(&dir.join("data.shapemap"))?,
        property_graph: pg.then(|| read(&dir.join("graph.pg"))).transpose()?,
        type_map: pg.then(|| read(&dir.join("graph.typemap"))).transpose()?,
        violations: violations
            .and_then(|v| v.labels.as_deref())
            .map(read_labels)
            .transpose()?,
        rdf_violations: violations
            .and_then(|v| v.rdf.as_ref())
            .map(|r| read_labels(&r.labels))
            .transpose()?,
    })
}

/// The schema as a file: the given path, or inline text written to `dir`.
fn schema_path(schema: &str, format: GenerationSchemaFormatParam, dir: &Path) -> ToolResult<PathBuf> {
    match resolve_spec(schema, "schema")? {
        InputSpec::Path(path) => Ok(path),
        InputSpec::Str(text) => {
            let path = dir.join(format!("schema.{}", format.extension()));
            std::fs::write(&path, text).map_err(generation_error)?;
            Ok(path)
        },
        _ => Err(ToolExecutionError::with_hint(
            "The generator reads the schema from text or a local file",
            "Pass the schema inline, or as a local file path",
        )),
    }
}

/// The generator configuration: the TOML in `config`, overridden by the other
/// parameters, with every output written to `dir`.
fn config(request: &GenerateDataRequest, dir: &Path) -> ToolResult<GeneratorConfig> {
    let mut config = match &request.config {
        Some(toml) => GeneratorConfig::from_toml_str(toml).map_err(|e| {
            ToolExecutionError::with_hint(
                format!("Invalid generator configuration: {e}"),
                "Pass a TOML configuration as accepted by `rudof generate --generator-config` (format v2), or omit `config`",
            )
        })?,
        None => {
            let mut config = GeneratorConfig::default();
            config.generation.entity_count = DEFAULT_ENTITIES;
            config
        },
    };
    if let Some(entities) = request.entities {
        config.generation.entity_count = entities;
    }
    if request.seed.is_some() {
        config.generation.seed = request.seed;
    }
    if let Some(fill) = request.property_fill {
        check_probability(fill, "property_fill")?;
        config.generation.property_fill_probability = fill;
    }

    config.output.write_stats = false;
    config.output.parallel_writing = false;
    if request.schema_format == GenerationSchemaFormatParam::PgSchema {
        config.output.format = OutputFormat::Pg;
        config.output.path = dir.join("graph.pg");
        config.output.shapemap = Some(dir.join("graph.typemap"));
        config.output.rdf = Some(dir.join("data.ttl"));
        config.output.rdf_shapemap = Some(dir.join("data.shapemap"));
    } else {
        config.output.format = OutputFormat::Turtle;
        config.output.path = dir.join("data.ttl");
        config.output.shapemap = Some(dir.join("data.shapemap"));
    }

    if let Some(violations) = &request.violations {
        check_probability(violations.rate, "violations.rate")?;
        let operators = match &violations.operators {
            None => ViolationOperator::ALL.to_vec(),
            Some(names) => names
                .iter()
                .map(|name| {
                    ViolationOperator::from_name(name).ok_or_else(|| {
                        let known: Vec<_> = ViolationOperator::ALL.iter().map(|op| op.name()).collect();
                        ToolExecutionError::with_hint(
                            format!("Unknown violation operator `{name}`"),
                            format!("Known operators: {}", known.join(", ")),
                        )
                    })
                })
                .collect::<ToolResult<Vec<_>>>()?,
        };
        let rdf = (request.schema_format == GenerationSchemaFormatParam::PgSchema).then(|| RdfReading {
            labels: dir.join("rdf_labels.jsonl"),
            closed: violations.closed.unwrap_or(true),
            keys_required: violations.keys_required.unwrap_or(true),
        });
        config.generation.violations = Some(ViolationsConfig {
            rate: violations.rate,
            operators,
            shapes: Vec::new(),
            labels: Some(dir.join("labels.jsonl")),
            rdf,
        });
    } else {
        // A `[violations]` section in `config` would write its report outside `dir`.
        config.generation.violations = None;
    }
    Ok(config)
}

fn check_probability(value: f64, argument: &str) -> ToolResult<()> {
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(ToolExecutionError::new(format!(
            "`{argument}` must be between 0.0 and 1.0, got {value}"
        )))
    }
}

fn read_labels(path: &Path) -> ToolResult<Vec<Value>> {
    std::fs::read_to_string(path)
        .map_err(generation_error)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).map_err(generation_error))
        .collect()
}

fn generation_error(error: impl Display) -> ToolExecutionError {
    ToolExecutionError::new(format!("Data generation failed: {error}"))
}

/// Turtle text as N-Triples, through a scratch rudof instance, so that data already
/// loaded in the session is left out.
fn to_ntriples(turtle: &str) -> Result<String, McpError> {
    let mut rudof = Rudof::new(RudofConfig::new());
    let phase_error = |phase: &str, e: String| {
        internal_error(
            "Triple count error",
            e,
            Some(json!({"operation":"generate_data_impl","phase":phase})),
        )
    };
    rudof
        .load_data()
        .with_data(&[InputSpec::str(turtle)])
        .with_data_format(&DataFormat::Turtle)
        .execute()
        .map_err(|e| phase_error("parse_generated", e.to_string()))?;
    let mut ntriples = Vec::new();
    rudof
        .serialize_data(&mut ntriples)
        .with_result_data_format(&ResultDataFormat::NTriples)
        .execute()
        .map_err(|e| phase_error("serialize_generated", e.to_string()))?;
    Ok(String::from_utf8_lossy(&ntriples).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::tools::shex_validate_tools_impl::{ValidateShexRequest, validate_shex_impl};

    const SHEX: &str = r#"
        prefix :    <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :Person { :name xsd:string ; :worksFor @:Org }
        :Org    { :label xsd:string }
    "#;

    const PGSCHEMA: &str = r#"
        CREATE NODE TYPE ( PersonType : Person { name: STRING }) ;
        CREATE NODE TYPE ( OrgType : Org { label: STRING }) ;
        CREATE EDGE TYPE (:@PersonType) -[WorksForType : worksFor]-> (:@OrgType)
    "#;

    fn request(schema: &str, format: GenerationSchemaFormatParam) -> GenerateDataRequest {
        GenerateDataRequest {
            schema: schema.to_string(),
            schema_format: format,
            entities: Some(10),
            seed: Some(7),
            property_fill: None,
            config: None,
            violations: None,
            load: None,
        }
    }

    async fn run(service: &RudofMcpService, request: GenerateDataRequest) -> GenerateDataResponse {
        let result = generate_data_impl(service, Parameters(request))
            .await
            .expect("generation should not fail at the protocol level");
        assert_ne!(result.is_error, Some(true), "generation failed: {:?}", result.content);
        serde_json::from_value(result.structured_content.expect("structured content"))
            .expect("the structured content should be a GenerateDataResponse")
    }

    async fn validate(service: &RudofMcpService, schema: &str, shapemap: &str) -> bool {
        let result = validate_shex_impl(
            service,
            Parameters(ValidateShexRequest {
                schema: schema.to_string(),
                schema_format: None,
                base_schema: None,
                base_data: None,
                maybe_node: None,
                maybe_shape: None,
                shapemap: Some(shapemap.to_string()),
                result_format: None,
                sort_by: None,
                strict_iris: None,
                external_resolvers: None,
            }),
        )
        .await
        .expect("validation should not fail at the protocol level");
        result.structured_content.expect("structured content")["conforms"]
            .as_bool()
            .expect("conforms is a boolean")
    }

    #[tokio::test]
    async fn generated_data_is_loaded_and_valid_against_its_shex_schema() {
        let service = RudofMcpService::new();
        let response = run(&service, request(SHEX, GenerationSchemaFormatParam::ShEx)).await;

        assert!(response.loaded);
        assert!(response.triple_count > 0);
        assert!(validate(&service, SHEX, &response.shapemap).await);
    }

    #[tokio::test]
    async fn the_same_seed_gives_the_same_data() {
        let service = RudofMcpService::new();
        let mut first = request(SHEX, GenerationSchemaFormatParam::ShEx);
        first.load = Some(false);
        let mut second = request(SHEX, GenerationSchemaFormatParam::ShEx);
        second.load = Some(false);

        let triples = |turtle: &str| -> std::collections::BTreeSet<String> {
            to_ntriples(turtle).unwrap().lines().map(str::to_string).collect()
        };
        assert_eq!(
            triples(&run(&service, first).await.data),
            triples(&run(&service, second).await.data)
        );
    }

    #[tokio::test]
    async fn violations_fail_exactly_the_reported_nodes() {
        let service = RudofMcpService::new();
        let mut broken = request(SHEX, GenerationSchemaFormatParam::ShEx);
        broken.entities = Some(40);
        broken.violations = Some(ViolationsRequest {
            rate: 0.3,
            operators: Some(vec!["drop-required".to_string()]),
            closed: None,
            keys_required: None,
        });
        let response = run(&service, broken).await;
        let labels = response.violations.expect("a report");
        assert!(!labels.is_empty());

        // Every reported node fails, and every node outside the report conforms.
        let reported: std::collections::HashSet<&str> = labels
            .iter()
            .filter(|l| l["kind"] != "trap")
            .filter_map(|l| l["node"].as_str())
            .collect();
        for entry in response.shapemap.split(",\n").map(str::trim).filter(|e| !e.is_empty()) {
            let node = entry.split('@').next().unwrap().trim_matches(|c| c == '<' || c == '>');
            assert_eq!(
                !validate(&service, SHEX, entry).await,
                reported.contains(node),
                "{entry}"
            );
        }
    }

    #[tokio::test]
    async fn a_pgschema_gives_a_property_graph_and_its_rdf_encoding() {
        let service = RudofMcpService::new();
        let mut broken = request(PGSCHEMA, GenerationSchemaFormatParam::PgSchema);
        broken.violations = Some(ViolationsRequest {
            rate: 0.3,
            operators: None,
            closed: None,
            keys_required: None,
        });
        let response = run(&service, broken).await;

        assert!(response.property_graph.is_some_and(|pg| !pg.is_empty()));
        assert!(response.type_map.is_some_and(|map| !map.is_empty()));
        assert!(response.triple_count > 0);
        assert!(response.violations.is_some());
        assert!(response.rdf_violations.is_some());
    }

    #[tokio::test]
    async fn an_unknown_operator_is_a_tool_error() {
        let service = RudofMcpService::new();
        let mut broken = request(SHEX, GenerationSchemaFormatParam::ShEx);
        broken.violations = Some(ViolationsRequest {
            rate: 0.1,
            operators: Some(vec!["no-such-operator".to_string()]),
            closed: None,
            keys_required: None,
        });
        let result = generate_data_impl(&service, Parameters(broken)).await.unwrap();
        assert_eq!(result.is_error, Some(true));
    }
}
