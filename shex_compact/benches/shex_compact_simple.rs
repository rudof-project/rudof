use criterion::{criterion_group, criterion_main, Criterion};
use shex_compact::ShExParser;

fn parse(str: &str) -> usize {
    let schema = ShExParser::parse(str, None).unwrap();
    schema.shapes().unwrap().len()
}

fn shex_compact_simple(c: &mut Criterion) {
    let str = r#"
    prefix : <http://example.org/>
    <S> {
        :p @<T>;
    }
    
    <T> {
        :q @<S>
    }"#;

    // Parse once to ensure it parses correctly the first time
    let n = parse(str);
    println!("Number of shapes: {:?}", n);
    c.bench_function("shex_compact_simple", |b| b.iter(|| parse(str)));
}

criterion_group!(benches, shex_compact_simple);
criterion_main!(benches);
