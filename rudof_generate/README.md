# Data Generator

A configurable synthetic data generator that creates data conforming to ShEx, SHACL or
PG-Schema schemas, as RDF or as a property graph.

## Features

- **Three schema languages**: ShEx, SHACL and PG-Schema, read into one intermediate representation; equivalent schemas generate the same graph from the same seed
- **Two data models**: RDF (Turtle, N-Triples) and property graphs (`.pg`, with a type map), and the RDF copy of a property graph
- **Valid by default**: minimum cardinalities are always respected; every run can write the ShapeMap (or type map) of the generated nodes, the exact targets for validating them
- **Controlled errors**: the `[violations]` section breaks a share of the entities on purpose and writes what a correct validator of each language must report
- **Structuredness control**: optional properties by scope (`fill`), or a target coherence that the generator searches for
- **Reproducible and parallel**: the same seed gives the same graph whatever the number of threads

## Quick Start

You can use these commands to test the application. Execute them from the root of the `rudof` repository, via the `rudof generate` subcommand.

### SHACL Examples

```bash
# Generate data from SHACL schema (auto-detected by .ttl extension)
rudof generate --schema examples/simple_shacl.ttl --output-file shacl_data.ttl --entities 100

# Generate with specific seed for reproducible SHACL data
rudof generate --schema examples/simple_shacl.ttl --output-file shacl_reproducible.ttl --entities 50 --seed 12345

# Generate from complex SHACL schema with more entities
rudof generate --schema examples/shacl/node_shacl.ttl --output-file complex_shacl_data.ttl --entities 200

# Use parallel processing for large SHACL datasets
rudof generate --schema examples/simple_shacl.ttl --output-file large_shacl_data.ttl --entities 5000 --parallel 8
```

### ShEx Examples

```bash
# Generate data from ShEx schema (auto-detected by .shex extension)
rudof generate --schema examples/simple.shex --output-file shex_data.ttl --entities 100

# Generate with inline parameters using example ShEx schema
rudof generate --schema rudof_generate/examples/schemas/basic/person.shex --output-file quick_shex_data.ttl --entities 100

# Generate with custom seed for reproducible ShEx results
rudof generate --schema rudof_generate/examples/schemas/basic/person.shex --entities 50 --seed 12345
```

### Configuration-Driven Examples

```bash
# Every output goes to the folder named in the configuration ([output] dir)
rudof generate --generator-config rudof_generate/examples/configs/basic.toml --schema rudof_generate/examples/schemas/basic/person.shex

# A property graph, its RDF copy and controlled errors, from a PG-Schema
rudof generate --generator-config rudof_generate/examples/configs/property_graph.toml --schema rudof_generate/examples/schemas/equivalence/references.pgs

# Show help for all options
rudof generate --help
```

> `--generator-config` is distinct from rudof's own `-c`/`--config-file`. See [generate](../docs/src/cli_usage/generate.md).

### Sample SHACL Schema (simple_shacl.ttl)

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
   sh:node    :Course ;
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

### Sample Generated Output

**From SHACL schema:**
```turtle
<http://example.org/Person-1> <http://example.org/name> "Diana Jones" ;
	<http://example.org/enrolledIn> <http://example.org/Course-1> ;
	<http://example.org/birthDate> "1971-03-12"^^<http://www.w3.org/2001/XMLSchema#date> ;
	a <http://example.org/Person> .
<http://example.org/Course-1> <http://example.org/name> "Advanced Mathematics" ;
	a <http://example.org/Course> .
```

**From ShEx schema:**
```turtle
<http://example.org/Person-1> a <http://example.org/Person> ;
	<http://example.org/name> "Fiona Rodriguez" .
<http://example.org/Course-1> a <http://example.org/Course> ;
	<http://example.org/name> "Computer Science" .
```


## Configuration

A configuration is a TOML (or JSON) file that starts with `version = 2`. Unknown keys are
errors, so a typo never silently does nothing. Every section is optional; the examples in
[`examples/configs/`](examples/configs/) cover each use. Command-line flags (`--entities`,
`--seed`, `--parallel`, `--output-file`, `--result-format`) override the file.

```toml
version = 2

[prefixes]                      # CURIEs for the rest of the file; rdf, rdfs, xsd and sh are built in
ex = "http://example.org/"

[run]
seed = 42                       # same seed => same graph, whatever `threads` is
threads = 4                     # omit to use every core
scale = { entities = 1000 }     # a total, split across shapes...
# scale = { per_shape = 1000 }  # ...or the same count of every shape

[defaults]                      # every shape and property, unless a rule below says otherwise
fill = 0.8                      # probability that an OPTIONAL property is generated
cardinality = "balanced"        # "min" | "max" | "uniform" | "balanced"
unbounded_cap = 3               # values generated when the schema gives no maximum

[[shape]]                       # one table per shape, matched by its id
match = "ex:PersonShape"
count = 300                     # or weight = 2.0 (a share of `entities`)
fill = 0.5

[[property]]                    # one table per property; a shape-scoped rule wins
match = { shape = "ex:PersonShape", path = "ex:email" }
fill = 1.0
cardinality = "max"

[values.datatypes."xsd:integer"]
generator = "integer"
parameters = { min = 0, max = 120 }

[values.properties."ex:phone"]
generator = "pattern"
parameters = { pattern = "[0-9]{3}-[0-9]{3}-[0-9]{4}" }

[violations]                    # break a share of the entities on purpose
rate = 0.1
operators = ["drop-required", "out-of-range"]   # all operators when omitted

[targets]                       # ask for a statistic; the generator searches the fill for it
coherence = 0.8
tolerance = 0.01

[output]
dir = "out"                     # every file goes here, under fixed names
formats = ["turtle"]            # "turtle" | "ntriples" | "pg" (with "pg", "turtle" adds the RDF copy)
stats = true
```

