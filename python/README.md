# pyrudof - Python bindings for Rudof

`pyrudof` provides Python bindings for [rudof](https://rudof-project.github.io/rudof),
a Rust library for Semantic Web operations.

At a high level it supports:

- Loading and serializing **RDF** and **PG** data.
- Loading, checking, serializing and validating **ShEx** schemas.
- Loading, serializing and validating **SHACL** shapes.
- Loading, serializing and validating **PGSchemas**.
- Loading, running and serializing **SPARQL** queries and query results.
- Converting and comparing schemas between supported formats.
- Loading and serializing **DCTAP** and **Service Descriptions**.
- Generating synthetic data from schemas.

## Installation

### Install from PyPI (recommended)

```sh
pip install pyrudof
```

`pyrudof` is built with [PyO3](https://pyo3.rs/) using abi3 wheels (Python 3.7+ compatible).

### Build from source

Building from source requires a working Rust toolchain and Python.

```sh
# Clone the repository
git clone https://github.com/rudof-project/rudof.git
cd rudof/python

# Optional but recommended: virtual environment
python -m venv .venv
# Linux/macOS:
source .venv/bin/activate
# Windows PowerShell:
# .\.venv\Scripts\Activate.ps1

# Build and install editable package
pip install maturin
pip install -e .
```

For a release wheel:

```sh
maturin build --release
pip install --force-reinstall target/wheels/pyrudof-*.whl
```

## Architecture and package structure

### Package layout

| Path | Purpose |
|---|---|
| `src/` | Rust implementation of the Python bindings (`Rudof`, `RudofConfig`, generator APIs, enums, and error mappings). |
| `stubs/pyrudof/` | Hand-maintained `.pyi` type stubs for static type checking and editor support. |
| `examples/` | Runnable example scripts organized by category, plus `examples.toml` manifest metadata. |
| `tests/` | Python `unittest` suite and manifest registry loader for example-based tests. |
| `docs/` | Sphinx docs, including the examples page generator (`generate_examples_doc.py`). |
| `Cargo.toml` / `pyproject.toml` | Rust crate and Python build configuration (`maturin` + PyO3). |

### Test architecture

Tests are dynamically generated from `examples/examples.toml`.

Each generated test:

1. Executes the registered Python example as a subprocess.
2. Asserts a zero exit code.
3. Checks configured `expected_output` substrings (if any).

| Component | Purpose |
|---|---|
| `tests/test_examples.py` | Builds dynamic `unittest.TestCase` classes per category/example. |
| `tests/examples_registry.py` | Loads and validates `examples/examples.toml` entries. |
| `examples/examples.toml` | Single source of truth for metadata and category order. |
| `examples/**/*.py` | Executable example scripts run by tests and referenced by docs. |

## Testing

All test commands are run from `python/tests`.

### Run the full suite

```sh
cd python/tests
python -m unittest discover -vvv
```

### Run a specific category or test

```sh
# All ShEx examples
python -m unittest test_examples.TestShexExamples -v

# All SHACL examples
python -m unittest test_examples.TestShaclExamples -v

# All data generation examples
python -m unittest test_examples.TestGenerateExamples -v

# A single example
python -m unittest test_examples.TestShexExamples.test_shex_validate_inline -v
```

### Run skipped examples

Examples marked with `skip_test = true` in the manifest are skipped by default.

```sh
# Linux/macOS
RUN_SKIPPED_EXAMPLES=1 python -m unittest test_examples -v

# Windows PowerShell
$env:RUN_SKIPPED_EXAMPLES="1"; python -m unittest test_examples -v
```

## Examples and documentation

The examples system is intentionally centralized:

- `examples/examples.toml` stores metadata (`key`, `source_file`, `title`, `description`, `category`, `files`, `expected_output`, `skip_test`).
- `examples/**/*.py` stores the executable code.
- `tests/test_examples.py` and `docs/generate_examples_doc.py` both read from that same manifest.

### Available categories

| Category | Description |
|---|---|
| `rdf` | RDF loading, serialization, node inspection |
| `sparql` | SELECT / CONSTRUCT query workflows |
| `shex` | ShEx loading, validation, and serialization |
| `shacl` | SHACL loading, validation, extraction, serialization |
| `dctap` | DCTAP profile handling |
| `endpoint` | Service description examples |
| `generate` | Synthetic data generation APIs |
| `utility` | Config, resets, version, module/error helpers |

### Add a new example

1. Create a runnable script under `python/examples/` (recommended: place it inside a category subfolder, for example `python/examples/shex/my_example.py`).
2. Register it in `python/examples/examples.toml`.
3. Run tests from `python/tests`.
4. Regenerate and check docs in `python/docs`.

Manifest template:

```toml
[[example]]
key = "my_example"
source_file = "shex/my_example.py"
title = "My Example"
description = "What this example demonstrates"
category = "shex"  # rdf | sparql | shex | shacl | dctap | endpoint | generate | utility
files = { data = "person.ttl" }  # optional referenced files
expected_output = ["Alice"]       # optional substrings (can be empty list)
# skip_test = true                 # optional (network/special runtime)
```

### Build docs locally

```sh
cd python/docs
python generate_examples_doc.py --update
python generate_examples_doc.py --check
python -m sphinx -b html . _build/html
```

Then open `python/docs/_build/html/index.html`.
