# Data Generator

A modern, configurable synthetic RDF data generator that creates realistic data conforming to ShEx schemas.

## Features

- **Configuration-driven**: Use TOML/JSON configuration files to control generation parameters
- **Parallel processing**: Generate data using multiple threads for better performance  
- **Parallel writing**: Automatically write to multiple files simultaneously for optimal I/O performance
- **Flexible field generation**: Composable field generators for different data types
- **ShEx schema support**: Generate data that conforms to ShEx shape definitions
- **Multiple output formats**: Support for Turtle, N-Triples, JSON-LD, and more

## Quick Start

You can use this commands to test the application

```bash
# Generate data using simple configuration file
cargo run -p data_generator -- --config data_generator/examples/simple_config.toml --shexfile data_generator/examples/schema.shex

# Generate with inline parameters using example schema
cargo run -p data_generator -- --shexfile data_generator/examples/schema.shex --output quick_data.ttl --entities 100

# Generate with custom seed for reproducible results
cargo run -p data_generator -- --shexfile data_generator/examples/schema.shex --entities 50 --seed 12345

# Use automatic parallel configuration for medium datasets
cargo run -p data_generator -- --config data_generator/examples/auto_parallel.toml --shexfile data_generator/examples/schema.shex

# Use high-performance parallel configuration for large datasets
cargo run -p data_generator -- --config data_generator/examples/parallel_config.toml --shexfile data_generator/examples/schema.shex

# Show help for all options
cargo run -p data_generator -- --help
```


## Normal Start

1. **Create a configuration file** (copy from examples below):
```bash
# Copy the simple ready-to-use config
cp examples/simple_config.toml my_config.toml

# Or copy the comprehensive example
cp examples/config.toml my_config.toml
```

2. **Run the generator**:
```bash
data_generator --config my_config.toml --shexfile schema.shex
```



## Usage

```bash
# Generate data using configuration file
data_generator --config config.toml --shexfile schema.shex

# Generate with inline parameters
data_generator --shexfile schema.shex --output data.ttl --entities 1000

# Generate with custom seed for reproducible results
data_generator --shexfile schema.shex --entities 500 --seed 12345

# Use multiple threads for faster generation
data_generator --shexfile schema.shex --entities 10000 --parallel 8

# Show help for all options
data_generator --help
```

## Configuration

See `examples/config.toml` for configuration options.

### Configuration Examples

#### Basic Configuration (config.toml)

```toml
# Basic data generation settings
[generation]
entity_count = 1000          # Number of entities to generate
seed = 12345                 # Random seed for reproducible results
entity_distribution = "Equal" # How to distribute entities across shapes
cardinality_strategy = "Balanced" # How to handle cardinalities

# Field generation settings
[field_generators.default]
locale = "en"               # Locale for generated text
quality = "Medium"          # Data quality level

# Output configuration
[output]
path = "generated_data.ttl" # Output file path
format = "Turtle"           # Output format
compress = false            # Whether to compress output
write_stats = true          # Write generation statistics

# Parallel processing
[parallel]
worker_threads = 4          # Number of worker threads
batch_size = 100           # Entity batch size
parallel_shapes = true     # Process shapes in parallel
parallel_fields = true     # Generate fields in parallel
```

#### Advanced Configuration with Custom Field Generators

