use bench_shex::{corpus::{self, Size}, pipeline};
use criterion::{BenchmarkId, Criterion};
use shex_validation::ValidatorConfig;

fn bench_compile(c: &mut Criterion) {
    let cases = corpus::load_all().expect("load corpus");
    let cfg = ValidatorConfig::default();

    for size in [Size::Small, Size::Large] {
        let mut group = c.benchmark_group(format!("compile_{}", size.tag()));
        for case in cases.iter().filter(|c| c.size == size) {
            let schema = pipeline::parse(&case.schema_src, Some(case.base.clone()), &case.source_iri).unwrap();
            let base = case.base.clone();

            group.bench_with_input(
                BenchmarkId::from_parameter(&case.id),
                &(schema, base, cfg.clone()),
                |b, (schema, base, cfg)| {
                    b.iter(|| pipeline::compile(schema, Some(base.clone()), cfg).unwrap());
                }
            );
        }
        group.finish();
    }
}

criterion::criterion_group!(benches, bench_compile);
criterion::criterion_main!(benches);
