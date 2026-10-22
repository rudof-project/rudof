# Exporting rudof as an MCP server

Part of `rudof` can be exported as an [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro),
which offers its capabilities to any MCP client: an AI assistant, an IDE extension or your own
program.

```sh
rudof mcp
```

That starts the server over the `stdio` transport, where the client launches `rudof` as a
subprocess and both exchange JSON-RPC messages through standard input and output. The alternative
is `streamable-http`, where the server runs on its own and serves several clients over HTTP POST
and GET, optionally streaming messages back with Server-Sent Events. By default it listens on
`127.0.0.1:8000` under the route `/rudof`.

Exporting RDF data to PlantUML, SVG or PNG needs [plantuml.jar](https://github.com/plantuml/plantuml/releases)
on disk and the `PLANTUML` environment variable pointing at it. Set it in the client
configuration (see [Connecting a client](#connecting-a-client)) or in the environment the server
is started from.

## What the server offers

| Capability | Feature |
|---------------|------------------------------------------------------|
| `tools` | 13 tools for validation, querying, data and session operations |
| `prompts` | Guided templates for common workflows |
| `resources` | Access to the loaded RDF data and to the external-resolver kinds |
| `resource templates` | URI template for reading the data in any supported format |
| `logging` | Log notifications with level filtering |
| `completions` | Argument completions for prompts and resources |

### Tools

Data management:

| Tool | Description |
|------|-------------|
| `load_rdf_data_from_sources` | Load RDF data from URLs, files, raw text, or SPARQL endpoints |
| `export_rdf_data` | Serialize RDF data to various formats (Turtle, JSON-LD, N-Triples, etc.) |
| `export_plantuml` | Generate a PlantUML diagram of the RDF graph |
| `export_image` | Generate an SVG or PNG visualization of the RDF graph |

Node inspection and query:

| Tool | Description |
|------|-------------|
| `node_info` | Show information about a node (outgoing and incoming arcs) |
| `execute_sparql_query` | Execute SPARQL queries (SELECT, CONSTRUCT, ASK) |

ShEx and SHACL:

| Tool | Description |
|------|-------------|
| `validate_shex` | Validate RDF data against a ShEx schema |
| `check_shex` | Check if a ShEx schema is well-formed |
| `show_shex` | Serialize a ShEx schema, with optional statistics and dependencies |
| `validate_shacl` | Validate RDF data against a SHACL schema |

Session management:

| Tool | Description |
|------|-------------|
| `reset_session_state` | Clear session state (all, or specific targets like data/shex/shacl/...) |
| `get_rudof_version` | Report the rudof version this MCP server is running |
| `manage_prefixes` | Show or manage the session's default prefixes (list/add/remove/rename/copy), mirroring the shell's `prefixes` command |

Data loaded in a session stays there until the session ends or `reset_session_state` clears it, and
every `load_rdf_data_from_sources` call merges into it. A relative file path is resolved against
the working directory the client started the server in; if you need to resolve paths elsewhere, pass
absolute paths or run a filesystem MCP server alongside this one.

### Prompts

| Prompt | Description |
|--------|-------------|
| `explore_rdf_node` | Interactive guide for exploring RDF node information and relationships |
| `analyze_rdf_data` | Comprehensive guide for analyzing RDF data structure and quality |
| `validation_guide` | Step-by-step guide for validating RDF data against ShEx or SHACL schemas |

### Tool parameters and results

Every parameter that takes one of a fixed set of values (an RDF format, a validation mode, a sort
order) declares those values in the tool's input schema, so a client reads them from `tools/list`
and never has to guess.

Each tool answers with both halves of an MCP tool result:

- `structuredContent` carries the facts, against the tool's output schema. RDF terms are typed
  rather than rendered: an IRI, a blank node, a literal with its datatype and language tag, or an
  RDF 1.2 triple term.
- `content` carries a short summary of those facts, and the rendered report keeps a field of its own
  in `structuredContent` (`report` for the validators, `results` for the rest).

| Tool | What `structuredContent` carries |
|------|----------------------------------|
| `validate_shacl` | `conforms`, `counts_by_severity`, one entry per result with its focus node, severity, constraint component, path, value, source shape, messages and nested `sh:detail` results, and the rendered `report` |
| `validate_shex` | `conforms`, `counts` by status, one `entries` item per `(node, shape)` pair with the validator's reason, and the rendered `report` |
| `execute_sparql_query` | `query_type`, plus `bindings` (variables and one row per solution, keyed by variable name) for a SELECT or `boolean` for an ASK |
| `node_info` | `outgoing_arcs` and `incoming_arcs`, each arc with its direction, depth, predicate and the term at its far end, plus `stats` |
| `show_shex` | `schema`, and `statistics` / `dependencies` when asked for with `show_statistics` / `show_dependencies` |
| `check_shex` | `well_formed`, and the checker's `report`. A schema that is not well-formed is an answer, not a tool error |

### Resources

The loaded RDF data is readable as a resource:

- `rudof://current-data`, in Turtle (the listed resource).
- `rudof://current-data/{format}`, the resource template, where `{format}` is one of `turtle`,
  `ntriples`, `rdfxml`, `jsonld`, `trig`, `nquads`, `n3`.

One more resource describes a `validate_shex` parameter:

- `rudof://shex/external-resolvers`, the external-shape resolver kinds the tool accepts.

## Changing MCP server settings

### Transport type

`--transport` (or `-t`) takes `stdio` (the default) or `streamable-http`:

```sh
rudof mcp --transport streamable-http
```

The options below only apply to `streamable-http`.

### Bind address

`--bind` (or `-b`) is the network interface the server listens on. It defaults to `127.0.0.1`,
reachable from this machine only, which is also the safest choice:

```sh
rudof mcp --transport streamable-http --bind 0.0.0.0
```

Binding to `0.0.0.0` or `::` makes the server reachable from every interface, so combine it with
firewall rules and with `--allowed-host`, which becomes **required**: a wildcard bind cannot be
resolved to the authority clients will use, so only loopback `Host` headers are accepted until you
name your own.

### Port and route

`--port` (or `-p`) defaults to `8000`; `--route` (or `-r`) defaults to `/rudof` and has to start
with a `/`:

```sh
rudof mcp --transport streamable-http --port 9000 --route /rdfserver
```

### Allowed origins and hosts

The MCP specification requires an HTTP server to validate two request headers, and rudof configures
both.

`Origin`: which browser origins may call the server. An origin is a `(scheme, host, port)` triple
per RFC 6454. By default only loopback origins are accepted (`http://localhost:*`,
`http://127.0.0.1:*`, `http://[::1]:*` and their `https` forms). A request carrying any other
`Origin` gets `403 Forbidden`; a request with no `Origin` header at all (curl, a CLI client,
server to server) is not subject to the check.

Set your own list with `--allowed-origin` (or `-o`), comma-separated or repeated. Every entry needs
a scheme, and should carry an explicit port or `:*`:

```sh
rudof mcp --transport streamable-http -o https://app.example.com:443 -o http://localhost:*
```

`Host`: which authorities clients may use to reach the server. This is what stops DNS rebinding.
Loopback is always accepted, and a concrete `--bind` address is added automatically. Name a
wildcard bind with `--allowed-host` (or `-H`):

```sh
rudof mcp --transport streamable-http --bind 0.0.0.0 -H rdf.example.com
```

The three options answer three different questions: `--bind` is which interface the server listens
on, `--allowed-origin` which web origins may call it (browser clients) and `--allowed-host` which
names it answers to (DNS-rebinding protection). They combine as needed, for instance to serve port
`8080` under `/rdf`, reachable on the local network as `rdf.example.com` and callable from one web
app:

```sh
rudof mcp --transport streamable-http --bind 0.0.0.0 --port 8080 --route /rdf -H rdf.example.com -o https://app.example.com:443
```

### Keeping the data of a containerized server

A server whose container is recreated on every tool call would lose the loaded data with it, so
when the directory of its state file exists, the server writes the data there and reads it back on
startup. The file is `/app/state/data.json`, so mounting a volume at `/app/state/` is enough;
`RUDOF_MCP_STATE_PATH` points it somewhere else.

## Connecting a client

Any [MCP client](https://modelcontextprotocol.io/clients) can talk to the server. The examples
below use Claude Desktop; the entries go under the MCP servers section of its configuration file.

Over `stdio`, the client launches the server itself:

```json
"rudof": {
  "command": "/path/to/rudof.exe",
  "args": [ "mcp" ],
  "env": {
      "PLANTUML": "/path/to/plantuml.jar"
   },
  "enabled": true
}
```

Over `streamable-http`, the server has to be running already (for example with
`rudof mcp --transport streamable-http`), and the client reaches it through `mcp-remote`, which
needs [Node.js](https://nodejs.org/es/download):

```json
"rudof": {
  "command": "npx",
  "args": [
    "mcp-remote",
    "http://localhost:8000/rudof",
    "--transport",
    "http-first"
  ],
  "enabled": true
}
```

## Prompt examples

Once connected, you drive `rudof` in natural language. The data can be loaded inline, from a file
or from a URL:

```
Import the following RDF data into the graph:

prefix : <http://example.org/>
prefix schema: <http://schema.org/>

:a schema:name  "Alice" ;
   :status      :Active ;
   schema:knows :a, :b  .

:b schema:name  "Bob"    ;
   :status      :Waiting ;
   schema:knows :c       .

:c schema:name  "Carol"  .

:d schema:name  23      .

:e schema:name  "Emily" ;
   schema:knows :d      .
```

With the data loaded, you can explore a node, export the graph to another serialization (JSON-LD,
Turtle, ...) or to a visualization (PlantUML, SVG, PNG):

```
I'd like to explore the RDF node ':a' and see all its relationships in the loaded graph.
```

```
Export the current RDF graph to JSON-LD format.
```

```
Export the current RDF graph to PNG.
```

Queries can be given as SPARQL:

```
Run the following SPARQL query on the current RDF graph:

prefix : <http://example.org/>
prefix schema: <http://schema.org/>

select ?person ?name ?status where {
  ?person schema:name ?name ;
          :status ?status .
}
```

or described in words, leaving the client to write the query:

```
For each person, return the total number of people they know either directly or indirectly with a relationship degree of 2, that is, they know someone who knows that person.
```

Validation works the same way with a ShEx schema:

```
Validate the current RDF graph against the following ShEx schema:

prefix : <http://example.org/>
prefix schema: <http://schema.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  schema:name xsd:string ;
  :status [:Active :Waiting]? ;
  schema:knows @:Person *
}
```

or with a SHACL shapes graph:

```
Validate the current RDF graph against this SHACL schema:

@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix schema: <http://schema.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex: <http://example.org/> .

ex:PersonShape a sh:NodeShape ;
  sh:targetClass schema:Person ;
  sh:property [
    sh:path schema:name ;
    sh:datatype xsd:string ;
    sh:minCount 1 ;
  ] .
```

A schema can also be checked on its own, without any data loaded:

```
Check if the following ShEx schema is valid:

prefix : <http://example.org/>
:Shape { :property . }
```
