//! Configuration format v2.
//!
//! A v2 file is parsed strictly (unknown keys are errors) and lowered onto
//! [`GeneratorConfig`], the settings the engine reads. Scopes resolve as
//! `[[property]]` > `[[shape]]` > `[defaults]`.
//!
//! ```toml
//! version = 2
//! [prefixes]
//! ex = "http://example.org/"
//! [run]
//! seed = 42
//! scale = { entities = 1000 }        # or { per_shape = 100 }
//! [defaults]
//! fill = 1.0
//! cardinality = "balanced"
//! [[shape]]
//! match = "ex:PersonShape"
//! count = 300
//! [[property]]
//! match = { shape = "ex:PersonShape", path = "ex:nick" }
//! fill = 0.2
//! [output]
//! dir = "out"
//! formats = ["turtle"]
//! ```

use super::{
    CardinalityStrategy, DatatypeConfig, EntityDistribution, GeneratorConfig, OutputFormat, PropertyConfig,
    PropertyRule, TargetsConfig, TypeOverrideConfig, ViolationOperator, ViolationsConfig,
};
use crate::{DataGeneratorError, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

/// The only format version this module reads.
pub const VERSION: u32 = 2;

/// Prefixes every config can use without declaring them.
const BUILTIN_PREFIXES: [(&str, &str); 4] = [
    ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
    ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
    ("xsd", "http://www.w3.org/2001/XMLSchema#"),
    ("sh", "http://www.w3.org/ns/shacl#"),
];

/// Sections that are part of the v2 design but not implemented yet. They are
/// rejected rather than ignored, so a config never silently does less than it says.
const PLANNED_SECTIONS: [&str; 2] = ["validate", "profile"];

/// Tolerance of a `[targets]` table that sets none.
const DEFAULT_TOLERANCE: f64 = 0.01;

/// File names inside `output.dir`.
pub const TURTLE_FILE: &str = "data.ttl";
pub const NTRIPLES_FILE: &str = "data.nt";
pub const PG_FILE: &str = "data.pg";
pub const SHAPEMAP_FILE: &str = "data.shapemap";
pub const TYPEMAP_FILE: &str = "data.typemap";
pub const LABELS_FILE: &str = "labels.jsonl";
pub const RDF_LABELS_FILE: &str = "labels-rdf.jsonl";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigV2 {
    pub version: u32,
    #[serde(default)]
    pub prefixes: BTreeMap<String, String>,
    #[serde(default)]
    pub run: Run,
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default, rename = "shape")]
    pub shapes: Vec<ShapeRule>,
    #[serde(default, rename = "property")]
    pub properties: Vec<PropertyRuleV2>,
    #[serde(default)]
    pub values: Values,
    #[serde(default)]
    pub output: Output,
    pub violations: Option<Violations>,
    pub targets: Option<Targets>,
}

/// `[targets]`: one field per metric the generator can search for.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Targets {
    pub coherence: Option<f64>,
    pub tolerance: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Violations {
    pub rate: f64,
    pub operators: Option<Vec<String>>,
    pub shapes: Option<Vec<String>>,
    pub rdf: Option<RdfReadingV2>,
}

