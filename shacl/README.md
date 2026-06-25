# SHACL

[SHACL](https://www.w3.org/TR/shacl/) implementation in Rust.

This project started as a re-implementation in Rust of [SHACL-s](https://github.com/weso/shacl-s).

## Validation modes

The crate exposes two engines selected via `ShaclValidationMode`:

- **`Native`** — designed for in-memory graphs. Constraints are evaluated with Rust
  logic. When the `sparql` Cargo feature is enabled, inline `sh:sparql`
  (SHACL-SPARQL) constraints are routed to the SPARQL execution path so that in-memory
  validation validates them as well. This is the recommended mode whenever the data already
  lives in the process.

- **`Sparql`** — designed for validating against a remote SPARQL endpoint. Every
  constraint is translated to SPARQL queries (`SELECT`/`ASK`). Use this mode when
  the data graph cannot be loaded in-memory; pick `Native` otherwise. This is only available
  with the `shacl` Rust feature.

## Configuration for tests

The tests depend on the [shacl-testsuite](https://w3c.github.io/data-shapes/data-shapes-test-suite/) which is available as a git submodule. In order to run tests it is necessary to run in the root folder of rudof:

```sh
 git submodule update --init --recursive 
```



