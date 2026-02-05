# Exporting rudof as an MCP server

Part of `rudof` functionality can be exported as an [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro), allowing it to provide its capabilities to external clients.

The MCP server supports two configurable transport methods:
- `stdio` (default): The client launches the MCP server as a subprocess. The server reads JSON-RPC messages from its standard input (stdin) and sends messages to its standard output (stdout).
- `streamable-http`: The server operates as an independent process that can handle multiple client connections. This transport uses HTTP POST and GET requests. Server can optionally make use of Server-Sent Events (SSE) to stream multiple server messages.

By default, the MCP server uses the `stdio` transport. When using `streamable-http` transport, the server binds to `127.0.0.1` (localhost) on port `8000` by default and exposes its functionality under the route name `rudof`.

You can start the MCP server with the following command:

```sh
rudof mcp
```

> ⚠️ **IMPORTANT**: To use Rudof’s functionality for exporting RDF data to visual formats (PlantUML, SVG, PNG), you must set the PLANTUML environment variable.
>
> For example, if you are using Claude Desktop as the MCP client (the example client in this documentation), follow these steps:
> 1. Download the [plantuml.jar](https://github.com/plantuml/plantuml/releases)
> 2. Place it in a fixed location, for example: `C:\ProgramData\PlantUML\plantuml.jar`
> 3. Set the environment variable in Claude Desktop’s client configuration file, in the rudof MCP section:
>
> ```json
> "rudof": {
>      "command": "/path/to/rudof",
>      "args": [ "mcp" ],
>      "env": {
>        "PLANTUML": "/path/to/plantuml.jar"
>      },
>      "enabled": true
>    }
> ```

## MCP Capabilities

The Rudof MCP server exposes the following capabilities:

| Capability    | Feature                                              |
|---------------|------------------------------------------------------|
| `tools`       | 12 tools for validation, querying, and data ops      |
| `prompts`     | Guided templates for common workflows                |
| `resources`   | Access to RDF data and format information            |
| `logging`     | Real-time log notifications with level filtering     |
| `completions` | Argument completions for tools and prompts           |
| `tasks`       | Async task support for long-running operations       |

### Available Tools

The MCP server provides 12 tools organized by functionality:

**Data Management:**

| Tool | Description |
|------|-------------|
| `load_rdf_data_from_sources` | Load RDF data from URLs, files, raw text, or SPARQL endpoints |
| `export_rdf_data` | Serialize RDF data to various formats (Turtle, JSON-LD, N-Triples, etc.) |
| `export_plantuml` | Generate PlantUML diagram of the RDF graph |
| `export_image` | Generate SVG or PNG visualization of the RDF graph |

**Node Inspection:**

| Tool | Description |
|------|-------------|
| `node_info` | Show information about a node (outgoing/incoming arcs) |

**Query:**

| Tool | Description |
|------|-------------|
| `execute_sparql_query` | Execute SPARQL queries (SELECT, CONSTRUCT, ASK, DESCRIBE) |

**ShEx Tools:**

| Tool | Description |
|------|-------------|
| `validate_shex` | Validate RDF data against a ShEx schema |
| `check_shex` | Check if a ShEx schema is well-formed |
| `shape_info` | Get information about a specific ShEx shape |
| `convert_shex` | Convert ShEx schema between formats (shexc, shexj, turtle) |
| `show_shex` | Parse and display ShEx schema with optional analysis |

**SHACL Tools:**

| Tool | Description |
|------|-------------|
| `validate_shacl` | Validate RDF data against a SHACL schema |

### Available Prompts

The MCP server provides guided templates for common workflows:

| Prompt | Description |
|--------|-------------|
| `explore_rdf_node` | Interactive guide for exploring RDF node information and relationships |
| `analyze_rdf_data` | Comprehensive guide for analyzing RDF data structure and quality |
| `validation_guide` | Step-by-step guide for validating RDF data against ShEx or SHACL schemas |
| `sparql_builder` | Interactive helper for building and understanding SPARQL queries |

### Available Resources

The server exposes resources for accessing RDF data and format information:

**Current RDF Data (multiple formats):**

- `rudof://current-data` - Turtle format
- `rudof://current-data/ntriples` - N-Triples format
- `rudof://current-data/rdfxml` - RDF/XML format
- `rudof://current-data/jsonld` - JSON-LD format
- `rudof://current-data/trig` - TriG format
- `rudof://current-data/nquads` - N-Quads format
- `rudof://current-data/n3` - Notation3 format

**Format Information:**

- `rudof://formats/rdf` - Supported RDF formats
- `rudof://formats/shex` - Supported ShEx formats
- `rudof://formats/shacl` - Supported SHACL formats
- `rudof://formats/node-modes` - Node inspection modes
- `rudof://formats/query-types` - Supported SPARQL query types
- `rudof://formats/query-results` - Query result formats
- `rudof://formats/shex-validation-result` - ShEx validation result formats
- `rudof://formats/shacl-validation-result` - SHACL validation result formats
- `rudof://formats/validation-reader-modes` - Reader modes (strict/lax)
- `rudof://formats/shex-validation-sort-options` - ShEx result sort options
- `rudof://formats/shacl-validation-sort-options` - SHACL result sort options

## Changing MCP Server Settings

### Changing the transport type

You can specify the transport type used by the MCP server with the `--transport` (or `-t`) option. Possible values include `streamable-http` and `stdio` (default).

For example, to start the server using the `streamable-http` transport:

```sh
rudof mcp --transport streamable-http
```

### Changing the bind address (streamable-http)

You can specify which network interface the server binds to using the `--bind` (or `-b`) option. This controls where the server listens for incoming connections. **Default:** `127.0.0.1` (localhost only, most secure)

For example, to bind to all IPv4 interfaces (allows network access):

```sh
rudof mcp --transport streamable-http --bind 0.0.0.0
```

> ⚠️ **Security Warning**: When binding to `0.0.0.0` or `::`, the server becomes accessible from any network interface. Always combine this with appropriate firewall rules and the `--allowed-network` option to restrict access.

### Changing the port (streamable-http)

You can specify a custom port using the `--port` (or `-p`) option.

By default, it is `8000`.

For example, to run the server on port 9000:

```sh
rudof mcp --port 9000
```

### Changing the route name (streamable-http)

The route name determines the path under which the MCP server is exposed.

By default, it is `rudof`, but you can change it with `--route` (or `-n`):

```sh
rudof mcp --route rdfserver
```

### Configuring allowed networks (streamable-http)

For security, the MCP server validates the `Origin` header of incoming HTTP requests. By default, only `localhost` connections are allowed (127.0.0.0/8 for IPv4 and ::1/128 for IPv6).

You can specify custom allowed networks using the `--allowed-network` (or `-n`) option. This option accepts IP addresses or networks in CIDR notation and can be specified multiple times.

```sh
rudof mcp --transport streamable-http -n 127.0.0.1 -n 192.168.1.0/24
```

> 💡 **Note**: The `--bind` and `--allowed-network` options serve different purposes:
> - `--bind` controls which network interface the server listens on
> - `--allowed-network` controls which origins are accepted via the Origin HTTP header
> 
> For maximum security, you can bind to `0.0.0.0` (to allow network access) while restricting allowed networks with `-n` flags.

### Combined example

You can combine all parameters as needed.

For example, to run the MCP server with `streamable-http` transport on port `8080` under the route `rdf`, allowing connections from localhost and a local network:

```sh
rudof mcp --transport streamable-http --bind 0.0.0.0 --port 8080 --route rdf --allowed-network 127.0.0.1 --allowed-network 192.168.1.0/24
```

## Connecting with Claude Desktop (Example MCP Client)

Once you have started rudof as an MCP server, you can configure a `client that supports MCP servers` to it. You can find a list of clients [here](https://modelcontextprotocol.io/clients).

As an example, we'll show how to connect the rudof MCP server to `Claude Desktop`, allowing you to interact with your RDF data directly through Claude.

### Claude Desktop’s Configuration

#### Using stdio transport (default)

In Claude Desktop's configuration file, add the following entry under your MCP servers section:
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

#### Using Streamable HTTP transport

If you want to test the streamable-http transport, you need to verify:

- `rudof MCP server` is running locally (for example, using `rudof mcp --transport streamable-http`).
- [Node.js](https://nodejs.org/es/download) is installed on your system (required to use the `mcp-remote` command).

In Claude Desktop's configuration file, add the following entry under your MCP servers section:
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

### Prompt Examples
Once connected, you can interact with `rudof` through `Claude Desktop` using natural language prompts.

Below are detailed examples that illustrate the key functionalities of the rudof MCP server.

#### Import RDF Data

You can import RDF data directly into the graph:

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

#### Retrieve Information About a Node

Once the RDF data is loaded, you can explore the relationships of any node in the graph:

```
I'd like to explore the RDF node ':a' and see all its relationships in the loaded graph.
```

#### Export RDF Data to Another Format

You can export the RDF data from your current graph into a variety of formats, including JSON-LD, Turtle, and more:

```
Export the current RDF graph to JSON-LD format.
```

Additionally, you can export the graph for visualization in formats such as PlantUML, SVG, and PNG:

```
Export the current RDF graph to PlantUML.
```

```
Export the current RDF graph to PNG.
```

#### Execute SPARQL Queries

You can also execute SPARQL queries against the RDF graph:

```
Run the following SPARQL query on the current RDF graph:

prefix : <http://example.org/>
prefix schema: <http://schema.org/>

select ?person ?name ?status where {
  ?person schema:name ?name ;
          :status ?status .
}
```

You can also express the query in natural language:

```
For each person, return the total number of people they know either directly or indirectly with a relationship degree of 2, that is, they know someone who knows that person.
```

#### Validate RDF Data with ShEx

You can validate your RDF data against a ShEx schema:

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

You can also check if a ShEx schema is well-formed:

```
Check if the following ShEx schema is valid:

prefix : <http://example.org/>
:Shape { :property . }
```

#### Validate RDF Data with SHACL

Similarly, you can validate RDF data against a SHACL shapes graph:

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

#### Convert ShEx Schema Formats

You can convert ShEx schemas between different formats:

```
Convert this ShEx schema to JSON format:

prefix : <http://example.org/>
:Person { :name . ; :age . }
```

#### Get Shape Information

Retrieve detailed information about a specific shape in a schema:

```
Show me information about the :Person shape in the previously loaded ShEx schema.
```