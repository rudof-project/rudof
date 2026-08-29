# Load data

`rudof load` copies RDF data into a LadybugDB property graph database. It is the RDF/SHACL entry point of the flow described in [discussion #747](https://github.com/rudof-project/rudof/discussions/747): validate the data with SHACL, derive a property graph schema, and let the database DDL constrain subsequent mutations.

```sh
rudof load <data...> [--shapes <shapes...>]
```

Also works inside [`rudof shell`](./shell.md#working-with-a-ladybugdb-database), the same as any other subcommand.

The database can be selected in two ways (in this order of precedence):

1. `--db <PATH>`: open the LadybugDB database at the given path.
2. A connection details file written by [`rudof connect`](./connect.md) (default: `.rudof-connection.toml` in the current directory, or the file given with `--connection <FILE>`).

Example:

```sh
$ rudof connect examples/db.lbug
$ rudof load examples/user.ttl --shapes examples/user.shacl
Loaded 13 triples from RDF data
Loaded SHACL shapes (4 shapes)
SHACL validation PASSED ✓
Creating schema (2 node table(s), 2 relationship table(s)) and loading data...
  Created node table: User
  ...
  Inserted 4 node(s)
  Inserted 4 relationship(s)
  ✓ Load complete!
```

## What it does

1. **Loads the RDF data** into an in-memory store.
2. **Validates the data with SHACL** (unless `--skip-validation` is given): either against the shapes graph given with `--shapes`, or against shapes embedded in the data itself. Any validation violation aborts the load, so the database only ever receives conforming data.
3. **Derives a property graph schema** from the data (classes from `rdf:type`, properties from predicates, relationships from predicates with IRI objects) — the same schema that [`rudof ddl`](./ddl.md) prints.
4. **Applies the DDL**: creates the node and relationship tables (existing tables are left untouched).
5. **Copies the data**: inserts nodes and relationships through Cypher `CREATE` statements.

Because the node/rel tables act as the schema of the database, subsequent mutations that do not conform to the DDL are rejected by the database itself. This is generally necessary, but not sufficient to guarantee full SHACL conformance — re-run `rudof shacl-validate` on the RDF source (or re-load) to check the data again.

## Options

| Option | Description |
|--------|-------------|
| `<DATA>...` | RDF data files to load |
| `--db <PATH>` | Path to the LadybugDB database (overrides connection details) |
| `--connection <FILE>` | Connection details file (default: `.rudof-connection.toml`) |
| `--skip-validation` | Copy the data without SHACL validation |
| `-s, --shapes <INPUT>` | Shapes graph: file, URI or `-`; if not set, shapes come from the data |
| `-f, --shapes-format <FORMAT>` | Shapes file format (default: `turtle`) |
| `--base-shapes <IRI>` | Base IRI for the shapes |
| `-t, --data-format <FORMAT>` | RDF data format (default: `turtle`) |
| `--base-data <IRI>` | Base IRI for the data |
| `--reader-mode <MODE>` | RDF reader mode (default: `strict`) |

## Querying the loaded data

Use [`rudof query --cypher`](./connect.md#querying-with-cypher) to run Cypher queries against the loaded database.
