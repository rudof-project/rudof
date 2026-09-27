use crate::cli::parser::ShexmapArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Context, Result, anyhow, bail};
use rudof_lib::formats::{InputSpec, ResultDataFormat};
use shex_ast::shexmap::{Source, SourceKind};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Implementation of the `shexmap` command: map RDF from one ShEx schema to another.
///
/// Binds the input node against the input schema (`%Map:{ %}` actions bind variables),
/// then materializes the output schema from the bindings (the same actions place them).
/// Either half can run alone: `--bindings-out` writes the bindings, `--bindings` reads them
/// back; `--check` analyses the schema pair without data.
pub struct ShexmapCommand {
    args: ShexmapArgs,
}

impl ShexmapCommand {
    pub fn new(args: ShexmapArgs) -> Self {
        Self { args }
    }

    fn load_schema(
        &self,
        ctx: &mut CommandContext,
        schema: &InputSpec,
        format: rudof_lib::formats::ShExFormat,
    ) -> Result<()> {
        let reader_mode = self.args.reader_mode.into();
        let mut loading = ctx
            .rudof
            .load_shex_schema(schema)
            .with_shex_schema_format(&format)
            .with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base_schema.as_deref() {
            loading = loading.with_base(base);
        }
        loading.execute()?;
        Ok(())
    }

    fn static_vars(&self) -> Result<HashMap<String, String>> {
        let Some(path) = &self.args.static_vars else {
            return Ok(HashMap::new());
        };
        let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let value: serde_json::Value =
            serde_json::from_str(&text).with_context(|| format!("parsing {} as JSON", path.display()))?;
        let Some(obj) = value.as_object() else {
            bail!("{} must hold a JSON object of variable IRIs to values", path.display());
        };
        Ok(obj
            .iter()
            .map(|(k, v)| {
                let text = match v {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                (k.clone(), text)
            })
            .collect())
    }
}

impl Command for ShexmapCommand {
    fn name(&self) -> &'static str {
        "shexmap"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let schema_format = self.args.schema_format.into();
        let output_schema_format = self.args.output_schema_format.into();
        let result_format: ResultDataFormat = self.args.result_format.into();
        let statics = self.static_vars()?;

        if self.args.check {
            let (Some(schema), Some(output_schema)) = (&self.args.schema, &self.args.output_schema) else {
                bail!("--check needs --schema and --output-schema");
            };
            self.load_schema(ctx, schema, schema_format)?;
            let input = ctx
                .rudof
                .shex_schema()
                .cloned()
                .ok_or_else(|| anyhow!("the input schema did not load"))?;
            self.load_schema(ctx, output_schema, output_schema_format)?;
            let mut check = ctx.rudof.shexmap_check(&input);
            if let Some(shape) = self.args.shape.as_deref() {
                check = check.with_input_shape(shape);
            }
            if let Some(shape) = self.args.output_shape.as_deref() {
                check = check.with_output_shape(shape);
            }
            if let Some(base) = self.args.base_schema.as_deref() {
                check = check.with_base_shapes(base);
            }
            let report = check.with_static_vars(statics.keys().cloned().collect()).execute()?;
            writeln!(ctx.writer, "{report}")?;
            if !report.ok() {
                bail!("the schemas do not map coherently");
            }
            return Ok(());
        }

        // 1. bindings: read them, or bind the data
        if let Some(bindings) = &self.args.bindings {
            let reader = bindings
                .open_read(Some("application/json"), "ShExMap bindings")
                .with_context(|| format!("opening bindings {}", bindings.source_name()))?;
            ctx.rudof.shexmap_load_bindings(reader)?;
        } else {
            let (Some(schema), Some(node)) = (&self.args.schema, &self.args.node) else {
                bail!("give --schema, --node and the input data, or --bindings");
            };
            if self.args.data.is_empty() {
                bail!("no input data: give an RDF file, URI or - for stdin (or --bindings)");
            }
            let data_format = self.args.data_format.into();
            let reader_mode = self.args.reader_mode.into();
            let mut loading = ctx
                .rudof
                .load_data()
                .with_data(&self.args.data)
                .with_data_format(&data_format)
                .with_reader_mode(&reader_mode);
            if let Some(base) = self.args.base_data.as_deref() {
                loading = loading.with_base(base);
            }
            loading.execute()?;
            self.load_schema(ctx, schema, schema_format)?;
            let mut bind = ctx
                .rudof
                .shexmap_bind(node)
                .with_strict(self.args.strict)
                .with_validate(!self.args.no_validate);
            if let Some(shape) = self.args.shape.as_deref() {
                bind = bind.with_shape(shape);
            }
            if let Some(base) = self.args.base_data.as_deref() {
                bind = bind.with_base_nodes(base);
            }
            if let Some(base) = self.args.base_schema.as_deref() {
                bind = bind.with_base_shapes(base);
            }
            bind.execute()?;
            if let Some(b) = ctx.rudof.shexmap_bindings()
                && b.ambiguous()
            {
                eprintln!(
                    "shexmap: warning: the input matches in {} ways that bind differently; using the first \
                     (--strict makes this an error)",
                    b.alternatives
                );
            }
        }

