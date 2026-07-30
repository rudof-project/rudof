use anyhow::{Context, Result};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ShExParser;
use shex_ast::ShapeMapParser;
use shex_ast::ir::map_action_extension::MapActionExtension;
use shex_ast::ir::map_state::MapState;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_actions_registry::SemanticActionsRegistry;
use shex_ast::ir::test_action_extension::TestActionExtension;
use shex_ast::shapemap::QueryShapeMap;
use shex_ast::{ResolveMethod, Schema};
use shex_validation::{Validator, ValidatorConfig};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;

const RUNS_PER_SIZE: usize = 5;
const STEP_SIZES: &[usize] = &[4, 8, 16, 32, 64, 128, 256, 512, 1024];
const CORPUS_SUBDIR: &str = "ShEx_shapes";

struct Case {
    n: usize,
    schema: PathBuf,
    data: PathBuf,
    shapemap: PathBuf,
}

fn discover_cases(root: &Path) -> Vec<Case> {
    STEP_SIZES
        .iter()
        .map(|&n| Case {
            n,
            schema: root
                .join(format!("{n}_steps"))
                .join(format!("shape_{n}_steps_ShEx.shex")),
            data: root
                .join(format!("{n}_steps"))
                .join(format!("data_graph_{n}_steps_ShEx.ttl")),
            shapemap: root
                .join(format!("{n}_steps"))
                .join(format!("shape_map_{n}_steps_entity_0.shex")),
        })
        .filter(|c| c.schema.exists() && c.data.exists() && c.shapemap.exists())
        .collect()
}

fn parse_schema(src: &str) -> Result<Schema> {
    let base = IriS::new_unchecked("http://example.org/");
    Ok(ShExParser::parse(src, None, &base)?)
}

fn compile_schema(schema: &Schema, cfg: &ValidatorConfig) -> Result<SchemaIR> {
    let registry = SemanticActionsRegistry::new().with(vec![
        Box::new(TestActionExtension::new()),
        Box::new(MapActionExtension::new(MapState::default())),
    ]);
    let mut ir = SchemaIR::new(registry);
    ir.populate_from_schema_json(schema, cfg.external_resolvers(), &ResolveMethod::default(), &None)?;
    Ok(ir)
}

fn load_rdf(src: &str) -> Result<OxigraphInMemory> {
    Ok(OxigraphInMemory::from_str(
        src,
        &RDFFormat::Turtle,
        Some("http://example.org/"),
        &ReaderMode::Strict,
    )?)
}

fn parse_shapemap(src: &str) -> Result<QueryShapeMap> {
    Ok(ShapeMapParser::parse(src, &None, &None, &None, &None)?)
}

#[derive(Default, Clone)]
struct Phases {
    parse: f64,
    compile: f64,
    load_rdf: f64,
    parse_sm: f64,
    validator_init: f64,
    validate: f64,
}

impl Phases {
    fn total(&self) -> f64 {
        self.parse + self.compile + self.load_rdf + self.parse_sm + self.validator_init + self.validate
    }
    fn add(&mut self, other: &Phases) {
        self.parse += other.parse;
        self.compile += other.compile;
        self.load_rdf += other.load_rdf;
        self.parse_sm += other.parse_sm;
        self.validator_init += other.validator_init;
        self.validate += other.validate;
    }
    fn scale(&mut self, factor: f64) {
        self.parse *= factor;
        self.compile *= factor;
        self.load_rdf *= factor;
        self.parse_sm *= factor;
        self.validator_init *= factor;
        self.validate *= factor;
    }
}

