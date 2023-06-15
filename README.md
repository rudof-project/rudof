# shex-rs

This repo is a Rust implementation of [ShEx](http://shex.io/).

The repo is divided in the following modules:

- [iri_s](https://github.com/weso/shex-rs/tree/master/iri_s) defines simple IRIs.
- [srdf](https://github.com/weso/shex-rs/tree/master/srdf) simple RDF model which will be used for validation.
- [srdf_oxgraph](https://github.com/weso/shex-rs/tree/master/srdf_oxgraph) simple RDF model implementation based on [RIO](https://github.com/oxigraph/oxigraph)
- [prefix_map](https://github.com/weso/shex-rs/tree/master/prefix_map) Prefix maps implementation.
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

## How to run the test-suite

The ShEx testsuite is included in a git submodule. In order to obtain it, it is necessary to do:

```sh
git submodule update --init --recursive
cargo test -p shex_testsuite
```

In order to run the validation tests in debug mode:

```
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld validation --debug
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