```toml
# Advanced configuration with custom field generators
[generation]
entity_count = 5000
seed = 98765
entity_distribution = "Weighted"
cardinality_strategy = "Random"

# Weighted distribution for different shape types
[generation.distribution_weights]
"http://example.org/Person" = 0.5        # 50% persons
"http://example.org/Organization" = 0.3  # 30% organizations  
"http://example.org/Course" = 0.2        # 20% courses

[field_generators.default]
locale = "en"
quality = "High"

# Custom integer generation with specific ranges
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer"]
generator = "integer"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer".parameters]
min = 1
max = 10000

# Custom decimal generation
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#decimal"]
generator = "decimal"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#decimal".parameters]
min = 0.0
max = 1000.0
precision = 2

# Custom date generation
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#date"]
generator = "date"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#date".parameters]
start_year = 1980
end_year = 2024

# Property-specific generators
[field_generators.properties."http://example.org/name"]
generator = "string"
parameters = {}

[field_generators.properties."http://example.org/email"]
generator = "string"
[field_generators.properties."http://example.org/email".parameters]
templates = [
    "{firstName}.{lastName}@{domain}",
    "{firstName}{lastName}{number}@{domain}",
    "info@{domain}",
    "contact@{domain}"
]

[field_generators.properties."http://example.org/legalName"]
generator = "string"
parameters = {}

# Output with compression
[output]
path = "large_dataset.ttl.gz"
format = "Turtle"
compress = true
write_stats = true

# High-performance parallel settings
[parallel]
worker_threads = 8
batch_size = 250
parallel_shapes = true
parallel_fields = true
```

#### Minimal Configuration

```toml
# Minimal configuration - uses defaults for most settings
[generation]
entity_count = 100

[output]
path = "simple_data.ttl"
```

#### Custom Entity Distribution

```toml
[generation]
entity_count = 2000
entity_distribution = "Custom"

# Exact entity counts per shape
[generation.custom_counts]
"http://example.org/Person" = 1000
"http://example.org/Organization" = 200
"http://example.org/Course" = 800

[output]
path = "custom_distribution.ttl"
```

### Using Configuration Files

```bash
# Use TOML configuration
data_generator --config config.toml --shexfile schema.shex

# Use JSON configuration  
data_generator --config config.json --shexfile schema.shex

# Override config with command line
data_generator --config config.toml --shexfile schema.shex --entities 5000 --output override.ttl
```

### Parallel Writing Examples

The data generator supports parallel writing to multiple files for improved I/O performance. The system can automatically detect the optimal number of files based on your dataset size and system capabilities.

#### Automatic File Count Detection

Set `parallel_file_count = 0` to enable automatic detection:

```bash
# Small dataset (50 entities) → automatically uses 1 file
cargo run --bin data_generator -- -c examples/small_auto.toml -s examples/schema.shex

# Medium dataset (1000 entities) → automatically uses 8 files  
cargo run --bin data_generator -- -c examples/auto_parallel.toml -s examples/schema.shex

# Large dataset (5000 entities) → automatically uses 16 files
cargo run --bin data_generator -- -c examples/large_auto.toml -s examples/schema.shex
```

#### Manual Parallel Writing Configuration

```toml
[output]
path = "dataset.ttl"
format = "Turtle"
parallel_writing = true      # Enable parallel writing
parallel_file_count = 8      # Write to 8 parallel files (manual setting)
```

#### Auto-Detection Configuration

```toml
[output]
path = "dataset.ttl"
format = "Turtle"
parallel_writing = true      # Enable parallel writing
parallel_file_count = 0      # 0 = auto-detect optimal count
```

**Auto-detection algorithm:**
- **Small datasets (≤1,000 triples)**: 1 file (no overhead)
- **Small-medium (1,001-5,000 triples)**: Up to 4 files
- **Medium (5,001-50,000 triples)**: Up to 8 files (2x CPU cores)
- **Large (>50,000 triples)**: Up to 16 files (2x CPU cores, capped)

**Output files:**
- `dataset_part_001.ttl`, `dataset_part_002.ttl`, etc.
- `dataset.manifest.txt` (lists all parallel files)
- `dataset.stats.json` (combined statistics)

**Performance benefits:**
- Small dataset: 28.6ms vs ~35ms sequential (no significant difference)
- Medium dataset: 143.3ms vs 381ms sequential (**62% faster**)
- Large dataset: 601ms vs ~1200ms sequential (**50% faster**)

#### JSON Configuration Example

