//! Generate data from a ShEx or SHACL schema and validate it against that same
//! schema with rudof's validators. Prototype of `rudof generate --validate`.
//!
//! ```sh
//! cargo run --release -p rudof_generate --example validate_generated -- \
//!     <schema.shex|schema.ttl> <out.ttl> [generator-config.toml] [entities] [seed] [fill]
//! ```
//!
//! Prints one line: `format=… entities=… secs=… load_secs=… validate_secs=… conformant=… violations=…`,
//! followed by the most frequent violation reasons.

#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use rudof_generate::adapters::shex::ShExToUnified;
    use rudof_generate::{DataGenerator, GeneratorConfig, SchemaFormat};
    use rudof_lib::formats::{
        DataFormat, InputSpec, ResultShExValidationFormat, ResultShaclValidationFormat, ShExFormat, ShaclFormat,
        ShapeMapFormat,
    };
    use rudof_lib::{Rudof, RudofConfig};
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::time::Instant;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let schema = PathBuf::from(args.first().ok_or("missing schema path")?);
    let output = PathBuf::from(args.get(1).ok_or("missing output path")?);
    let format = match schema.extension().and_then(|e| e.to_str()) {
        Some("shex") => SchemaFormat::ShEx,
        Some("pgs") => SchemaFormat::PgSchema,
        _ => SchemaFormat::Shacl,
    };

    let mut config = match args.get(2).filter(|a| !a.is_empty() && *a != "-") {
        Some(path) => GeneratorConfig::from_toml_file(path)?,
        None => GeneratorConfig::default(),
    };
    if let Some(n) = args.get(3) {
        config.generation.entity_count = n.parse()?;
    }
    if let Some(seed) = args.get(4) {
        config.generation.seed = Some(seed.parse()?);
    }
    if let Some(fill) = args.get(5) {
        config.generation.property_fill_probability = fill.parse()?;
    }
    config.output.path = output.clone();
    config.output.write_stats = false;
    if format == SchemaFormat::PgSchema {
        config.output.format = rudof_generate::config::OutputFormat::Pg;
        if config.output.shapemap.is_none() {
            config.output.shapemap = Some(output.with_extension("map"));
        }
    }
    let type_map = config.output.shapemap.clone();
    let entities = config.generation.entity_count;

    let started = Instant::now();
    let mut generator = DataGenerator::new(config)?;
    generator.run_with_format(&schema, Some(format)).await?;
    let secs = started.elapsed().as_secs_f64();

    let schema_text = std::fs::read_to_string(&schema)?;
    if format == SchemaFormat::PgSchema {
        let type_map = type_map.ok_or("a type map path is needed")?;
        return validate_property_graph(&schema_text, &output, &type_map, entities, secs);
    }

    // Parsing the data is timed apart, so that validate_secs compares with validators
    // that are timed on an already parsed graph.
    let load_started = Instant::now();
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof
        .load_data()
        .with_data(&[InputSpec::path(&output)])
        .with_data_format(&DataFormat::Turtle)
        .execute()?;
    let load_secs = load_started.elapsed().as_secs_f64();

    let validation_started = Instant::now();

    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
    let (mut conformant, mut violations) = (0usize, 0usize);
    match format {
        SchemaFormat::ShEx => {
            // The generator's exact ShapeMap when it wrote one (each node against the
            // shape that generated it); otherwise one query association per shape
            // with a target class, which also checks a node typed with several
            // classes against every one of their shapes.
            let shapemap = match type_map.as_ref().filter(|p| p.exists()) {
                Some(path) => std::fs::read_to_string(path)?,
                None => {
                    let (model, _) = ShExToUnified.convert_schema(schema_text.clone()).await?;
                    model
                        .shapes
                        .values()
                        .filter_map(|s| s.target_class.as_ref().map(|c| format!("{{FOCUS a <{c}>}}@<{}>", s.id)))
                        .collect::<Vec<_>>()
                        .join(",\n")
                },
            };
            rudof
                .load_shex_schema(&InputSpec::str(&schema_text))
                .with_shex_schema_format(&ShExFormat::ShExC)
                .execute()?;
            rudof
                .load_shapemap(&InputSpec::str(&shapemap))
                .with_shapemap_format(&ShapeMapFormat::Compact)
                .execute()?;
            rudof.validate_shex().execute()?;
            let mut buf = Vec::new();
            rudof
                .serialize_shex_validation_results(&mut buf)
                .with_result_shex_validation_format(&ResultShExValidationFormat::Csv)
                .execute()?;
            let report = String::from_utf8(buf)?;
            if let Ok(path) = std::env::var("VALIDATION_REPORT") {
                std::fs::write(path, &report)?;
            }
            let mut lines = report.lines().peekable();
            while let Some(line) = lines.next() {
                if !(line.starts_with('<') || line.starts_with("_:")) {
                    continue;
                }
                match line.split(',').nth(2) {
                    Some("OK") => conformant += 1,
                    Some("FAIL") => {
                        violations += 1;
                        // The reason is on the following "rejected:" line(s), if any.
                        while let Some(next) = lines.peek() {
                            if next.starts_with('<') || next.starts_with("_:") {
                                break;
                            }
                            if let Some((_, why)) = next.split_once("rejected: ") {
                                let why = why.trim_end_matches('"');
                                let key = why.split(" but got").next().unwrap_or(why).to_string();
                                *reasons.entry(key).or_default() += 1;
                            }
                            lines.next();
                        }
                    },
                    _ => {},
                }
            }
        },
        SchemaFormat::PgSchema => unreachable!("validated above"),
        SchemaFormat::Shacl => {
            rudof
                .load_shacl_shapes()
                .with_shacl_schema(&InputSpec::path(&schema))
                .with_shacl_schema_format(&ShaclFormat::Turtle)
                .execute()?;
            rudof.validate_shacl().execute()?;
            let mut buf = Vec::new();
            rudof
                .serialize_shacl_validation_results(&mut buf)
                .with_result_shacl_validation_format(&ResultShaclValidationFormat::NTriples)
                .execute()?;
            let report = String::from_utf8(buf)?;
            if let Ok(path) = std::env::var("VALIDATION_REPORT") {
                std::fs::write(path, &report)?;
            }
            for line in report.lines() {
                if line.contains("<http://www.w3.org/ns/shacl#resultSeverity>") {
                    violations += 1;
                }
                if line.contains("<http://www.w3.org/ns/shacl#sourceConstraintComponent>")
                    && let Some(component) = line.split_whitespace().nth(2)
                {
                    *reasons.entry(component.to_string()).or_default() += 1;
                }
            }
        },
    }

    let validate_secs = validation_started.elapsed().as_secs_f64();
    println!(
        "format={format:?} entities={entities} secs={secs:.3} load_secs={load_secs:.3} validate_secs={validate_secs:.3} conformant={conformant} violations={violations}"
    );
    let mut top: Vec<_> = reasons.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));
    for (reason, count) in top.into_iter().take(8) {
        println!("  {count:>7}  {reason}");
    }
    Ok(())
}

