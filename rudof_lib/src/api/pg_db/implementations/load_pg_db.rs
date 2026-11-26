use crate::{
    Result, Rudof,
    errors::PgDbError,
    formats::{DataFormat, DataReaderMode, InputSpec, ShaclFormat},
};
use oxrdf::Triple as OxTriple;
use std::io::Write;
use std::path::Path;

/// Loads `data`, validates it against SHACL shapes (unless
/// `skip_validation`), derives a property graph schema, and copies the data
/// into the connected database.
///
/// Reuses `rudof.load_data()`/`rudof.load_shacl_shapes()`/
/// `rudof.validate_shacl()` rather than re-implementing SHACL validation --
/// notably, `load_shacl_shapes()` already falls back to shapes embedded in
/// the data itself when `shapes` is not given.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn load_pg_db<W: Write>(
    rudof: &mut Rudof,
    data: &[InputSpec],
    db_path: Option<&Path>,
    db_read_only: bool,
    shapes: Option<&InputSpec>,
    shapes_format: Option<&ShaclFormat>,
    base_shapes: Option<&str>,
    skip_validation: bool,
    data_format: Option<&DataFormat>,
    base_data: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    writer: &mut W,
) -> Result<(usize, usize)> {
    let conn_info = super::resolve_connection(rudof, db_path, db_read_only)?;
    if conn_info.read_only {
        return Err(PgDbError::ReadOnlyConnection.into());
    }

    // ── Step 1: load the RDF data into session state ────────────────────
    let mut loading = rudof.load_data().with_data(data);
    if let Some(df) = data_format {
        loading = loading.with_data_format(df);
    }
    if let Some(rm) = reader_mode {
        loading = loading.with_reader_mode(rm);
    }
    if let Some(base) = base_data {
        loading = loading.with_base(base);
    }
    loading.execute()?;

    let all_triples = all_triples(rudof)?;
    writeln_progress(writer, format!("Loaded {} triples from RDF data", all_triples.len()))?;

    // ── Step 2: SHACL validation (unless skipped) ────────────────────────
    if skip_validation {
        writeln_progress(
            writer,
            "SHACL validation skipped (--skip-validation): the database DDL enforces conformance".to_string(),
        )?;
    } else {
        let mut load_shapes = rudof.load_shacl_shapes();
        if let Some(s) = shapes {
            load_shapes = load_shapes.with_shacl_schema(s);
        }
        if let Some(f) = shapes_format {
            load_shapes = load_shapes.with_shacl_schema_format(f);
        }
        if let Some(b) = base_shapes {
            load_shapes = load_shapes.with_base(b);
        }
        load_shapes.execute()?;

        let shape_count = rudof.shacl_shapes.as_ref().map(|s| s.iter().count()).unwrap_or(0);
        writeln_progress(writer, format!("Loaded SHACL shapes ({shape_count} shapes)"))?;

        rudof.validate_shacl().execute()?;

        let report = rudof
            .shacl_validation_results
            .as_ref()
            .ok_or_else(|| PgDbError::DataSourceSpec {
                message: "SHACL validation produced no results".to_string(),
            })?;

        if !report.conforms() {
            writeln_progress(
                writer,
                format!("SHACL validation FAILED — {} violation(s):", report.results().len()),
            )?;
            for result in report.results() {
                let msgs: Vec<String> = result.message().iter().map(|(_lang, s)| s.clone()).collect();
                writeln_progress(
                    writer,
                    format!(
                        "  - [{}] {}: {}",
                        result.severity(),
                        result.focus_node(),
                        msgs.join("; ")
                    ),
                )?;
            }
            return Err(PgDbError::ShaclViolations {
                violations: report.results().len(),
            }
            .into());
        }
        writeln_progress(writer, "SHACL validation PASSED ✓".to_string())?;
    }

    // ── Step 3: derive schema, apply DDL, insert data ────────────────────
    let model = super::schema::derive_pg_schema(&all_triples);
    writeln_progress(
        writer,
        format!(
            "Creating schema ({} node table(s), {} relationship table(s)) and loading data...",
            model.node_table_count(),
            model.rel_table_count()
        ),
    )?;

    let (node_count, rel_count) = super::db::load_data(&conn_info, &model, &all_triples, writer)?;
    writeln_progress(writer, format!("  Inserted {node_count} node(s)"))?;
    writeln_progress(writer, format!("  Inserted {rel_count} relationship(s)"))?;
    writeln_progress(writer, "  ✓ Load complete!".to_string())?;

    Ok((node_count, rel_count))
}

fn all_triples(rudof: &mut Rudof) -> Result<Vec<OxTriple>> {
    let data = rudof.data.as_mut().ok_or(PgDbError::NoDataLoaded)?;
    let rdf = data.unwrap_rdf_mut();
    let triples = rdf.all_triples().map_err(|error| PgDbError::DataSourceSpec {
        message: format!("Failed to enumerate triples: {error}"),
    })?;
    Ok(triples.collect())
}

fn writeln_progress<W: Write>(writer: &mut W, msg: String) -> Result<()> {
    writeln!(writer, "{msg}").map_err(|error| PgDbError::FailedIoOperation {
        error: error.to_string(),
    })?;
    Ok(())
}
