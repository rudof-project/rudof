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

The `rudof` CLI is intentionally stateless: every command takes all its parameters and is deterministic (see [discussion #748](https://github.com/rudof-project/rudof/discussions/748)). Following the pattern proposed there and in [discussion #747](https://github.com/rudof-project/rudof/discussions/747), `connect` is the only command with a side effect: it writes the connection details to a file that `load` and `query --dialect cypher` consume.

The connection file is a regular TOML file that can be inspected, committed or edited:

```toml
engine = "lbug"
path = "/absolute/path/to/db.lbug"
read_only = false
```

## Options

| Option | Description |
|--------|-------------|
| `<PATH>` | Path to the database directory |
| `--engine <ENGINE>` | Database engine (default: `lbug`, the only one supported today — see [Database engines](#database-engines)) |
| `--in-memory`, `-m` | Create a transient in-memory database |
| `--read-only`, `-r` | Open the database in read-only mode |
| `--connection <FILE>` | Connection details file (default: `.rudof-connection.toml`) |

In-memory databases cannot be reused by later commands because they do not outlive the process, so `connect --in-memory` explicitly refuses to store connection details.

## Database engines

`--engine` picks the database technology `connect` opens. Only `lbug` ([LadybugDB](https://github.com/LadybugDB/ladybug)) exists today, so it's also the default — you won't normally need to pass it. It's a real flag rather than a hardcoded assumption because the choice is persisted into the connection file's own `engine` field: `load` and `query --dialect cypher` read that field to know how to open the database, so a second engine can be added later purely by teaching those two commands (and `--engine`) about it, without another CLI-shape change.

## Related commands

- [`rudof load`](./load.md): validate RDF data with SHACL and copy it into the connected database
- [`rudof query --dialect cypher`](./connect.md#querying-with-cypher): run Cypher queries against the connected database
- [`rudof ddl`](./ddl.md): generate the DDL for a property graph database without opening one

## Querying with Cypher

The [`query`](./sparql.md) command also accepts Cypher queries — given via `-q`/`--query`, same as SPARQL — against a LadybugDB database, selected either via `--db <PATH>` or through the connection details file written by `rudof connect`:

```sh
rudof query --dialect cypher -q "MATCH (n:User) RETURN n.name"
rudof query --dialect cypher -q "MATCH (a:User)-[:knows]->(b:User) RETURN a.name, b.name" --db examples/db.lbug
rudof query --dialect cypher -q query.cypher
```

`--dialect` defaults to `sparql`, so plain `rudof query -q ...` is unaffected; `--db`/`--connection`/`--read-only` are rejected with an error unless `--dialect cypher` is also given, since they're meaningless for a SPARQL query.
