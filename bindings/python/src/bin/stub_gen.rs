//! Regenerates `python/pyrudof/_pyrudof.pyi` from the `gen_stub_*` annotations.
//!
//! Run with: `cargo run --bin stub_gen --features stub-gen`
//!
//! The output path is not hardcoded: it comes from `pyproject.toml`'s `[tool.maturin] python-source` + `module-name`.

fn main() -> pyo3_stub_gen::Result<()> {
    let stub = pyrudof::stub_info()?;
    stub.generate()?;
    Ok(())
}