        // 2. write the bindings, if asked
        if let Some(out) = &self.args.bindings_out {
            if out == "-" {
                ctx.rudof.shexmap_serialize_bindings(&mut ctx.writer, true)?;
            } else {
                let mut file = BufWriter::new(File::create(out).with_context(|| format!("creating {out}"))?);
                ctx.rudof.shexmap_serialize_bindings(&mut file, true)?;
            }
        }

        // 3. materialize, if asked
        let Some(output_schema) = &self.args.output_schema else {
            if self.args.bindings_out.is_none() {
                bail!("nothing to do: give --output-schema and/or --bindings-out");
            }
            return Ok(());
        };
        self.load_schema(ctx, output_schema, output_schema_format)?;
        let into_spec = self.args.into.as_ref().map(|p| InputSpec::Path(p.clone()));
        let mut buffer: Vec<u8> = Vec::new();
        let summary = {
            let mut m = ctx
                .rudof
                .shexmap_materialize(&mut buffer)
                .with_static_vars(statics)
                .with_strict(self.args.strict)
                .with_result_format(&result_format);
            if let Some(root) = self.args.root.as_deref() {
                m = m.with_root(root);
            }
            if let Some(shape) = self.args.output_shape.as_deref() {
                m = m.with_shape(shape);
            }
            if let Some(base) = self.args.base_data.as_deref() {
                m = m.with_base_nodes(base);
            }
            if let Some(base) = self.args.base_schema.as_deref() {
                m = m.with_base_shapes(base);
            }
            if let Some(into) = &into_spec {
                m = m.with_into(into);
            }
            m.execute()?
        };
        if summary.alternatives > 1 {
            eprintln!(
                "shexmap: warning: the bindings fit the output schema in {} ways; using the one that uses the \
                 most bindings",
                summary.alternatives
            );
        }
        if let Some(path) = &self.args.provenance {
            let mut file = BufWriter::new(File::create(path).with_context(|| format!("creating {}", path.display()))?);
            for (triple, source) in summary.chosen.triples.iter().zip(&summary.chosen.provenance) {
                writeln!(file, "{}", serde_json::to_string(&provenance_json(triple, source))?)?;
            }
        }
        match (&self.args.into, &self.args.common.output) {
            (Some(into), None) => {
                // written back in place, unless -o names another file (handled by the context's writer)
                std::fs::write(into, &buffer).with_context(|| format!("writing {}", into.display()))?;
                eprintln!(
                    "shexmap: {}: {} triples added, {} removed",
                    into.display(),
                    summary.added.len(),
                    summary.removed.len()
                );
            },
            _ => {
                ctx.writer.write_all(&buffer)?;
            },
        }
        Ok(())
    }
}

/// A triple's provenance record as JSON: terms in N3, the input node likewise.
fn provenance_json(triple: &shex_ast::shexmap::MapTriple, source: &Source) -> serde_json::Value {
    let n3 = shex_ast::shexmap::term::n3;
    let (kind, detail) = match &source.kind {
        SourceKind::Variable(v) => ("variable", serde_json::Value::String(v.clone())),
        SourceKind::Code(c) => ("code", serde_json::Value::String(c.clone())),
        SourceKind::Named(v) => ("named", serde_json::Value::String(v.clone())),
        SourceKind::Keyed(c) => ("keyed", serde_json::Value::String(c.clone())),
        SourceKind::Constant => ("constant", serde_json::Value::Bool(true)),
        SourceKind::Structural => ("structural", serde_json::Value::Bool(true)),
    };
    serde_json::json!({
        "subject": n3(&triple.subject),
        "predicate": format!("<{}>", triple.predicate.as_str()),
        "object": n3(&triple.object),
        "constraint": source.predicate,
        "scope": source.scope,
        "node": source.node.as_ref().map(n3),
        "reads": source.reads.iter().map(|(path, var)| serde_json::json!([path, var])).collect::<Vec<_>>(),
        "statics": source.statics,
        "kind": kind,
        kind: detail,
    })
}
