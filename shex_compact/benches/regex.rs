use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use nom_locate::LocatedSpan;
use shex_compact::{hex, hex_refactor};

fn regex_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Regex compare");
    for i in 1..2 {
        let txt = LocatedSpan::new("A");
        group.bench_with_input(BenchmarkId::new("No_regex", i), &txt, |b, txt| {
            b.iter(|| hex(*txt))
        });
        group.bench_with_input(BenchmarkId::new("Regex", i), &txt, |b, txt| {
            b.iter(|| hex_refactor(*txt))
        });
    }
    group.finish();
}

criterion_group!(benches, regex_compare);
criterion_main!(benches);
