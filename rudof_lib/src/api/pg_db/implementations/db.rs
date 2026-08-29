//! LadybugDB-backed database operations: opening/validating a database,
//! applying a [`PgSchemaModel`]'s DDL, inserting RDF triples following that
//! model, and running Cypher queries.
//!
//! No live connection is ever cached: `lbug::Connection<'a>` borrows its
//! `Database`, so each operation here opens its own short-lived
//! `Database`+`Connection` and drops it before returning, exactly like the
//! CLI's `connect`/`load`/`query` commands already did before this module
//! existed.

use super::schema::{PgSchemaModel, RDF_TYPE, esc, node_columns, object_str, sanitize_prop_name, subject_str, term_value};
use crate::{CypherQueryResult, PgDbConnection, errors::PgDbError};
use lbug::{Connection, Database, NodeVal, RelVal, SystemConfig, Value as LbugValue};
use oxrdf::{Term as OxTerm, Triple as OxTriple};
use std::collections::HashMap;
use std::io::Write;

type Result<T> = std::result::Result<T, PgDbError>;

/// Open (but do not persist) the database described by `conn_info`, purely
/// to validate that it can be opened/connected to. Used by `connect_pg_db`.
pub(crate) fn verify_connection(conn_info: &PgDbConnection) -> Result<()> {
    let db = open(conn_info)?;
    let _conn = connect(&db)?;
    Ok(())
}

/// Open (but do not persist) a transient in-memory database, purely to
/// validate that `lbug` can create one. Used by `connect_pg_db --in-memory`.
pub(crate) fn verify_in_memory(read_only: bool) -> Result<()> {
    let db = Database::in_memory(SystemConfig::default().read_only(read_only)).map_err(|error| {
        PgDbError::FailedOpenDatabase {
            path: "<in-memory>".to_string(),
            error: error.to_string(),
        }
    })?;
    let _conn = connect(&db)?;
    Ok(())
}

fn open(conn_info: &PgDbConnection) -> Result<Database> {
    Database::new(&conn_info.path, SystemConfig::default().read_only(conn_info.read_only)).map_err(|error| {
        PgDbError::FailedOpenDatabase {
            path: conn_info.path.display().to_string(),
            error: error.to_string(),
        }
    })
}

fn connect(db: &Database) -> Result<Connection<'_>> {
    Connection::new(db).map_err(|error| PgDbError::FailedConnect {
        error: error.to_string(),
    })
}

/// Apply `model`'s DDL to the database and insert `triples`, following the
/// model. Returns the (node count, relationship count) inserted. Progress
/// (including non-fatal warnings) is written to `writer`.
pub(crate) fn load_data<W: Write>(
    conn_info: &PgDbConnection,
    model: &PgSchemaModel,
    triples: &[OxTriple],
    writer: &mut W,
) -> Result<(usize, usize)> {
    if conn_info.read_only {
        return Err(PgDbError::ReadOnlyConnection);
    }
    let db = open(conn_info)?;
    let conn = connect(&db)?;
    apply_ddl(&conn, model, writer)?;
    let node_count = insert_nodes(&conn, triples, model)?;
    let rel_count = insert_rels(&conn, triples, model, writer)?;
    Ok((node_count, rel_count))
}

/// Run a Cypher query against the database described by `conn_info`.
pub(crate) fn run_cypher_query(conn_info: &PgDbConnection, cypher: &str) -> Result<CypherQueryResult> {
    let db = open(conn_info)?;
    let conn = connect(&db)?;

    let result = conn.query(cypher).map_err(|error| PgDbError::FailedCypherQuery {
        error: error.to_string(),
    })?;

    let columns = result.get_column_names();
    let compiling_time_ms = result.get_compiling_time();
    let execution_time_ms = result.get_execution_time();

    let rows = result
        .map(|row| row.into_iter().map(value_to_json).collect())
        .collect();

    Ok(CypherQueryResult {
        columns,
        rows,
        compiling_time_ms,
        execution_time_ms,
    })
}

// ============================================================================
// DDL application and data insertion (moved from the former
// rudof_cli/src/commands/pg_mapping.rs)
// ============================================================================

