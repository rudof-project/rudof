//! Shared RDF → property graph mapping used by the `ddl` and `load` commands.
//!
//! This module derives a property graph schema from RDF data (node tables
//! from `rdf:type` classes, relationship tables from predicates whose object
//! is an IRI) and can render it as DDL for different dialects
//! ([`DdlDialect::Cypher`] and [`DdlDialect::Gql`]).
//!
//! The [`PgSchemaModel`] intermediate representation is deliberately simple
//! and dialect-agnostic so that other schema sources can feed the same DDL
//! emitters in the future. In particular, a bridge from the `pgschema` crate
//! ([`pgschema::PropertyGraphSchema`]) to [`PgSchemaModel`] would allow
//! `rudof ddl --schema user.pgs --dialect cypher`, reusing the property
//! graph schema already understood by `pgschema-validate` (see discussion
//! #747).

use anyhow::Result;
use lbug::Connection;
use oxrdf::{Term as OxTerm, Triple as OxTriple};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Write;

/// The IRI of `rdf:type`.
pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

/// Target dialect for generated DDL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdlDialect {
    /// LadybugDB/Kùzu-style Cypher DDL (`CREATE NODE TABLE` / `CREATE REL TABLE`).
    Cypher,
    /// ISO GQL-style graph type DDL (`CREATE GRAPH TYPE` with `NODE TYPE` /
    /// `EDGE TYPE` declarations).
    Gql,
}

/// A node table derived from an RDF class.
#[derive(Debug, Clone)]
pub struct PgNodeTable {
    /// The IRI of the RDF class this table was derived from.
    ///
    /// Not needed for DDL emission itself, but kept so the model can be
    /// bridged to/from `pgschema::PropertyGraphSchema` (which is keyed by
    /// full IRIs) without losing information.
    #[allow(dead_code)]
    pub type_iri: String,
    /// Table name in the target database.
    pub name: String,
    /// Property columns (all mapped to `STRING`); the `id` primary key column
    /// holds the subject IRI and is implicit.
    pub properties: BTreeSet<String>,
}

/// A relationship table derived from an RDF predicate with IRI objects.
#[derive(Debug, Clone)]
pub struct PgRelTable {
    /// The IRI of the RDF predicate this table was derived from.
    pub pred_iri: String,
    /// Table name in the target database.
    pub name: String,
    /// Source node table.
    pub from_table: String,
    /// Target node table.
    pub to_table: String,
}

/// Dialect-agnostic property graph schema derived from RDF data.
#[derive(Debug, Clone, Default)]
pub struct PgSchemaModel {
    /// Node tables keyed by RDF class IRI.
    pub node_tables: BTreeMap<String, PgNodeTable>,
    /// Relationship tables keyed by RDF predicate IRI.
    pub rel_tables: BTreeMap<String, PgRelTable>,
}

impl PgSchemaModel {
    pub fn node_table_count(&self) -> usize {
        self.node_tables.len()
    }

    pub fn rel_table_count(&self) -> usize {
        self.rel_tables.len()
    }
}

// ============================================================================
// Schema derivation
// ============================================================================

/// Derive a property graph schema from RDF triples.
///
/// - Subjects with at least one `rdf:type` triple become nodes; each class
///   becomes a node table.
/// - Predicates become property columns (literals and IRI values alike).
/// - Predicates whose object is a typed node become relationship tables.
pub fn derive_pg_schema(triples: &[OxTriple]) -> PgSchemaModel {
    let mut model = PgSchemaModel::default();

    // subject → set of class IRIs
    let mut subject_types: HashMap<String, BTreeSet<String>> = HashMap::new();
    for triple in triples {
        if triple.predicate.as_str() == RDF_TYPE
            && let OxTerm::NamedNode(_) | OxTerm::BlankNode(_) = &triple.object
        {
            let class = object_str(&triple.object);
            subject_types
                .entry(subject_str(&triple.subject))
                .or_default()
                .insert(class.clone());
            model.node_tables.entry(class.clone()).or_insert_with(|| PgNodeTable {
                name: sanitize_name(&class),
                type_iri: class,
                properties: BTreeSet::new(),
            });
        }
    }

    // Collect properties per class and candidate relationships
    for triple in triples {
        let pred = triple.predicate.as_str();
        if pred == RDF_TYPE {
            continue;
        }
        let subj = subject_str(&triple.subject);
        if let Some(types) = subject_types.get(&subj) {
            let prop = sanitize_prop_name(pred);
            for class in types {
                if let Some(table) = model.node_tables.get_mut(class) {
                    table.properties.insert(prop.clone());
                }
            }
        }

        // Predicates with IRI objects pointing at typed nodes become rel tables
        if let OxTerm::NamedNode(_) = &triple.object {
            let obj = object_str(&triple.object);
            let from = subject_types.get(&subj).and_then(|ts| ts.iter().next());
            let to = subject_types.get(&obj).and_then(|ts| ts.iter().next());
            if let (Some(from_class), Some(to_class)) = (from, to) {
                model.rel_tables.entry(pred.to_string()).or_insert_with(|| PgRelTable {
                    pred_iri: pred.to_string(),
                    name: sanitize_name(pred),
                    from_table: sanitize_name(from_class),
                    to_table: sanitize_name(to_class),
                });
            }
        }
    }

    model
}

