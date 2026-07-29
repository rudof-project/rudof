use bench_shex::corpus::{self, Case, Size};
use criterion::{BenchmarkId, Criterion};
use rudof_lib::{
    Rudof, RudofConfig,
    formats::{DataFormat, DataReaderMode, InputSpec, ShExFormat, ShapeMapFormat},
};
use std::fs::File;
use std::path::{Path, PathBuf};

fn bench_validate_precompiled(c: &mut Criterion) {
    let cases = corpus::load_all().expect("load corpus");
    let config = RudofConfig::new();

    for size in [Size::Small, Size::Large] {
        let mut group = c.benchmark_group(format!("validate_precompiled_{}", size.tag()));
        group.sample_size(20);
        for case in cases.iter().filter(|c| c.size == size) {
            let cache_path = match ensure_precompiled(case, &config) {
                Ok(path) => path,
                Err(err) => {
                    eprintln!(
                        "warning: skipping {} — precompiled cache round-trip failed: {err}",
                        case.id
                    );
                    continue;
                },
            };

            group.bench_with_input(
                BenchmarkId::new("from_source", &case.id),
                case,
                |b, case| {
                    b.iter(|| run_from_source(&config, case));
                },
            );

            group.bench_with_input(
                BenchmarkId::new("from_precompiled", &case.id),
                &(case, cache_path.clone()),
                |b, (case, cache_path)| {
                    b.iter(|| run_from_precompiled(&config, case, cache_path));
                },
            );
        }
        group.finish();
    }
}

fn run_from_source(config: &RudofConfig, case: &Case) {
    let mut rudof = Rudof::new(config.clone());

    rudof
        .load_shex_schema(&InputSpec::path(&case.schema_path))
        .with_base(case.base.as_str())
        .with_shex_schema_format(&ShExFormat::ShExC)
        .with_reader_mode(&DataReaderMode::Strict)
        .execute()
        .unwrap();

    load_data_and_validate(&mut rudof, case);
}

fn run_from_precompiled(config: &RudofConfig, case: &Case, cache_path: &Path) {
    let mut rudof = Rudof::new(config.clone());

    rudof
        .load_shex_schema_precompiled(&InputSpec::path(cache_path))
        .with_reader_mode(&DataReaderMode::Strict)
        .execute()
        .unwrap();

    load_data_and_validate(&mut rudof, case);
}

fn load_data_and_validate(rudof: &mut Rudof, case: &Case) {
    rudof
        .load_data()
        .with_data(&[InputSpec::path(&case.data_path)])
        .with_data_format(&DataFormat::Turtle)
        .with_base(case.base.as_str())
        .with_reader_mode(&DataReaderMode::Strict)
        .execute()
        .unwrap();

    rudof
        .load_shapemap(&InputSpec::path(&case.shapemap_path))
        .with_shapemap_format(&ShapeMapFormat::Compact)
        .execute()
        .unwrap();

    rudof.validate_shex().execute().unwrap();
}

/// Writes the precompiled `.rsir` for `case` under `bench-shex/corpus/precompiled/<size>/<case_id>.rsir` if it does not
/// already exist, and verifies that it can be read back.
fn ensure_precompiled(case: &Case, config: &RudofConfig) -> anyhow::Result<PathBuf> {
    let cache_path = precompiled_path(case);

    if !cache_path.exists() {
        if let Some(parent) = cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut rudof = Rudof::new(config.clone());
        rudof
            .load_shex_schema(&InputSpec::path(&case.schema_path))
            .with_base(case.base.as_str())
            .with_shex_schema_format(&ShExFormat::ShExC)
            .with_reader_mode(&DataReaderMode::Strict)
            .execute()?;

        let mut writer = File::create(&cache_path)?;
        rudof.compile_shex_schema_to_file(&mut writer).execute()?;
    }

    let mut probe = Rudof::new(config.clone());
    probe
        .load_shex_schema_precompiled(&InputSpec::path(&cache_path))
        .with_reader_mode(&DataReaderMode::Strict)
        .execute()?;

    Ok(cache_path)
}

fn precompiled_path(case: &Case) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join("precompiled")
        .join(case.size.tag())
        .join(format!("{}.rsir", sanitize(&case.id)))
}

/// Case ids may contain characters that are awkward in file names (slashes, spaces). Fold them to `_` so a case id maps to exactly one cache file.
fn sanitize(id: &str) -> String {
    id.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '_' })
        .collect()
}

criterion::criterion_group!(benches, bench_validate_precompiled);
criterion::criterion_main!(benches);
