use crate::cli::{parser::CompareArgs, wrappers::ResultCompareFormatCli};
use crate::commands::base::{Command, CommandContext};
use anyhow::{Ok, Result};
use iri_s::MimeType;

/// Implementation of the `compare` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Compare command logic.
pub struct CompareCommand {
    /// Arguments specific to Compare command.
    args: CompareArgs,
}

impl CompareCommand {
    pub fn new(args: CompareArgs) -> Self {
        Self { args }
    }
}

impl Command for CompareCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "compare"
    }

    /// Executes the Compare command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let mut reader1 = self
            .args
            .schema1
            .open_read(Some(self.args.format1.mime_type()), "Compare1")?;
        let mut reader2 = self
            .args
            .schema2
            .open_read(Some(self.args.format2.mime_type()), "Compare2")?;

        let shaco = ctx.rudof.compare_schemas(
            &mut reader1,
            &mut reader2,
            (&self.args.input_mode1).into(),
            (&self.args.input_mode2).into(),
            (&self.args.format1).into(),
            (&self.args.format2).into(),
            self.args.base1.as_ref().map(|i| i.as_str()),
            self.args.base2.as_ref().map(|i| i.as_str()),
            &(&self.args.reader_mode).into(),
            self.args.shape1.as_deref(),
            self.args.shape2.as_deref(),
            Some(&self.args.schema1.source_name()),
            Some(&self.args.schema2.source_name()),
        )?;

        match self.args.result_format {
            ResultCompareFormatCli::Internal => {
                writeln!(ctx.writer, "{shaco}")?;
            },
            ResultCompareFormatCli::Json => {
                let json = serde_json::to_string_pretty(&shaco)?;
                writeln!(ctx.writer, "{json}")?;
            },
        }

        Ok(())
    }
}
