# generate

The `generate` command creates synthetic RDF data from ShEx or SHACL schemas. It's designed to be easy to use for beginners while offering powerful configuration options for advanced users.

## Quick Start (For Beginners)

The simplest way to generate data is with just a schema file:

```sh
# Generate 10 entities from a ShEx schema (format is auto-detected)
rudof generate -s examples/user.shex -o data.ttl

# Generate 20 entities from a SHACL schema
rudof generate -s examples/simple_shacl.ttl -n 20 -o data.ttl
```

That's it! The command will automatically detect your schema format and generate valid RDF data.

## Synopsis

```sh
rudof generate [OPTIONS] --schema <SCHEMA_FILE>
```

## Step-by-Step Tutorial

### Step 1: Simple Generation from ShEx

Let's start with a simple ShEx schema that defines a Person:

**Example schema (`person.shex`):**
```shex
prefix : <http://example.org/> 
prefix schema: <http://schema.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:User {
  schema:name   xsd:string             ;
  schema:knows  @:User               * ;
  :status       [ :Active :Waiting ] ? ;
}
```

**Generate 5 users:**
```sh
rudof generate -s person.shex -n 5 -o users.ttl
```

**Output (`users.ttl`):**
```turtle
<http://example.org/User-1> a <http://example.org/User> ;
    <http://schema.org/name> "Alice Johnson" .

<http://example.org/User-2> a <http://example.org/User> ;
    <http://schema.org/name> "Bob Smith" ;
    <http://schema.org/knows> <http://example.org/User-1> .

<http://example.org/User-3> a <http://example.org/User> ;
    <http://schema.org/name> "Charlie Brown" ;
    <http://example.org/status> <http://example.org/Active> .
```

### Step 2: Simple Generation from SHACL

Let's use a SHACL schema for Persons and Courses:

**Example schema (`education.ttl`):**
```turtle
@prefix :       <http://example.org/> .
@prefix sh:     <http://www.w3.org/ns/shacl#> .
@prefix xsd:    <http://www.w3.org/2001/XMLSchema#> .
        
:Person a sh:NodeShape ;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] ;
  sh:property [                   
   sh:path     :birthDate ; 
   sh:maxCount 1; 
   sh:datatype xsd:date ;
  ] ;
  sh:property [                   
   sh:path     :enrolledIn ; 
   sh:node     :Course ;
  ] .

:Course a sh:NodeShape;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] .
```

**Generate 8 entities:**
```sh
rudof generate -s education.ttl -n 8 -o education_data.ttl
```

**Output will include Persons with names, birthdates, and enrolled Courses:**
```turtle
<http://example.org/Person-1> <http://example.org/name> "Diana Martinez" ;
    <http://example.org/birthDate> "1995-03-15"^^<http://www.w3.org/2001/XMLSchema#date> ;
    <http://example.org/enrolledIn> <http://example.org/Course-1> ;
    a <http://example.org/Person> .

<http://example.org/Course-1> <http://example.org/name> "Computer Science 101" ;
    a <http://example.org/Course> .
```

### Step 3: Reproducible Generation (Same Data Every Time)

Use the `--seed` option to generate the same data every time (useful for testing):

```sh
rudof generate -s person.shex -n 10 --seed 42 -o reproducible.ttl
```

Run this command multiple times, and you'll always get the same output!

### Step 4: Different Output Formats

Generate data in different RDF formats:

```sh
# N-Triples format (one triple per line)
rudof generate -s person.shex -n 5 -r ntriples -o data.nt

# RDF/XML format
rudof generate -s person.shex -n 5 -r rdfxml -o data.rdf
```

### Step 5: Large Datasets with Parallel Processing

For generating large datasets faster, use parallel processing:

```sh
# Generate 1000 entities using 4 CPU cores
rudof generate -s person.shex -n 1000 -p 4 -o large_dataset.ttl

# Generate 10000 entities using 8 CPU cores
rudof generate -s education.ttl -n 10000 -p 8 -o huge_dataset.ttl
```

## Command-Line Options

### Required Options

- `-s, --schema <SCHEMA_FILE>` - Path to your schema file (ShEx or SHACL)

### Common Options

- `-n, --entities <COUNT>` - Number of entities to generate (default: 10)
- `-o, --output-file <FILE>` - Where to save the generated data (default: stdout)
- `-r, --result-format <FORMAT>` - Output format: `turtle`, `ntriples`, `rdfxml`, `trig`, `n3`, `nquads` (default: `turtle`)

### Advanced Options

- `-f, --schema-format <FORMAT>` - Force schema format: `auto`, `shex`, or `shacl` (default: `auto`)
- `--seed <NUMBER>` - Random seed for reproducible results
- `-p, --parallel <THREADS>` - Number of CPU threads to use (default: auto)
- `-c, --config <FILE>` - Use a configuration file for advanced settings
- `--force-overwrite` - Overwrite output file if it exists

## Using Configuration Files (Advanced)

For more control, you can use a configuration file:

### Basic Configuration File

Create a file called `generator_config.toml`:

```toml
# How many entities to generate
[generation]
entity_count = 100
seed = 12345                    # For reproducible results
entity_distribution = "Equal"   # Equal distribution across shapes
cardinality_strategy = "Balanced" # Balanced cardinality handling

# Default settings for generated data
[field_generators.default]
locale = "en"      # Language for generated text
quality = "Medium" # Data quality: Low, Medium, or High

# Where to save the output
[output]
path = "generated_data.ttl"
format = "Turtle"  # Options: Turtle, NTriples
compress = false
write_stats = true # Generate a stats file

# Performance settings
[parallel]
worker_threads = 4     # Number of CPU threads
batch_size = 100      # Entities per batch
parallel_shapes = true
parallel_fields = true
```

