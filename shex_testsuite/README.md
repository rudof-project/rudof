# ShEx testsuite

This module contains the code that runs the ShEx testsuite for the ShEx implementation written in Rust.

It has a command line interface that can be used to run some specific tests.

# Usage

```
Usage: shex_testsuite [OPTIONS]

Options:
  -m, --manifest <Manifest FILE (.jsonld)>
          Name of Manifest file [default: shex_testsuite/shexTest/validation/manifest.jsonld]
  -c, --config <Config file>
          [default: shex_testsuite/config.toml]
  -x, --run_mode <MANIFEST_RUN_MODE>
          [default: collect-errors] [possible values: collect-errors, fail-first-error]
  -f, --manifest_mode <MANIFEST_MODE>
          [possible values: schemas, validation, negative-syntax, negative-structure]
  -p, --print_result_mode <PRINT_RESULT_MODE>
          [default: basic] [possible values: basic, failed, passed, not-implemented, all]
  -e, --entry <Entry names>

  -t, --trait <Trait names>

  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples

By default it runs the validation tests and shows some statistics

```
cargo run -p shex_testsuite
Passed: 244, Failed: 340, Skipped: 24, Not implemented: 558
```

Run testsuite to check that the well-formed schemas are read

```
cargo run -p shex_testsuite -- -p failed -m shex_testsuite/shexTest/schemas/manifest.jsonld -f schemas
```
