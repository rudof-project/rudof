# Subgraph test suite (issue #957)

`tests/subgraph_suite.rs` runs each directory in this folder as a test case. Each case contains these files:

| File | Content |
|------|---------|
| `schema.shex` | The ShEx schema, in ShExC. |
| `data.ttl` | The RDF data, in Turtle. |
| `shapemap.sm` | The query shape map. It uses the prefixes of the schema and of the data. |
| `valid.nt` | The expected valid subgraph: the triples that the conformant pairs match, and the valid triples of the pairs that they refer to. |
| `invalid.nt` | The expected invalid subgraph: the triples that the validator reads and that are not valid. |
| `visited.nt` | The expected visited subgraph: all the triples that the validator reads. |

The test compares each graph as a set of N-Triples lines. Thus, the cases do not use blank nodes.

The suite also makes sure that the valid and invalid subgraphs do not overlap and that together they are equal to the visited subgraph.

Write `valid.nt` by hand. `visited.nt` records the triples that the validator reads. If you change the triples that the validator reads, generate `visited.nt` again with this command:

```sh
BLESS_SUBGRAPH=1 cargo test -p shex_validation --test subgraph_suite
```

Then examine the differences before you commit them.
