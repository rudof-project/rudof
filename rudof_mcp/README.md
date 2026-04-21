<div align="center">

# Rudof MCP Server

A [Model Context Protocol](https://modelcontextprotocol.io/) server for RDF validation, querying, and data transformation

</div>

---

## Overview

`rudof_mcp` exposes the [Rudof](https://crates.io/crates/rudof_lib) library to any MCP-compatible AI client. It supports Shape Expressions (ShEx), SHACL, SPARQL, and multiple RDF serialization formats, making it straightforward to integrate semantic web operations into AI workflows.

### What You Can Do

- **Load RDF data** — From URLs, local files, raw text, or SPARQL endpoints
- **Validate** — Against ShEx and SHACL schemas
- **Query** — Execute SPARQL SELECT, CONSTRUCT, ASK, and DESCRIBE
- **Inspect** — Explore outgoing/incoming arcs for any RDF node
- **Visualize** — Generate PlantUML diagrams and SVG/PNG images
- **Export** — Serialize data to Turtle, N-Triples, RDF/XML, JSON-LD, and more


## Quick Start

### Claude Desktop (Stdio)

In Claude Desktop’s configuration file, add the following entry:

```json
{
  "mcpServers": {
    "rudof": {
        "command": "/path/to/rudof.exe",
        "args": [ "mcp" ],
        "env": {
            "PLANTUML": "/path/to/plantuml.jar"
        },
        "enabled": true
    }
  }
}
```

> ⚠️ **Visualization support**: The `export_image` tool require PlantUML. To enable them:
> 1. Download [plantuml.jar](https://github.com/plantuml/plantuml/releases) and place it at a fixed path (e.g. `C:\ProgramData\PlantUML\plantuml.jar`).
> 2. Set the `PLANTUML` environment variable in the `env` block above to point to that path.

### HTTP Client

To run the MCP server with `streamable-http` transport on port `8080` under the route `rdf`, allowing connections from localhost and a local network:

```bash
rudof mcp --transport streamable-http --bind 0.0.0.0 --port 8080 --route rdf --allowed-network 127.0.0.1 --allowed-network 192.168.1.0/24
```

> 💡 **Docker**: The server persists loaded RDF data across ephemeral container restarts by saving state to `/app/state/data.json`. Mount a Docker volume at `/app/state/` to preserve data between runs. Override the path with the `RUDOF_MCP_STATE_PATH` environment variable.


## Transport Types

### Stdio

- For CLI tools, IDE extensions (VS Code, IntelliJ, etc.), and Docker
- The MCP client spawns the server as a subprocess
- Communication via stdin/stdout using JSON-RPC

### Streamable HTTP

- For web clients and multi-client deployments
- JSON-RPC over HTTP POST with Server-Sent Events (SSE) for streaming
- Session management via the `MCP-Session-Id` header
- Origin header validation and configurable IP allowlist for security
- CORS support via `tower-http`


## MCP Capabilities

| Capability | Description |
|------------|-------------|
| **Tools** | 10 tools for validation, querying, and data operations |
| **Prompts** | Guided templates for common validation workflows |
| **Resources** | Current RDF data exposed in multiple formats |
| **Resource Templates** | URI template for accessing RDF data in any supported format |
| **Logging** | MCP `notifications/message` with RFC 5424 level filtering |
| **Completions** | Argument completions for prompts and resources |


## Tools

### Data Management

| Tool | Description |
|------|-------------|
| `load_rdf_data_from_sources` | Load RDF data from URLs, files, raw text, or SPARQL endpoints |
| `export_rdf_data` | Serialize RDF data to Turtle, N-Triples, RDF/XML, JSON-LD, etc. |
| `export_plantuml` | Generate a PlantUML diagram of the RDF graph |
| `export_image` | Generate an SVG or PNG visualization |

### Node Inspection

| Tool | Description |
|------|-------------|
| `node_info` | Show outgoing and incoming arcs for an RDF node |

### SPARQL

| Tool | Description |
|------|-------------|
| `execute_sparql_query` | Execute SELECT, CONSTRUCT, ASK, or DESCRIBE queries |

### ShEx

| Tool | Description |
|------|-------------|
| `validate_shex` | Validate the loaded RDF data against a ShEx schema |
| `check_shex` | Check whether a ShEx schema is syntactically well-formed |
| `show_shex` | Parse and display a ShEx schema with optional analysis |

### SHACL

| Tool | Description |
|------|-------------|
| `validate_shacl` | Validate the loaded RDF data against a SHACL schema |


## Prompts

| Prompt | Description |
|--------|-------------|
| `explore_rdf_node` | Guide for exploring RDF node relationships |
| `analyze_rdf_data` | Comprehensive data structure and quality analysis |
| `validation_guide` | Step-by-step ShEx/SHACL validation workflow |


## Resources

### Data

| URI | Description |
|-----|-------------|
| `rudof://current-data` | Currently loaded RDF data in Turtle format |

### Resource Templates

| URI Template | Description |
|--------------|-------------|
| `rudof://current-data/{format}` | Currently loaded RDF data in the specified format. `{format}` must be one of: `turtle`, `ntriples`, `rdfxml`, `jsonld`, `trig`, `nquads`, `n3` |

### Format Metadata

| URI | Description |
|-----|-------------|
| `rudof://formats/rdf` | Supported RDF serialization formats |
| `rudof://formats/node-modes` | Available node inspection modes |
| `rudof://formats/query-types` | Supported SPARQL query types |
| `rudof://formats/query-results` | Supported SPARQL result formats |
| `rudof://formats/shex` | Supported ShEx formats |
| `rudof://formats/shex-validation-result` | ShEx validation result formats |
| `rudof://formats/validation-reader-modes` | Reader modes (strict/lax) |
| `rudof://formats/shex-validation-sort-options` | Sort options for ShEx results |
| `rudof://formats/shacl` | Supported SHACL formats |
| `rudof://formats/shacl-validation-result` | SHACL validation result formats |
| `rudof://formats/shacl-validation-sort-options` | Sort options for SHACL results |


## Security

The HTTP transport enforces:

- **Origin validation** — Requests with invalid `Origin` headers return `403 Forbidden`
- **Protocol version validation** — Invalid MCP protocol versions return `400 Bad Request`
- **IP allowlist** — Requests from non-allowed networks are rejected
- **Session management** — Sessions can be terminated via `HTTP DELETE`


## Dependencies

| Crate | Role |
|-------|------|
| [`rudof_lib`](https://crates.io/crates/rudof_lib) | Core RDF engine (ShEx, SHACL, SPARQL) |
| [`rmcp`](https://crates.io/crates/rmcp) | Rust MCP SDK — protocol and macro infrastructure |
| [`axum`](https://crates.io/crates/axum) | HTTP server for Streamable HTTP transport |
| [`tokio`](https://crates.io/crates/tokio) | Async runtime |
| [`ipnetwork`](https://crates.io/crates/ipnetwork) | IP/CIDR parsing for the network allowlist |

---

<div align="center">

*For more information about the Model Context Protocol, visit the [official MCP documentation](https://modelcontextprotocol.io/).*

</div>
