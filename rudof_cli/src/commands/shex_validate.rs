use crate::cli::parser::ShexValidateArgs;
use crate::cli::prefix_expand::resolve_prefixed_resource;
use crate::cli::wrappers::{DataFormatCli, resolve_backend};
use crate::commands::base::{Command, CommandContext};
use anyhow::{Context, Result, anyhow};
use rudof_lib::Rudof;
use rudof_lib::formats::{BackendSpec, DataFormat, InputSpec, IriNormalizationMode, ShExFormat};
use rudof_lib::types::{CountingSink, GraphSink, RdfSink, TriplesSink};
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use rudof_rdf::rdf_impl::OxigraphInMemory;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

/// Implementation of the `shex-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute ShexValidate logic.
pub struct ShexValidateCommand {
    /// Arguments specific to shex-validate.
    args: ShexValidateArgs,
}

impl ShexValidateCommand {
    pub fn new(args: ShexValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for ShexValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shex-validate"
    }

    /// Executes the shex-validate logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // Discovery flag short-circuits the rest of the command.
        if self.args.list_external_resolvers {
            print_external_resolvers(&mut ctx.writer)?;
            return Ok(());
        }

        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let schema_format = self.args.schema_format.into();
        let sort_order = self.args.sort_by.into();
        let result_format = self.args.result_format.into();
        let map_state = self.args.map_state.clone();

        // External-shape resolvers must be registered before `load_shex_schema`
        // runs, since the compiler reads them from the validator config to
        // rewrite EXTERNAL declarations during AST→IR.
        for spec in &self.args.external_resolvers {
            ctx.rudof.add_external_resolver(spec)?;
        }

        // Cap validation work at `--max-steps` (default 100). Without this, a
        // shape validated against a live/remote RDF source (e.g. a SPARQL
        // endpoint) can, in pathological cases, chase an unbounded number of
        // dependencies instead of failing fast.
        {
            let mut cfg = ctx.rudof.config().execute().clone();
            let vc = cfg
                .shex_validator()
                .clone()
                .with_max_steps(Some(self.args.max_steps))
                .with_show_intermediate_results(self.args.show_intermediate_results);
            cfg = cfg.with_shex_validator(vc);
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
                .with_backend(backend.clone())
                .with_endpoint_strategy(self.args.strategy.into())
                .with_endpoint_limits(self.args.common.endpoint_limits());
            if !self.args.data.is_empty() {
                loading = loading.with_data(&self.args.data);
            }
            if let Some(base) = self.args.base_data.as_deref() {
                loading = loading.with_base(base);
            }
            loading.execute()?;
        }

        if let Some(compiled_schema) = self.args.compiled_schema.as_ref() {
            // `--compiled-schema` is a dedicated shortcut for the same
            // `ShExFormat::Binary` handling that `-f binary` drives -- both
            // load the precompiled `SchemaIR` cache directly, skipping
            // parsing, imports and AST-to-IR compilation.
            ctx.rudof
                .load_shex_schema(compiled_schema)
                .with_reader_mode(&reader_mode)
                .with_shex_schema_format(&ShExFormat::Binary)
                .execute()?;
        } else if let Some(schema) = self.args.schema.as_ref() {
            let schema = expand_prefixed_input(schema, ctx, &backend)?;
            let mut shex_schema_loading = ctx
                .rudof
                .load_shex_schema(&schema)
                .with_reader_mode(&reader_mode)
                .with_shex_schema_format(&schema_format);
            if let Some(base) = self.args.base_schema.as_deref() {
                shex_schema_loading = shex_schema_loading.with_base(base);
            }

            shex_schema_loading.execute()?;

            if let Some(cache_path) = self.args.compile_to.as_deref() {
                // Likewise, `--compile-to` is a shortcut for
                // `serialize_shex_schema`'s `ShExFormat::Binary` result
                // format -- the same one `-r binary` drives on the `shex`
                // command.
                let file = File::create(cache_path)
                    .with_context(|| format!("Failed to create precompiled cache file '{}'", cache_path.display()))?;
                let mut writer = BufWriter::new(file);
                ctx.rudof
                    .serialize_shex_schema(&mut writer)
                    .with_result_shex_format(&ShExFormat::Binary)
                    .execute()?;
            }
        }
        // Neither `--schema` nor `--compiled-schema` given: reuse whatever
        // ShEx schema is already loaded in the session (or fail below with
        // a clear "no schema loaded" error if there isn't one).

