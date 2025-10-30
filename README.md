# rudof

[![Latest Version](https://img.shields.io/crates/v/rudof-cli.svg)](https://crates.io/crates/rudof-cli)
[![PyPI](https://img.shields.io/pypi/v/pyrudof)](https://pypi.org/project/pyrudof/)
[![rudof](https://github.com/rudof-project/rudof/actions/workflows/ci.yml/badge.svg)](https://github.com/rudof-project/rudof/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/rudof-project/rudof/status.svg)](https://deps.rs/repo/github/rudof-project/rudof)
[![CodeScene general](https://codescene.io/images/analyzed-by-codescene-badge.svg)](https://codescene.io/projects/72637)

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

- [Documentation](https://rudof-project.github.io/rudof/)
- [Using rudof as Jupyter notebooks](https://rudof-project.github.io/tutorials)
- [Installation](https://github.com/rudof-project/rudof?tab=readme-ov-file#installation)
- [List of issues](https://github.com/rudof-project/rudof/issues)
- [Discussion](https://github.com/rudof-project/rudof/discussions)
- [FAQ](https://github.com/rudof-project/rudof/wiki/FAQ)
- [How to guides](https://github.com/rudof-project/rudof/wiki/How%E2%80%90to-guides)
- [Roadmap](https://github.com/rudof-project/rudof/issues/1)

## Features

`rudof` currently supports the following:

- RDF and RDF 1.2 parsing, conversion and visualization.
- SPARQL querying to RDF data and endpoints
- Parsing SPARQL service description
- ShEx
- SHACL
- DCTAP

Future features we are planning to add:

- rdf-config
- LinkML

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
#### Troubleshooting
If the example doesn’t work as expected, here are a few things you can try:
- **Use the --release flag** to compile in release mode, which can resolve some build issues and improve performance:

```sh
cargo run --release -- validate examples/user.ttl --schema examples/user.shex --shapemap examples/user.sm
```

- **Run the command inside WSL** (Windows Subsystem for Linux). If you're using Windows, compiling the project in WSL can help resolve environment-related issues, as Rust tends to compile more reliably and efficiently in Linux-based systems.

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

### Alternative for Linux

To create a binary for Linux with debug information:

```sh
cargo build --target x86_64-unknown-linux-gnu
```

The binary will be created in: `target/x86_64-unknown-linux-gnu/debug/rudof`

If you want a release binary which is more optimized, you can run:

```sh
cargo build --target x86_64-unknown-linux-gnu
```

In this case, the binary will be `target/x86_64-unknown-linux-gnu/release/rudof`

## Docker

The library is also published as a Docker image
(`angelip2303/rudof:latest`).

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
RDF data shapes implementation in Rust

Usage: rudof [OPTIONS] [COMMAND]

Commands:
  mcp             Export rudof as an MCP server
  shapemap        Show information about ShEx ShapeMaps
  shex            Show information about ShEx schemas
  validate        Validate RDF data using ShEx or SHACL
  shex-validate   Validate RDF using ShEx schemas
  shacl-validate  Validate RDF data using SHACL shapes
  data            Show information about RDF data
  node            Show information about a node in an RDF Graph
  shacl           Show information about SHACL shapes The SHACL schema can be passed through the data options or the optional schema options to provide an interface similar to Shacl-validate
  dctap           Show information and process DCTAP files
  convert         Convert between different Data modeling technologies
  compare         Compare two shapes (which can be in different formats)
  rdf-config      Show information about SPARQL service
  service         Show information about SPARQL service
  query           Run SPARQL queries
  generate        Generate synthetic RDF data from ShEx or SHACL schemas
  help            Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...  
  -h, --help      Print help (see more with '--help')
  -V, --version   Print version
```

You can see the [manual](https://rudof-project.github.io/rudof/)

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
