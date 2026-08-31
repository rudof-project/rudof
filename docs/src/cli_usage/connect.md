# Connect to a database

`rudof connect` opens (creating it if necessary) a [LadybugDB](https://github.com/LadybugDB/ladybug) database and stores the connection details in a small file so that later, stateless, commands can reuse them.

```sh
rudof connect <path-to-db>
```

Also works inside [`rudof shell`](./shell.md#working-with-a-ladybugdb-database), the same as any other subcommand.

Example:

```sh
$ rudof connect examples/db.lbug
LadybugDB database opened successfully
  Path: examples/db.lbug
  Builder storage version: 43
  Library source: release:LadybugDB/ladybug/latest
  Connection details stored in '.rudof-connection.toml' (used by `load` and `query --dialect cypher`)
```

## Why a connection file?

The `rudof` CLI is intentionally stateless: every command takes all its parameters and is deterministic. `connect` is the only command with a side effect: it writes the connection details to a file that `load` and `query --dialect cypher` consume.

The connection file is a regular TOML file that can be inspected, committed or edited:

```toml
backend = "lbug"
path = "/absolute/path/to/db.lbug"
read_only = false
```

## Options

| Option | Description |
|--------|-------------|
| `<PATH>` | Path to the database directory |
| `--backend <BACKEND>` | Backend to connect to (default: `lbug`, the only one that can actually be connected to today — see [Backends](#backends)) |
| `--in-memory`, `-m` | Create a transient in-memory database |
| `--read-only`, `-r` | Open the database in read-only mode |
| `--connection <FILE>` | Connection details file (default: `.rudof-connection.toml`) |

In-memory databases cannot be reused by later commands because they do not outlive the process, so `connect --in-memory` explicitly refuses to store connection details.

## Backends

`--backend` is the same flag and type used across the rest of `rudof` (see [The `--backend` flag](./backend.md)) — `memory`, `qlever`, `endpoint=<URL_OR_NAME>`, and `lbug`. `connect` only actually knows how to open something for `lbug` ([LadybugDB](https://github.com/LadybugDB/ladybug)) today, which is why it's the default; any other value is rejected with a clear error, e.g.:

```sh
$ rudof connect --backend memory some/path
Error: Property Graph database error: Unsupported database engine 'memory'. Valid engines are: lbug
```

Reusing the shared `--backend` flag/type here — rather than a `connect`-specific one — means a future backend that *can* be connected to (a networked triple store like Jena Fuseki or GraphDB, for instance) only needs a new `BackendSpec` variant, not a new argument shape for `connect` itself. The choice is persisted into the connection file's own `backend` field: `load` and `query --dialect cypher` read that field to know how to open the database.

## Full workflow

Connect, preview the DDL, validate and load, then query.

`connect`, `ddl`, `load` and `query --dialect cypher` are usually used together: connect to a database, optionally preview the schema `rudof` will derive, validate and load RDF data into it, then query it with Cypher.

Given a small dataset and matching SHACL shapes:

```turtle
# data.ttl
@prefix : <http://example.org/> .

:alice a :User ;
    :name "Alice" ;
    :knows :bob .

:bob a :User ;
    :name "Bob" .
```

```turtle
# shapes.ttl
@prefix : <http://example.org/> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

:UserShape a sh:NodeShape ;
    sh:targetClass :User ;
    sh:property [
        sh:path :name ;
        sh:datatype xsd:string ;
        sh:minCount 1 ;
    ] .
```

1. **Connect** to (creating) the database. This is the only command with a side effect — it writes `.rudof-connection.toml` so the following commands don't need to repeat the path.

   ```sh
   $ rudof connect db.lbug
   LadybugDB database opened successfully
     Path: db.lbug
     Connection details stored in '.rudof-connection.toml' (used by `load` and `query --dialect cypher`)
   ```

2. **Preview the DDL** `rudof` will derive from the data — stateless, doesn't touch the database, useful to sanity-check the schema before loading anything (see [`ddl`](./ddl.md)):

   ```sh
   $ rudof ddl data.ttl --dialect cypher
   CREATE NODE TABLE User (id STRING, knows STRING, name STRING, PRIMARY KEY(id));
   CREATE REL TABLE knows (FROM User TO User);
   ```

3. **Validate and load** the data (see [`load`](./load.md)): validates against the SHACL shapes, aborting on any violation, then applies that same DDL and inserts the data.

   ```sh
   $ rudof load data.ttl --shapes shapes.ttl
   Loaded 5 triples from RDF data
   Loaded SHACL shapes (2 shapes)
   SHACL validation PASSED ✓
   Creating schema (1 node table(s), 1 relationship table(s)) and loading data...
     Created node table: User
     Created relationship table: knows (User → User)
     Inserted 2 node(s)
     Inserted 1 relationship(s)
     ✓ Load complete!
   ```

4. **Query** the loaded data with Cypher (see [Querying with Cypher](#querying-with-cypher) below), through the same connection details file — no `--db` needed:

   ```sh
   $ rudof query --dialect cypher -q "MATCH (n:User) RETURN n.name ORDER BY n.name"
   Query result (2 tuples, 1 columns):
   Columns: ["n.name"]
     ["Alice"]
     ["Bob"]
   ```

The same four commands work identically inside [`rudof shell`](./shell.md#working-with-a-ladybugdb-database), one per line, with the connection persisting across lines the same way it persists across separate CLI invocations.

## Related commands

- [`rudof load`](./load.md): validate RDF data with SHACL and copy it into the connected database
- [`rudof query --dialect cypher`](./connect.md#querying-with-cypher): run Cypher queries against the connected database
- [`rudof ddl`](./ddl.md): generate the DDL for a property graph database without opening one
- [The `--backend` flag](./backend.md): the same flag/type as `--backend` above, used by `data`/`node`/`query`/`shacl*`/`validate` to select an RDF backend

## Querying with Cypher

The [`query`](./query.md) command also accepts Cypher queries — given via `-q`/`--query`, same as SPARQL — against a LadybugDB database, selected either via `--db <PATH>` or through the connection details file written by `rudof connect`:

```sh
rudof query --dialect cypher -q "MATCH (n:User) RETURN n.name"
rudof query --dialect cypher -q "MATCH (a:User)-[:knows]->(b:User) RETURN a.name, b.name" --db examples/db.lbug
rudof query --dialect cypher -q query.cypher
```

`--dialect` defaults to `sparql`, so plain `rudof query -q ...` is unaffected; `--db`/`--connection`/`--read-only` are rejected with an error unless `--dialect cypher` is also given, since they're meaningless for a SPARQL query.
