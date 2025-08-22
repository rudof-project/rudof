use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use shex_compact::ShExParser;

fn parse() {
    let str = r#"
    <A> {
       <a> [ 1 ];
       <b> [ 2 ];
       <c> [ 3 ]
    }"#;
    ShExParser::parse(str, None).unwrap();
}

fn shex_parse_benchmark(c: &mut Criterion) {
    // test once to make sure it parses correctly the first time
    parse();
    // real benchmark
    c.bench_function("shex_parse", |b| b.iter(parse));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(20, Output::Flamegraph(None)));
    targets = shex_parse_benchmark
}
criterion_main!(benches);
