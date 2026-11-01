use crate::cli::parser::{CompareArgs, CompareFormatCli, CompareModeCli};
use crate::commands::base::{Command, CommandContext};
use anyhow::{Result, anyhow, bail};
use rudof_lib::formats::{ComparisonFormat, DataFormat, DataReaderMode, ResultComparisonFormat};
use rudof_lib::types::{GraphRelation, SubgraphMode};
use std::str::FromStr;

/// Implementation of the `compare` command.
///
/// With `--mode schema` (the default) it compares two schemas; with
/// `--mode data` it compares two RDF graphs up to blank node renaming.
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
        match self.args.mode {
            CompareModeCli::Schema => self.compare_schemas(ctx),
            CompareModeCli::Data => self.compare_data(ctx),
        }
    }
}

impl CompareCommand {
    fn compare_schemas(&self, ctx: &mut CommandContext) -> Result<()> {
        let args = &self.args;
        let schema1 = args
            .schema1
            .as_ref()
            .ok_or_else(|| anyhow!("--schema1 is required to compare schemas"))?;
        let format1 = schema_format(args.format1)?;
        let format2 = schema_format(args.format2)?;
        let input_mode1 = args.input_mode1.into();
        let input_mode2 = args.input_mode2.into();
        let result_format = args.result_format.into();

        let mut comparison = ctx
            .rudof
            .show_schema_comparison(
                schema1,
                &args.schema2,
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
        let format1 = data_format(args.format1)?;
        let format2 = data_format(args.format2)?;
        let reader_mode: DataReaderMode = args.reader_mode.into();
        let mode: SubgraphMode = args.subgraph_mode.into();

        let mut b = match &args.schema1 {
            Some(first) => {
                let mut b = ctx.rudof.compare_data(first, &args.schema2);
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
                ctx.rudof.compare_loaded_data(&args.schema2)
            },
        };
        if let Some(f) = &format2 {
            b = b.with_format2(f);
        }
        if let Some(base) = args.base2.as_deref() {
            b = b.with_base2(base);
        }
        let relation = b.with_reader_mode(&reader_mode).with_mode(&mode).execute()?;

        let (first, second) = if args.schema1.is_some() {
            ("the first graph", "the second graph")
        } else {
            ("the loaded data", "the given graph")
        };
        match ResultComparisonFormat::from(args.result_format) {
            ResultComparisonFormat::Internal => writeln!(
                ctx.writer,
                "{relation}: {} (mode: {mode})",
                explain(relation, first, second)
            )?,
            ResultComparisonFormat::Json => {
                let json = serde_json::json!({ "relation": relation, "mode": mode });
                writeln!(ctx.writer, "{}", serde_json::to_string_pretty(&json)?)?
            },
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

fn explain(relation: GraphRelation, first: &str, second: &str) -> String {
    match relation {
        GraphRelation::Isomorphic => format!("{first} and {second} are equal up to blank node renaming"),
        GraphRelation::Equivalent => format!("{first} and {second} contain each other, but are not isomorphic"),
        GraphRelation::Subgraph => format!("{first} is contained in {second}, but not the other way round"),
        GraphRelation::Supergraph => format!("{second} is contained in {first}, but not the other way round"),
        GraphRelation::Different => format!("neither {first} nor {second} is contained in the other"),
    }
}
