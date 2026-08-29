# Generate DDL

`rudof ddl` derives a property graph schema from RDF data and generates the DDL needed to materialize it in a property graph database. It is stateless: it never opens a database, which makes it useful to inspect the schema before loading anything.

```sh
rudof ddl <data...> --dialect <cypher|gql>
```

Also works inside [`rudof shell`](./shell.md#working-with-a-ladybugdb-database), the same as any other subcommand.

The schema is discovered from the data:

- Every class used in an `rdf:type` triple becomes a **node table** (one column per predicate, plus an `id` primary key column holding the subject IRI).
- Every predicate whose object is an IRI referring to a typed node becomes a **relationship table**.

## Dialects

### `--dialect cypher`

LadybugDB/Kùzu-style Cypher DDL:

```sh
$ rudof ddl examples/user.ttl --dialect cypher
CREATE NODE TABLE User (id STRING, knows STRING, name STRING, PRIMARY KEY(id));
CREATE NODE TABLE Post (id STRING, author STRING, title STRING, PRIMARY KEY(id));
CREATE REL TABLE knows (FROM User TO User);
CREATE REL TABLE author (FROM Post TO User);
```

These are exactly the statements that [`rudof load`](./load.md) applies to the database before inserting data.

### `--dialect gql`

ISO GQL-style graph type DDL (`CREATE GRAPH TYPE` with `NODE TYPE`/`EDGE TYPE` declarations), which documents the graph type in the dialect of the GQL standard:

```sh
$ rudof ddl examples/user.ttl --dialect gql
CREATE GRAPH TYPE rudof_graph (
  NODE TYPE User (id STRING, knows STRING, name STRING),
  NODE TYPE Post (id STRING, author STRING, title STRING),
  EDGE TYPE knows (FROM User TO User),
  EDGE TYPE author (FROM Post TO User)
);
```

The name of the graph type can be customized with `--graph-type-name <NAME>`.

## Options

| Option | Description |
|--------|-------------|
| `<DATA>...` | RDF data files used to derive the schema |
| `--dialect <DIALECT>` | `cypher` (default) or `gql` |
| `--graph-type-name <NAME>` | Graph type name used by the `gql` dialect |
| `-t, --data-format <FORMAT>` | RDF data format (default: `turtle`) |
| `--base-data <IRI>` | Base IRI for the data |
| `--reader-mode <MODE>` | RDF reader mode (default: `strict`) |
| `-o, --output-file <FILE>` | Write the DDL to a file instead of the terminal |

Progress messages are written to stderr, so the generated DDL on stdout can be piped directly to another tool:

```sh
rudof ddl data.ttl --dialect cypher 2>/dev/null | cypher-shell
```

## Relationship with `pgschema`

`rudof` already supports [PGSchema](./pgschema.md) for validating YARS-PG property graph data with `rudof pgschema-validate`. The `ddl` command is the DDL-emitting counterpart of that infrastructure: both operate on property graph schemas (node types with properties, edge types with endpoints) and differ only in serialization target — `pgschema` serializes to the PGSchema-C format, while `ddl` serializes to executable Cypher/GQL DDL.

The internal schema model produced by `ddl` is deliberately dialect-agnostic so that a future bridge can feed a `PGSchema` schema (`.pgs` file) directly into the DDL emitters, e.g. `rudof ddl --schema user.pgs --dialect cypher`, and conversely emit a PGSchema-C schema from RDF data. See [discussion #747](https://github.com/rudof-project/rudof/discussions/747) for the context (bridging RDF/SHACL semantics with storage-oriented graph technologies like Cypher and GQL).
