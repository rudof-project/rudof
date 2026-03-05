# pyrudof — Python bindings for Rudof

[![PyPI](https://img.shields.io/pypi/v/pyrudof)](https://pypi.org/project/pyrudof/)
[![Docs](https://readthedocs.org/projects/pyrudof/badge/?version=latest)](https://pyrudof.readthedocs.io/en/latest/)

`pyrudof` provides Python bindings for [rudof](https://rudof-project.github.io/rudof), a Rust library for working with RDF data validation and related Semantic Web technologies.

**Key features:**

- **ShEx & SHACL validation** — validate RDF graphs against Shape Expressions and SHACL shapes.
- **DCTAP conversion** — read Dublin Core Tabular Application Profiles and convert them to ShEx.
- **SPARQL queries** — run SELECT / CONSTRUCT queries against local data or remote endpoints.
- **Schema comparison** — compare two schemas for structural equivalence.
- **UML visualization** — generate PlantUML diagrams from schemas and data.
- **Synthetic data generation** — create RDF data from ShEx or SHACL schemas via `rudof_generate`.

**Links:**
[PyPI](https://pypi.org/project/pyrudof/) ·
[Documentation](https://pyrudof.readthedocs.io/en/latest/) ·
[Tutorials (Jupyter)](https://rudof-project.github.io/tutorials)

## Building from source

`pyrudof` is built with [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/).

```sh
# Clone the repository
git clone https://github.com/rudof-project/rudof.git
cd rudof/python

# (Optional) create a virtual environment
python3 -m venv .venv
source .venv/bin/activate   # Linux/macOS
# .venv\Scripts\Activate.ps1  # Windows PowerShell

# Install maturin and build
pip install maturin
pip install -e .
```

For a release-optimised wheel:

```sh
maturin build --release
pip install --force-reinstall target/wheels/pyrudof-*.whl
```

## Testing

### Run the full test suite

```sh
cd python/tests
python -m unittest discover -vvv
```

### Run only the example tests

```sh
python -m unittest test_examples -v
```

### Run a specific category or test

```sh
# All ShEx examples
python -m unittest test_examples.TestShexExamples -v

# A single example
python -m unittest test_examples.TestShexExamples.test_shex_validate -v

# SHACL API tests
python -m unittest test_shacl -v

# Data generation tests
python -m unittest test_generate -v
```

### Test architecture

Tests are **auto-generated** from `examples/examples.toml`. When `test_examples.py` is loaded, it reads the manifest and dynamically creates one `unittest.TestCase` class per category, with one test method per example. Each test:

1. Launches the `.py` file as a **subprocess** (exactly as a user would run it).
2. Asserts a zero exit code and non-empty stdout.
3. Checks any `expected_output` substrings declared in the manifest.

Examples that require network access, a PlantUML JAR, or special runtimes are marked with `skip_test = true` in the TOML and are automatically skipped.

| Test file | What it covers |
|---|---|
| `test_examples.py` | All examples from the manifest |
| `test_shacl.py` | SHACL validation API |
| `test_generate.py` | `GeneratorConfig` / `DataGenerator` API |

### CI integration

The [GitHub Actions workflow](../.github/workflows/python.yml) builds wheels on **Linux, Windows, and macOS**, then runs the full test suite including example tests on every push and pull request. The CI also verifies that `examples.rst` is up to date via `generate_examples_doc.py --check`.

## Examples and documentation

### Single source of truth

The example system is designed to **eliminate duplication**. Code lives in one place only:

```
examples/*.py          →  the executable code (authoritative)
examples/examples.toml →  metadata: title, description, category, files, expected_output, skip_test
```

Both the test suite and the documentation generator read from these two sources. There is no inline code in the registry or in the RST file.

### Adding a new example

1. **Create** a `.py` file in `python/examples/` — it must be a runnable script that prints output.

2. **Register** it in `examples/examples.toml`:

   ```toml
   [[example]]
   key = "my_example"
   source_file = "my_example.py"
   title = "My Example"
   description = "What this example demonstrates"
   category = "shex"          # shex | shacl | rdf | dctap | sparql | endpoint | generate | uml | utility
   files = {data = "my_data.ttl"}   # optional: referenced files
   expected_output = ["some expected string"]  # optional: substrings to check
   # skip_test = true         # uncomment if it needs network, PlantUML, etc.
   ```

3. **Run tests** to verify:

   ```sh
   cd python/tests
   python -m unittest test_examples -v
   ```

4. **Regenerate** the documentation:

   ```sh
   python python/docs/generate_examples_doc.py --update
   ```

5. **Verify** the docs are in sync (this is what CI runs):

   ```sh
   python python/docs/generate_examples_doc.py --check
   ```

### Available categories

| Category | Description |
|---|---|
| `shex` | ShEx validation |
| `shacl` | SHACL validation |
| `rdf` | RDF parsing and serialization |
| `dctap` | DCTAP profiles and conversion |
| `sparql` | SPARQL queries (local data) |
| `endpoint` | Remote endpoints (skipped in CI) |
| `generate` | Synthetic data generation |
| `uml` | PlantUML visualization (skipped in CI) |
| `utility` | Module introspection and testing |

### Building the documentation locally

```sh
cd python/docs
python -m sphinx -b html . _build/html
```

Then open `_build/html/index.html` in a browser.

## Using `rudof_generate`

`pyrudof` includes bindings for synthetic RDF data generation.

### Basic usage

```python
import pyrudof

config = pyrudof.GeneratorConfig()
config.set_entity_count(100)
config.set_seed(42)
config.set_output_path("output.ttl")
config.set_output_format(pyrudof.OutputFormat.Turtle)

generator = pyrudof.DataGenerator(config)
generator.run("schema.shex")
```

### Configuration options

```python
config = pyrudof.GeneratorConfig()

# Generation
config.set_entity_count(1000)
config.set_seed(42)
config.set_schema_format(pyrudof.SchemaFormat.ShEx)   # or .Shacl
config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
# Strategies: Minimum, Maximum, Random, Balanced

# Output
config.set_output_path("data.ttl")
config.set_output_format(pyrudof.OutputFormat.Turtle)  # or .NTriples
config.set_compress(False)
config.set_write_stats(True)

# Parallelism
config.set_worker_threads(4)
config.set_batch_size(100)
config.set_parallel_writing(True)
config.set_parallel_file_count(4)
```

### Configuration files

```python
# Load / save TOML
config = pyrudof.GeneratorConfig.from_toml_file("config.toml")
config.to_toml_file("saved.toml")

# Load / save JSON
config = pyrudof.GeneratorConfig.from_json_file("config.json")
```

See the advanced examples in `examples/advanced_generate_example.py` and `examples/config_file_example.py` for more patterns.
