use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified constraint model that abstracts over ShEx and SHACL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConstraintModel {
    pub shapes: HashMap<String, UnifiedShape>,
    pub dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedShape {
    pub id: String,
    pub target_class: Option<String>,
    pub properties: Vec<UnifiedPropertyConstraint>,
    /// No property other than the declared ones (and `ignored_properties`) may
    /// appear on an instance: ShEx `CLOSED`, SHACL `sh:closed true`.
    pub closed: bool,
    /// Properties a closed shape tolerates without declaring them
    /// (SHACL `sh:ignoredProperties`).
    #[serde(default)]
    pub ignored_properties: Vec<String>,
    /// Every label an instance carries, for property graph schemas (a node type
    /// may require several); `target_class` is the first. Empty for RDF schemas,
    /// whose instances carry `target_class` only.
    #[serde(default)]
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPropertyConstraint {
    pub property_iri: String,
    pub constraints: Vec<UnifiedConstraint>,
    pub min_cardinality: Option<u32>,
    pub max_cardinality: Option<u32>,
    /// The cardinality counts only the values that satisfy `constraints` (ShEx
    /// triple constraints, SHACL `sh:qualifiedValueShape`), not every value of the
    /// property (SHACL `sh:minCount` and `sh:maxCount`). It decides whether an
    /// extra non-matching value also breaks the maximum.
    #[serde(default)]
    pub counts_matching_values: bool,
    /// Every value of the property must satisfy `constraints`. False for ShEx
    /// `EXTRA` predicates and SHACL qualified value shapes, which only count the
    /// matching values and tolerate the others.
    #[serde(default = "yes")]
    pub all_values_match: bool,
}

fn yes() -> bool {
    true
}

/// The datatype generated for literal values whose datatype the schema leaves
/// open. It is a generation choice, not a constraint: the IR does not record it,
/// so no violation operator treats it as one.
pub const DEFAULT_LITERAL_DATATYPE: &str = "http://www.w3.org/2001/XMLSchema#string";

impl UnifiedPropertyConstraint {
    /// The datatype to generate: the declared one, or [`DEFAULT_LITERAL_DATATYPE`]
    /// when values may be literals and nothing else fixes them (no datatype, shape,
    /// class, value set or fixed value).
    pub fn generated_datatype(&self) -> Option<String> {
        let mut literal_allowed = true;
        for c in &self.constraints {
            match c {
                UnifiedConstraint::Datatype(d) => return Some(d.clone()),
                UnifiedConstraint::ShapeReference(_)
                | UnifiedConstraint::Class(_)
                | UnifiedConstraint::In(_)
                | UnifiedConstraint::HasValue(_) => return None,
                UnifiedConstraint::NodeKind(kind) => {
                    literal_allowed &= matches!(
                        kind,
                        NodeKind::Literal | NodeKind::BlankNodeOrLiteral | NodeKind::IriOrLiteral
                    )
                },
                _ => {},
            }
        }
        literal_allowed.then(|| DEFAULT_LITERAL_DATATYPE.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnifiedConstraint {
    Datatype(String),
    ShapeReference(String),
    /// Values are instances of the class (SHACL `sh:class`): they carry its
    /// `rdf:type`, which is all that is required of them. Unlike a shape
    /// reference, the value need not conform to any shape, so a failing value
    /// does not make the node that refers to it fail.
    Class(String),
    NodeKind(NodeKind),
    Pattern(String),
    MinInclusive(Value),
    MaxInclusive(Value),
    MinExclusive(Value),
    MaxExclusive(Value),
    MinLength(u32),
    MaxLength(u32),
    In(Vec<Value>),
    HasValue(Value),
    /// Values are literals tagged with one of `tags`. With `ranges` a tag also
    /// admits its subtags (`en` accepts `en-GB`), as in SHACL `sh:languageIn` and
    /// ShEx `[@en~]`; without, the tag must match exactly (ShEx `[@en]`).
    /// Tags compare case-insensitively in both cases.
    Language {
        tags: Vec<String>,
        ranges: bool,
    },
    /// No two values share a language tag (SHACL `sh:uniqueLang true`).
    UniqueLang,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeKind {
    Iri,
    BlankNode,
    Literal,
    BlankNodeOrIri,
    BlankNodeOrLiteral,
    IriOrLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Iri(String),
    Literal(String, Option<String>), // value, datatype
    BlankNode(String),
}

impl UnifiedConstraintModel {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn add_shape(&mut self, shape: UnifiedShape) {
        let shape_id = shape.id.clone();

        // Extract dependencies from shape properties
        let mut deps = Vec::new();
        for prop in &shape.properties {
            for constraint in &prop.constraints {
                if let UnifiedConstraint::ShapeReference(target_shape) = constraint {
                    deps.push(target_shape.clone());
                }
            }
        }

        self.dependencies.insert(shape_id.clone(), deps);
        self.shapes.insert(shape_id, shape);
    }

    pub fn get_shape(&self, shape_id: &str) -> Option<&UnifiedShape> {
        self.shapes.get(shape_id)
    }

    pub fn get_dependencies(&self, shape_id: &str) -> Option<&Vec<String>> {
        self.dependencies.get(shape_id)
    }

    /// The shape whose instances carry `class` (its target, or one of its
    /// labels in a property graph); the
    /// smallest id when several do, so the choice does not depend on map order.
    pub fn shape_for_class(&self, class: &str) -> Option<&str> {
        let bare = |iri: &str| iri.trim_matches(['<', '>']).to_string();
        let class = bare(class);
        self.shapes
            .values()
            .filter(|s| s.target_class.iter().chain(&s.labels).any(|c| bare(c) == class))
            .map(|s| s.id.as_str())
            .min()
    }
}

impl Default for UnifiedConstraintModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Whether a literal's language `tag` satisfies a [`UnifiedConstraint::Language`].
///
/// Tags compare case-insensitively (RDF 1.1, BCP 47). With `ranges`, a tag matches a
/// range when it equals it or extends it with `-subtags` (RFC 4647 basic filtering,
/// as `sh:languageIn` and ShEx language stems use); `*` matches any tag.
pub fn language_matches(tag: &str, tags: &[String], ranges: bool) -> bool {
    let tag = tag.to_ascii_lowercase();
    tags.iter().any(|t| {
        let t = t.to_ascii_lowercase();
        if ranges {
            t == "*" || tag == t || tag.starts_with(&format!("{t}-"))
        } else {
            tag == t
        }
    })
}
