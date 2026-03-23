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

### Validation tests

By default it runs the validation tests and shows some statistics

```sh
cargo run -p shex_testsuite
Passed: 1062, Failed: 82, Skipped: 22, Not implemented: 0
```

### Check that the well-formed schemas are read

```sh
cargo run -p shex_testsuite -- -p failed -m shex_testsuite/shexTest/schemas/manifest.jsonld -f schemas
```

### Run a concrete test by name

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld -e nPlus1
```


### Run tests of some specific trait

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld -t sht:TriplePattern
```

### To choose between running the tests with JSON-LD or Compact syntax

ShExC syntax

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld -p failed-simple --shex_syntax shexc
. . .
Passed: 1057, Failed: 87, Skipped: 22, Not implemented: 0
```

JSON-LD syntax

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld -p failed-simple --shex_syntax shexj
. . .
Passed: 1062, Failed: 82, Skipped: 22, Not implemented: 0
```

At this moment the main difference between both is:

```
Tests that fail in ShExC
1dotNS2SingleComment_pass-noOthers
1dotNS2_pass-noOthers
1dotNSdefault_pass-noOthers
1val1emptylanguageStem_fail-empty
1val1emptylanguageStem_fail-integer
1val1emptylanguageStem_fail-literal

Tests that fail in JSON
nPlus1
```
