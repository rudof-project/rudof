# `rudof_mcp`

The `rudof_mcp` crate provides an implementation of an [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro) that exposes the core functionality of the `rudof` library.
It is built using the [`rmcp`](https://crates.io/crates/rmcp) SDK.

## MCP Transport Types

The Rudof MCP server supports two configurable transport mechanisms:

- **`stdio` (default):**
  The client launches the MCP server as a subprocess. The server reads JSON-RPC messages from standard input (`stdin`) and writes responses to standard output (`stdout`).

- **`streamable-http`:**
  The server runs as an independent process capable of handling multiple concurrent client connections.
  This transport uses HTTP `POST` and `GET` requests and can optionally leverage Server-Sent Events (SSE) to stream multiple server messages.

  The Rudof MCP server allows configuring:
  - Bind address
  - Port
  - Route name
  - Allowed networks

  It also implements the security requirements defined by the MCP specification:
  - Origin Validation
  - Protocol Version Validation
  - Session Management

## MCP Capabilities

The Rudof MCP server exposes the following capabilities:

| Capability    | Description                                           |
|--------------|-------------------------------------------------------|
| `tools`       | 12 tools for validation, querying, and data operations |
| `prompts`     | Guided templates for common workflows                  |
| `resources`   | Access to RDF data and format metadata                 |
| `logging`     | Real-time log notifications with level filtering       |
| `completions` | Argument completion for tools and prompts              |
| `tasks`       | Asynchronous task support for long-running operations  |

## Available Tools

The MCP server provides 12 tools grouped by functionality.

### Data Management

| Tool | Description |
|------|-------------|
| `load_rdf_data_from_sources` | Load RDF data from URLs, files, raw text, or SPARQL endpoints |
| `export_rdf_data` | Serialize RDF data into multiple formats (Turtle, JSON-LD, N-Triples, etc.) |
| `export_plantuml` | Generate a PlantUML diagram of the RDF graph |
| `export_image` | Generate SVG or PNG visualizations of the RDF graph |

> ⚠️ **IMPORTANT**: The `export_image` tool generates SVG or PNG visualizations of the RDF graph using [plantuml.jar](https://github.com/plantuml/plantuml/releases).

### Node Inspection

| Tool | Description |
|------|-------------|
| `node_info` | Retrieve information about a node (incoming and outgoing arcs) |

### Query

| Tool | Description |
|------|-------------|
| `execute_sparql_query` | Execute SPARQL queries (SELECT, CONSTRUCT, ASK) |

### ShEx Tools

| Tool | Description |
|------|-------------|
| `validate_shex` | Validate RDF data against a ShEx schema |
| `check_shex` | Verify whether a ShEx schema is well-formed |
| `shape_info` | Retrieve information about a specific ShEx shape |
| `convert_shex` | Convert ShEx schemas between formats (ShExC, ShExJ, Turtle) |
| `show_shex` | Parse and display ShEx schemas with optional analysis |

### SHACL Tools

| Tool | Description |
|------|-------------|
| `validate_shacl` | Validate RDF data against a SHACL schema |

## Available Prompts

The MCP server includes guided templates for common workflows:

| Prompt | Description |
|--------|-------------|
| `explore_rdf_node` | Interactive guide for exploring RDF nodes and their relationships |
| `analyze_rdf_data` | Comprehensive guide for analyzing RDF data structure and quality |
| `validation_guide` | Step-by-step guide for validating RDF data with ShEx or SHACL |
| `sparql_builder` | Interactive assistant for building and understanding SPARQL queries |

## Available Resources

The server exposes resources for accessing RDF data and format metadata.

### Current RDF Data (multiple formats)

- `rudof://current-data` — Turtle format
- `rudof://current-data/ntriples` — N-Triples format
- `rudof://current-data/rdfxml` — RDF/XML format
- `rudof://current-data/jsonld` — JSON-LD format
- `rudof://current-data/trig` — TriG format
- `rudof://current-data/nquads` — N-Quads format
- `rudof://current-data/n3` — Notation3 format

### Format Information

- `rudof://formats/rdf` — Supported RDF formats
- `rudof://formats/shex` — Supported ShEx formats
- `rudof://formats/shacl` — Supported SHACL formats
- `rudof://formats/node-modes` — Node inspection modes
- `rudof://formats/query-types` — Supported SPARQL query types
- `rudof://formats/query-results` — Query result formats
- `rudof://formats/shex-validation-result` — ShEx validation result formats
- `rudof://formats/shacl-validation-result` — SHACL validation result formats
- `rudof://formats/validation-reader-modes` — Reader modes (strict/lax)
- `rudof://formats/shex-validation-sort-options` — ShEx result sorting options
- `rudof://formats/shacl-validation-sort-options` — SHACL result sorting options

## Usage

We can run Rudof MCP using different transport types depending on the environment (CLI tools, IDE extensions, or HTTP clients).

### Run Rudof MCP using Stdio transport

```rust
use rudof_mcp::{run_mcp, McpConfig};

run_mcp(McpConfig::default())?;
```

### Run Rudof MCP using HTTP transport (localhost only)

```rust
use rudof_mcp::{run_mcp, McpConfig, TransportType};

let config = McpConfig {
    transport: TransportType::StreamableHTTP,
    bind_address: Some("127.0.0.1".to_string()),
    port: Some(8080),
    route_path: Some("mcp".to_string()),
    allowed_networks: None, // Defaults to localhost only
};

run_mcp(config)?;
```

### Run Rudof MCP using HTTP transport with custom allowed networks

```rust
use rudof_mcp::{run_mcp, McpConfig, TransportType};

let config = McpConfig {
    transport: TransportType::StreamableHTTP,
    bind_address: Some("0.0.0.0".to_string()),
    port: Some(9000),
    route_path: Some("mcp".to_string()),
    allowed_networks: Some(vec![
        "127.0.0.1".to_string(),
        "192.168.1.0/24".to_string(),
        "::1".to_string(),
    ]),
};

run_mcp(config)?;
```

### Run Rudof MCP asynchronously (inside an existing Tokio runtime)

```rust
use rudof_mcp::{run_mcp_async, McpConfig, TransportType};

let config = McpConfig {
    transport: TransportType::StreamableHTTP,
    bind_address: Some("127.0.0.1".to_string()),
    port: Some(8080),
    route_path: Some("mcp".to_string()),
    allowed_networks: None,
};

tokio::spawn(async move {
    run_mcp_async(config).await.unwrap();
});
```

## Dependencies

This crate primarily depends on:

- [`rmcp`](https://crates.io/crates/rmcp) — Rust MCP SDK for protocol implementation
- [`rudof_lib`](https://crates.io/crates/rudof_lib) — Core Rudof library for RDF operations
- [`axum`](https://crates.io/crates/axum) — Modern HTTP server for Streamable HTTP transport
- [`tokio`](https://crates.io/crates/tokio) — High-performance asynchronous runtime
- [`ipnetwork`](https://crates.io/crates/ipnetwork) — IP address and network parsing and validation

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_mcp).
