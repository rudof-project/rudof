use bench_shex::{
    corpus::{self, Size},
    pipeline,
};
use criterion::{BenchmarkId, Criterion};
use shex_validation::ValidatorConfig;

fn bench_validate(c: &mut Criterion) {
    let cases = corpus::load_all().expect("load corpus");
    let cfg = ValidatorConfig::default();

    for size in [Size::Small, Size::Large] {
        let mut group = c.benchmark_group(format!("validate_{}", size.tag()));
        group.sample_size(20);
        for case in cases.iter().filter(|c| c.size == size) {
            let schema_ast = pipeline::parse(&case.schema_src, Some(case.base.clone()), &case.source_iri).unwrap();
            let schema_ir = pipeline::compile(&schema_ast, Some(case.base.clone()), &cfg).unwrap();
            let validator = pipeline::validator_init(&schema_ir, &cfg).unwrap();
            let rdf = pipeline::load_rdf(&case.data_src, &case.base).unwrap();
            let shapemap = pipeline::parse_shapemap(&case.shapemap_src).unwrap();

            group.bench_with_input(
                BenchmarkId::from_parameter(&case.id),
                &(validator, shapemap, rdf, schema_ir),
                |b, (validator, shapemap, rdf, schema_ir)| {
                    b.iter(|| pipeline::validate(validator, shapemap, rdf, schema_ir).unwrap());
                },
            );
        }
        group.finish();
    }
}

criterion::criterion_group!(benches, bench_validate);
criterion::criterion_main!(benches);
