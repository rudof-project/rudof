# Rudof Python bindings

The Python bindings for [rudof](https://rudof-project.github.io/) are called `pyrudof`. They are available at [pypi](https://pypi.org/project/pyrudof/).

For more information, you can access the [readthedocs documentation](https://pyrudof.readthedocs.io/en/latest/). We keep several tutorials about rudof as Jupyter notebooks in: [https://rudof-project.github.io/tutorials].

After compiling and installing this module, a Python library  called `pyrudof` should be available.  

## Build the development version

This module is based on [pyo3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/).

To build and install the development version of `pyrudof` you need to clone this git repository, go to the `python` directory (the one this README is in) and run:

```
pip install maturin
```

followed by:

```sh
pip install .
```

If you are using `.env`, you can do the following:

```sh
python3 -m venv .venv
```

followed by: 

```sh
source .venv/bin/activate
```

or

```sh
source .venv/bin/activate.fish
```

and once you do that, you can locally install que package as:

```sh
pip install -e .
```

## Running the tests

Go to the tests folder: 

```sh
cd tests
```

and run: 

```sh
python3 -m unittest discover -vvv
```

## Using rudof_generate

The `pyrudof` package includes bindings for `rudof_generate`, which allows you to generate synthetic RDF data from ShEx or SHACL schemas.

### Basic Example

```python
import pyrudof

# Create configuration
config = pyrudof.GeneratorConfig()
config.set_entity_count(100)
config.set_output_path("output.ttl")
config.set_output_format(pyrudof.OutputFormat.Turtle)

# Create generator
generator = pyrudof.DataGenerator(config)

# Load schema and generate data
generator.run("schema.shex")
```

### Configuration Options

The `GeneratorConfig` class provides many configuration options:

```python
config = pyrudof.GeneratorConfig()

# Generation parameters
config.set_entity_count(1000)           # Number of entities to generate
config.set_seed(42)                     # Random seed for reproducibility

# Schema format
config.set_schema_format(pyrudof.SchemaFormat.ShEx)  # or SchemaFormat.SHACL

# Output configuration
config.set_output_path("data.ttl")
config.set_output_format(pyrudof.OutputFormat.Turtle)  # or OutputFormat.NTriples
config.set_compress(False)              # Whether to compress output
config.set_write_stats(True)            # Write generation statistics

# Cardinality strategy
config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
# Options: Minimum, Maximum, Random, Balanced

# Parallel processing
config.set_worker_threads(4)            # Number of worker threads
config.set_batch_size(100)              # Batch size for processing
config.set_parallel_writing(True)       # Enable parallel file writing
config.set_parallel_file_count(4)       # Number of output files (when parallel)
```

### Loading Schemas

You can load schemas in different ways:

```python
# Load ShEx schema
generator.load_shex_schema("schema.shex")

# Load SHACL schema
generator.load_shacl_schema("shapes.ttl")

# Auto-detect schema format
generator.load_schema_auto("schema_file")

# Then generate data
generator.generate()
```

### Complete Workflow

The `run()` method provides a convenient way to load a schema and generate data in one step:

```python
# Auto-detect format
generator.run("schema.shex")

# Specify format explicitly
generator.run_with_format("shapes.ttl", pyrudof.SchemaFormat.SHACL)
```

### Configuration Files

You can also load configuration from TOML or JSON files:

```python
# Load from TOML
config = pyrudof.GeneratorConfig.from_toml_file("config.toml")

# Load from JSON
config = pyrudof.GeneratorConfig.from_json_file("config.json")

# Save configuration
config.to_toml_file("saved_config.toml")
```

### Available Enums

- **SchemaFormat**: `ShEx`, `SHACL`
- **OutputFormat**: `Turtle`, `NTriples`
- **CardinalityStrategy**: `Minimum`, `Maximum`, `Random`, `Balanced`

For more examples, see the `examples/generate_example.py` file.