fn run_one(case: &Case) -> Result<(Phases, String)> {
    let schema_src = std::fs::read_to_string(&case.schema)?;
    let data_src = std::fs::read_to_string(&case.data)?;
    let sm_src = std::fs::read_to_string(&case.shapemap)?;

    let cfg = ValidatorConfig::default();

    let mut sum = Phases::default();
    let mut status_str = String::from("?");

    for _ in 0..RUNS_PER_SIZE {
        let mut phases = Phases::default();

        let t = Instant::now();
        let schema = parse_schema(&schema_src).context("parse schema")?;
        phases.parse = t.elapsed().as_secs_f64();

        let t = Instant::now();
        let ir = compile_schema(&schema, &cfg).context("compile schema")?;
        phases.compile = t.elapsed().as_secs_f64();

        let t = Instant::now();
        let rdf = load_rdf(&data_src).context("load rdf")?;
        phases.load_rdf = t.elapsed().as_secs_f64();

        let t = Instant::now();
        let shapemap = parse_shapemap(&sm_src).context("parse shapemap")?;
        phases.parse_sm = t.elapsed().as_secs_f64();

        let t = Instant::now();
        let validator = Validator::new(&ir, &cfg).context("validator init")?;
        phases.validator_init = t.elapsed().as_secs_f64();

        let t = Instant::now();
        let result = validator
            .validate_shapemap(&shapemap, &rdf, &ir, &None)
            .context("validate")?;
        phases.validate = t.elapsed().as_secs_f64();

        status_str = result
            .iter()
            .next()
            .map(|(_, _, s)| match s {
                shex_ast::shapemap::ValidationStatus::Conformant(_) => "OK".to_string(),
                shex_ast::shapemap::ValidationStatus::NonConformant(_) => "FAIL".to_string(),
                other => format!("{:?}", other),
            })
            .unwrap_or_else(|| "-".to_string());

        sum.add(&phases);
    }

    sum.scale(1.0 / RUNS_PER_SIZE as f64);
    Ok((sum, status_str))
}

fn extract_root() -> PathBuf {
    if let Ok(dir) = std::env::var("CARGO_TARGET_TMPDIR") {
        return PathBuf::from(dir).join("bench-issue730-corpus");
    }
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(dir).join("bench-issue730-corpus");
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/bench-issue730-corpus")
}

fn ensure_extracted(target: &Path) -> Result<PathBuf> {
    let expected = target.join(CORPUS_SUBDIR);
    if expected.exists() {
        return Ok(expected);
    }
    let zip_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join("shex_shapes.zip");
    std::fs::create_dir_all(target).with_context(|| format!("create {}", target.display()))?;
    let file = File::open(&zip_path).with_context(|| format!("open {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file).with_context(|| format!("read zip {}", zip_path.display()))?;
    archive
        .extract(target)
        .with_context(|| format!("extract into {}", target.display()))?;
    eprintln!("Extracted corpus into {}", expected.display());
    Ok(expected)
}

fn main() -> Result<()> {
    let root = ensure_extracted(&extract_root())?;
    let cases = discover_cases(&root);

    if cases.is_empty() {
        anyhow::bail!("no cases found under {}", root.display());
    }

    println!("Corpus: {}", root.display());
    println!(
        "Runs per size: {} (reporting mean per phase, all in ms)\n",
        RUNS_PER_SIZE
    );
    println!(
        "{:>6} {:>6} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12} {:>12}",
        "steps", "verd.", "parse", "compile", "load_rdf", "parse_sm", "val_init", "validate", "total"
    );
    println!(
        "{:>6} {:>6} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12} {:>12}",
        "-----", "-----", "-----", "-------", "--------", "--------", "--------", "--------", "-----"
    );

    for case in &cases {
        let (p, status) = run_one(case)?;
        println!(
            "{:>6} {:>6} {:>10.2} {:>10.2} {:>10.2} {:>10.2} {:>12.2} {:>12.4} {:>12.2}",
            case.n,
            status,
            p.parse * 1000.0,
            p.compile * 1000.0,
            p.load_rdf * 1000.0,
            p.parse_sm * 1000.0,
            p.validator_init * 1000.0,
            p.validate * 1000.0,
            p.total() * 1000.0
        );
    }
    Ok(())
}
