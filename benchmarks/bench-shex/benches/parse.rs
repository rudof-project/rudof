use bench_shex::{corpus::{self, Size}, pipeline};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn bench_parse(c: &mut Criterion) {
    let cases = corpus::load_all().expect("load corpus");

    for size in [Size::Small, Size::Large] {
        let mut group = c.benchmark_group(format!("parse_{}", size.tag()));
        for case in cases.iter().filter(|c| c.size == size) {
            group.throughput(Throughput::Bytes(case.schema_src.len() as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(&case.id),
                case,
                |b, case| {
                    b.iter(|| {
                        pipeline::parse(&case.schema_src, Some(case.base.clone()), &case.source_iri).unwrap()
                    });
                }
            );
        }
        group.finish();
    }
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);

