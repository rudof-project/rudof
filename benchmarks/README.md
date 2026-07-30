# Benchmarks

Performance benchmarks for [rudof](https://github.com/rudof-project/rudof).

Current crates:

- [`bench-shex/`](./bench-shex/) — Criterion benchmarks for the ShEx pipeline (parse, compile, validator init, validate, end-to-end).

Benchmarks are not published to crates.io and are not executed in CI. Run them locally with:

```
cargo bench -p bench-shex
```

See each sub-crate's `README.md` for details.
