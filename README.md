# shex-rs

This repo is a Rust implementation of [ShEx](http://shex.io/). At this moment, the implementation only supports Shapes written in JSON-LD syntax and a simple command line tool `sx`.

This software is work-in-progress yet.

### Installation

<details markdown="block">
<summary>Instructions</summary>

#### Official releases

You can download a binary from the [latest release](https://github.com/weso/shex-rs/releases/latest) page.

#### Compiling from source

`shex-rs` has been implemented in Rust and is compiled using [cargo](https://doc.rust-lang.org/cargo/). The command `cargo run` can be used to compile and run locally the code.

#### Docker

TBD

</details>

### Usage

#### Validate an example

The folder `examples` contains several example files with ShEx schemas and RDF data.

Validate a specific node with a shape:

```
sx validate --data examples/user.ttl --schema examples/user.jsonld --node :a --shape http://example.org/User
```

## Command line usage

```
$ sx --help
Usage: sx [OPTIONS] [COMMAND]
Commands:
  schema
  validate
  data
  node
  help
          Print this message or the help of the given subcommand(s)
```

### Obtaining information about a schema

```
Usage: sx schema [OPTIONS] --schema <Schema file name>

Options:
  -s, --schema <Schema file name>
          
  -f, --schema-format <Schema format>
          [default: shexj] [possible values: internal, shexc, shexj]
  -r, --result-schema-format <Result schema format>
          [default: shexj] [possible values: internal, shexc, shexj]
  -h, --help
          Print help
```

### Obtaining information about RDF data

```
Usage: sx data [OPTIONS] --data <RDF data path>

Options:
  -d, --data <RDF data path>           
  -t, --data-format <RDF Data format>  [default: turtle] [possible values: turtle]
  -h, --help                           Print help
```

### Obtaining information about a node in RDF data

```
Usage: sx node [OPTIONS] --node <NODE> --data <RDF data path>

Options:
  -n, --node <NODE>                    
  -d, --data <RDF data path>           
  -t, --data-format <RDF Data format>  [default: turtle] [possible values: turtle]
  -h, --help                           Print help
```

### Validating an RDF node against some data

```
Usage: sx validate [OPTIONS] --schema <Schema file name> --node <NODE> --shape <shape label> --data <RDF data path>

Options:
  -s, --schema <Schema file name>      
  -f, --schema-format <Schema format>  [default: shexj] [possible values: internal, shexc, shexj]
  -n, --node <NODE>                    
  -l, --shape <shape label>            
  -d, --data <RDF data path>           
  -t, --data-format <RDF Data format>  [default: turtle] [possible values: turtle]
  -m, --max-steps <max steps to run>   [default: 100]
  -h, --help                           Print help
```

## Main modules

The repo is divided in the following modules:

- [iri_s](https://github.com/weso/shex-rs/tree/master/iri_s) defines simple IRIs.
- [srdf](https://github.com/weso/shex-rs/tree/master/srdf) simple RDF model which will be used for validation.
- [srdf_oxgraph](https://github.com/weso/shex-rs/tree/master/srdf_oxgraph) simple RDF model implementation based on [RIO](https://github.com/oxigraph/oxigraph)
- [prefixmap](https://github.com/weso/shex-rs/tree/master/prefixmap) Prefix maps implementation.
- [shex_ast](https://github.com/weso/shex-rs/tree/master/shex_ast) defines the ShEx Abstract syntax
- [shex_pest](https://github.com/weso/shex-rs/tree/master/shex_pest) defines a compact syntax parser using [PEST](https://pest.rs/)
- [shex_antlr](https://github.com/weso/shex-rs/tree/master/shex_antlr) attempt to define ShEx compact grammar parser based on ANTLR. This is no longer maintained.
- [shex_testsuite](https://github.com/weso/shex-rs/tree/master/shex_testsuite) contains the code required to run the ShEx testsuite.

## Publishing the crates

```sh
cargo workspaces publish 
```

## Worskpaces

The project is using cargo workspaces wihch can be installed with:

```
cargo install cargo-workspaces
```

## Unit-testing

In order to test all the sub-projects

```
cargo test --all
```

Testing one specific subproject:

```
cargo test -p shex_validation
```

## Using the ShEx test-suite

The ShEx testsuite is included in a git submodule. In order to obtain it, it is necessary to do:

```sh
git submodule update --init --recursive
cargo run -p shex_testsuite
```

```
Usage: shex_testsuite [OPTIONS]

Options:
  -m, --manifest <Manifest FILE (.jsonld)>
          Name of Manifest file [default: shex_testsuite/shexTest/validation/manifest.jsonld]
  -c, --config <Config file>
          [default: shex_testsuite/config.yml]
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

```
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld validation 
```

### Schemas tests

```
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/schemas/manifest.jsonld -f schemas -p failed
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
