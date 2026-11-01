use crate::cli::parser::{CompareArgs, CompareFormatCli, CompareModeCli};
use crate::commands::base::{Command, CommandContext};
use anyhow::{Result, anyhow, bail};
use rudof_lib::formats::{ComparisonFormat, DataFormat, DataReaderMode, ResultComparisonFormat};
use rudof_lib::types::{SubgraphMode, describe_relation};
use std::str::FromStr;

/// Implementation of the `compare` command.
///
/// It compares two schemas (`--schema1` and `--schema2`) or two RDF graphs up
/// to blank node renaming (`--data1` and `--data2`). The options say which,
/// unless `--mode` does.
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
        match compare_mode(&self.args)? {
            CompareModeCli::Schema => self.compare_schemas(ctx),
            CompareModeCli::Data => self.compare_data(ctx),
        }
    }
}

/// The mode given with `--mode`, or else the one the input options imply.
fn compare_mode(args: &CompareArgs) -> Result<CompareModeCli> {
    let schemas = args.schema1.is_some() || args.schema2.is_some();
    let data = args.data1.is_some() || args.data2.is_some();
    match (args.mode, schemas, data) {
        (_, true, true) => bail!(
            "Use --schema1 and --schema2 to compare schemas, or --data1 and --data2 to compare RDF data, not both"
        ),
        (Some(CompareModeCli::Schema), false, true) => {
            bail!("--data1 and --data2 compare RDF data: use --mode data, or leave --mode out")
        },
        (Some(CompareModeCli::Data), true, false) => {
            bail!("--schema1 and --schema2 compare schemas: use --mode schema, or leave --mode out")
        },
        (Some(mode), _, _) => Ok(mode),
        (None, true, false) => Ok(CompareModeCli::Schema),
        (None, false, true) => Ok(CompareModeCli::Data),
        (None, false, false) => bail!(
            "Nothing to compare: use --schema1 and --schema2 to compare schemas, or --data1 and --data2 to \
             compare RDF data"
        ),
    }
}

impl CompareCommand {
    fn compare_schemas(&self, ctx: &mut CommandContext) -> Result<()> {
        let args = &self.args;
        let (Some(schema1), Some(schema2)) = (&args.schema1, &args.schema2) else {
            bail!("Comparing schemas needs --schema1 and --schema2");
        };
        let format1 = schema_format(args.format1)?;
        let format2 = schema_format(args.format2)?;
        let input_mode1 = args.input_mode1.into();
        let input_mode2 = args.input_mode2.into();
        let result_format = args.result_format.into();

        let mut comparison = ctx
            .rudof
            .show_schema_comparison(
                schema1,
                schema2,
                &format1,
                &format2,
                &input_mode1,
                &input_mode2,
                &mut ctx.writer,
            )
            .with_result_format(&result_format);

        if let Some(base1) = args.base1.as_deref() {
            comparison = comparison.with_base1(base1);
        }
        if let Some(base2) = args.base2.as_deref() {
            comparison = comparison.with_base2(base2);
        }
        if let Some(shape1) = args.shape1.as_deref() {
            comparison = comparison.with_shape1(shape1);
        }
        if let Some(shape2) = args.shape2.as_deref() {
            comparison = comparison.with_shape2(shape2);
        }
        if let Some(show_time) = args.show_time {
            comparison = comparison.with_show_time(show_time);
        }

        comparison.execute()?;

        Ok(())
    }

    fn compare_data(&self, ctx: &mut CommandContext) -> Result<()> {
        let args = &self.args;
        let Some(data2) = &args.data2 else {
            bail!(
                "Comparing data needs --data2, and --data1 too unless the first graph is the loaded data (in the shell)"
            );
        };
        let format1 = data_format(args.format1)?;
        let format2 = data_format(args.format2)?;
        let reader_mode: DataReaderMode = args.reader_mode.into();
        let mode: SubgraphMode = args.subgraph_mode.into();

        let mut b = match &args.data1 {
            Some(data1) => {
                let mut b = ctx.rudof.compare_data(data1, data2);
                if let Some(f) = &format1 {
                    b = b.with_format1(f);
                }
                if let Some(base) = args.base1.as_deref() {
                    b = b.with_base1(base);
                }
                b
            },
            None => {
                if !ctx.rudof.has_data() {
                    bail!(
                        "Comparing data needs --data1 and --data2, or only --data2 to compare it with the \
                         loaded data (load some with `data` in the shell)"
                    );
                }
                ctx.rudof.compare_loaded_data(data2)
            },
        };
        if let Some(f) = &format2 {
            b = b.with_format2(f);
        }
        if let Some(base) = args.base2.as_deref() {
            b = b.with_base2(base);
        }
        let b = b.with_reader_mode(&reader_mode).with_mode(&mode);
        let result_format = ResultComparisonFormat::from(args.result_format);

        if !args.show_diff {
            let relation = b.execute()?;
            match result_format {
                ResultComparisonFormat::Internal => writeln!(
                    ctx.writer,
                    "{}",
                    describe_relation(relation, mode, args.data1.is_none())
                )?,
                ResultComparisonFormat::Json => {
                    let json = serde_json::json!({ "relation": relation, "mode": mode });
                    writeln!(ctx.writer, "{}", serde_json::to_string_pretty(&json)?)?
                },
            }
            return Ok(());
        }

        let b = match args.exact_search_limit {
            Some(limit) => b.with_exact_search_limit(limit),
            None => b,
        };
        let diff = b.execute_diff()?;
        match result_format {
            ResultComparisonFormat::Internal => write!(ctx.writer, "{diff}")?,
            ResultComparisonFormat::Json => writeln!(ctx.writer, "{}", serde_json::to_string_pretty(&diff.diff)?)?,
        }
        Ok(())
    }
}

/// The schema format of `format`, ShExC if not given.
fn schema_format(format: Option<CompareFormatCli>) -> Result<ComparisonFormat> {
    match format {
        None => Ok(ComparisonFormat::ShExC),
        Some(f) => ComparisonFormat::from_str(&f.to_string())
            .map_err(|_| anyhow!("Format '{f}' is not a schema format: use shexc, shexj, turtle, rdfxml or ntriples")),
    }
}

/// The RDF format of `format`, or None for the default (Turtle).
fn data_format(format: Option<CompareFormatCli>) -> Result<Option<DataFormat>> {
    format
        .map(|f| {
            DataFormat::from_str(&f.to_string()).map_err(|_| {
                anyhow!("Format '{f}' is not an RDF format: use turtle, ntriples, rdfxml, trig, n3, nquads or jsonld")
            })
        })
        .transpose()
}
