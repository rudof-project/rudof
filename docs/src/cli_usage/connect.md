# Connect to a database

`rudof connect` opens (creating it if necessary) a [LadybugDB](https://github.com/LadybugDB/ladybug) database and stores the connection details in a small file so that later, stateless, commands can reuse them.

```sh
rudof connect <path-to-db>
```

Example:

```sh
$ rudof connect examples/db.lbug
LadybugDB database opened successfully
  Path: examples/db.lbug
  Builder storage version: 42
  Library source: release:LadybugDB/ladybug/latest
  Connection details stored in '.rudof-connection.toml' (used by `load` and `query --cypher`)
```

## Why a connection file?

The `rudof` CLI is intentionally stateless: every command takes all its parameters and is deterministic (see [discussion #748](https://github.com/rudof-project/rudof/discussions/748)). Following the pattern proposed there and in [discussion #747](https://github.com/rudof-project/rudof/discussions/747), `connect` is the only command with a side effect: it writes the connection details to a file that `load` and `query --cypher` consume.

The connection file is a regular TOML file that can be inspected, committed or edited:

```toml
engine = "ladybug"
path = "/absolute/path/to/db.lbug"
read_only = false
```

## Options

| Option | Description |
|--------|-------------|
| `<PATH>` | Path to the LadybugDB database directory |
| `--in-memory`, `-m` | Create a transient in-memory database |
| `--read-only`, `-r` | Open the database in read-only mode |
| `--connection <FILE>` | Connection details file (default: `.rudof-connection.toml`) |

In-memory databases cannot be reused by later commands because they do not outlive the process, so `connect --in-memory` explicitly refuses to store connection details.

## Related commands

- [`rudof load`](./load.md): validate RDF data with SHACL and copy it into the connected database
- [`rudof query --cypher`](./connect.md#querying-with-cypher): run Cypher queries against the connected database
- [`rudof ddl`](./ddl.md): generate the DDL for a property graph database without opening one

## Querying with Cypher

The [`query`](./sparql.md) command also accepts Cypher queries against a LadybugDB database, either via `--db <PATH>` or through the connection details file written by `rudof connect`:

```sh
rudof query --cypher "MATCH (n:User) RETURN n.name"
rudof query --cypher "MATCH (a:User)-[:knows]->(b:User) RETURN a.name, b.name" --db examples/db.lbug
```