// ============================================================================
// DDL emission
// ============================================================================

/// Render the schema model as a sequence of DDL statements for `dialect`.
pub fn emit_ddl(model: &PgSchemaModel, dialect: DdlDialect, graph_type_name: &str) -> String {
    match dialect {
        DdlDialect::Cypher => emit_cypher_ddl(model),
        DdlDialect::Gql => emit_gql_ddl(model, graph_type_name),
    }
}

/// LadybugDB/Kùzu-style Cypher DDL.
fn emit_cypher_ddl(model: &PgSchemaModel) -> String {
    let mut stmts: Vec<String> = Vec::new();
    for table in model.node_tables.values() {
        stmts.push(format!(
            "CREATE NODE TABLE {} ({}, PRIMARY KEY(id));",
            table.name,
            node_columns(table).join(", ")
        ));
    }
    for rel in model.rel_tables.values() {
        stmts.push(format!(
            "CREATE REL TABLE {} (FROM {} TO {});",
            rel.name, rel.from_table, rel.to_table
        ));
    }
    stmts.join("\n")
}

/// ISO GQL-style graph type DDL.
fn emit_gql_ddl(model: &PgSchemaModel, graph_type_name: &str) -> String {
    let mut decls: Vec<String> = Vec::new();
    for table in model.node_tables.values() {
        decls.push(format!(
            "  NODE TYPE {} ({})",
            table.name,
            node_columns(table).join(", ")
        ));
    }
    for rel in model.rel_tables.values() {
        decls.push(format!(
            "  EDGE TYPE {} (FROM {} TO {})",
            rel.name, rel.from_table, rel.to_table
        ));
    }
    format!("CREATE GRAPH TYPE {} (\n{}\n);", graph_type_name, decls.join(",\n"))
}

/// Property columns of a node table, including the `id` primary key column.
fn node_columns(table: &PgNodeTable) -> Vec<String> {
    let mut cols = vec!["id STRING".to_string()];
    cols.extend(table.properties.iter().map(|p| format!("{p} STRING")));
    cols
}

// ============================================================================
// Database loading
// ============================================================================

/// Apply the schema model to a LadybugDB database, creating node and
/// relationship tables (existing tables are left untouched).
pub fn apply_ddl(conn: &Connection, model: &PgSchemaModel, writer: &mut dyn Write) -> Result<()> {
    for table in model.node_tables.values() {
        let sql = format!(
            "CREATE NODE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY(id));",
            table.name,
            node_columns(table).join(", ")
        );
        match conn.query(&sql) {
            Ok(_) => writeln!(writer, "  Created node table: {}", table.name)?,
            Err(e) => {
                writeln!(writer, "  Note: node table '{}' may already exist: {e}", table.name)?;
            },
        }
    }
    for rel in model.rel_tables.values() {
        let sql = format!(
            "CREATE REL TABLE IF NOT EXISTS {} (FROM {} TO {});",
            rel.name, rel.from_table, rel.to_table
        );
        match conn.query(&sql) {
            Ok(_) => writeln!(
                writer,
                "  Created relationship table: {} ({} → {})",
                rel.name, rel.from_table, rel.to_table
            )?,
            Err(e) => {
                writeln!(
                    writer,
                    "  Note: relationship table '{}' may already exist: {e}",
                    rel.name
                )?;
            },
        }
    }
    Ok(())
}

