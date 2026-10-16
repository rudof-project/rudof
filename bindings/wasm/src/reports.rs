//! Snapshots of validation and query results, mirroring the report classes of
//! the Python bindings. They own their data and serialize (with camelCase keys)
//! to the plain objects returned to JavaScript.

use rudof_lib::types::{
    ArcDirection, NeighborArc as CoreNeighborArc, QueryResult, ResultShapeMap, ShExValidationStatus,
    ShaclValidationReport as CoreShaclReport, Triple as CoreTriple,
};
#[cfg(feature = "pgschema")]
use rudof_lib::types::{PgSchemaResultAssociation, PgSchemaValidationResult};
use serde::Serialize;

/// The result of ShEx validation: one entry per node/shape association.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShExValidationReport {
    /// `true` if every entry is conformant. An empty report is vacuously
    /// conformant; check `entries` to tell the two apart.
    pub conforms: bool,
    pub entries: Vec<ShExValidationEntry>,
    /// The entries that are not conformant.
    pub violations: Vec<ShExValidationEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShExValidationEntry {
    /// The focus node.
    pub node: String,
    /// The shape label it was checked against.
    pub shape: String,
    /// `conformant`, `nonconformant`, `pending` or `inconsistent`.
    pub status: String,
    /// The reason or appinfo carried by the status, if any.
    pub details: Option<String>,
}

impl From<&ResultShapeMap> for ShExValidationReport {
    fn from(results: &ResultShapeMap) -> Self {
        let entries: Vec<ShExValidationEntry> = results
            .iter()
            .map(|(node, shape, status)| ShExValidationEntry {
                node: node.to_string(),
                shape: shape.to_string(),
                status: status.code(),
                details: status_details(status),
            })
            .collect();
        let violations: Vec<ShExValidationEntry> =
            entries.iter().filter(|e| e.status != "conformant").cloned().collect();
        ShExValidationReport {
            conforms: violations.is_empty(),
            entries,
            violations,
        }
    }
}

fn status_details(status: &ShExValidationStatus) -> Option<String> {
    match status {
        ShExValidationStatus::Conformant(info) => Some(info.to_string()),
        ShExValidationStatus::NonConformant(info) => Some(info.to_string()),
        ShExValidationStatus::Pending => None,
        ShExValidationStatus::Inconsistent(conformant, non_conformant) => {
            Some(format!("Conformant: {conformant}, Non-conformant: {non_conformant}"))
        },
    }
}

/// The result of SHACL validation.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShaclValidationReport {
    /// `true` if the data conforms to the shapes graph, as reported by the
    /// validator.
    pub conforms: bool,
    /// Every validation result in the report.
    pub entries: Vec<ShaclValidationEntry>,
    /// Alias of `entries`, as in the Python bindings.
    pub violations: Vec<ShaclValidationEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShaclValidationEntry {
    /// The node that failed validation.
    pub focus_node: String,
    /// The property path the result was found at, if any.
    pub path: Option<String>,
    /// The value that caused the result, if any.
    pub value: Option<String>,
    /// The shape that reported the result, if any.
    pub source_shape: Option<String>,
    /// The SHACL constraint component that reported the result.
    pub constraint_component: String,
    /// `Violation`, `Warning`, `Info`, ... or the IRI of a custom severity.
    pub severity: String,
    /// The `sh:message`s attached to the result, with their language tags.
    pub messages: Vec<ShaclMessage>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShaclMessage {
    pub text: String,
    pub lang: Option<String>,
}

impl From<&CoreShaclReport> for ShaclValidationReport {
    fn from(report: &CoreShaclReport) -> Self {
        let entries: Vec<ShaclValidationEntry> = report
            .results()
            .iter()
            .map(|result| {
                let mut messages: Vec<ShaclMessage> = result
                    .message()
                    .iter()
                    .map(|(lang, text)| ShaclMessage {
                        text: text.clone(),
                        lang: lang.as_ref().map(ToString::to_string),
                    })
                    .collect();
                messages.sort_by(|a, b| a.lang.cmp(&b.lang));
                ShaclValidationEntry {
                    focus_node: result.focus_node().to_string(),
                    path: result.path().map(ToString::to_string),
                    value: result.value().map(ToString::to_string),
                    source_shape: result.source().map(ToString::to_string),
                    constraint_component: result.constraint_component().to_string(),
                    severity: result.severity().to_string(),
                    messages,
                }
            })
            .collect();
        ShaclValidationReport {
            conforms: report.conforms(),
            violations: entries.clone(),
            entries,
        }
    }
}

/// The result of a SPARQL query: a SELECT fills `variables` and `rows`, an ASK
/// fills `boolean`, and a CONSTRUCT or DESCRIBE fills `graph`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum QueryResults {
    Select {
        variables: Vec<String>,
        /// One row per solution, with the value of each variable (in the order
        /// of `variables`), or `null` when it is unbound.
        rows: Vec<Vec<Option<String>>>,
    },
    Ask {
        boolean: bool,
    },
    Graph {
        graph: String,
    },
}

