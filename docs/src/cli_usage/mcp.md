# Exporting rudof as an MCP server

Part of `rudof` functionality can be exported as an [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro), allowing it to provide its capabilities to external clients.

The MCP server supports two configurable transport methods:
- `stdio` (default): The client launches the MCP server as a subprocess. The server reads JSON-RPC messages from its standard input (stdin) and sends messages to its standard output (stdout).
- `streamable-http`: The server operates as an independent process that can handle multiple client connections. This transport uses HTTP POST and GET requests. Server can optionally make use of Server-Sent Events (SSE) to stream multiple server messages.

By default, the MCP server uses the `stdio` transport. When using `streamable-http` transport, the server listens on `http://127.0.0.1:8000` by default and exposes its functionality under the route name `rudof`.

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

## Changing MCP Server Settings

### Changing the transport type

You can specify the transport type used by the MCP server with the `--transport` (or `-t`) option. Possible values include `streamable-http` and `stdio` (default).

For example, to start the server using the `streamable-http` transport:

```sh
rudof mcp --transport streamable-http
```

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

### Combined example

You can combine all parameters as needed.

For example, to run the MCP server with `streamable-http` transport on port `8080` under the route `rdf`:

```sh
rudof mcp --transport streamable-http --port 8080 --route-name rdf
```

## Using Streamable HTTP Transport

If you want to use the `streamable-http` transport, you need to deploy an Authorization Server (AS) for authentication. This section explains how to set it up using Docker Desktop.

### Deploying the Authorization Server

The Authorization Server is based on Keycloak and can be deployed using Docker Compose. The configuration file is located at `/rudof-mcp/src/as`. To start it:

1. Navigate to the directory containing the docker-compose file:
```sh
   cd /rudof-mcp/src/as
```

2. Start the services with Docker Compose:
```sh
   docker-compose up -d
```

3. The Authorization Server will be accessible at `http://localhost:8080` (username: `admin`, password: `admin`).


> ⚠️ **IMPORTANT** By default, the Authorization Server is configured with the following audience: `http://localhost:8000/rudof`.
> If you start the MCP server with a different port or route name, you must update the audience  configuration in the Authorization Server accordingly.
>For example, if you start the server with:
>```sh
>rudof mcp --transport streamable-http --port 9000 --route-name rdfserver
>```
>You need to change the audience in the Authorization Server to `http://localhost:9000/rdfserver`


## Connecting with Claude Desktop (Example MCP Client)

Once you have started rudof as an MCP server, you can configure a `client that supports MCP servers` to it. You can find a list of clients [here](https://modelcontextprotocol.io/clients).

As an example, we'll show how to connect the rudof MCP server to `Claude Desktop`, allowing you to interact with your RDF data directly through Claude.

### Claude Desktop’s Configuration

#### Using stdio transport (default)

In Claude Desktop's configuration file, add the following entry under your MCP servers section:
```json
"rudof": {
  "command": "/path/to/rudof",
  "args": [ "mcp" ],
  "env": {
      "PLANTUML": "/path/to/plantuml.jar"
   },
  "enabled": true
}
```

> **Note**: Replace `/path/to/rudof` with the actual path to your `rudof` binary.

#### Using Streamable HTTP transport

If you want to test the streamable-http transport, you need to verify:

- `rudof MCP server` is running locally (for example, using `rudof mcp --transport streamable-http`).
- [Node.js](https://nodejs.org/es/download) is installed on your system (required to use the `mcp-remote` command).
- The Authorization Server is deployed and running (see `Using HTTP-SSE Transport` section).

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
