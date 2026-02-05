<div align="center">

# Rudof MCP Server

A powerful Model Context Protocol server for RDF validation, querying, and data transformation

</div>

---

## 🚀 Overview

Rudof MCP is a comprehensive [Model Context Protocol](https://modelcontextprotocol.io/) server implementation that exposes the powerful **Rudof** library to AI assistants and MCP-compatible clients. Seamlessly integrate RDF validation, SPARQL querying, and semantic data transformation into your AI workflows.

### What Can You Do?

- 📊 **Load & Manipulate RDF Data** — From files, URLs, or SPARQL endpoints
- ✅ **Validate RDF Data** — Against ShEx and SHACL schemas
- 🔍 **Execute SPARQL Queries** — SELECT, CONSTRUCT, and ASK
- 🧭 **Explore RDF Graphs** — Through intuitive node inspection tools
- 🔄 **Convert Schemas** — Between different serialization formats
- 📈 **Visualize Data** — Generate PlantUML diagrams and images


## 🔌 Transport Types

### Stdio Transport

**Perfect for:**
- 🔧 CLI tools, command-line interfaces and IDE extensions (VSCode, IntelliJ, etc.)
- 💻 Local process-to-process communication
- 👤 Single client connections

The client launches the MCP server as a subprocess, with communication via stdin/stdout using JSON-RPC messages.

### Streamable HTTP Transport

**Ideal for:**
- 🌐 Web-based MCP clients
- 🔗 Remote connections over HTTP/HTTPS
- 👥 Multiple concurrent client connections

**Features:**
- JSON-RPC over HTTP POST
- Server-Sent Events (SSE) for real-time streaming
- Session management with `MCP-Session-Id` header
- Origin header validation for security


## 🎯 MCP Capabilities

| Capability | Description |
|------------|-------------|
| **🛠️ Tools** | 10+ tools for validation, querying, and data operations |
| **📝 Prompts** | Guided templates for common validation workflows |
| **📦 Resources** | Access to current RDF data in multiple formats |
| **📋 Logging** | Real-time log notifications with RFC 5424 level filtering |
| **⌨️ Completions** | Argument completions for tools and prompts |
| **⚡ Tasks** | Async task support for long-running operations (SEP-1686) |


## 🛠️ Available Tools

### Data Management

| Tool | Description |
|------|-------------|
| `load_rdf_data_from_sources` | Load RDF data from URLs, files, or raw text |
| `export_rdf_data` | Serialize RDF data to Turtle, N-Triples, RDF/XML, etc. |
| `export_plantuml` | Generate PlantUML diagram of the RDF graph |
| `export_image` | Generate SVG or PNG visualization |

### Node Inspection

| Tool | Description |
|------|-------------|
| `node_info` | Show outgoing/incoming arcs for an RDF node |

### SPARQL Queries

| Tool | Description |
|------|-------------|
| `execute_sparql_query` | Execute SELECT, CONSTRUCT, or ASK queries |

### ShEx Validation & Analysis

| Tool | Description |
|------|-------------|
| `validate_shex` | Validate RDF data against a ShEx schema |
| `check_shex` | Check if a ShEx schema is syntactically valid |
| `shape_info` | Get information about a specific ShEx shape |
| `convert_shex` | Convert schemas between ShExC, ShExJ, Turtle formats |
| `show_shex` | Parse and display schema with optional analysis |

### SHACL Validation

| Tool | Description |
|------|-------------|
| `validate_shacl` | Validate RDF data against a SHACL schema |


## 📝 Interactive Prompts

Guided templates for common workflows:

| Prompt | Description |
|--------|-------------|
| `explore_rdf_node` | Guide for exploring RDF node relationships |
| `analyze_rdf_data` | Comprehensive data structure and quality analysis |
| `validation_guide` | Step-by-step ShEx/SHACL validation workflow |
| `sparql_builder` | Interactive helper for building SPARQL queries |


## 📦 Available Resources

The server exposes RDF data and metadata through MCP resources with support for pagination and subscriptions.

### Data Resources

| Resource URI | Name | Description |
|--------------|------|-------------|
| `rudof://current-data` | Current RDF Data (Turtle) | Currently loaded RDF data in Turtle format |
| `rudof://current-data/ntriples` | Current RDF Data (N-Triples) | Currently loaded RDF data in N-Triples format |
| `rudof://current-data/rdfxml` | Current RDF Data (RDF/XML) | Currently loaded RDF data in RDF/XML format |
| `rudof://current-data/jsonld` | Current RDF Data (JSON-LD) | Currently loaded RDF data in JSON-LD format |
| `rudof://current-data/trig` | Current RDF Data (TriG) | Currently loaded RDF data in TriG format |
| `rudof://current-data/nquads` | Current RDF Data (N-Quads) | Currently loaded RDF data in N-Quads format |
| `rudof://current-data/n3` | Current RDF Data (N3) | Currently loaded RDF data in Notation3 format |
| `rudof://formats/rdf` | Supported RDF Formats | List of all supported RDF data formats |

### Node Resources

| Resource URI | Name | Description |
|--------------|------|-------------|
| `rudof://formats/node-modes` | Node Inspection Modes | Available modes for node inspection |

### Query Resources

| Resource URI | Name | Description |
|--------------|------|-------------|
| `rudof://formats/query-types` | Supported SPARQL Query Types | Supported SPARQL query types |
| `rudof://formats/query-results` | Supported Query Result Formats | Supported SPARQL query result formats |

### ShEx Resources

| Resource URI | Name | Description |
|--------------|------|-------------|
| `rudof://formats/shex` | Supported ShEx Formats | All supported ShEx schema formats |
| `rudof://formats/shex-validation-result` | ShEx Validation Result Formats | Supported ShEx validation result formats |
| `rudof://formats/validation-reader-modes` | Validation Reader Modes | Available reader modes (strict/lax) |
| `rudof://formats/shex-validation-sort-options` | ShEx Validation Sort Options | Available sort options for results |

### SHACL Resources

| Resource URI | Name | Description |
|--------------|------|-------------|
| `rudof://formats/shacl` | Supported SHACL Formats | All supported SHACL schema formats |
| `rudof://formats/shacl-validation-result` | SHACL Validation Result Formats | Supported SHACL validation result formats |
| `rudof://formats/shacl-validation-sort-options` | SHACL Validation Sort Options | Available sort options for results |


## 🔒 Security

The HTTP transport implements comprehensive MCP specification security requirements:

- ✅ **Origin Validation** — Invalid Origin headers return HTTP 403 Forbidden
- ✅ **Protocol Version Validation** — Invalid versions return HTTP 400 Bad Request
- ✅ **Session Management** — Explicit session termination via HTTP DELETE

---

## 📚 Dependencies

Built on robust, well-maintained libraries:

- [`rmcp`](https://crates.io/crates/rmcp) — Rust MCP SDK for protocol implementation
- [`rudof_lib`](https://crates.io/crates/rudof_lib) — Core Rudof library for RDF operations
- [`axum`](https://crates.io/crates/axum) — Modern HTTP server for StreamableHTTP transport
- [`tokio`](https://crates.io/crates/tokio) — High-performance async runtime
- [`ipnetwork`](https://crates.io/crates/ipnetwork) — IP address and network parsing and validation for allowed network configuration

---

<div align="center">

*For more information about the Model Context Protocol, visit the [official MCP documentation](https://modelcontextprotocol.io/)📖.*

</div>

---