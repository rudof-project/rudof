use crate::cli::parser::LoadArgs;
use crate::commands::base::{Command, CommandContext};
use crate::commands::connect::ConnectionDetails;
use crate::commands::pg_mapping::{apply_ddl, derive_pg_schema, insert_triples};
use anyhow::{Context, Result};
use lbug::{Connection, Database};
use oxrdf::Triple as OxTriple;
use rudof_iri::MimeType;
use rudof_lib::formats::DataFormat;
use rudof_rdf::rdf_core::RDFFormat;
use shacl::ir::IRSchema;
use shacl::validator::processor::{GraphValidation, ShaclProcessor};
use shacl::validator::store::Graph;
use sparql_service::RdfData;
use std::io::Write;

/// Implementation of the `load` command.
///
/// Copies RDF data into a LadybugDB property graph database. By default the
/// data is validated against SHACL shapes before anything is inserted, so
/// the database only ever receives conforming data; subsequent mutations are
/// constrained by the node/rel tables (DDL) created from the data
/// (see discussion #747).
pub struct LoadCommand {
    args: LoadArgs,
}

impl LoadCommand {
    pub fn new(args: LoadArgs) -> Self {
        Self { args }
    }
}

impl Command for LoadCommand {
    fn name(&self) -> &'static str {
        "load"
    }

    #[allow(clippy::too_many_lines)]
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let args = &self.args;

        // ── Step 1: Resolve and open the database ────────────────────────────
        let details = ConnectionDetails::resolve(args.db.as_deref(), args.connection.as_deref())?;
        if details.read_only {
            anyhow::bail!("The connection details in use are read-only; a writable database is required to load data");
        }
        let db = Database::new(&details.path, lbug::SystemConfig::default())
            .with_context(|| format!("Failed to open LadybugDB database at '{}'", details.path.display()))?;
        let conn = Connection::new(&db).context("Failed to connect to LadybugDB")?;

        // ── Step 2: Resolve formats ──────────────────────────────────────────
        let data_format: DataFormat = args.data_format.into();
        let rdf_format: RDFFormat = (&data_format).try_into().context("Unsupported RDF data format")?;
        let reader_mode: rudof_rdf::rdf_impl::ReaderMode = {
            let mode: rudof_lib::formats::DataReaderMode = args.reader_mode.into();
            mode.into()
        };

        // SHACL shapes format → RDFFormat via the rudof_lib bridge
        let shapes_format_lib: rudof_lib::formats::ShaclFormat = args.shapes_format.into();
        let shacl_rdf_format: RDFFormat = shapes_format_lib
            .try_into()
            .map_err(|e: rudof_lib::errors::ShaclError| anyhow::anyhow!("Unsupported SHACL shapes format: {e}"))?;
        let shacl_inner_format: shacl::types::ShaclFormat = shapes_format_lib.into();

        // ── Step 3: Load RDF data into memory ────────────────────────────────
        let mut rdf_data = RdfData::new();
        for spec in &args.data {
            let mut data_reader = spec
                .open_read(Some(data_format.mime_type()), "RDF data for load")
                .with_context(|| format!("Failed to open data source '{}'", spec.source_name()))?;

            rdf_data
                .merge_from_reader(
                    &mut data_reader,
                    spec.source_name().as_str(),
                    &rdf_format,
                    args.base_data.as_deref(),
                    &reader_mode,
                )
                .with_context(|| format!("Failed to parse RDF data from '{}'", spec.source_name()))?;
        }

        let all_triples: Vec<OxTriple> = rdf_data.all_triples().context("Failed to enumerate triples")?.collect();
        writeln!(ctx.writer, "Loaded {} triples from RDF data", all_triples.len())?;

        // ── Step 4: Validate with SHACL (unless skipped) ─────────────────────
        if args.skip_validation {
            writeln!(
                ctx.writer,
                "SHACL validation skipped (--skip-validation): the database DDL enforces conformance"
            )?;
        } else {
            let shacl_shapes = if let Some(shapes_spec) = &args.shapes {
                let mut shapes_reader = shapes_spec
                    .open_read(Some(shacl_inner_format.mime_type()), "SHACL shapes")
                    .with_context(|| format!("Failed to open shapes from '{}'", shapes_spec.source_name()))?;

                let mut shapes_data = String::new();
                std::io::Read::read_to_string(&mut shapes_reader, &mut shapes_data)
                    .context("Failed to read SHACL shapes file")?;

                IRSchema::from_str(
                    &shapes_data,
                    &shacl_rdf_format,
                    args.base_shapes.as_deref(),
                    &reader_mode,
                )
                .map_err(|e| anyhow::anyhow!("Failed to parse SHACL shapes: {e}"))?
            } else {
                let triples_str = ntriples_string(&all_triples);
                IRSchema::from_str(
                    &triples_str,
                    &RDFFormat::NTriples,
                    args.base_data.as_deref(),
                    &reader_mode,
                )
                .map_err(|e| anyhow::anyhow!("Failed to parse SHACL shapes from data: {e}"))?
            };

            let shape_count = shacl_shapes.iter().count();
            writeln!(ctx.writer, "Loaded SHACL shapes ({shape_count} shapes)")?;

            let graph: Graph = rdf_data.clone().into();
            let mut validator: GraphValidation = graph.into();

            let report = ShaclProcessor::validate(
                &mut validator,
                &shacl_shapes,
                &shacl::validator::ShaclValidationMode::Native,
            )
            .map_err(|e| anyhow::anyhow!("SHACL validation failed: {e}"))?;

            if !report.conforms() {
                writeln!(
                    ctx.writer,
                    "SHACL validation FAILED — {} violation(s):",
                    report.results().len()
                )?;
                for result in report.results() {
                    let msgs: Vec<String> = result.message().iter().map(|(_lang, s)| s.clone()).collect();
                    writeln!(
                        ctx.writer,
                        "  - [{}] {}: {}",
                        result.severity(),
                        result.focus_node(),
                        msgs.join("; ")
                    )?;
                }
                anyhow::bail!("Data does not conform to SHACL shapes; aborting load");
            }
            writeln!(ctx.writer, "SHACL validation PASSED ✓")?;
        }

        // ── Step 5: Derive schema, apply DDL and copy the data ──────────────
        let model = derive_pg_schema(&all_triples);
        writeln!(
            ctx.writer,
            "Creating schema ({} node table(s), {} relationship table(s)) and loading data...",
            model.node_table_count(),
            model.rel_table_count()
        )?;

        apply_ddl(&conn, &model, &mut ctx.writer)?;
        let (node_count, rel_count) = insert_triples(&conn, &all_triples, &model)?;
        writeln!(ctx.writer, "  Inserted {node_count} node(s)")?;
        writeln!(ctx.writer, "  Inserted {rel_count} relationship(s)")?;

        writeln!(ctx.writer, "  ✓ Load complete!")?;

        Ok(())
    }
}

/// Convert all triples to a simple N-Triples string for re-parsing when the
/// shapes are embedded in the data.
#[allow(clippy::print_stderr, clippy::use_debug)]
fn ntriples_string(triples: &[OxTriple]) -> String {
    let mut buf = String::new();
    for triple in triples {
        use std::fmt::Write;
        let _ = writeln!(buf, "{} {} {} .", triple.subject, triple.predicate, triple.object);
    }
    buf
}
