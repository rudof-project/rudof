# rudof

[![Latest Version](https://img.shields.io/crates/v/rudof-cli.svg)](https://crates.io/crates/rudof-cli)
[![PyPI](https://img.shields.io/pypi/v/pyrudof)](https://pypi.org/project/pyrudof/)
[![rudof](https://github.com/rudof-project/rudof/actions/workflows/ci.yml/badge.svg)](https://github.com/rudof-project/rudof/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/rudof-project/rudof/status.svg)](https://deps.rs/repo/github/rudof-project/rudof)

This repo contains an RDF data shapes library implemented in Rust.
The implementation supports
[ShEx](http://shex.io/),
[SHACL](https://www.w3.org/TR/shacl/),
[DCTap](https://www.dublincore.org/specifications/dctap/)
and conversions between different RDF data modeling formalisms.

The code can be used as a Rust library
but it also contains a binary called `rudof`
which can be used as an RDF playground.

We provide binaries for Linux, Windows, Mac and Docker
(see [releases](https://github.com/rudof-project/rudof/releases)),
as well as Python bindings.

- [Installation](https://github.com/rudof-project/rudof?tab=readme-ov-file#installation)
- [List of issues](https://github.com/rudof-project/rudof)
- [Discussion](https://github.com/rudof-project/rudof/discussions)
- [FAQ](https://github.com/rudof-project/rudof/wiki/FAQ)
- [How to guides](https://github.com/rudof-project/rudof/wiki/How%E2%80%90to-guides)
- [Roadmap](https://github.com/rudof-project/rudof/issues/1)

## Installation

### Official releases

You can download a binary from the [latest release](https://github.com/rudof-project/rudof/releases/latest) page.
There you will also find the compiled packages
for the installation on your system using a package manager.

#### Ubuntu

Download the binary from <https://github.com/rudof-project/rudof/releases>
and install the `.deb` package running the following commands after replacing X.X.X by the latest version:

```sh
wget https://github.com/rudof-project/rudof/releases/download/vX.X.X/rudof_vX.X.X_amd64.deb
sudo dpkg -i rudof_vX.X.X_amd64.deb
```

#### Windows

The binary can be downloaded from <https://github.com/rudof-project/rudof/releases>.

#### Mac

The binary is available at:
<https://github.com/rudof-project/rudof/releases>
so you can download the corresponding binary to your machine.

The usual way to run/install a binary in Mac is to download it in a folder,
add that folder to your PATH and activating the binary using:

```sh
chmod +x <binary_file>
```

After that, the processor may complain the first time about security
and you have to agree to use it.
Once you agree, it should work.

<details markdown="block">
<summary>Compiling from source</summary>

### Compiling from source

`rudof` has been implemented in Rust
and is compiled using [cargo](https://doc.rust-lang.org/cargo/).
The command `cargo run` can be used to compile and run locally the code.

For example:

```sh
cargo run -- validate examples/user.ttl --schema examples/user.shex --shapemap examples/user.sm
```

### Compiling from source and installing the binary (Debian)

Install `cargo deb` (only the first time)

```sh
cargo install cargo-deb
```

Create the `.deb` package by:

```sh
cargo deb
```

And run:

```sh
sudo dpkg -i target/debian/rudof_0.0.11-1_amd64.deb
```

## Docker

The library is also published as a Docker image.

</details>

## Usage

### Some examples

The folder `examples` contains several example files with ShEx schemas and RDF data.

### Validate a simple RDF file with a ShEx schema using a ShapeMap

```sh
rudof validate examples/user.ttl --schema examples/user.shex --shapemap examples/user.sm
```

We maintain a Wiki page with some common [Usage scenarios and How-to guides](https://github.com/rudof-project/rudof/wiki/Howto-guides).

### Debugging information

It is possible to change the debug level information with:

```sh
export RUST_LOG=value
```

where `value` can be `debug` to show more verbose information or `info` to show basic information.

## Command line usage

```sh
A tool to process and validate RDF data using shapes, and convert between different RDF data models

Usage: rudof [OPTIONS] [COMMAND]

Commands:
  shapemap        Show information about ShEx ShapeMaps
  shex            Show information about ShEx schemas
  validate        Validate RDF data using ShEx or SHACL
  shex-validate   Validate RDF using ShEx schemas
  shacl-validate  Validate RDF data using SHACL shapes
  data            Show information about RDF data
  node            Show information about a node in an RDF Graph
  shacl           Show information about SHACL shapes
  dctap           Show information and process DCTAP files
  convert         Convert between different Data modeling technologies
  service         Show information about SPARQL service
  query           Run SPARQL queries
  help            Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...


  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Obtaining information about a ShEx schema

```sh
$ rudof shex --help
Show information about ShEx schemas

Usage: rudof shex [OPTIONS] --schema <Schema file name>

Options:
  -s, --schema <Schema file name>

  -f, --format <Schema format>
          [default: shexc] [possible values: internal, simple, shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads]
  -r, --result-format <Result schema format>
          [default: shexj] [possible values: internal, simple, shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads]
  -t, --show elapsed time <SHOW_TIME>
          [possible values: true, false]
      --statistics <SHOW_STATISTICS>
          [possible values: true, false]
  -o, --output-file <Output file name, default = terminal>

      --reader-mode <RDF Reader mode>
          [default: strict] [possible values: lax, strict]
      --force-overwrite

  -c, --config-file <Config file name>
          Config file path, if unset it assumes default config
  -h, --help
          Print help
```

### Obtaining information about RDF data

```sh
$ rudof data --help
Show information about RDF data

Usage: rudof data [OPTIONS] [DATA]...

Arguments:
  [DATA]...

Options:
  -t, --data-format <RDF Data format>
          [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads]
      --reader-mode <RDF Reader mode>
          RDF Reader mode [default: strict] [possible values: lax, strict]
  -o, --output-file <Output file name, default = terminal>

  -r, --result-format <Ouput result format>
          [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads]
  -c, --config-file <Config file name>
          Config file path, if unset it assumes default config
      --force-overwrite

  -h, --help
          Print help
```

### Obtaining information about a node in RDF data

This command can be useful to obtain the neighbourhood of a node.

```sh
$ rudof node --help
Show information about a node in an RDF Graph

Usage: rudof node [OPTIONS] --node <NODE> [DATA]...

Arguments:
  [DATA]...

Options:
  -n, --node <NODE>

  -t, --data-format <RDF Data format>
          [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads]
  -e, --endpoint <Endpoint with RDF data>

      --reader-mode <RDF Reader mode>
          RDF Reader mode [default: strict] [possible values: lax, strict]
  -m, --show-node-mode <Show Node Mode>
          [default: outgoing] [possible values: outgoing, incoming, both]
      --show hyperlinks

  -p, --predicates <PREDICATES>

  -o, --output-file <Output file name, default = terminal>

  -c, --config <Path to config file>

      --force-overwrite

  -h, --help
          Print help
```

For example, the following command shows the neighbourhood of node `wd:Q80` in the Wikidata endpoint.

```sh
rudof node -e wikidata -n wd:Q80
```

### Validating an RDF node against some data

```sh
$ rudof validate --help
Validate RDF data using ShEx or SHACL

Usage: rudof validate [OPTIONS] [DATA]...

Arguments:
  [DATA]...


Options:
  -M, --mode <Validation mode>
          [default: shex]
          [possible values: shex, shacl]

  -s, --schema <Schema file name>


  -f, --schema-format <Schema format>
          [possible values: internal, simple, shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads]

  -m, --shapemap <ShapeMap>


      --shapemap-format <ShapeMap format>
          [default: compact]
          [possible values: compact, internal]

  -n, --node <NODE>


  -l, --shape-label <shape label (default = START)>


  -t, --data-format <RDF Data format>
          [default: turtle]
          [possible values: turtle, ntriples, rdfxml, trig, n3, nquads]

  -e, --endpoint <Endpoint with RDF data>


      --max-steps <max steps to run>
          [default: 100]

  -S, --shacl-mode <SHACL validation mode>
          Execution mode

          [default: native]

          Possible values:
          - native: We use a Rust native engine in an imperative manner (performance)
          - sparql: We use a  SPARQL-based engine, which is declarative

      --reader-mode <RDF Reader mode>
          RDF Reader mode

          [default: strict]
          [possible values: lax, strict]

  -o, --output-file <Output file name, default = terminal>


      --force-overwrite


  -c, --config-file <Config file name>
          Config file path, if unset it assumes default config

  -h, --help
          Print help (see a summary with '-h')
```

Example: Assuming there a ShEx file in `examples/user.shex` and an RDF turtle file in `examples/user.ttl` we can ask to validate node `:a` with shape label `:User` using:

```sh
rudof validate -s examples/user.shex -d examples/user.ttl -n :a -l :User
```

If there is a shapemap in `examples/user.sm`, we can validate using:

```sh
rudof validate -s examples/user.shex -d examples/user.ttl -m examples/user.sm
```

### Validating an RDF node against some SHACL Shape

```sh
rudof shacl-validate --shapes examples/simple_shacl.ttl examples/simple.ttl
```

### Conversion between shapes formalisms

```sh
$ rudof convert --help
Convert between different Data modeling technologies

Usage: rudof convert [OPTIONS] --input-mode <Input mode> --source-file <Source file name> --export-mode <Result mode>

Options:
  -c, --config <Path to config file>

  -m, --input-mode <Input mode>
          [possible values: shacl, shex, dctap]
      --force-overwrite

  -s, --source-file <Source file name>

  -f, --format <Input file format>
          [default: shexc] [possible values: csv, shexc, shexj, turtle, xlsx]
  -r, --result-format <Result format>
          [default: default] [possible values: default, internal, json, shexc, shexj, turtle, plantuml, html, svg, png]
  -o, --output-file <Output file name, default = terminal>

  -t, --target-folder <Target folder>

  -l, --shape-label <shape label (default = START)>

      --reader-mode <RDF Reader mode>
          RDF Reader mode [default: strict] [possible values: lax, strict]
  -x, --export-mode <Result mode>
          [possible values: sparql, shex, uml, html]
  -h, --help
          Print help
```

## Main modules

The repo is divided in the following modules:

- [iri_s](https://github.com/rudof-project/rudof/tree/master/iri_s) defines simple IRIs.
- [srdf](https://github.com/rudof-project/rudof/tree/master/srdf) simple RDF model which will be used for validation.
- [prefixmap](https://github.com/rudof-project/rudof/tree/master/prefixmap) Prefix maps implementation.
- [shapemap](https://github.com/rudof-project/rudof/tree/master/shapemap) ShapeMap implementation.
- [shex_ast](https://github.com/rudof-project/rudof/tree/master/shex_ast) defines the ShEx Abstract syntax
- [shex_compact](https://github.com/rudof-project/rudof/tree/master/shex_compact) contains the code required to handle ShEx compact syntax.
- [shex_validation](https://github.com/rudof-project/rudof/tree/master/shex_validation) contains the code required to validate RDF using ShEx.
- [shex_testsuite](https://github.com/rudof-project/rudof/tree/master/shex_testsuite) contains the code required to run the ShEx testsuite.
- [shacl_ast](https://github.com/rudof-project/rudof/tree/master/shacl_ast) defines the SHACL core Abstract syntax.
- [shacl_validation](https://github.com/rudof-project/rudof/tree/master/shacl_validation) contains the code required to validate RDF using SHACL.
- [dctap](https://github.com/rudof-project/rudof/tree/master/dctap) contains the code required to do handle DCTAP files.
- [shapes_converter](https://github.com/rudof-project/rudof/tree/master/shapes_converter) contains the code required to do conversion between different shapes formalisms.

## Publishing the crates

```sh
cargo workspaces publish 
```

## Worskpaces

The project is using cargo workspaces wihch can be installed with:

```sh
cargo install cargo-workspaces
```

## Unit-testing

In order to test all the sub-projects

```sh
cargo test --all
```

Testing one specific subproject:

```sh
cargo test -p shex_validation
```

## Using the ShEx test-suite

The ShEx testsuite is included in a git submodule. In order to obtain it, it is necessary to do:

```sh
git submodule update --init --recursive
cargo run -p shex_testsuite
```

```sh
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

### Validation conformance tests for ShEx

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld validation 
```

### Schemas tests

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/schemas/manifest.jsonld -f schemas -p failed
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

### Contribution

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.
