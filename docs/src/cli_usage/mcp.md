# Exporting rudof as an MCP server

A part of `rudof` functionality can be exported as an [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro), allowing it to provide its capabilities to external clients over HTTP.

By default, the MCP server will listen on port `8000`, at host `127.0.0.1`, and expose its functionality under the route name `rudof`.

You can start the MCP server with the following command:

```sh
rudof mcp
```

This will start a local MCP server accessible at:

```sh
http://127.0.0.1:8000/rudof
```

## Changing the port

You can specify a custom port using the `--port` (or `-p`) option.

For example, to run the server on port 9000:

```sh
rudof mcp --port 9000
```

## Changing the route name

The route name determines the path under which the MCP server is exposed.

By default, it is `rudof`, but you can change it with `--route-name` (or `-n`):

```sh
rudof mcp --route-name rdfserver
```

## Changing the host address

By default, `rudof` binds the MCP server to `127.0.0.1` (localhost).

To make it accessible from other machines, you can specify a different host address using the `--host` option.

For example, to bind to all interfaces:

```sh
rudof mcp --host 0.0.0.0
```

## Combined example

You can combine all parameters as needed.

For example, to run the MCP server on port 8080, accessible from any host, under the route rdf:

```sh
rudof mcp --host 0.0.0.0 --port 8080 --route-name rdf
```

# More information

Once you have started rudof as an MCP server, you can configure a client that supports MCP servers to it. You can find a list of clients [here](https://modelcontextprotocol.io/clients)