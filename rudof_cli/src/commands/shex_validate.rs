use crate::cli::parser::ShexValidateArgs;
use crate::cli::wrappers::resolve_backend;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Result, anyhow};
use rudof_lib::Rudof;
use rudof_lib::formats::IriNormalizationMode;
use std::io::Write;

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

        // `schema` is `required_unless_present = list_external_resolvers`, so we
        // know it must be Some here (the listing case returned earlier).
        let schema = self
            .args
            .schema
            .as_ref()
            .ok_or_else(|| anyhow!("--schema is required for shex-validate"))?;

        let backend = resolve_backend(self.args.common.backend.as_ref(), self.args.endpoint.as_deref());

        let mut loading = ctx
            .rudof
            .load_data()
            .with_data(&self.args.data)
            .with_data_format(&data_format)
            .with_reader_mode(&reader_mode)
            .with_backend(backend);
        if let Some(base) = self.args.base_data.as_deref() {
            loading = loading.with_base(base);
        }
        loading.execute()?;

        let mut shex_schema_loading = ctx
            .rudof
            .load_shex_schema(schema)
            .with_reader_mode(&reader_mode)
            .with_shex_schema_format(&schema_format);
        if let Some(base) = self.args.base_schema.as_deref() {
            shex_schema_loading = shex_schema_loading.with_base(base);
        }

        shex_schema_loading.execute()?;

        if let Some(shapemap) = &self.args.shapemap {
            let mut shapemap_loading = ctx.rudof.load_shapemap(shapemap);

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

fn print_external_resolvers<W: Write>(writer: &mut W) -> Result<()> {
    writeln!(writer, "Available external-shape resolvers:")?;
    writeln!(writer)?;
    for info in Rudof::list_external_resolvers() {
        writeln!(writer, "  {:<18} {}", info.spec_syntax, info.description)?;
    }
    Ok(())
}
