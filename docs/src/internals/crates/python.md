# `bindings/python` (`pyrudof`)

## Overview

The `bindings/python` crate (published as `pyrudof`) is the Python bindings layer for the Rudof ecosystem.
It exposes the Rust APIs from [`rudof_lib`](./rudof_lib.md) and `rudof_generate` to Python users.

### Scope of exposed functionality

`pyrudof` currently exposes Python APIs for:

- Loading and serializing **RDF** and **PG** data.
- Loading, checking, serializing and validating **ShEx** schemas.
- Loading, serializing and validating **SHACL** shapes.
- Loading, serializing and validating **PGSchemas**.
- Loading, running and serializing **SPARQL** queries.
- Converting and comparing schemas between supported formats.
- Loading and serializing **DCTAP** and **Service Descriptions**.
- Generating synthetic data from schemas.

## Architecture and Package Structure

### Package structure map

The crate ships a mixed Rust/Python package: `[tool.maturin] python-source = "python"`
makes `python/pyrudof/` the importable package, and the compiled extension is installed
inside it as the private submodule `pyrudof._pyrudof`.

| Path | Role | Notes |
|---|---|---|
| `src/lib.rs` | PyO3 module declaration and exports | Central export surface; the `#[pymodule]` is `_pyrudof` |
| `src/macros.rs` | Shared binding macros | Enum bridging, error wrapping, GIL-release helpers |
| `src/error.rs` | Python exception types | One exception class per failure domain |
| `src/input.rs`, `src/output.rs` | Input/output plumbing | `PathLike`/`str`/URL inputs, captured-string outputs |
| `src/config.rs` | `RudofConfig` wrapper | Default config and `from_path` |
| `src/api/**` | The `Rudof` session, split by domain | `api/session.rs` holds the class; `api/mod.rs` is declarations only |
| `src/formats/**` | Format enums and result types | One file per format family |
| `src/generate/**` | Generator API wrappers | Includes the Tokio runtime bridge |
| `src/bin/stub_gen.rs` | Regenerates the type stub | `cargo run --bin stub_gen --features stub-gen` |
| `python/pyrudof/__init__.py` | Public Python surface | Re-exports every name from `pyrudof._pyrudof` |
| `python/pyrudof/py.typed` | PEP 561 marker | Makes the annotations visible to type checkers |
| `python/pyrudof/_pyrudof/__init__.pyi` | Generated type stub | Checked in; CI fails if regenerating it produces a diff |
| `examples/examples.toml` | Examples manifest | Categories, order, metadata, test policy |
| `examples/_registry.py` | Manifest parsing and validation | Validates source files and metadata |
| `examples/**` | Executable Python examples | Organized by domain folders |
| `tests/test_examples.py` | Parametrized example tests | One test per manifest entry |
| `docs/generate_examples_doc.py` | Auto-generates examples docs | Reuses `examples._registry` |

### Two layers

`pyrudof` is a pure-Python package (`python/pyrudof/__init__.py`, hand-written) and
`pyrudof._pyrudof` is the PyO3 extension maturin builds from `src/`.

Exposing a new class therefore touches three places, and forgetting one fails differently
in each case:

- `src/lib.rs` — register it on the module, or it does not exist at runtime.
- `python/pyrudof/__init__.py` — add it to the import list *and* to `__all__`, or it is
  reachable only as `pyrudof._pyrudof.X`.
- The stub — annotate the type with `#[gen_stub_pyclass]` / `#[gen_stub_pymethods]` and
  rerun `cargo run --bin stub_gen --features stub-gen`.

### Test architecture

The test system is manifest-driven and intentionally avoids duplicate test logic.

#### Components

| Component | Responsibility |
|---|---|
| `examples/examples.toml` | Declares categories, order, examples, files, expected_output, skip_test |
| `examples/_registry.py` | Validates manifest entries and builds in-memory catalog |
| `tests/test_examples.py` | One parametrized `pytest` case per manifest entry |
| `examples/**.py` | Executable scripts run exactly as end users would run them |

#### Execution model

For each manifest entry not marked `skip_test`, the test:

1. Changes the working directory to `bindings/python/examples`.
2. Runs the script in-process via `runpy.run_path(..., run_name="__main__")`.
3. Asserts every configured expected_output substring appears in stdout.

A `skip_test` entry is still compiled and checked for a `main()`, and a separate test
asserts that every `.py` file under `examples/` is registered in the manifest — an
unregistered example is an untested one.

`pytest` is configured in `pyproject.toml` (`testpaths`, `pythonpath`), so it runs from `bindings/python/`.

### Docs generation architecture

`docs/generate_examples_doc.py` reuses `examples/_registry.py` to avoid dual parsing logic.

This creates a single-source pipeline:

- Manifest + example files -> tests
- Manifest + example files -> `docs/examples.rst`

Benefits:

- Lower drift risk between docs and runnable examples.
- Uniform category ordering and metadata usage.

## Dependencies

This crate primarily depends on:

- [`pyo3`](https://crates.io/crates/pyo3) — Rust/Python interop layer used to expose module classes, enums, and exceptions.
- [`pythonize`](https://crates.io/crates/pythonize) — Conversion helpers between Rust values and Python-friendly representations.
- [`rudof_lib`](https://crates.io/crates/rudof_lib) — Core Rudof semantic web facade wrapped by the `Rudof` Python class.
- [`rudof_generate`](https://crates.io/crates/rudof_generate) — Data generation engine wrapped by `GeneratorConfig` and `DataGenerator`.
- [`tokio`](https://crates.io/crates/tokio) — Async runtime used by generation wrappers to run async Rust operations from synchronous Python APIs.

For packaging and developer workflows, it also uses:

- [`maturin`](https://crates.io/crates/maturin) — Build backend for Python wheels and editable installs.
- [`pyo3-stub-gen`](https://crates.io/crates/pyo3-stub-gen) — Generates the checked-in `.pyi` stub from the Rust annotations.
- [`mypy`](https://pypi.org/project/mypy/) — Static type checking, run in `--strict` mode over the package, examples and tests.
- [`pytest`](https://pypi.org/project/pytest/) — Test runner for the example suite.
- [`sphinx`](https://pypi.org/project/Sphinx/) — Documentation generation.
- [`sphinx-lint`](https://pypi.org/project/sphinx-lint/) — Documentation lint checks.
- [`sphinxawesome-theme`](https://pypi.org/project/sphinxawesome-theme/) — Sphinx theme used by the crate docs.