impl From<&QueryResult> for QueryResults {
    fn from(result: &QueryResult) -> Self {
        match result {
            QueryResult::Ask(answer) => QueryResults::Ask { boolean: *answer },
            QueryResult::Construct(graph) => QueryResults::Graph { graph: graph.clone() },
            QueryResult::Select(solutions) => {
                // The variable list is per solution; take it from the first one, as
                // `serialize_query_results` does for its header.
                let variables = solutions
                    .iter()
                    .next()
                    .map(|s| s.variables().iter().map(ToString::to_string).collect())
                    .unwrap_or_default();
                let rows = solutions
                    .iter()
                    .map(|solution| {
                        (0..solution.variables().len())
                            .map(|i| solution.find_solution(i).map(ToString::to_string))
                            .collect()
                    })
                    .collect();
                QueryResults::Select { variables, rows }
            },
        }
    }
}

/// The result of property graph schema validation: one entry per node/type
/// association.
#[cfg(feature = "pgschema")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PgSchemaValidationReport {
    /// `true` if the property graph conforms to the schema.
    pub conforms: bool,
    pub entries: Vec<PgSchemaValidationEntry>,
    /// The entries that do not conform.
    pub violations: Vec<PgSchemaValidationEntry>,
}

#[cfg(feature = "pgschema")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PgSchemaValidationEntry {
    pub node_id: String,
    pub type_name: String,
    pub conforms: bool,
    /// The errors (when it does not conform) or evidences, joined by `; `.
    pub details: String,
}

#[cfg(feature = "pgschema")]
impl From<&PgSchemaValidationResult> for PgSchemaValidationReport {
    fn from(result: &PgSchemaValidationResult) -> Self {
        let entries: Vec<PgSchemaValidationEntry> = result.associations.iter().map(Into::into).collect();
        let violations = entries.iter().filter(|e| !e.conforms).cloned().collect();
        PgSchemaValidationReport {
            conforms: result.is_valid,
            entries,
            violations,
        }
    }
}

#[cfg(feature = "pgschema")]
impl From<&PgSchemaResultAssociation> for PgSchemaValidationEntry {
    fn from(association: &PgSchemaResultAssociation) -> Self {
        let details = association
            .details
            .as_ref()
            .map_left(|errors| join(errors.iter().map(ToString::to_string)))
            .map_right(|evidences| join(evidences.iter().map(ToString::to_string)))
            .into_inner();
        PgSchemaValidationEntry {
            node_id: association.node_id.clone(),
            type_name: association.type_name.clone(),
            conforms: association.conforms,
            details,
        }
    }
}

#[cfg(feature = "pgschema")]
fn join(parts: impl Iterator<Item = String>) -> String {
    parts.collect::<Vec<_>>().join("; ")
}

/// The arcs around a node, as returned by `node_neighborhood`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeNeighborhood {
    pub arcs: Vec<NeighborArc>,
    /// `true` if there were more arcs than the requested limit.
    pub truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighborArc {
    /// The node whose neighborhood the arc belongs to.
    pub root: String,
    /// `outgoing` or `incoming`.
    pub direction: String,
    /// Distance from the root, starting at 1.
    pub depth: usize,
    /// The node the arc starts from.
    pub node: String,
    pub predicate: String,
    /// The node at the other end of the arc.
    pub neighbor: String,
    /// `true` for the last arc of `node` in this direction.
    pub is_last: bool,
}

impl From<&CoreNeighborArc> for NeighborArc {
    fn from(arc: &CoreNeighborArc) -> Self {
        NeighborArc {
            root: arc.root.to_string(),
            direction: match arc.direction {
                ArcDirection::Outgoing => "outgoing",
                ArcDirection::Incoming => "incoming",
            }
            .to_string(),
            depth: arc.depth,
            node: arc.node.to_string(),
            predicate: arc.predicate.to_string(),
            neighbor: arc.neighbor.to_string(),
            is_last: arc.is_last,
        }
    }
}

/// The result of checking a ShEx schema with `check_shex`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShExCheck {
    /// `true` if the schema is well formed.
    pub valid: bool,
    /// The messages reported by the check.
    pub message: String,
}

/// An external shape resolver that can be added with `add_external_resolver`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalResolver {
    pub name: String,
    pub description: String,
    /// The syntax of the spec string that configures it.
    pub spec_syntax: String,
}

/// The triples matching a pattern, as returned by `triples`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Triples {
    pub triples: Vec<Triple>,
    /// `true` if more triples matched than the requested limit.
    pub truncated: bool,
}

/// A triple of the loaded RDF data.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Triple {
    /// An IRI or a blank node.
    pub subject: String,
    pub predicate: String,
    /// An IRI, a blank node or a literal.
    pub object: String,
}

impl From<&CoreTriple> for Triple {
    fn from(triple: &CoreTriple) -> Self {
        Triple {
            subject: triple.subject.to_string(),
            predicate: triple.predicate.to_string(),
            object: triple.object.to_string(),
        }
    }
}
