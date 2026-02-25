use crate::cli::parser::{
    CommonArgsOutputForceOverWrite, PgSchemaValidateArgs, ShaclValidateArgs, ShexValidateArgs, ValidateArgs,
};
use crate::cli::wrappers::ValidationModeCli;
use crate::commands::{
    PgSchemaValidateCommand, ShaclValidateCommand, ShexValidateCommand,
    base::{Command, CommandContext},
};
use anyhow::Result;

/// Implementation of the `validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute validation logic.
pub struct ValidateCommand {
    /// Arguments specific to validate.
    args: ValidateArgs,
}

impl ValidateCommand {
    pub fn new(args: ValidateArgs) -> Self {
        Self { args }
    }

    /// Convert ValidateArgs to ShexValidateArgs
    fn to_shex_args(&self) -> ShexValidateArgs {
        ShexValidateArgs {
            data: self.args.data.clone(),
            schema: self.args.schema.clone(),
            schema_format: self.args.schema_format.clone(),
            shapemap: self.args.shapemap.clone(),
            shapemap_format: self.args.shapemap_format.clone(),
            node: self.args.node.clone(),
            sort_by: self.args.sort_by.clone().into(),
            shape: self.args.shape.clone(),
            data_format: self.args.data_format.clone(),
            base_schema: self.args.base_schema.clone(),
            base_data: self.args.base_data.clone(),
            reader_mode: self.args.reader_mode.clone(),
            endpoint: self.args.endpoint.clone(),
            result_format: self.args.result_format.clone().into(),
            common: self.args.common.clone(),
        }
    }

    /// Convert ValidateArgs to ShaclValidateArgs  
    fn to_shacl_args(&self) -> ShaclValidateArgs {
        ShaclValidateArgs {
            data: self.args.data.clone(),
            data_format: self.args.data_format.clone(),
            base_data: self.args.base_data.clone(),
            reader_mode: self.args.reader_mode.clone(),
            shapes: self.args.schema.clone(),
            shapes_format: self.args.schema_format.clone().map(|f| f.try_into().unwrap()),
            base_shapes: self.args.base_schema.clone(),
            endpoint: self.args.endpoint.clone(),
            mode: self.args.shacl_validation_mode.clone(),
            result_format: self.args.result_format.clone().into(),
            sort_by: self.args.sort_by.clone().into(),
            common: self.args.common.clone(),
        }
    }

    /// Convert ValidateArgs to PgSchemaValidateArgs
    fn to_pgschema_args(&self) -> PgSchemaValidateArgs {
        PgSchemaValidateArgs {
            schema: self.args.schema.clone(),
            data: self.args.data.clone(),
            data_format: self.args.data_format.clone(),
            shapemap: self.args.shapemap.clone(),
            shapemap_format: self.args.shapemap_format.clone(),
            result_validation_format: self.args.result_format.clone().try_into().unwrap(),
            common: CommonArgsOutputForceOverWrite {
                output: self.args.common.output.clone(),
                force_overwrite: self.args.common.force_overwrite,
            },
        }
    }
}

impl Command for ValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "validate"
    }

    /// Executes the validate logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        match self.args.validation_mode {
            ValidationModeCli::ShEx => {
                let shex_args = self.to_shex_args();
                let cmd = ShexValidateCommand::new(shex_args);
                cmd.execute(ctx)
            },
            ValidationModeCli::Shacl => {
                let shacl_args = self.to_shacl_args();
                let cmd = ShaclValidateCommand::new(shacl_args);
                cmd.execute(ctx)
            },
            ValidationModeCli::PGSchema => {
                let pgschema_args = self.to_pgschema_args();
                let cmd = PgSchemaValidateCommand::new(pgschema_args);
                cmd.execute(ctx)
            },
        }
    }
}
