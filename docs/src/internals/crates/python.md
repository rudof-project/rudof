# `python` (`pyrudof`)

## Overview

The `python` crate (published as `pyrudof`) is the Python bindings layer for the Rudof ecosystem.
It exposes the Rust APIs from [`rudof_lib`](./rudof_lib.md) and `rudof_generate` to Python users through [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/).

### Scope of exposed functionality

`pyrudof` currently exposes Python APIs for:

- Loading and serializing **RDF** and **PG** data.
- Loading, checking, serializing and validating **ShEx** schemas.
- Loading, serializing and validating **SHACL** shapes.
- Loading, serializing and validating **PGSchemas**.
- Loading, running and serializing **SPARQL** queries and query results.
- Converting and comparing schemas between supported formats.
- Loading and serializing **DCTAP** and **Service Descriptions**.
- Generating synthetic data from schemas.

## Architecture and Package Structure

### Package structure map

| Path | Role | Notes |
|---|---|---|
| `src/lib.rs` | PyO3 module declaration and exports | Central export surface via `#[pymodule_export]` |
| `src/pyrudof_lib.rs` | Main `Rudof` class and core enums/errors | Largest binding unit |
| `src/pyrudof_config.rs` | `RudofConfig` wrapper | Default config and `from_path` |
| `src/pyrudof_generate.rs` | Generator API wrappers | Includes Tokio runtime bridge |
| `stubs/pyrudof/__init__.pyi` | Re-export typing surface | Public typing entrypoint |
| `stubs/pyrudof/pyrudof.pyi` | Detailed type stubs | Method signatures and enum values |
| `examples/examples.toml` | Examples manifest | Categories, order, metadata, test policy |
| `examples/**` | Executable Python examples | Organized by domain folders |
| `tests/test_examples.py` | Dynamic test class generation | Subprocess execution of examples |
| `tests/examples_registry.py` | Manifest parsing and validation | Validates source files and metadata |
| `docs/generate_examples_doc.py` | Auto-generates examples docs | Reuses registry logic from tests |

### Test architecture

The test system is manifest-driven and intentionally avoids duplicate test logic.

#### Components

| Component | Responsibility |
|---|---|
| `examples/examples.toml` | Declares categories, order, examples, files, `expected_output`, `skip_test` |
| `tests/examples_registry.py` | Validates manifest entries and builds in-memory catalog |
| `tests/test_examples.py` | Dynamically creates one `unittest.TestCase` class per category |
| `examples/**.py` | Executable scripts run exactly as end users would run them |

#### Execution model

For each manifest entry, the generated test:

1. Checks `skip_test` policy.
2. Runs the script as a subprocess (`python <source_file>`) with `cwd` set to `python/examples`.
3. Asserts exit code is zero.
4. Verifies configured `expected_output` substrings if provided.

### Docs generation architecture

`docs/generate_examples_doc.py` reuses `tests/examples_registry.py` to avoid dual parsing logic.

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
- [`mypy`](https://pypi.org/project/mypy/) — Static type checking for Python stubs.
- [`ruff`](https://pypi.org/project/ruff/) — Linting and formatting.
- [`sphinx`](https://pypi.org/project/Sphinx/) — Documentation generation.
- [`sphinx-lint`](https://pypi.org/project/sphinx-lint/) — Documentation lint checks.
- [`sphinxawesome-theme`](https://pypi.org/project/sphinxawesome-theme/) — Sphinx theme used by the crate docs.
