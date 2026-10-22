# `rudof_mcp`

`rudof_mcp` exposes [`rudof_lib`](https://crates.io/crates/rudof_lib) as a
[Model Context Protocol](https://modelcontextprotocol.io/) server, so any MCP client can load RDF
data, query it with SPARQL, validate it against ShEx or SHACL, inspect nodes and export the graph
to other serializations or to a diagram. It is built on the [`rmcp`](https://crates.io/crates/rmcp)
SDK.

The server is reached through the CLI:

```sh
rudof mcp                                   # stdio, the default
rudof mcp --transport streamable-http        # HTTP on 127.0.0.1:8000/rudof
```

A client that spawns it as a subprocess needs the command and, for the image export tools, the path
to `plantuml.jar`:

```json
{
  "mcpServers": {
    "rudof": {
      "command": "/path/to/rudof",
      "args": ["mcp"],
      "env": { "PLANTUML": "/path/to/plantuml.jar" },
      "enabled": true
    }
  }
}
```

## Documentation

- [Exporting rudof as an MCP server](https://rudof-project.github.io/rudof/cli_usage/mcp.html):
  transports, options, security settings, the tools, prompts and resources on offer, and how to
  connect a client. Start here if you want to use the server.
- [`rudof_mcp` internals](https://rudof-project.github.io/rudof/internals/crates/rudof_mcp.html):
  how the crate is laid out, how a tool is declared, and how to embed the server in your own
  program.
- [API reference](https://docs.rs/rudof_mcp).