```json
{
  "generation": {
    "entity_count": 1000,
    "seed": 12345,
    "entity_distribution": "Equal",
    "cardinality_strategy": "Balanced"
  },
  "field_generators": {
    "default": {
      "locale": "en",
      "quality": "Medium"
    },
    "datatypes": {
      "http://www.w3.org/2001/XMLSchema#integer": {
        "generator": "integer",
        "parameters": {
          "min": 1,
          "max": 10000
        }
      },
      "http://www.w3.org/2001/XMLSchema#string": {
        "generator": "string",
        "parameters": {}
      }
    },
    "properties": {
      "http://example.org/name": {
        "generator": "string",
        "parameters": {}
      }
    }
  },
  "output": {
    "path": "generated_data.ttl",
    "format": "Turtle",
    "compress": false,
    "write_stats": true
  },
  "parallel": {
    "worker_threads": 4,
    "batch_size": 100,
    "parallel_shapes": true,
    "parallel_fields": true
  }
}
```

### Configuration Options Reference

#### Generation Settings
- `entity_count`: Total number of entities to generate
- `seed`: Random seed for reproducible results (optional)
- `entity_distribution`: How to distribute entities across shapes
  - `"Equal"`: Equal distribution across all shapes
  - `"Weighted"`: Use weights to control distribution  
  - `"Custom"`: Specify exact counts per shape
- `cardinality_strategy`: How to handle property cardinalities
  - `"Minimum"`: Use minimum cardinality values
  - `"Maximum"`: Use maximum cardinality values
  - `"Random"`: Random values within cardinality range
  - `"Balanced"`: Deterministic but varied distribution

#### Field Generator Settings
- `locale`: Language/locale for generated text (`"en"`, `"es"`, `"fr"`)
- `quality`: Data quality level (`"Low"`, `"Medium"`, `"High"`)
- `datatypes`: Custom generators for specific XSD datatypes
- `properties`: Custom generators for specific properties

#### Output Settings  
- `path`: Output file path
- `format`: Output format (`"Turtle"`, `"NTriples"`, `"JSONLD"`, `"RdfXml"`)
- `compress`: Whether to compress output file
- `write_stats`: Include generation statistics
- `parallel_writing`: Enable writing to multiple parallel files for better I/O performance
- `parallel_file_count`: Number of parallel files (0 = auto-detect optimal count)

#### Parallel Processing
- `worker_threads`: Number of parallel worker threads
- `batch_size`: Entity batch size for processing
- `parallel_shapes`: Process different shapes in parallel
- `parallel_fields`: Generate field values in parallel

### Tips

- **Start simple**: Use the minimal configuration and gradually add customizations
- **Test with small datasets**: Use low entity counts (10-100) while configuring
- **Use fixed seeds**: Set a `seed` value for reproducible results during development
- **Monitor performance**: Increase `worker_threads` for large datasets
- **Enable parallel writing**: Set `parallel_writing = true` and `parallel_file_count = 0` for automatic optimization
- **Validate output**: Check generated data conforms to your ShEx schema expectations

### Output Files

When you run the generator with `write_stats = true`, you'll get:

1. **Data file** (`generated_data.ttl`): The actual RDF data in your chosen format
2. **Statistics file** (`generated_data.stats.json`): Generation statistics including:
   - Total triples generated
   - Entity counts per shape type
   - Generation performance metrics
   - Data distribution information

Example statistics:
```json
{
  "total_triples": 15248,
  "generation_time": "497ms",
  "shape_counts": {
    "http://example.org/Person": 334,
    "http://example.org/Organization": 333,
    "http://example.org/Course": 333
  }
}
```

## Architecture

The generator is built with a modular, functional architecture:

- `config/`: Configuration management and validation
- `field_generators/`: Composable field value generators  
- `shape_processing/`: ShEx schema parsing and analysis
- `parallel_generation/`: Parallel data generation engine
- `output/`: Multiple format output writers