        if let Some(shapemap) = &self.args.shapemap {
            let shapemap = expand_prefixed_input(shapemap, ctx, &backend)?;
            let mut shapemap_loading = ctx.rudof.load_shapemap(&shapemap);

            if let Some(base_nodes) = self.args.base_data.as_deref() {
                shapemap_loading = shapemap_loading.with_base_nodes(base_nodes);
            }
            if let Some(base_shapes) = self.args.base_schema.as_deref() {
                shapemap_loading = shapemap_loading.with_base_shapes(base_shapes);
            }
            let aux_shapemap_format;
            if let Some(shapemap_format) = self.args.shapemap_format {
                aux_shapemap_format = shapemap_format.into();
                shapemap_loading = shapemap_loading.with_shapemap_format(&aux_shapemap_format);
            }
            shapemap_loading.execute()?;
        }

        let iri_mode = if self.args.strict_iris {
            IriNormalizationMode::Strict
        } else {
            IriNormalizationMode::Lax
        };

        if let Some(node) = self.args.node.as_deref() {
            let mut node_shape = ctx.rudof.add_node_shape_to_shapemap(node).with_iri_mode(iri_mode);
            if let Some(shape) = self.args.shape.as_deref() {
                node_shape = node_shape.with_shape(shape);
            }
            if let Some(base) = self.args.base_data.as_deref() {
                node_shape = node_shape.with_base_nodes(base);
            }
            if let Some(base) = self.args.base_schema.as_deref() {
                node_shape = node_shape.with_base_shapes(base);
            }
            node_shape.execute()?;
        }

        let new_sink = |ctx: &CommandContext| -> Result<Arc<Mutex<GraphSink>>> {
            let mut sink = GraphSink::new(OxigraphInMemory::new());
            sink.declare_prefixes(ctx.rudof.prefixes().execute())?;
            Ok(Arc::new(Mutex::new(sink)))
        };
        // `--statistics` counts the triples read, and the valid and invalid ones:
        // the read sink keeps a graph, to tell the distinct triples apart, while
        // the valid and invalid triples are only counted. A subgraph is only kept
        // as a graph when `--visited`, `--valid` or `--invalid` asks for it.
        let read_graph = if self.args.statistics || self.args.visited.is_some() {
            Some(new_sink(ctx)?)
        } else {
            None
        };
        let valid_graph = self.args.valid.as_ref().map(|_| new_sink(ctx)).transpose()?;
        let invalid_graph = self.args.invalid.as_ref().map(|_| new_sink(ctx)).transpose()?;
        let graph_or_count = |graph: &Option<Arc<Mutex<GraphSink>>>| match graph {
            Some(graph) => Some(TriplesSink::new(graph.clone())),
            None if self.args.statistics => Some(TriplesSink::new(Arc::new(Mutex::new(CountingSink::new())))),
            None => None,
        };
        let valid_sink = graph_or_count(&valid_graph);
        let invalid_sink = graph_or_count(&invalid_graph);

        let mut validation = ctx.rudof.validate_shex();
        if let Some(sink) = &read_graph {
            validation = validation.with_read_sink(TriplesSink::new(sink.clone()));
        }
        if let Some(sink) = &valid_sink {
            validation = validation.with_valid_sink(sink.clone());
        }
        if let Some(sink) = &invalid_sink {
            validation = validation.with_invalid_sink(sink.clone());
        }
        validation.execute()?;

