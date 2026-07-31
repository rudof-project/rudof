#[cfg(not(target_family = "wasm"))]
mod native {
    use anyhow::{Context, Result};
    use rudof_lib::formats::{DataFormat, InputSpec, ShExFormat, ShapeMapFormat};
    use rudof_lib::{Rudof, RudofConfig};
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

    /// Preload schema, data and shapemap once; then time only `validate_shex`.
    fn mean_validate_time(case: &Case) -> Result<f64> {
        let mut rudof = Rudof::new(RudofConfig::default());
        let schema_spec = InputSpec::Path(case.schema.clone());
        let data_spec = [InputSpec::Path(case.data.clone())];
        let sm_spec = InputSpec::Path(case.shapemap.clone());

        rudof
            .load_shex_schema(&schema_spec)
            .with_shex_schema_format(&ShExFormat::ShExC)
            .execute()
            .context("load_shex_schema")?;
        rudof
            .load_data()
            .with_data(&data_spec)
            .with_data_format(&DataFormat::Turtle)
            .execute()
            .context("load_data")?;
        rudof
            .load_shapemap(&sm_spec)
            .with_shapemap_format(&ShapeMapFormat::Compact)
            .execute()
            .context("load_shapemap")?;

        let mut sum = 0.0;
        for _ in 0..RUNS_PER_SIZE {
            let t = Instant::now();
            rudof.validate_shex().execute().context("validate_shex")?;
            sum += t.elapsed().as_secs_f64();
        }
        Ok(sum / RUNS_PER_SIZE as f64)
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

    pub fn run() -> Result<()> {
        let root = ensure_extracted(&extract_root())?;
        let cases = discover_cases(&root);

        if cases.is_empty() {
            anyhow::bail!("no cases found under {}", root.display());
        }

        println!("Corpus: {}", root.display());
        println!(
            "Runs per size: {} (reporting mean of validate_shex, ms)\n",
            RUNS_PER_SIZE
        );
        println!("{:>6}  {:>15}", "steps", "validate (ms)");
        println!("{:>6}  {:>15}", "-----", "-------------");

        for case in &cases {
            let mean = mean_validate_time(case)?;
            println!("{:>6}  {:>15.4}", case.n, mean * 1000.0);
        }
        Ok(())
    }
}

#[cfg(not(target_family = "wasm"))]
fn main() -> anyhow::Result<()> {
    native::run()
}

#[cfg(target_family = "wasm")]
fn main() {}
