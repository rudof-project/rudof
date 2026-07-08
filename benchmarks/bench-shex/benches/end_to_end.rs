use bench_shex::corpus::{self, Size};
use criterion::{BenchmarkId, Criterion};
use rudof_lib::{
    Rudof, RudofConfig,
    formats::{DataFormat, DataReaderMode, InputSpec, ResultShExValidationFormat, ShExFormat, ShapeMapFormat},
};

fn bench_end_to_end(c: &mut Criterion) {
    let cases = corpus::load_all().expect("load corpus");
    let config = RudofConfig::new();

    for size in [Size::Small, Size::Large] {
        let mut group = c.benchmark_group(format!("end_to_end_{}", size.tag()));
        group.sample_size(20);
        for case in cases.iter().filter(|c| c.size == size) {
            group.bench_with_input(BenchmarkId::from_parameter(&case.id), case, |b, case| {
                b.iter(|| {
                    let mut rudof = Rudof::new(config.clone());

                    // Stage 1 + 2 + 3
                    rudof
                        .load_shex_schema(&InputSpec::path(&case.schema_path))
                        .with_base(case.base.as_str())
                        .with_shex_schema_format(&ShExFormat::ShExC)
                        .with_reader_mode(&DataReaderMode::Strict)
                        .execute()
                        .unwrap();

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

                    // Stage 4
                    rudof.validate_shex().execute().unwrap();

                    let mut buf = Vec::new();
                    rudof
                        .serialize_shex_validation_results(&mut buf)
                        .with_result_shex_validation_format(&ResultShExValidationFormat::Compact)
                        .execute()
                        .unwrap();
                });
            });
        }
        group.finish();
    }
}

criterion::criterion_group!(benches, bench_end_to_end);
criterion::criterion_main!(benches);
