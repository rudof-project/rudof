# rudof_mcp

Export rudof_lib functionality as a [Model-Context-Protocol](https://modelcontextprotocol.io/) server with configurable transports.

## Features

- **Dual Transport Support**: Choose between stdio and HTTP SSE based on your use case.
- **Configurable Server**: Port and route path configuration for HTTP transport.
- **MCP Protocol Support**: Full support for MCP Protocol versions 2025-06-18 (current), 2025-03-26 (fallback), and 2024-11-05 (deprecated).
- **OAuth2/OIDC Authentication**: Security for HTTP transport with JWT validation.
- **Real-time Notifications**: Server-to-client notifications via Server-Sent Events (HTTP) or JSON-RPC (stdio) with shared notification channel.
- **Resource Subscriptions**: Clients can subscribe to specific resources for updates.
- **Argument Completion**: Intelligent completion suggestions for prompt and resource arguments (pending implementation).
- **Dynamic Log Levels**: Runtime log level configuration via MCP logging protocol.
- **Session Management**: Explicit session termination support with DELETE requests.
- **CORS Support**: Cross-Origin Resource Sharing enabled for web clients (HTTP transport).

## Architecture

```
rudof_mcp/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                      
│   ├── as/                                       # Authentication server (Keycloak)
│   │   ├── docker-compose.yml
│   │   └── keycloak/
│   ├── server/
│   │   ├── mod.rs                
│   │   ├── server.rs                             # Main entry point + tracing
│   │   ├── transport.rs                          # TransportType enum
│   │   ├── stdio_server.rs                       # Stdio transport implementation
│   │   └── http/                                 # HTTP-specific modules
│   │       ├── mod.rs
│   │       ├── server.rs                         # HTTP server implementation
│   │       ├── config.rs                         # HTTP configuration constants
│   │       ├── middleware.rs                     # Protocol & origin guards
│   │       └── auth/                             # Authentication module
│   │           ├── mod.rs
│   │           ├── config.rs                     # AuthConfig, JWKS cache
│   │           ├── validation.rs                 # Token verification & validation
│   │           ├── middleware.rs                 # Authorization guard
│   │           └── metadata.rs                   # OAuth2 discovery endpoint
│   └── rudof_mcp_service/                        # MCP service implementation
│       ├── mod.rs
│       ├── service.rs                            # Main service state
│       ├── handlers.rs                           # MCP request handlers
│       ├── errors.rs                             # Error types & helpers
│       ├── resource_templates.rs                 # Resource template definitions
│       ├── tools/                                # Tool implementations
│       │   ├── mod.rs
│       │   ├── tools_impl.rs                     # Tool router & annotations
│       │   ├── data_tools_impl.rs                # RDF data loading/export tools
│       │   ├── node_tools_impl.rs                # Node inspection tools
│       │   ├── query_tools_impl.rs               # SPARQL query tools
│       │   └── shex_validate_tools_impl.rs       # ShEx validation tools
│       ├── prompts/                              # Prompt implementations
│       │   ├── mod.rs
│       │   ├── prompts_impl.rs                   # Prompt router
│       │   ├── data_prompts_impl.rs              # Data analysis prompts
│       │   ├── node_prompts_impl.rs              # Node exploration prompts
│       │   ├── query_prompts_impl.rs             # Query optimization prompts
│       │   └── validation_prompts_impl.rs        # Validation error prompts
│       └── resources/                            # Resource implementations
│           ├── mod.rs
│           ├── resources_impl.rs                 # Resource router
│           ├── data_resources_impl.rs            # Current data resources
│           ├── node_resources_impl.rs            # Node mode resources
│           ├── query_resources_impl.rs           # Query result resources
│           └── shex_validate_resources_impl.rs   # Validation resources
```

## Available Tools

The MCP server exposes the following tools:

1. **load_rdf_data_from_sources**: Load RDF data from remote sources (URLs, files, raw text) or SPARQL endpoint into the server's datastore.
2. **export_rdf_data**: Serialize and return the RDF stored on the server in the requested format.
3. **export_plantuml**: Generate a PlantUML diagram of the RDF stored on the server.
4. **export_image**: Generate an image (SVG or PNG) visualization of the RDF stored on the server.
5. **node_info**: Show information about a node (outgoing/incoming arcs) from the RDF stored on the server.
6. **execute_sparql_query**: Execute a SPARQL query (SELECT, CONSTRUCT, ASK, DESCRIBE) against the RDF stored on the server.
7. **validate_shex**: Validate RDF data against a ShEx schema using the provided inputs.

## Available Prompts

The MCP server exposes the following prompts:

1. **explore_rdf_node**: Interactive guide for exploring RDF node information, relationships, and graph structure.
2. **analyze_rdf_data**: Comprehensive guide for analyzing loaded RDF data structure, patterns, and quality.
3. **generate_test_data**: Generate conformant RDF test data examples from a ShEx schema.
4. **optimize_sparql_query**: Get suggestions to optimize SPARQL query performance and efficiency.
5. **suggest_shex_schema**: Get help creating a ShEx schema for your RDF data domain.
6. **explain_validation_errors**: Understand and fix ShEx validation errors with detailed explanations.
7. **debug_shex_schema**: Debug ShEx schema syntax, reference, and logical errors.

## Available Resources

The MCP server exposes the following resources:

1. **rudof://current-data**: Currently loaded RDF data in Turtle format
2. **rudof://current-data/ntriples**: Currently loaded RDF data in N-Triples format
3. **rudof://current-data/rdfxml**: Currently loaded RDF data in RDF/XML format
4. **rudof://current-data/jsonld**: Currently loaded RDF data in JSON-LD format
5. **rudof://current-data/trig**: Currently loaded RDF data in TriG format
6. **rudof://current-data/nquads**: Currently loaded RDF data in N-Quads format
7. **rudof://current-data/n3**: Currently loaded RDF data in N3 format
8. **rudof://formats/rdf**: List of supported RDF formats
9. **rudof://formats/node-modes**: Available node exploration modes (outgoing, incoming, both)
10. **rudof://formats/query-types**: Supported SPARQL query types
11. **rudof://formats/query-results**: Supported query result formats
12. **rudof://formats/shex**: Supported ShEx schema formats
13. **rudof://formats/validation-result**: Supported validation result formats
14. **rudof://formats/validation-reader-modes**: Available reader modes for validation
15. **rudof://formats/validation-sort-options**: Available sort options for validation results
