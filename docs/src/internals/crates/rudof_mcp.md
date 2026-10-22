# `rudof_mcp`

`rudof_mcp` implements an [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro)
over [`rudof_lib`](./rudof_lib.md), built on the [`rmcp`](https://crates.io/crates/rmcp) SDK. The
CLI's `mcp` command is a thin wrapper around it.

This page is about the crate. What the server offers a client (its tools, prompts and resources)
and how to configure and connect it are documented in
[Exporting rudof as an MCP server](../../cli_usage/mcp.md).

## Architecture and package structure

Neither module is compiled for WebAssembly, where there is no process to serve from.

### `server/`

Everything about getting the server running, with no knowledge of what it serves.

- **`server_impl.rs`**: `McpConfig` (transport, bind address, port, route, allowed origins and
  hosts) with its defaults, plus `run_mcp`, which creates its own Tokio runtime, and
  `run_mcp_async` for callers that already have one. Both dispatch on the transport.
- **`transport.rs`**: `TransportType`, either `Stdio` or `StreamableHTTP`.
- **`stdio_server.rs`**: serves one `RudofMcpService` over `rmcp`'s stdio transport. Nothing other
  than MCP messages may reach stdout, so every diagnostic goes through `tracing` to stderr.
- **`streamable_http/http_server.rs`**: mounts `rmcp`'s `StreamableHttpService` on an `axum` route
  and builds a service per session through `LocalSessionManager`. The checks the specification
  requires (`Origin`, `Host`, `MCP-Protocol-Version`, session routing and `DELETE` termination) are
  `rmcp`'s, configured here from `McpConfig`; an empty origin list would turn validation off, so it
  is filled with the loopback defaults instead. CORS is left to a reverse proxy.

### `service/`

The MCP surface. One `RudofMcpService` per session, so an HTTP deployment can hold several at once.

- **`mcp_service.rs`**: `RudofMcpService` itself, the session state: a `Rudof` instance behind a
  mutex (the loaded data, schemas and results every tool reads and writes), the tool and prompt
  routers, the client's minimum log level and the log rate limiter. It also answers the completion
  requests for prompt arguments and resource URIs.
- **`handlers.rs`**: the `ServerHandler` implementation. `get_info` advertises the capabilities and
  announces rudof as the implementation; resources, completions and `logging/setLevel` are handled
  here, while tool and prompt calls are dispatched by the `#[tool_handler]` and
  `#[prompt_handler]` macros to the routers.
- **`tools/`**: `tools_impl.rs` declares every tool with `#[tool]` (name, title, description,
  output schema and annotations) and delegates to a sibling `*_tools_impl.rs` holding the request
  and response types and the logic. `params.rs` wraps the `rudof_lib` format and mode enums as
  parameter types narrowed to the values the server implements, so the accepted values reach the
  input schema. `helpers.rs` carries what the tools share: `ToolExecutionError`, input spec
  resolution, structured-content serialization and the code-block previews.
- **`prompts/`** and **`resources/`**, **`resource_templates/`**: the same split, a declaration
  module plus one implementation file per domain.
- **`errors.rs`**: the protocol errors, each with its JSON-RPC code, context object and trace.
- **`logging.rs`**: the `notifications/message` notifications, filtered by the level the client
  set and rate limited so a chatty tool cannot flood it.
- **`state.rs`**: the state file a containerized server restores its data from. It is only used
  when the directory of the path (`/app/state/data.json`, or `RUDOF_MCP_STATE_PATH`) exists.

## Two kinds of error

A tool reports a problem with its input as a successful response with `isError: true`, built from
`ToolExecutionError` with a message and a hint, which a model can read and retry from. A schema
that fails to parse, a format that cannot be produced or a missing PlantUML installation belong
here. The protocol errors in `errors.rs` are for what a client cannot fix by rewriting the call:
an unknown resource, a serialization failure, an internal error in the engine.

## Adding a tool

1. Add the request and response types, plus the logic, to the `*_tools_impl.rs` of its domain (or a
   new one, registered in `tools/mod.rs`). Both types derive `JsonSchema`, every parameter with a
   fixed set of values comes from `params.rs`, and the response is serialized with
   `serialize_structured`.
2. Declare it in `tools_impl.rs` with `#[tool]`, pass the output schema with `schema_for_output`
   and write the annotations honestly: a tool that writes session state is not `read_only_hint`.
3. Return the facts as structured content and a short summary as text, not the whole report twice.
4. Document it in the [user manual](../../cli_usage/mcp.md) and, when it changes what a client
   sees, in the changelog.

## Resolving file paths

A tool argument can be inline content, a URL or a local path, told apart by
`rudof_lib::formats::InputSpec`. A relative path is resolved against the working directory of the
server process, the one its client started it in. The server deliberately has no directory of its
own to resolve against: re-rooting a client's paths is a filesystem server's job, and under the
HTTP transport the sessions sharing one process would have to agree on it.

## Embedding the server

```rust
use rudof_mcp::server::{run_mcp, McpConfig};

run_mcp(McpConfig::default())?; // stdio
```

```rust
use rudof_mcp::server::{run_mcp, McpConfig, TransportType};

let config = McpConfig {
    transport: TransportType::StreamableHTTP,
    bind_address: "0.0.0.0".to_string(),
    port: 9000,
    route_path: "/mcp".to_string(),
    allowed_origins: vec!["https://app.example.com:443".to_string()],
    // A wildcard bind accepts only loopback `Host` headers until named here
    allowed_hosts: vec!["rdf.example.com".to_string()],
};

run_mcp(config)?;
```

Inside an existing runtime, `run_mcp_async(config).await` takes the same configuration without
creating a runtime of its own.

## Dependencies

- [`rmcp`](https://crates.io/crates/rmcp): the Rust MCP SDK, protocol and macros.
- [`rudof_lib`](./rudof_lib.md): every RDF operation the tools perform.
- [`axum`](https://crates.io/crates/axum): the HTTP server behind the streamable-HTTP transport.
- [`tokio`](https://crates.io/crates/tokio): the async runtime.

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_mcp).