/// Validate the property graph at `output` with rudof's PG-Schema validator, for
/// every association of the type map. The report (`VALIDATION_REPORT`) is CSV in
/// the form of the ShEx one: `id,type,OK|FAIL,details`.
#[cfg(not(target_family = "wasm"))]
fn validate_property_graph(
    schema: &str,
    output: &std::path::Path,
    type_map: &std::path::Path,
    entities: usize,
    secs: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    use pgschema::parser::{map_builder::MapBuilder, pg_builder::PgBuilder, pgs_builder::PgsBuilder};
    use std::time::Instant;

    let load_started = Instant::now();
    let schema = PgsBuilder::new().parse_pgs(schema)?;
    let graph = PgBuilder::new().parse_pg(&std::fs::read_to_string(output)?)?;
    let map = MapBuilder::new().parse_map(&std::fs::read_to_string(type_map)?)?;
    let load_secs = load_started.elapsed().as_secs_f64();

    let validation_started = Instant::now();
    let result = map.validate(&schema, &graph)?;
    let validate_secs = validation_started.elapsed().as_secs_f64();

    let mut report = String::new();
    let (mut conformant, mut violations) = (0usize, 0usize);
    for a in &result.associations {
        if a.conforms {
            conformant += 1;
        } else {
            violations += 1;
        }
        let status = if a.conforms { "OK" } else { "FAIL" };
        let details = format!("{:?}", a.details).replace('"', "'");
        report.push_str(&format!(
            "{},{},{status},\"{details}\"
",
            a.node_id, a.type_name
        ));
    }
    if let Ok(path) = std::env::var("VALIDATION_REPORT") {
        std::fs::write(path, &report)?;
    }
    println!(
        "format=PgSchema entities={entities} secs={secs:.3} load_secs={load_secs:.3} validate_secs={validate_secs:.3} conformant={conformant} violations={violations}"
    );
    Ok(())
}