/// Insert RDF triples into a LadybugDB database following the schema model.
///
/// Returns the number of inserted nodes and relationships.
pub fn insert_triples(conn: &Connection, triples: &[OxTriple], model: &PgSchemaModel) -> Result<(usize, usize)> {
    let node_count = insert_nodes(conn, triples, model)?;
    let rel_count = insert_rels(conn, triples, model)?;
    Ok((node_count, rel_count))
}

/// Insert nodes into LadybugDB from RDF triples.
fn insert_nodes(conn: &Connection, triples: &[OxTriple], model: &PgSchemaModel) -> Result<usize> {
    // Group triples by subject
    let mut subject_triples: HashMap<String, Vec<&OxTriple>> = HashMap::new();
    for triple in triples {
        let s = subject_str(&triple.subject);
        subject_triples.entry(s).or_default().push(triple);
    }

    let mut node_count = 0;

    for (subject, sts) in &subject_triples {
        // Determine the tables this subject belongs to
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

        // Collect properties (predicate → value), excluding rdf:type
        let mut props: Vec<(String, String)> = Vec::new();
        for t in sts {
            if t.predicate.as_str() == RDF_TYPE {
                continue;
            }
            let pred_name = sanitize_prop_name(t.predicate.as_str());
            let val = term_value(&t.object);
            props.push((pred_name, val));
        }

        // Insert into each type table
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
                        return Err(anyhow::anyhow!("Failed to insert node into '{table_name}': {e}"));
                    }
                },
            }
        }
    }

    Ok(node_count)
}

/// Insert relationships into LadybugDB from RDF triples with IRI objects.
fn insert_rels(conn: &Connection, triples: &[OxTriple], model: &PgSchemaModel) -> Result<usize> {
    // Build subject → classes lookup
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

            // Only NamedNode objects represent relationships
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
                        writeln_err(format!(
                            "  Warning: failed to create relationship '{}' ({src_id} → {dst_id}): {e}",
                            rel.name
                        ))?;
                    }
                },
            }
        }
    }

    Ok(rel_count)
}

/// Write a warning to stderr (progress output must not pollute generated
/// DDL written to stdout/`--output-file`).
fn writeln_err(msg: String) -> Result<()> {
    #[allow(clippy::print_stderr)]
    {
        eprintln!("{msg}");
    }
    Ok(())
}

// ============================================================================
// Name sanitization and term helpers
// ============================================================================

/// Sanitize an IRI into a valid LadybugDB table name.
fn sanitize_name(iri: &str) -> String {
    let name = local_name(iri);
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    if sanitized.is_empty() || sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("T_{sanitized}")
    } else {
        sanitized
    }
}

/// Sanitize a predicate IRI into a valid property name.
fn sanitize_prop_name(iri: &str) -> String {
    let name = local_name(iri);
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    if sanitized.is_empty() || sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("p_{sanitized}")
    } else {
        sanitized
    }
}

/// Return the local part of an IRI (after the last `#` or `/`).
fn local_name(iri: &str) -> &str {
    iri.rsplit(['#', '/']).next().unwrap_or(iri)
}

/// Escape a string value for use in Cypher single-quoted strings.
fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Get the string representation of an RDF term (for property values).
#[allow(clippy::use_debug)]
fn term_value(t: &OxTerm) -> String {
    match t {
        OxTerm::NamedNode(n) => n.as_str().to_string(),
        OxTerm::Literal(lit) => lit.value().to_string(),
        OxTerm::BlankNode(bn) => format!("_:{}", bn.as_str()),
        OxTerm::Triple(triple) => format!("<<{triple}>>"),
    }
}

/// Get the string representation of an RDF subject.
#[allow(clippy::use_debug)]
fn subject_str(s: &oxrdf::NamedOrBlankNode) -> String {
    match s {
        oxrdf::NamedOrBlankNode::NamedNode(n) => n.as_str().to_string(),
        oxrdf::NamedOrBlankNode::BlankNode(bn) => format!("_:{}", bn.as_str()),
    }
}

/// Get the string representation of an RDF object term.
#[allow(clippy::use_debug)]
fn object_str(o: &OxTerm) -> String {
    match o {
        OxTerm::NamedNode(n) => n.as_str().to_string(),
        OxTerm::Literal(lit) => lit.value().to_string(),
        OxTerm::BlankNode(bn) => format!("_:{}", bn.as_str()),
        OxTerm::Triple(triple) => format!("<<{triple}>>"),
    }
}
