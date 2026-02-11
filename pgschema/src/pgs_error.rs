use crate::{card::Card, key::Key, type_name::TypeName, value::Value};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Serialize, Debug, Clone, PartialEq)]
pub enum PgsError {
    #[error("Serialization error: {error}")]
    SerializationError { error: String },

    #[error("Error flushing CSV writer: {error}")]
    FlushingCSVWriter { error: String },

    #[error("Error writing CSV record: {error}")]
    WritingCSVRecord { error: String },

    #[error("Error writing CSV header: {error}")]
    WritingCSVHeader { error: String },

    #[error("Error parsing as number: {0}")]
    InvalidNumber(String),

    #[error("Not found type: {0}")]
    MissingType(TypeName),

    #[error("Invalid regex pattern /{pattern}/: {error}")]
    InvalidRegex { pattern: String, error: String },

    #[error("Not found node with label: {label}")]
    MissingNodeLabel { label: String },

    #[error("Not found edge with label: {label}")]
    MissingEdgeLabel { label: String },

    #[error("Not found node/edge with label: {label}")]
    MissingNodeEdgeLabel { label: String },

    #[error("Not found node/edge type with label: {label}")]
    MissingNodeEdgeTypeLabel { label: String },

    #[error("Key not found in RecordType: {key} in Closed record type {record_type}")]
    KeyNotFoundClosedRecordType { key: Key, record_type: String },

    #[error("Cardinality doesn't match: {expected}, count {count}")]
    CardinalityMismatch { expected: Card, count: usize },

    #[error("Predicate {predicate_name} failed with value {value}")]
    PredicateFailed { predicate_name: String, value: Value },

    #[error("Missing keys in record type: {record_type}, keys: {keys:?}")]
    MissingKeys { keys: String, record_type: String },

    #[error("Extra keys in closed record type: {record_type}, keys: {keys:?}")]
    ExtraKeysNotOpen { keys: String, record_type: String },

    #[error("Parser error parsing property graph schema: {error}")]
    ParserError { error: String },

    #[error("Parser error parsing property graph: {error}")]
    PGParserError { error: String },

    #[error("Parser error parsing type map: {error}")]
    MapParserError { error: String },

    #[error("Labels do not match: record labels {record_labels}, type labels {type_labels}")]
    LabelsDifferent { record_labels: String, type_labels: String },

    #[error("Error reading type map from file: {path}: {error}")]
    TypeMapFileReadError { path: String, error: String },

    #[error("Error reading property graph from file: {path}: {error}")]
    PGFileReadError { path: String, error: String },

    #[error("Error reading property graph schema from file: {path}: {error}")]
    PGSchemaFileReadError { path: String, error: String },

    #[error("Record does not conform to type content:\n Record:\n{record}\n Types:\n{type_content}")]
    RecordContentFails { record: String, type_content: String },

    #[error("Type mismatch in operation {operation}: expected {expected}, found {found}")]
    TypeMismatch {
        operation: String,
        expected: String,
        found: String,
    },

    #[error("Condition ({condition}) for value: {value}")]
    ConditionFailed { condition: String, value: String },

    #[error("Missing association: node {node}, type {type_name}")]
    MissingAssociation { node: String, type_name: String },

    #[error("Duplicate edge type name: {type_name}")]
    DuplicateEdgeTypeName { type_name: String },

    #[error("Node {node} can't conform to edge type {edge_semantics}")]
    NodeNotConformsEdgeType {
        label: String,
        type_name: String,
        node: String,
        edge_semantics: String,
    },

    #[error("Edge {edge} can't conform to node type {node_semantics}")]
    EdgeNotConformsNodeType {
        label: String,
        type_name: String,
        edge: String,
        node_semantics: String,
    },

    #[error("Invalid date value: {date}")]
    InvalidDate { date: String, error: String },
}