fn apply_ddl<W: Write>(conn: &Connection, model: &PgSchemaModel, writer: &mut W) -> Result<()> {
    for table in model.node_tables.values() {
        let sql = format!(
            "CREATE NODE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY(id));",
            table.name,
            node_columns(table).join(", ")
        );
        match conn.query(&sql) {
            Ok(_) => writeln_progress(writer, format!("  Created node table: {}", table.name))?,
            Err(e) => writeln_progress(
                writer,
                format!("  Note: node table '{}' may already exist: {e}", table.name),
            )?,
        }
    }
    for rel in model.rel_tables.values() {
        let sql = format!(
            "CREATE REL TABLE IF NOT EXISTS {} (FROM {} TO {});",
            rel.name, rel.from_table, rel.to_table
        );
        match conn.query(&sql) {
            Ok(_) => writeln_progress(
                writer,
                format!(
                    "  Created relationship table: {} ({} → {})",
                    rel.name, rel.from_table, rel.to_table
                ),
            )?,
            Err(e) => writeln_progress(
                writer,
                format!("  Note: relationship table '{}' may already exist: {e}", rel.name),
            )?,
        }
    }
    Ok(())
}

fn insert_nodes(conn: &Connection, triples: &[OxTriple], model: &PgSchemaModel) -> Result<usize> {
    let mut subject_triples: HashMap<String, Vec<&OxTriple>> = HashMap::new();
    for triple in triples {
        let s = subject_str(&triple.subject);
        subject_triples.entry(s).or_default().push(triple);
    }

    let mut node_count = 0;

    for (subject, sts) in &subject_triples {
        let tables: Vec<&str> = sts
            .iter()
            .filter(|t| t.predicate.as_str() == RDF_TYPE)
            .filter_map(|t| {
                let obj = object_str(&t.object);
                model.node_tables.get(&obj).map(|table| table.name.as_str())
            })
            .collect();

        if tables.is_empty() {
            continue;
        }

        let mut props: Vec<(String, String)> = Vec::new();
        for t in sts {
            if t.predicate.as_str() == RDF_TYPE {
                continue;
            }
            let pred_name = sanitize_prop_name(t.predicate.as_str());
            let val = term_value(&t.object);
            props.push((pred_name, val));
        }

        for table_name in &tables {
            let id_val = esc(subject);
            let prop_pairs = props
                .iter()
                .map(|(k, v)| format!("{k}: '{}'", esc(v)))
                .collect::<Vec<_>>()
                .join(", ");

            let all_pairs = if prop_pairs.is_empty() {
                format!("id: '{id_val}'")
            } else {
                format!("id: '{id_val}', {prop_pairs}")
            };

            let insert_sql = format!("CREATE (:{table_name} {{ {all_pairs} }});");

            match conn.query(&insert_sql) {
                Ok(_) => node_count += 1,
                Err(e) => {
                    if !e.to_string().contains("duplicate key") {
                        return Err(PgDbError::FailedInsertNode {
                            table: (*table_name).to_string(),
                            error: e.to_string(),
                        });
                    }
                },
            }
        }
    }

    Ok(node_count)
}

fn insert_rels<W: Write>(
    conn: &Connection,
    triples: &[OxTriple],
    model: &PgSchemaModel,
    writer: &mut W,
) -> Result<usize> {
    let mut subject_types: HashMap<String, Vec<String>> = HashMap::new();
    for triple in triples {
        if triple.predicate.as_str() == RDF_TYPE {
            let obj = object_str(&triple.object);
            if model.node_tables.contains_key(&obj) {
                let s = subject_str(&triple.subject);
                subject_types.entry(s).or_default().push(obj);
            }
        }
    }

    let mut rel_count = 0;

    for rel in model.rel_tables.values() {
        for triple in triples {
            if triple.predicate.as_str() != rel.pred_iri {
                continue;
            }
            if !matches!(&triple.object, OxTerm::NamedNode(_)) {
                continue;
            }

            let subj_str = subject_str(&triple.subject);
            let obj_str = object_str(&triple.object);

            let Some(subj_types) = subject_types.get(&subj_str) else {
                continue;
            };
            let Some(obj_types) = subject_types.get(&obj_str) else {
                continue;
            };
            if subj_types.is_empty() || obj_types.is_empty() {
                continue;
            }

            let src_id = esc(&subj_str);
            let dst_id = esc(&obj_str);

            let cypher = format!(
                "MATCH (a:{} {{ id: '{src_id}' }}), \
                 (b:{} {{ id: '{dst_id}' }}) \
                 CREATE (a)-[:{}]->(b);",
                rel.from_table, rel.to_table, rel.name
            );

            match conn.query(&cypher) {
                Ok(_) => rel_count += 1,
                Err(e) => {
                    if !e.to_string().contains("duplicate key") {
                        writeln_progress(
                            writer,
                            format!(
                                "  Warning: failed to create relationship '{}' ({src_id} → {dst_id}): {e}",
                                rel.name
                            ),
                        )?;
                    }
                },
            }
        }
    }

    Ok(rel_count)
}

