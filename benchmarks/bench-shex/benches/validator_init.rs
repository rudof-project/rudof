use bench_shex::{
    corpus::{self, Size},
    pipeline,
};
use criterion::{BenchmarkId, Criterion};
use shex_validation::ValidatorConfig;

fn bench_validator_init(c: &mut Criterion) {
    let cases = corpus::load_all().expect("load corpus");
    let cfg = ValidatorConfig::default();

    for size in [Size::Small, Size::Large] {
        let mut group = c.benchmark_group(format!("validator_init_{}", size.tag()));
        for case in cases.iter().filter(|c| c.size == size) {
            let schema_ast = pipeline::parse(&case.schema_src, Some(case.base.clone()), &case.source_iri).unwrap();
            let schema_ir = pipeline::compile(&schema_ast, Some(case.base.clone()), &cfg).unwrap();

            group.bench_with_input(
                BenchmarkId::from_parameter(&case.id),
                &(schema_ir, cfg.clone()),
                |b, (schema_ir, cfg)| {
                    b.iter(|| pipeline::validator_init(schema_ir, cfg).unwrap());
                },
            );
        }
        group.finish();
    }
}

criterion::criterion_group!(benches, bench_validator_init);
criterion::criterion_main!(benches);