/// `[violations.rdf]`: how the RDF copy of a property graph reads (see
/// [`super::RdfReading`]). Both defaults are those of a faithful translation
/// (closed shapes, required keys).
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RdfReadingV2 {
    pub closed: Option<bool>,
    pub keys_required: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub seed: Option<u64>,
    pub threads: Option<usize>,
    pub scale: Option<Scale>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scale {
    /// Entities in total, split across shapes (equally, or by `[[shape]] weight`).
    pub entities: Option<usize>,
    /// Entities of every shape.
    pub per_shape: Option<usize>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Defaults {
    pub fill: Option<f64>,
    pub cardinality: Option<String>,
    pub unbounded_cap: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShapeRule {
    #[serde(rename = "match")]
    pub shape: String,
    pub count: Option<usize>,
    pub weight: Option<f64>,
    pub fill: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertyRuleV2 {
    #[serde(rename = "match")]
    pub target: PropertyMatch,
    pub fill: Option<f64>,
    pub cardinality: Option<String>,
    pub unbounded_cap: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertyMatch {
    pub shape: Option<String>,
    pub path: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Values {
    #[serde(default)]
    pub datatypes: HashMap<String, DatatypeConfig>,
    #[serde(default)]
    pub properties: HashMap<String, PropertyConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    /// Folder every output is written to, under fixed file names.
    pub dir: Option<PathBuf>,
    /// "turtle", "ntriples" and/or "pg"; with "pg", an RDF copy is "turtle".
    pub formats: Option<Vec<String>>,
    pub stats: Option<bool>,
}

impl ConfigV2 {
    /// Parse a v2 document, rejecting unknown keys and unsupported sections.
    pub fn parse(content: &str) -> Result<Self> {
        let table: toml::Table = toml::from_str(content)?;
        if !table.contains_key("version") {
            return Err(missing_version());
        }
        for section in PLANNED_SECTIONS {
            if table.contains_key(section) {
                return Err(config_error(format!(
                    "[{section}] is part of the v2 design but not yet supported"
                )));
            }
        }
        let config: Self = toml::from_str(content)?;
        config.check_version()
    }

    /// Parse a v2 document written as JSON.
    pub fn parse_json(content: &str) -> Result<Self> {
        let value: serde_json::Value = serde_json::from_str(content)?;
        for section in PLANNED_SECTIONS {
            if value.get(section).is_some() {
                return Err(config_error(format!(
                    "\"{section}\" is part of the v2 design but not yet supported"
                )));
            }
        }
        if value.get("version").is_none() {
            return Err(missing_version());
        }
        let config: Self = serde_json::from_value(value)?;
        config.check_version()
    }

    fn check_version(self) -> Result<Self> {
        if self.version != VERSION {
            return Err(config_error(format!(
                "unsupported config version {} (this build reads version {VERSION})",
                self.version
            )));
        }
        Ok(self)
    }

    /// Lower onto the settings the engine reads.
    pub fn lower(self) -> Result<GeneratorConfig> {
        let prefixes = self.prefix_table();
        let expand = |s: &str| expand_curie(s, &prefixes);
        let mut config = GeneratorConfig::default();

        // run
        config.generation.seed = self.run.seed;
        config.parallel.worker_threads = self.run.threads;
        let per_shape = match &self.run.scale {
            None => None,
            Some(Scale {
                entities: Some(entities),
                per_shape: None,
            }) => {
                config.generation.entity_count = *entities;
                None
            },
            Some(Scale {
                entities: None,
                per_shape: Some(n),
            }) => Some(*n),
            Some(_) => {
                return Err(config_error(
                    "run.scale takes either `entities` (a total) or `per_shape`, not both or neither".to_string(),
                ));
            },
        };

        // defaults
        if let Some(fill) = self.defaults.fill {
            config.generation.property_fill_probability = check_fill(fill, "defaults.fill")?;
        }
        if let Some(c) = &self.defaults.cardinality {
            config.generation.cardinality_strategy = cardinality(c, "defaults.cardinality")?;
        }
        if let Some(cap) = self.defaults.unbounded_cap {
            config.generation.unbounded_property_values = cap;
            config.generation.unbounded_reference_values = cap;
        }

        // shapes
        let mut counts = HashMap::new();
        let mut weights = HashMap::new();
        for shape in &self.shapes {
            let id = expand(&shape.shape)?;
            if let Some(count) = shape.count {
                counts.insert(id.clone(), count);
            }
            if let Some(weight) = shape.weight {
                weights.insert(id.clone(), weight);
            }
            if let Some(fill) = shape.fill {
                let entry = config
                    .generation
                    .type_overrides
                    .entry(id)
                    .or_insert_with(TypeOverrideConfig::default);
                entry.property_fill_probability = Some(check_fill(fill, "shape.fill")?);
            }
        }
        config.generation.entity_distribution = match (counts.is_empty(), weights.is_empty()) {
            _ if per_shape.is_some() && !weights.is_empty() => {
                return Err(config_error(
                    "`weight` splits a total; with run.scale.per_shape use `count` on [[shape]] tables".to_string(),
                ));
            },
            (_, true) if per_shape.is_some() => EntityDistribution::PerShape {
                default: per_shape.unwrap_or_default(),
                counts,
            },
            (true, true) => EntityDistribution::Equal,
            (false, true) => EntityDistribution::Custom(counts),
            (true, false) => EntityDistribution::Weighted(weights),
            (false, false) => {
                return Err(config_error(
                    "use either `count` or `weight` on [[shape]] tables, not both".to_string(),
                ));
            },
        };

        // properties
        for rule in &self.properties {
            config.generation.property_rules.push(PropertyRule {
                shape: rule.target.shape.as_deref().map(expand).transpose()?,
                path: expand(&rule.target.path)?,
                fill: rule.fill.map(|f| check_fill(f, "property.fill")).transpose()?,
                cardinality: rule
                    .cardinality
                    .as_deref()
                    .map(|c| cardinality(c, "property.cardinality"))
                    .transpose()?,
                unbounded_cap: rule.unbounded_cap,
            });
        }

        // values
        for (datatype, generator) in self.values.datatypes {
            config.field_generators.datatypes.insert(expand(&datatype)?, generator);
        }
        for (property, generator) in self.values.properties {
            config.field_generators.properties.insert(expand(&property)?, generator);
        }

        // output
        let dir = self.output.dir.clone().unwrap_or_else(|| PathBuf::from("."));
        let formats = self
            .output
            .formats
            .clone()
            .unwrap_or_else(|| vec!["turtle".to_string()]);
        for format in &formats {
            if !["turtle", "ntriples", "pg"].contains(&format.as_str()) {
                return Err(config_error(format!(
                    "output.formats: `{format}` is not supported (use \"turtle\", \"ntriples\" or \"pg\")"
                )));
            }
        }
        let has = |f: &str| formats.iter().any(|x| x == f);
        let pg = has("pg");
        if has("turtle") && has("ntriples") {
            return Err(config_error(
                "output.formats: choose one RDF format, \"turtle\" or \"ntriples\"".to_string(),
            ));
        }
        if pg && has("ntriples") {
            return Err(config_error(
                "output.formats: the RDF copy of a property graph is written as Turtle; use [\"pg\", \"turtle\"]"
                    .to_string(),
            ));
        }
        let rdf_copy = pg && has("turtle");
        if pg {
            config.output.format = OutputFormat::Pg;
            config.output.path = dir.join(PG_FILE);
            config.output.shapemap = Some(dir.join(TYPEMAP_FILE));
            if rdf_copy {
                config.output.rdf = Some(dir.join(TURTLE_FILE));
                config.output.rdf_shapemap = Some(dir.join(SHAPEMAP_FILE));
            }
        } else if has("ntriples") {
            config.output.format = OutputFormat::NTriples;
            config.output.path = dir.join(NTRIPLES_FILE);
            config.output.shapemap = Some(dir.join(SHAPEMAP_FILE));
        } else {
            config.output.format = OutputFormat::Turtle;
            config.output.path = dir.join(TURTLE_FILE);
            config.output.shapemap = Some(dir.join(SHAPEMAP_FILE));
        }
        if let Some(stats) = self.output.stats {
            config.output.write_stats = stats;
        }

        // targets
        if let Some(targets) = self.targets {
            let tolerance = targets.tolerance.unwrap_or(DEFAULT_TOLERANCE);
            if tolerance <= 0.0 {
                return Err(config_error(format!(
                    "targets.tolerance must be positive, got {tolerance}"
                )));
            }
            if let Some(coherence) = targets.coherence
                && !(0.0..=1.0).contains(&coherence)
            {
                return Err(config_error(format!(
                    "targets.coherence must be between 0.0 and 1.0, got {coherence}"
                )));
            }
            if targets.coherence.is_none() {
                return Err(config_error(
                    "[targets] sets no metric; supported: `coherence`".to_string(),
                ));
            }
            config.generation.targets = Some(TargetsConfig {
                coherence: targets.coherence,
                tolerance,
            });
        }

        // violations
        if let Some(v) = self.violations {
            if !(0.0..=1.0).contains(&v.rate) {
                return Err(config_error(format!(
                    "violations.rate must be between 0.0 and 1.0, got {}",
                    v.rate
                )));
            }
            let operators = match v.operators {
                None => ViolationOperator::ALL.to_vec(),
                Some(names) => names
                    .iter()
                    .map(|n| {
                        ViolationOperator::from_name(n).ok_or_else(|| {
                            let known: Vec<_> = ViolationOperator::ALL.iter().map(|o| o.name()).collect();
                            config_error(format!(
                                "unknown violation operator `{n}` (known: {})",
                                known.join(", ")
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
            };
            config.generation.violations = Some(ViolationsConfig {
                rate: v.rate,
                operators,
                shapes: v
                    .shapes
                    .unwrap_or_default()
                    .iter()
                    .map(|s| expand(s))
                    .collect::<Result<_>>()?,
                labels: Some(dir.join(LABELS_FILE)),
                rdf: match (rdf_copy, v.rdf) {
                    (true, reading) => {
                        let reading = reading.unwrap_or_default();
                        Some(super::RdfReading {
                            labels: dir.join(RDF_LABELS_FILE),
                            closed: reading.closed.unwrap_or(true),
                            keys_required: reading.keys_required.unwrap_or(true),
                        })
                    },
                    (false, None) => None,
                    (false, Some(_)) => {
                        return Err(config_error(
                            "[violations.rdf] reads the RDF copy of a property graph: set output.formats = [\"pg\", \"turtle\"]"
                                .to_string(),
                        ));
                    },
                },
            });
        }

        Ok(config)
    }

    fn prefix_table(&self) -> BTreeMap<String, String> {
        let mut table: BTreeMap<String, String> = BUILTIN_PREFIXES
            .iter()
            .map(|(p, iri)| (p.to_string(), iri.to_string()))
            .collect();
        table.extend(self.prefixes.clone());
        table
    }
}

/// Expand a CURIE (`ex:Thing`), strip `<…>`, or pass a full IRI through.
fn expand_curie(value: &str, prefixes: &BTreeMap<String, String>) -> Result<String> {
    if let Some(iri) = value.strip_prefix('<').and_then(|v| v.strip_suffix('>')) {
        return Ok(iri.to_string());
    }
    if value.contains("://") {
        return Ok(value.to_string());
    }
    match value.split_once(':') {
        Some((prefix, local)) => prefixes.get(prefix).map(|ns| format!("{ns}{local}")).ok_or_else(|| {
            config_error(format!(
                "undeclared prefix `{prefix}` in `{value}`; add it to [prefixes]"
            ))
        }),
        None => Err(config_error(format!("`{value}` is neither an IRI nor a CURIE"))),
    }
}

fn check_fill(fill: f64, key: &str) -> Result<f64> {
    if (0.0..=1.0).contains(&fill) {
        Ok(fill)
    } else {
        Err(config_error(format!("{key} must be between 0.0 and 1.0, got {fill}")))
    }
}

fn cardinality(value: &str, key: &str) -> Result<CardinalityStrategy> {
    match value {
        "min" => Ok(CardinalityStrategy::Minimum),
        "max" => Ok(CardinalityStrategy::Maximum),
        "uniform" => Ok(CardinalityStrategy::Random),
        "balanced" => Ok(CardinalityStrategy::Balanced),
        other => Err(config_error(format!(
            "{key} `{other}` is not supported (use \"min\", \"max\", \"uniform\" or \"balanced\")"
        ))),
    }
}

fn missing_version() -> DataGeneratorError {
    config_error(
        "the generator config needs `version = 2` (the v1 format was removed; see rudof_generate/examples/configs)"
            .to_string(),
    )
}

fn config_error(message: String) -> DataGeneratorError {
    DataGeneratorError::Config(message)
}
