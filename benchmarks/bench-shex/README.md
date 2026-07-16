# bench-shex

Covers the ShEx pipeline. Cases are vendored under `bench-shex/corpus/{small,large}.zip`, each containing a `manifest.toml` and its case files. On first use the zips are extracted into `target/bench-shex-corpus/{small,large}/` (respecting `CARGO_TARGET_TMPDIR` / `CARGO_TARGET_DIR`).

- **small**: minimal cases from [shexTest](https://github.com/shexSpec/shexTest) (single-shape schemas, tiny RDF). Times in the µs–ms range.
- **large**: real-world schemas (FHIR R5). Times in the ms–s range.

Each Criterion benchmark splits into two groups (`<stage>_small`, `<stage>_large`) so plots in each group share a comparable scale.

## Stages

| Bench            | What it measures                                  |
|------------------|---------------------------------------------------|
| `parse`          | ShExC text to AST                                 |
| `compile`        | AST to IR                                         |
| `validator_init` | IR to `Validator` (negation-cycle check)          |
| `validate`       | Run validation                                    |
| `end_to_end`     | Full `rudof_lib` flow: load → validate → serialize |

## Running

All benches:

```bash
cargo bench -p bench-shex
```

A specific stage:

```bash
cargo bench -p bench-shex --bench validate
```

A subset by group or case:

```bash
cargo bench -p bench-shex --bench compile -- compile_small
cargo bench -p bench-shex --bench validate -- 'large/fhir-r5'
```

HTML reports are written to `target/criterion/`; open `target/criterion/report/index.html` for the ov
