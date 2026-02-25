use crate::cli::parser::{ConvertArgs, ShexArgs};
use crate::cli::wrappers::{InputConvertModeCli, OutputConvertModeCli};
use crate::commands::{ShexCommand, base::{Command, CommandContext}};
use crate::output::ColorSupport;
use anyhow::{Result, anyhow};
use rudof_lib::{
    ReaderMode, ShExFormatter, ShapeMapParser,
    convert::{
        InputConvertFormat, OutputConvertFormat, run_shacl_convert, run_shacl2shex, run_shex2html, run_shex2sparql,
        run_shex2uml, run_tap2html, run_tap2shex, run_tap2uml,
    },
    rdf_reader_mode::RDFReaderMode,
};

/// Implementation of the `convert` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Convert command logic.
pub struct ConvertCommand {
    /// Arguments specific to Convert command.
    args: ConvertArgs,
}

impl ConvertCommand {
    pub fn new(args: ConvertArgs) -> Self {
        Self { args }
    }
    /// Convert ValidateArgs to ShexValidateArgs
    fn to_shex_args(&self) -> ShexArgs {
        ShexArgs {
            schema: self.args.file.clone(),
            schema_format: self.args.format.clone().try_into().unwrap(),
            shape: None,
            base: self.args.base.clone(),
            result_schema_format: self.args.result_format.clone().try_into().unwrap(),
            common: self.args.common.clone(),
            show_time: self.args.show_time,
            show_schema: Some(true),
            compile: Some(false),
            reader_mode: self.args.reader_mode.clone(),
            show_dependencies: Some(false),
            show_statistics: Some(false),
        }
    }
}

impl Command for ConvertCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "convert"
    }

    /// Executes the Convert command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let formatter = match ctx.color {
            ColorSupport::NoColor => ShExFormatter::default().without_colors(),
            _ => ShExFormatter::default(),
        };
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();

        match (&self.args.input_mode, &self.args.output_mode) {
            (InputConvertModeCli::ShEx, OutputConvertModeCli::ShEx) => {
                let shex_args = self.to_shex_args();
                let cmd = ShexCommand::new(shex_args);
                cmd.execute(ctx)
            },
            (InputConvertModeCli::Shacl, OutputConvertModeCli::Shacl) => {
                let input_format: InputConvertFormat = (&self.args.format).into();
                let input_format = input_format.to_shacl_format().unwrap();
                let output_format: OutputConvertFormat = (&self.args.result_format).into();
                let output_format = output_format.to_shacl_format().unwrap();

                run_shacl_convert(
                    &self.args.file,
                    &input_format,
                    &self.args.base,
                    &output_format,
                    &reader_mode,
                    ctx.rudof.config(),
                    &mut ctx.writer,
                )?;

                Ok(())
            },
            (InputConvertModeCli::Dctap, OutputConvertModeCli::ShEx) => {
                run_tap2shex(
                    &self.args.file,
                    &(&self.args.format).into(),
                    &(&self.args.result_format).into(),
                    ctx.rudof.config(),
                    &mut ctx.writer,
                    &formatter,
                )?;

                Ok(())
            },
            (InputConvertModeCli::ShEx, OutputConvertModeCli::Sparql) => {
                let maybe_shape = match &self.args.shape {
                    None => None,
                    Some(shape_str) => {
                        let iri_shape = ShapeMapParser::parse_iri_ref(&shape_str)?;
                        Some(iri_shape)
                    },
                };

                run_shex2sparql(
                    &self.args.file,
                    &(&self.args.format).into(),
                    &self.args.base,
                    maybe_shape,
                    ctx.rudof.config(),
                    &reader_mode,
                    &mut ctx.writer,
                )?;

                Ok(())
            },
            (InputConvertModeCli::ShEx, OutputConvertModeCli::Uml) => {
                run_shex2uml(
                    &self.args.file,
                    &(&self.args.format).into(),
                    &self.args.base,
                    &(&self.args.result_format).into(),
                    &self.args.shape,
                    ctx.rudof.config(),
                    &reader_mode,
                    &mut ctx.writer,
                )?;

                Ok(())
            },
            (InputConvertModeCli::Shacl, OutputConvertModeCli::ShEx) => {
                run_shacl2shex(
                    &self.args.file,
                    &(&self.args.format).into(),
                    &self.args.base,
                    &(&self.args.result_format).into(),
                    ctx.rudof.config(),
                    &reader_mode,
                    &formatter,
                    &mut ctx.writer,
                )?;

                Ok(())
            },
            (InputConvertModeCli::ShEx, OutputConvertModeCli::Html) => match &self.args.target_folder {
                None => Err(anyhow!(
                    "Conversion from ShEx to HTML requires an output parameter to indicate where to write the generated HTML files"
                )),
                Some(output_path) => {
                    run_shex2html(
                        &self.args.file,
                        &(&self.args.format).into(),
                        &self.args.base,
                        output_path,
                        &self.args.template_folder,
                        ctx.rudof.config(),
                        &reader_mode,
                    )?;

                    Ok(())
                },
            },
            (InputConvertModeCli::Dctap, OutputConvertModeCli::Uml) => {
                run_tap2uml(
                    &self.args.file,
                    &(&self.args.format).into(),
                    &self.args.shape,
                    &(&self.args.result_format).into(),
                    ctx.rudof.config(),
                    &mut ctx.writer,
                )?;

                Ok(())
            },
            (InputConvertModeCli::Dctap, OutputConvertModeCli::Html) => match &self.args.target_folder {
                None => Err(anyhow!(
                    "Conversion from DCTAP to HTML requires parameter `target-folder` to indicate where to write the generated HTML files"
                )),
                Some(output_path) => {
                    run_tap2html(
                        &self.args.file,
                        &(&self.args.format).into(),
                        output_path,
                        &self.args.template_folder,
                        ctx.rudof.config(),
                    )?;

                    Ok(())
                },
            },
            _ => {
                let input_mode = &self.args.input_mode;
                let output_mode = &self.args.output_mode;
                Err(anyhow!(
                    "Conversion from {input_mode} to {output_mode} is not supported yet"
                ))
            },
        }
    }
}
