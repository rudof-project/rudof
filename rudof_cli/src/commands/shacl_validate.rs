use crate::cli::parser::ShaclValidateArgs;
use crate::cli::wrappers::resolve_backend;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Context, Result, anyhow};
use rudof_lib::formats::{BackendSpec, DataFormat};
use rudof_lib::types::{GraphSink, RdfSink, TriplesSink};
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use rudof_rdf::rdf_impl::OxigraphInMemory;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

/// Implementation of the `shacl-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shacl Validate logic.
pub struct ShaclValidateCommand {
    /// Arguments specific to shacl-validate.
    args: ShaclValidateArgs,
}

impl ShaclValidateCommand {
    pub fn new(args: ShaclValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for ShaclValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shacl-validate"
    }

    /// Executes the shacl-validate logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let shacl_schema_format = self.args.shapes_format.into();
        let shacl_validation_mode = self.args.mode.into();
        let sort_order = self.args.sort_by.into();
        let result_format = self.args.result_format.into();

        // Only override the config's store_errors/store_evidences/evidence_shapes_only/
        // recursion_semantics flags when the corresponding CLI flag is explicitly
        // passed, so the defaults (errors kept, evidence not, everything when
        // evidence is on, LFP) survive otherwise.
        if self.args.no_errors
            || self.args.with_evidences
            || self.args.evidences_shapes_only
            || self.args.recursion_semantics.is_some()
            || self.args.fetch_strategy.is_some()
        {
            let mut cfg = ctx.rudof.config().execute().clone();
            let mut sc = cfg.shacl().clone();
            if self.args.no_errors {
                sc = sc.with_store_errors(false);
            }
            if self.args.with_evidences {
                sc = sc.with_store_evidences(true);
            }
            if self.args.evidences_shapes_only {
                sc = sc.with_evidence_shapes_only(true);
            }
            if let Some(semantics) = self.args.recursion_semantics {
                sc = sc.with_recursion_semantics(semantics.into());
            }
            if let Some(strategy) = self.args.fetch_strategy {
                sc = sc.with_fetch_strategy(strategy.into());
            }
            cfg = cfg.with_shacl(sc);
            ctx.rudof.update_config(cfg).execute();
        }

        let backend = resolve_backend(&self.args.common);
        let has_data_source =
            !self.args.data.is_empty() || matches!(backend, BackendSpec::Endpoint(_) | BackendSpec::Lbug);

        if has_data_source {
            let mut loading = ctx
                .rudof
                .load_data()
                .with_data_format(&data_format)
                .with_reader_mode(&reader_mode)
                .with_backend(backend)
                .with_endpoint_limits(self.args.common.endpoint_limits());
            if !self.args.data.is_empty() {
                loading = loading.with_data(&self.args.data);
            }
            if let Some(base) = self.args.base_data.as_deref() {
                loading = loading.with_base(base);
            }
            loading.execute()?;
        }

        // With no new data and no `--shapes`, there's nothing new to load:
        // reuse whichever SHACL shapes are already loaded in the session.
        if has_data_source || self.args.shapes.is_some() {
            let mut loading_schema = ctx
                .rudof
                .load_shacl_shapes()
                .with_shacl_schema_format(&shacl_schema_format)
                .with_reader_mode(&reader_mode);
            if let Some(shacl_schema) = &self.args.shapes {
                loading_schema = loading_schema.with_shacl_schema(shacl_schema);
            }
            if let Some(base) = self.args.base_shapes.as_deref() {
                loading_schema = loading_schema.with_base(base);
            }
            loading_schema.execute()?;
        }

        let read_sink = if self.args.statistics || self.args.validated_graph.is_some() {
            let mut sink = GraphSink::new(OxigraphInMemory::new());
            sink.declare_prefixes(ctx.rudof.prefixes().execute())?;
            Some(Arc::new(Mutex::new(sink)))
        } else {
            None
        };

        let mut validation = ctx
            .rudof
            .validate_shacl()
            .with_shacl_validation_mode(&shacl_validation_mode);
        if let Some(sink) = &read_sink {
            validation = validation.with_read_sink(TriplesSink::new(sink.clone()));
        }
        validation.execute()?;

        // Buffered when statistics follow: some result formats do not end in a
        // newline, and the statistics have to start on a line of their own.
        let mut report = Vec::new();
        ctx.rudof
            .serialize_shacl_validation_results(&mut report)
            .with_shacl_validation_sort_order_mode(&sort_order)
            .with_result_shacl_validation_format(&result_format)
            .execute()?;
        ctx.writer.write_all(&report)?;
        if self.args.statistics && !report.is_empty() && !report.ends_with(b"\n") {
            writeln!(ctx.writer)?;
        }

        if let Some(sink) = &read_sink {
            let mut sink = sink.lock().map_err(|e| anyhow!("{e}"))?;
            let report = sink.finish()?;
            if self.args.statistics {
                writeln!(
                    ctx.writer,
                    "Triples read: {} (distinct: {})",
                    report.triples,
                    sink.graph().len()
                )?;
            }
            if let Some(path) = &self.args.validated_graph {
                let format = RDFFormat::try_from(DataFormat::from(self.args.validated_graph_format))?;
                let file = File::create(path)
                    .with_context(|| format!("Failed to create validated graph file '{}'", path.display()))?;
                let mut writer = BufWriter::new(file);
                sink.graph()
                    .serialize(&format, &mut writer)
                    .map_err(|e| anyhow!("Failed to write validated graph: {e}"))?;
                writer.flush()?;
            }
        }

        Ok(())
    }
}
