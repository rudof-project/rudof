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

- `-s, --schema <SCHEMA_FILE>` - Path to your schema file (ShEx, SHACL, or PG-Schema as a `.pgs` file)

### Common Options

- `-n, --entities <COUNT>` - Number of entities to generate (default: 10)
- `-o, --output-file <FILE>` - Where to save the generated data (default: stdout)
- `-r, --result-format <FORMAT>` - Output format: `turtle`, `ntriples`, `rdfxml`, `trig`, `n3`, `nquads`, `jsonld`, `pg` (default: `turtle`)

### Advanced Options

- `-f, --schema-format <FORMAT>` - Force schema format: `auto`, `shex`, or `shacl` (default: `auto`; `auto` reads a `.pgs` file as PG-Schema)
- `--seed <NUMBER>` - Random seed for reproducible results
- `-p, --parallel <THREADS>` - Number of CPU threads to use (default: auto)
- `--force-overwrite` - Overwrite output file if it exists
- `--generator-config <FILE>` - Fine-grained generation settings (TOML or JSON), see below

`rudof generate` also accepts `-c, --config-file <FILE>`, the standard rudof TOML config shared by every subcommand (see [Configuration](../general/configuration.md)). That is a different file from `--generator-config` below and has no effect on generation.

## Using Configuration Files (Advanced)

`--generator-config <FILE>` loads the generation settings, a TOML (or JSON) file that
starts with `version = 2`. Unknown keys are errors. `--entities`, `--result-format`,
`--seed`, `--parallel` and `--output-file` on the command line override the matching
setting in the file. The full format is described in the
[rudof_generate README](https://github.com/rudof-project/rudof/tree/master/rudof_generate#configuration),
with one example per use in `rudof_generate/examples/configs/`.

### Basic Configuration File

```toml
version = 2

[run]
seed = 42                       # same seed => same graph, whatever `threads` is
threads = 4
scale = { entities = 1000 }     # a total; or { per_shape = 1000 }

[defaults]
fill = 0.8                      # probability that an OPTIONAL property is generated
cardinality = "balanced"        # "min" | "max" | "uniform" | "balanced"

[output]
dir = "out"                     # writes out/data.ttl and out/data.shapemap
formats = ["turtle"]            # "turtle" | "ntriples" | "pg"
```

### Per-Shape and Per-Property Settings

```toml
version = 2

[prefixes]
ex = "http://example.org/"

[[shape]]
match = "ex:PersonShape"
count = 300
fill = 0.5

[[property]]
match = { shape = "ex:PersonShape", path = "ex:email" }
fill = 1.0

[values.datatypes."xsd:integer"]
generator = "integer"
parameters = { min = 18, max = 65 }
```

### Controlled Errors and Target Coherence

```toml
version = 2

[violations]                    # break 10% of the entities; out/labels.jsonl lists what must fail
rate = 0.1

[targets]                       # search the fill until the output has this coherence
coherence = 0.8

[output]
dir = "out"
```

`[violations]` breaks a share (`rate`) of the generated entities, one constraint each. The
`operators` key chooses how; without it, every operator that applies to the schema is
used:

| Operator | What it does |
|---|---|
| `drop-required` | removes a required property |
| `exceed-max` | adds a value beyond the maximum cardinality |
| `wrong-datatype`, `ill-typed-literal` | a value of another datatype, or an invalid lexical form of the declared one |
| `out-of-range`, `pattern-break`, `not-in-value-set` | a value outside a range, a pattern or a value set |
| `dangling-reference` | a reference to a node that does not conform to the referenced shape |
| `closed-extra` | a property that a closed shape does not allow |
| `wrong-language`, `duplicate-language`, `lang-string-for-string` | language tags that break `sh:languageIn`, `sh:uniqueLang` or a plain string |

`labels.jsonl` then says what a correct validator of the schema's language must report,
one line per node:

- `direct`: the node was broken, with the `operator` and the `property`;
- `induced`: the node fails through a reference to a broken node, with its `cause`;
- `trap`: the node was changed, but it still conforms. Examples are a language tag in
  another case, or `030` for `30` under a range. A validator that reports a trap is wrong.

### PG-Schema and Property Graphs

A `.pgs` schema generates a property graph:

```sh
rudof generate -s persons.pgs --generator-config pg.toml
```

```toml
version = 2

[run]
scale = { per_shape = 500 }

[violations]
rate = 0.2

[violations.rdf]                # the report for the RDF copy of the graph
closed = true                   # keep PG-Schema's closed records (false: open shapes)
keys_required = true            # keep its required keys (false: optional keys)

[output]
dir = "out"
formats = ["pg", "turtle"]      # out/data.pg, out/data.typemap, out/data.ttl, out/data.shapemap
```

`data.pg` is the graph in rudof's property graph format. `data.typemap` gives the type of
every node and edge, for `rudof pgschema-validate`. With `turtle`, the graph is also written
as RDF, in an encoding over which a ShEx or SHACL schema can say the same as the
PG-Schema. With errors, `labels.jsonl` reports for PG-Schema, counting nodes and edges.
`labels-rdf.jsonl` reports for ShEx and SHACL on the RDF copy, under the defaults that
`[violations.rdf]` chooses.

### Validating the Output

`--validate` validates the generated data against the schema it came from, and exits with
an error on violations:

- ShEx: each node against the shape that generated it, through the ShapeMap;
- SHACL: through the targets of the shapes.

```sh
rudof generate -s schema.shex -n 1000 -o out/data.ttl --validate
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
version = 2

[run]
seed = 12345
threads = 8
scale = { entities = 5000 }

[output]
dir = "large_dataset"
formats = ["ntriples"]
stats = true
```

**Generate:**
```sh
rudof generate --schema courses.ttl --generator-config large_config.toml
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
✅ Closed shapes (CLOSED) and EXTRA
✅ Language tags (language sets and stems)

### SHACL Features

✅ Node shapes (sh:NodeShape)
✅ Property shapes (sh:property)
✅ Cardinality (sh:minCount, sh:maxCount)
✅ Datatypes (sh:datatype)
✅ Node kinds (sh:nodeKind)
✅ Value constraints (sh:in, sh:hasValue, sh:pattern, ranges)
✅ References between shapes (sh:node, sh:class, sh:qualifiedValueShape)
✅ Closed shapes (sh:closed)
✅ Language tags (sh:languageIn, sh:uniqueLang)

### PG-Schema Features

✅ Node types with one or more labels
✅ Keys: required, `OPTIONAL` and lists (`T *`, `T {m,n}`)
✅ `CHECK` ranges and `REGEX` patterns
✅ Edge types between typed endpoints (`(:@T)`)
❌ Edge types with properties (left out, with a warning)

## See Also

- [shex](./shex.md) - Process and display ShEx schemas
- [shacl](./shacl.md) - Process and display SHACL shapes
- [validate](./validate.md) - Validate generated data against schemas
- [data](./data.md) - Process and display RDF data

## Need More Help?

- Check the examples in the `examples/` directory
- Look at `rudof_generate/examples/` for more configuration examples
- Read the [FAQ](../references/faq.md) for common questions
- Join the [discussion](https://github.com/rudof-project/rudof/discussions) for community help