        ctx.rudof
            .serialize_shex_validation_results(&mut ctx.writer)
            .with_shex_validation_sort_order_mode(&sort_order)
            .with_result_shex_validation_format(&result_format)
            .execute()?;

        let finish = |sink: &Option<TriplesSink>| -> Result<u64> {
            match sink {
                Some(sink) => Ok(sink.0.lock().map_err(|e| anyhow!("{e}"))?.finish()?.triples),
                None => Ok(0),
            }
        };
        let valid_triples = finish(&valid_sink)?;
        let invalid_triples = finish(&invalid_sink)?;
        if let Some(sink) = &read_graph {
            let mut sink = sink.lock().map_err(|e| anyhow!("{e}"))?;
            let report = sink.finish()?;
            if self.args.statistics {
                writeln!(
                    ctx.writer,
                    "Triples read: {} (distinct: {})",
                    report.triples,
                    sink.graph().len()
                )?;
                writeln!(ctx.writer, "Valid triples: {valid_triples}")?;
                writeln!(ctx.writer, "Invalid triples: {invalid_triples}")?;
            }
        }
        for (path, format, graph) in [
            (&self.args.visited, self.args.visited_format, &read_graph),
            (&self.args.valid, self.args.valid_format, &valid_graph),
            (&self.args.invalid, self.args.invalid_format, &invalid_graph),
        ] {
            if let (Some(path), Some(graph)) = (path, graph) {
                write_graph(path, format, &*graph.lock().map_err(|e| anyhow!("{e}"))?)?;
            }
        }

        if let Some(map_state_path) = map_state {
            ctx.rudof
                .serialize_map_state(&mut std::fs::File::create(map_state_path)?)
                .execute()?;
        }

        Ok(())
    }
}

/// Writes the graph of `sink` to `path` in `format`.
fn write_graph(path: &Path, format: DataFormatCli, sink: &GraphSink) -> Result<()> {
    let format = RDFFormat::try_from(DataFormat::from(format))?;
    let file = File::create(path).with_context(|| format!("Failed to create graph file '{}'", path.display()))?;
    let mut writer = BufWriter::new(file);
    sink.graph()
        .serialize(&format, &mut writer)
        .map_err(|e| anyhow!("Failed to write graph to '{}': {e}", path.display()))?;
    writer.flush()?;
    Ok(())
}

/// Expands `spec` if it's a CURIE-like literal (e.g. `es:E371`, produced by
/// `InputSpec::FromStr` when the token isn't a real file) into the IRI its
/// prefix stands for, checked against the session's default prefixes and,
/// if `-e`/`--endpoint` named a config-registered endpoint, that endpoint's
/// own prefixes — mirroring the expansion the interactive shell already
/// does for `-s`/`-m`/`-q` (see `crate::shell::repl::resolve_prefixed_resource`).
/// Any other `InputSpec` variant (an existing path, a URL, `-`) passes through
/// unchanged.
fn expand_prefixed_input(spec: &InputSpec, ctx: &CommandContext, backend: &BackendSpec) -> Result<InputSpec> {
    let InputSpec::Str(raw) = spec else {
        return Ok(spec.clone());
    };

    let default_prefixes = ctx.rudof.prefixes().execute();
    let endpoint_prefixes = backend.endpoint().and_then(|name| {
        ctx.rudof
            .config()
            .execute()
            .rdf_data()
            .find_endpoint(name)
            .map(|endpoint| endpoint.prefixmap().clone())
    });

    match resolve_prefixed_resource(raw, &default_prefixes, endpoint_prefixes.as_ref()) {
        Some(expanded) => InputSpec::from_str(&expanded).map_err(Into::into),
        None => Ok(spec.clone()),
    }
}

pub(crate) fn print_external_resolvers<W: Write>(writer: &mut W) -> Result<()> {
    writeln!(writer, "Available external-shape resolvers:")?;
    writeln!(writer)?;
    for info in Rudof::list_external_resolvers() {
        writeln!(writer, "  {:<18} {}", info.spec_syntax, info.description)?;
    }
    Ok(())
}
