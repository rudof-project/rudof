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
//! scale = { entities = 1000 }
//! [defaults]
//! fill = 1.0
//! cardinality = "balanced"
//! [[shape]]
//! match = "ex:PersonShape"
//! count = 300
//! [[property]]
//! match = { shape = "ex:PersonShape", path = "ex:nick" }
//! fill = 0.2
//! ```

use super::{
    CardinalityStrategy, DatatypeConfig, EntityDistribution, GeneratorConfig, OutputFormat, PropertyConfig,
    PropertyRule, TypeOverrideConfig, ViolationOperator, ViolationsConfig,
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
const PLANNED_SECTIONS: [&str; 3] = ["validate", "targets", "profile"];

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
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Violations {
    pub rate: f64,
    pub operators: Option<Vec<String>>,
    pub shapes: Option<Vec<String>>,
    pub labels: Option<PathBuf>,
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
    pub entities: usize,
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
    pub path: Option<PathBuf>,
    pub format: Option<String>,
    pub stats: Option<bool>,
    pub shapemap: Option<PathBuf>,
}

impl ConfigV2 {
    /// Parse a v2 document, rejecting unknown keys and unsupported sections.
    pub fn parse(content: &str) -> Result<Self> {
        let table: toml::Table = toml::from_str(content)?;
        for section in PLANNED_SECTIONS {
            if table.contains_key(section) {
                return Err(config_error(format!(
                    "[{section}] is part of the v2 design but not yet supported"
                )));
            }
        }
        let config: Self = toml::from_str(content)?;
        if config.version != VERSION {
            return Err(config_error(format!(
                "unsupported config version {} (this build reads version {VERSION})",
                config.version
            )));
        }
        Ok(config)
    }

    /// Lower onto the settings the engine reads.
    pub fn lower(self) -> Result<GeneratorConfig> {
        let prefixes = self.prefix_table();
        let expand = |s: &str| expand_curie(s, &prefixes);
        let mut config = GeneratorConfig::default();

        // run
        config.generation.seed = self.run.seed;
        config.parallel.worker_threads = self.run.threads;
        if let Some(scale) = &self.run.scale {
            config.generation.entity_count = scale.entities;
        }

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
        if let Some(path) = self.output.path {
            config.output.path = path;
        }
        if let Some(format) = &self.output.format {
            config.output.format = match format.as_str() {
                "turtle" => OutputFormat::Turtle,
                "ntriples" => OutputFormat::NTriples,
                other => {
                    return Err(config_error(format!(
                        "output.format `{other}` is not supported (use \"turtle\" or \"ntriples\")"
                    )));
                },
            };
        }
        if let Some(stats) = self.output.stats {
            config.output.write_stats = stats;
        }
        config.output.shapemap = self.output.shapemap;

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
                labels: v.labels,
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

fn config_error(message: String) -> DataGeneratorError {
    DataGeneratorError::Config(message)
}
