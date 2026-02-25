use crate::cli::parser::PgSchemaValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::{data_format::DataFormat, pgschema_format::PgSchemaResultFormat};

/// Implementation of the `pgschema-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute PgSchema Validate command logic.
pub struct PgSchemaValidateCommand {
    /// Arguments specific to PgSchema Validate command.
    args: PgSchemaValidateArgs,
}

impl PgSchemaValidateCommand {
    pub fn new(args: PgSchemaValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for PgSchemaValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "pgschema-validate"
    }

    /// Executes the PgSchema Validate command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // Convert CLI types to library types
        let data_format: DataFormat = (&self.args.data_format).into();
        let result_format: PgSchemaResultFormat = (&self.args.result_validation_format).into();

        // Load PG data from all sources and merge them
        let graph = ctx.rudof.load_pg_data(&self.args.data, &data_format)?;

        // Load schema (required)
        let schema_input = self
            .args
            .schema
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Schema must be provided for PGSchema validation"))?;
        let schema = ctx.rudof.load_pg_schema(schema_input)?;

        // Load type map (required)
        let map_input = self
            .args
            .shapemap
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Type map must be provided for PGSchema validation"))?;
        let type_map = ctx.rudof.load_pg_typemap(map_input)?;

        // Perform validation
        let result = ctx.rudof.validate_pgschema(&schema, &graph, &type_map)?;

        // Output results based on format
        match result_format {
            PgSchemaResultFormat::Compact => {
                write!(ctx.writer, "{}", result)?;
            },
            PgSchemaResultFormat::Json => {
                result.as_json(&mut ctx.writer)?;
            },
            PgSchemaResultFormat::Csv => {
                result.as_csv(&mut ctx.writer, true)?;
            },
            PgSchemaResultFormat::Details => {
                anyhow::bail!("Details format not yet implemented for PGSchema validation");
            },
        }

        Ok(())
    }
}
