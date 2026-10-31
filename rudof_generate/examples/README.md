# rudof generate examples

Every schema here generates data that is valid against it; the test
`tests/conformance/examples.rs` checks this for each file, with the validator of its
language. The equivalence and cross-language tests read their schemas from this folder.

| Folder | What it holds |
|---|---|
| `configs/` | generator configurations: `config_v2.toml` (the current format, annotated), and older ones kept for reference (`simple_config.toml`, `config.toml`, `comprehensive_config.toml`, `parallel_config.toml`, `auto_parallel.toml`, `pattern_test_config.toml`) |
| `schemas/basic/` | small ShEx schemas to start with: `person.shex` (a recursive user shape) and `patterns.shex` (regular expressions) |
| `schemas/equivalence/` | the same constructs in ShEx, SHACL and PG-Schema, `<name>.{shex,ttl,pgs}`: they generate the same graph from the same seed, valid in the three languages. `keys` and `facets` are in the common fragment of ShEx, SHACL and PG-Schema; `labels` and `references` are shared outside it; `ldbc-snb` is the LDBC Social Network Benchmark schema (edge types with properties are left out of the RDF versions, since they are not generated) |
| `schemas/cross-language/` | translations of `equivalence/references.pgs` that keep ShEx's or SHACL's own defaults (open shapes; optional properties in SHACL), for the RDF reading of a broken property graph (`[violations.rdf]` with `closed = false`, `keys_required = false`) |
| `schemas/violations/` | schemas for controlled violations: a person schema in ShEx and SHACL, closed shapes, language tags, term equality, `EXTRA`, qualified value shapes, `sh:class` next to `sh:node`, recursion, and a PG-Schema (`persons.pgs`) |
| `conformance_cases/` | runnable scenarios with their configurations: a valid baseline, a lossy SHACL schema, sparse generation |
| `validate_generated.rs` | generate from a schema and validate the output with rudof's validators |

The RDF versions of the PG-Schemas use the encoding the generator writes for a property
graph: a node of type `T` is typed with its labels `http://rudof.example/pg/label/L`, key
`k` is the predicate `http://rudof.example/pg/key/k`, and an edge of type `E` with label
`l` is the predicate `http://rudof.example/pg/edge/E/l/conforming-source`.

## Try it

```sh
# The same data from three languages (compare the three outputs)
rudof generate -s rudof_generate/examples/schemas/equivalence/references.shex -n 30 --seed 7 -o shex.ttl
rudof generate -s rudof_generate/examples/schemas/equivalence/references.ttl  -n 30 --seed 7 -o shacl.ttl

# Generate and validate in one step (PG-Schema writes a .pg graph and a .map type map)
cargo run -p rudof_generate --example validate_generated -- \
    rudof_generate/examples/schemas/equivalence/references.pgs out.pg - 30 7
```

Python: see `bindings/python/examples/generate/` and the notebook
`bindings/python/examples/generate/generate_demo.ipynb`.
