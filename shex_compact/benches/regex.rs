use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use shex_compact::ShExParser;
use shex_compact::{hex, hex_refactor};
use nom_locate::LocatedSpan;

fn regex_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Regex compare");
    for i in 1..1 {
        let txt = LocatedSpan::new("A");
        group.bench_with_input(
            BenchmarkId::new("No_regex", i), 
            &txt, 
            |b, txt| b.iter(|| hex(*txt)));
        group.bench_with_input(BenchmarkId::new("Regex", i), 
            &txt, 
            |b, txt| b.iter(|| hex_refactor(*txt)));
    }
    group.finish();
}

criterion_group!(benches, regex_compare);
criterion_main!(benches);