**Use it:**
```sh
rudof generate -s person.shex -c generator_config.toml
```

### Minimal Configuration

For a minimal configuration, you only need:

```toml
[generation]
entity_count = 50

[output]
path = "output.ttl"
```

### Advanced Configuration with Custom Generators

For power users who want fine-grained control:

```toml
[generation]
entity_count = 1000
seed = 98765

# Custom ranges for integer fields
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer"]
generator = "integer"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer".parameters]
min = 1
max = 10000

# Custom date ranges
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#date"]
generator = "date"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#date".parameters]
start_year = 1980
end_year = 2024

# Custom email templates
[field_generators.properties."http://example.org/email"]
generator = "string"
[field_generators.properties."http://example.org/email".parameters]
templates = [
    "{firstName}.{lastName}@example.com",
    "{firstName}{number}@company.org"
]

[output]
path = "custom_data.ttl"
format = "Turtle"
write_stats = true

[parallel]
worker_threads = 8
batch_size = 250
```

## Complete Examples

### Example 1: Simple User Directory

**Schema (`users.shex`):**
```shex
prefix : <http://example.org/> 
prefix schema: <http://schema.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:User {
  schema:name   xsd:string  ;
  schema:email  xsd:string  ;
  :age         xsd:integer ? ;
}
```

**Generate:**
```sh
rudof generate -s users.shex -n 20 -o users.ttl
```

### Example 2: Course Enrollment System

**Schema (`courses.ttl`):**
```turtle
@prefix : <http://example.org/> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        
:Student a sh:NodeShape ;
   sh:property [
    sh:path :studentId ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
  ] ;
  sh:property [
    sh:path :name ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
  ] ;
  sh:property [
    sh:path :enrolledIn ;
    sh:node :Course ;
  ] .

:Course a sh:NodeShape ;
   sh:property [
    sh:path :courseName ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
  ] .
```

**Generate:**
```sh
rudof generate -s courses.ttl -n 50 -o enrollment_data.ttl
```

### Example 3: Large Dataset with Configuration

**Config (`large_config.toml`):**
```toml
[generation]
entity_count = 5000
seed = 12345

[output]
path = "large_dataset.ttl"
format = "NTriples"
write_stats = true

[parallel]
worker_threads = 8
batch_size = 250
```

**Generate:**
```sh
rudof generate -s courses.ttl -c large_config.toml
```

## Understanding the Output

When you generate data with `write_stats = true` (in config) or by default, you get:

1. **Data file** (e.g., `data.ttl`) - Your generated RDF data
2. **Stats file** (e.g., `data.stats.json`) - Generation statistics

**Example stats file:**
```json
{
  "total_triples": 245,
  "generation_time": "89ms",
  "shape_counts": {
    "http://example.org/Person": 10,
    "http://example.org/Course": 8
  }
}
```

## Tips for Beginners

1. **Start small**: Begin with `-n 5` or `-n 10` to see what gets generated
2. **Use examples**: The `examples/` directory has ready-to-use schemas
3. **Check the output**: Open the generated `.ttl` file in a text editor to see the data
4. **Use fixed seeds**: Add `--seed 42` to get the same data every time (helpful for learning)
5. **Try different formats**: Experiment with `-r ntriples` or `-r turtle` to see different RDF serializations

## Common Use Cases

### Testing Your Application

Generate test data that matches your schema:
```sh
rudof generate -s my_schema.shex -n 100 --seed 42 -o test_data.ttl
```

### Creating Documentation Examples

Generate small, consistent examples:
```sh
rudof generate -s schema.shex -n 3 --seed 999 -o example.ttl
```

### Performance Testing

Generate large datasets quickly:
```sh
rudof generate -s schema.shex -n 100000 -p 8 -o large_test.ttl
```

### Multiple Test Datasets

Generate different datasets with different seeds:
```sh
rudof generate -s schema.shex -n 50 --seed 1 -o dataset1.ttl
rudof generate -s schema.shex -n 50 --seed 2 -o dataset2.ttl
rudof generate -s schema.shex -n 50 --seed 3 -o dataset3.ttl
```

## Troubleshooting

**Problem**: "Schema from stdin is not supported"
- **Solution**: Always provide a file path with `-s filename.shex`

**Problem**: "File already exists"
- **Solution**: Add `--force-overwrite` flag or delete the existing file

**Problem**: Output is empty or very small
- **Solution**: Increase entity count with `-n 100` or check your schema

**Problem**: Generation is slow
- **Solution**: Add `-p 4` or `-p 8` to use parallel processing

## What the Generator Supports

### ShEx Features

✅ Shape expressions and constraints  
✅ Cardinality (*, +, ?, {n,m})  
✅ Datatypes (xsd:string, xsd:integer, xsd:date, etc.)  
✅ Node kinds (IRI, Literal, BNode)  
✅ Value constraints and enumerations  
✅ References between shapes (@:ShapeName)

### SHACL Features

✅ Node shapes (sh:NodeShape)  
✅ Property shapes (sh:property)  
✅ Cardinality (sh:minCount, sh:maxCount)  
✅ Datatypes (sh:datatype)  
✅ Node kinds (sh:nodeKind)  
✅ Value constraints (sh:in, sh:pattern)  
✅ References between shapes (sh:node)

## See Also

- [shex](./shex.md) - Process and display ShEx schemas
- [shacl](./shacl.md) - Process and display SHACL shapes
- [validate](./validate.md) - Validate generated data against schemas
- [data](./data.md) - Process and display RDF data

## Need More Help?

- Check the examples in the `examples/` directory
- Look at `data_generator/examples/` for more configuration examples
- Read the [FAQ](../references/faq.md) for common questions
- Join the [discussion](https://github.com/rudof-project/rudof/discussions) for community help