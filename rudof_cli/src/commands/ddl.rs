use crate::cli::parser::DdlArgs;
use crate::cli::wrappers::DdlDialectCli;
use crate::commands::base::{Command, CommandContext};
use crate::commands::pg_mapping::{DdlDialect, derive_pg_schema, emit_ddl};
use anyhow::{Context, Result};
use oxrdf::Triple as OxTriple;
use rudof_iri::MimeType;
use rudof_lib::formats::DataFormat;
use rudof_rdf::rdf_core::RDFFormat;
use sparql_service::RdfData;
use std::io::Write;

/// Implementation of the `ddl` command.
///
/// Derives a property graph schema from RDF data and writes the DDL needed
/// to materialize it in a property graph database. This command is
/// stateless: it never opens a database.
pub struct DdlCommand {
    args: DdlArgs,
}

impl DdlCommand {
    pub fn new(args: DdlArgs) -> Self {
        Self { args }
    }
}

impl Command for DdlCommand {
    fn name(&self) -> &'static str {
        "ddl"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // ── Step 1: Resolve formats ──────────────────────────────────────────
        let data_format: DataFormat = self.args.data_format.into();
        let rdf_format: RDFFormat = (&data_format).try_into().context("Unsupported RDF data format")?;
        let reader_mode: rudof_rdf::rdf_impl::ReaderMode = {
            let mode: rudof_lib::formats::DataReaderMode = self.args.reader_mode.into();
            mode.into()
        };

        // ── Step 2: Load RDF data into memory ────────────────────────────────
        let mut rdf_data = RdfData::new();
        for spec in &self.args.data {
            let mut data_reader = spec
                .open_read(Some(data_format.mime_type()), "RDF data for ddl")
                .with_context(|| format!("Failed to open data source '{}'", spec.source_name()))?;

            rdf_data
                .merge_from_reader(
                    &mut data_reader,
                    spec.source_name().as_str(),
                    &rdf_format,
                    self.args.base_data.as_deref(),
                    &reader_mode,
                )
                .with_context(|| format!("Failed to parse RDF data from '{}'", spec.source_name()))?;
        }

        let all_triples: Vec<OxTriple> = rdf_data.all_triples().context("Failed to enumerate triples")?.collect();

        // ── Step 3: Derive the property graph schema ────────────────────────
        let model = derive_pg_schema(&all_triples);
        warn_stderr(format!(
            "Derived {} node table(s) and {} relationship table(s) from {} triples",
            model.node_table_count(),
            model.rel_table_count(),
            all_triples.len()
        ));

        // ── Step 4: Emit DDL ─────────────────────────────────────────────────
        let dialect: DdlDialect = match self.args.dialect {
            DdlDialectCli::Cypher => DdlDialect::Cypher,
            DdlDialectCli::Gql => DdlDialect::Gql,
        };
        let ddl = emit_ddl(&model, dialect, &self.args.graph_type_name);
        writeln!(ctx.writer, "{ddl}")?;

        Ok(())
    }
}

/// Progress messages go to stderr so that the generated DDL written to
/// stdout/`--output-file` stays clean (e.g. pipeable to a database shell).
fn warn_stderr(msg: String) {
    #[allow(clippy::print_stderr)]
    {
        eprintln!("{msg}");
    }
}