fn writeln_progress<W: Write>(writer: &mut W, msg: String) -> Result<()> {
    writeln!(writer, "{msg}").map_err(|error| PgDbError::FailedIoOperation {
        error: error.to_string(),
    })
}

// ============================================================================
// lbug::Value -> serde_json::Value
// ============================================================================

/// Converts an `lbug` query result value to JSON.
///
/// Nodes/relationships become JSON objects (`id`/`label`/`properties`),
/// scalars map directly, and lists/arrays map recursively. The long tail of
/// less common lbug types (dates, intervals, timestamps, structs, maps,
/// unions, UUIDs, decimals, blobs, recursive relationships) falls back to
/// `lbug::Value`'s own `Display` rendering as a JSON string, rather than
/// hand-writing an exact representation for each -- this keeps the mapping
/// total (never panics/errors) while still being exact for the common case
/// (property values, which are always one of the scalar types above).
fn value_to_json(value: LbugValue) -> serde_json::Value {
    use serde_json::Value as J;
    match value {
        LbugValue::Null(_) => J::Null,
        LbugValue::Bool(b) => J::Bool(b),
        LbugValue::Int8(x) => J::from(x),
        LbugValue::Int16(x) => J::from(x),
        LbugValue::Int32(x) => J::from(x),
        LbugValue::Int64(x) => J::from(x),
        LbugValue::UInt8(x) => J::from(x),
        LbugValue::UInt16(x) => J::from(x),
        LbugValue::UInt32(x) => J::from(x),
        LbugValue::UInt64(x) => J::from(x),
        LbugValue::Double(x) => serde_json::Number::from_f64(x).map_or(J::Null, J::Number),
        LbugValue::Float(x) => serde_json::Number::from_f64(f64::from(x)).map_or(J::Null, J::Number),
        LbugValue::String(s) => J::String(s),
        LbugValue::Json(j) => j,
        LbugValue::List(_, items) | LbugValue::Array(_, items) => J::Array(items.into_iter().map(value_to_json).collect()),
        LbugValue::Node(node) => node_to_json(&node),
        LbugValue::Rel(rel) => rel_to_json(&rel),
        other => J::String(other.to_string()),
    }
}

fn node_to_json(node: &NodeVal) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), serde_json::Value::String(node.get_node_id().to_string()));
    map.insert(
        "label".to_string(),
        serde_json::Value::String(node.get_label_name().clone()),
    );
    map.insert("properties".to_string(), properties_to_json(node.get_properties()));
    serde_json::Value::Object(map)
}

fn rel_to_json(rel: &RelVal) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("src".to_string(), serde_json::Value::String(rel.get_src_node().to_string()));
    map.insert("dst".to_string(), serde_json::Value::String(rel.get_dst_node().to_string()));
    map.insert(
        "label".to_string(),
        serde_json::Value::String(rel.get_label_name().clone()),
    );
    map.insert("properties".to_string(), properties_to_json(rel.get_properties()));
    serde_json::Value::Object(map)
}

fn properties_to_json(properties: &[(String, LbugValue)]) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = properties
        .iter()
        .map(|(k, v)| (k.clone(), value_to_json(v.clone())))
        .collect();
    serde_json::Value::Object(map)
}
