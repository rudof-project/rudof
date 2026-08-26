use crate::cli::parser::ShexValidateArgs;
use crate::cli::prefix_expand::resolve_prefixed_resource;
use crate::cli::wrappers::resolve_backend;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Context, Result};
use rudof_lib::Rudof;
use rudof_lib::formats::{BackendSpec, InputSpec, IriNormalizationMode};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;

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
            let vc = cfg.shex_validator().clone().with_max_steps(Some(self.args.max_steps));
            cfg = cfg.with_shex_validator(vc);
            ctx.rudof.update_config(cfg).execute();
        }

        let backend = resolve_backend(&self.args.common);
        let has_data_source = !self.args.data.is_empty() || matches!(backend, BackendSpec::Endpoint(_));

        if has_data_source {
            let mut loading = ctx
                .rudof
                .load_data()
                .with_data_format(&data_format)
                .with_reader_mode(&reader_mode)
                .with_backend(backend.clone())
                .with_endpoint_strategy(self.args.strategy.into());
            if !self.args.data.is_empty() {
                loading = loading.with_data(&self.args.data);
            }
            if let Some(base) = self.args.base_data.as_deref() {
                loading = loading.with_base(base);
            }
            loading.execute()?;
        }

        if let Some(compiled_schema) = self.args.compiled_schema.as_ref() {
            ctx.rudof
                .load_shex_schema_precompiled(compiled_schema)
                .with_reader_mode(&reader_mode)
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
                let file = File::create(cache_path)
                    .with_context(|| format!("Failed to create precompiled cache file '{}'", cache_path.display()))?;
                let mut writer = BufWriter::new(file);
                ctx.rudof.compile_shex_schema_to_file(&mut writer).execute()?;
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

        ctx.rudof.validate_shex().execute()?;

        ctx.rudof
            .serialize_shex_validation_results(&mut ctx.writer)
            .with_shex_validation_sort_order_mode(&sort_order)
            .with_result_shex_validation_format(&result_format)
            .execute()?;

        if let Some(map_state_path) = map_state {
            ctx.rudof
                .serialize_map_state(&mut std::fs::File::create(map_state_path)?)
                .execute()?;
        }

        Ok(())
    }
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

fn print_external_resolvers<W: Write>(writer: &mut W) -> Result<()> {
    writeln!(writer, "Available external-shape resolvers:")?;
    writeln!(writer)?;
    for info in Rudof::list_external_resolvers() {
        writeln!(writer, "  {:<18} {}", info.spec_syntax, info.description)?;
    }
    Ok(())
}
