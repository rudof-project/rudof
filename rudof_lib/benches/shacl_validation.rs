#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

use std::fs::File;
use std::io::BufReader;

use criterion::black_box;
use criterion::Criterion;
use criterion_macro::criterion;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::RudofError;
use rudof_lib::ShaclValidationMode;
use rudof_lib::ShapesGraphSource;
use rudof_lib::ValidationReport;
use shacl_ast::ShaclFormat;
use srdf::RDFFormat;

mod perf;

fn custom() -> Criterion {
    Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100))
}

fn shacl_validation(rudof: &mut Rudof) -> Result<ValidationReport, RudofError> {
    rudof.validate_shacl(
        &ShaclValidationMode::Native,
        &ShapesGraphSource::CurrentSchema,
    )
}

#[criterion(custom())]
fn bench(c: &mut Criterion) {
    // -- SETUP --

    let mut rudof = Rudof::new(&RudofConfig::default());

    let reader = match File::open("../examples/book.ttl") {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let _ = rudof.read_shacl(reader, &ShaclFormat::Turtle, None, &srdf::ReaderMode::Lax);

    let reader = match File::open("../examples/book_conformant.ttl") {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let _ = rudof.read_data(reader, &RDFFormat::Turtle, None, &srdf::ReaderMode::Lax);

    // -- BENCH FUNCTION --

    c.bench_function("SHACL Validation", |b| {
        b.iter(|| shacl_validation(black_box(&mut rudof)))
    });
}