Scopes resolve field by field: `[[property]]` > `[[shape]]` > `[defaults]`. Required
properties are always generated; `fill` only decides how often an optional one appears.

**Output files** in `[output] dir`:

| File | When |
|---|---|
| `data.ttl` / `data.nt` | RDF output, or the RDF copy of a property graph |
| `data.shapemap` | the shape of every generated RDF node (a ShEx ShapeMap) |
| `data.pg`, `data.typemap` | property graph output and the type of every node and edge |
| `labels.jsonl` | with `[violations]`: every node (and edge) a validator of the schema's language must report |
| `labels-rdf.jsonl` | with `[violations]` and `formats = ["pg", "turtle"]`: the same for the RDF copy; `[violations.rdf]` sets whether its shapes are `closed` and its keys required (`keys_required`), both `true` by default |
| `data.<ext>.stats.json` | with `stats = true`: counts, timing and conformance metrics |

**Targets.** `[targets] coherence = c` makes the generator search the default fill (with
the default cardinality at `max`) until the coherence of the output (Duan et al., 2011,
with every node typed by its shape) is within `tolerance` of `c`; shape and property rules
keep their own fill, and the data stays valid. A target outside the reachable range yields
the closest graph and a warning. Further metrics will be further fields of `[targets]`.

### Statistics

With `[output] stats = true`, you'll get:

1. **Data file** (`generated_data.ttl`): The actual RDF data in your chosen format
2. **Statistics file** (`generated_data.stats.json`): Generation statistics including:
   - Total triples generated
   - Entity counts per shape type
   - Generation performance metrics
   - Data distribution information
   - Triple validity percentage
   - Shape translation loss percentage

Example statistics:
```json
{
  "total_triples": 15248,
  "generation_time": "497ms",
  "shape_counts": {
    "http://example.org/Person": 334,
    "http://example.org/Organization": 333,
    "http://example.org/Course": 333
  },
  "conformance_metrics": {
    "total_generated_triples": 15248,
    "valid_triples": 14991,
    "triple_validity_percentage": 98.31,
    "original_schema_constraints": 42,
    "represented_constraints_in_unified": 38,
    "shape_translation_loss_percentage": 9.52
  }
}
```

## Architecture

- `config/`: the configuration (`v2.rs` parses and checks a file, `mod.rs` holds the settings the engine reads)
- `adapters/`: ShEx, SHACL and PG-Schema readers into the intermediate representation
- `ir/`: the intermediate representation
- `engine/`: the two-phase generator (entities, then references)
- `values/`: value generators
- `violations/`: mutation operators and the report of what must fail
- `metrics/`: coherence and the conformance metrics
- `sinks/`: RDF and property graph writers

## ShEx And SHACL Coverage Matrix

This matrix summarizes what the current generator can translate into the unified model and therefore use for generation and metric computation.

| Feature | ShEx | SHACL | Status | Notes |
| --- | --- | --- | --- | --- |
| Shape declarations | Yes | Yes | Supported | Core shape IDs are preserved |
| Property constraints | Yes | Yes | Supported | Property IRI and associated constraints are extracted |
| Cardinality | Yes | Yes | Supported | `min`/`max` in ShEx, `sh:minCount`/`sh:maxCount` in SHACL |
| Datatype | Yes | Yes | Supported | Mapped to unified datatype constraints |
| Node kind | Yes | Yes | Supported | Basic node kinds are mapped |
| Pattern | Yes | Yes | Supported | Regex patterns are extracted when present |
| Length facets | Yes | Yes | Partially supported | `minLength`/`maxLength` supported; exact length is not translated |
| Numeric range facets | Yes | Yes | Supported | Inclusive/exclusive bounds are extracted |
| Value set / enumeration | Yes | Yes | Supported | `in` and `hasValue` are represented |
| Shape references | Yes | Yes | Supported | Recursive or referenced shapes are mapped |
| Closed shapes | Partial | Partial | Partially supported | Present in the model, but not fully extracted from schemas |
| Triple expression composition | Yes | Partial | Partially supported | `EachOf` / `OneOf` are handled; some advanced references are not |
| Logical combinators | Limited | No | Not fully supported | SHACL `sh:or`, `sh:and`, `sh:not`, `sh:xone` are not fully translated |
| Qualified value shapes | Limited | No | Not fully supported | Not mapped in the unified model |
| SPARQL-based constraints | No | No | Not supported | Outside current generator scope |
| Semantic actions / imports | Limited | Limited | Not supported | Not preserved in the unified model |
