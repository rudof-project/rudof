//! RDF -> property graph mapping shared by `pg_db_ddl` and `load_pg_db`.
//!
//! This module derives a property graph schema from RDF data (node tables
//! from `rdf:type` classes, relationship tables from predicates whose object
//! is an IRI) and can render it as DDL for different dialects
//! ([`DdlDialect::Cypher`] and [`DdlDialect::Gql`]).
//!
//! The [`PgSchemaModel`] intermediate representation is deliberately simple
//! and dialect-agnostic so that other schema sources could feed the same DDL
//! emitters in the future (see discussion #747).

use crate::formats::DdlDialect;
use oxrdf::{Term as OxTerm, Triple as OxTriple};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// The IRI of `rdf:type`.
pub(crate) const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

/// A node table derived from an RDF class.
#[derive(Debug, Clone)]
pub(crate) struct PgNodeTable {
    /// The IRI of the RDF class this table was derived from.
    ///
    /// Not needed for DDL emission itself, but kept so the model can be
    /// bridged to/from a `PropertyGraphSchema` (keyed by full IRIs) without
    /// losing information.
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
pub(crate) struct PgRelTable {
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
pub(crate) struct PgSchemaModel {
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
pub(crate) fn derive_pg_schema(triples: &[OxTriple]) -> PgSchemaModel {
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
pub(crate) fn emit_ddl(model: &PgSchemaModel, dialect: DdlDialect, graph_type_name: &str) -> String {
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
pub(crate) fn node_columns(table: &PgNodeTable) -> Vec<String> {
    let mut cols = vec!["id STRING".to_string()];
    cols.extend(table.properties.iter().map(|p| format!("{p} STRING")));
    cols
}

// ============================================================================
// Name sanitization and term helpers
// ============================================================================

/// Sanitize an IRI into a valid database table name.
pub(crate) fn sanitize_name(iri: &str) -> String {
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
pub(crate) fn sanitize_prop_name(iri: &str) -> String {
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
pub(crate) fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Get the string representation of an RDF term (for property values).
pub(crate) fn term_value(t: &OxTerm) -> String {
    match t {
        OxTerm::NamedNode(n) => n.as_str().to_string(),
        OxTerm::Literal(lit) => lit.value().to_string(),
        OxTerm::BlankNode(bn) => format!("_:{}", bn.as_str()),
        OxTerm::Triple(triple) => format!("<<{triple}>>"),
    }
}

/// Get the string representation of an RDF subject.
pub(crate) fn subject_str(s: &oxrdf::NamedOrBlankNode) -> String {
    match s {
        oxrdf::NamedOrBlankNode::NamedNode(n) => n.as_str().to_string(),
        oxrdf::NamedOrBlankNode::BlankNode(bn) => format!("_:{}", bn.as_str()),
    }
}

/// Get the string representation of an RDF object term.
pub(crate) fn object_str(o: &OxTerm) -> String {
    match o {
        OxTerm::NamedNode(n) => n.as_str().to_string(),
        OxTerm::Literal(lit) => lit.value().to_string(),
        OxTerm::BlankNode(bn) => format!("_:{}", bn.as_str()),
        OxTerm::Triple(triple) => format!("<<{triple}>>"),
    }
}
