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

> ⚠️ **IMPORTANT** If you want to use rudof's functionality to export RDF data to visual formats (PlantUML, SVG, PNG), you must set up `PlantUML` **before starting the MCP server**.
>
> 1. Download the [plantuml.jar](https://github.com/plantuml/plantuml/releases)
> 2. Place it in `C:\ProgramData\PlantUML\plantuml.jar`
> 3. Set the environment variable:
>
> ```sh
> $env:PLANTUML="C:\ProgramData\PlantUML\plantuml.jar"
> ```

## Changing Server Settings
### Changing the port

You can specify a custom port using the `--port` (or `-p`) option.

For example, to run the server on port 9000:

```sh
rudof mcp --port 9000
```

### Changing the route name

The route name determines the path under which the MCP server is exposed.

By default, it is `rudof`, but you can change it with `--route-name` (or `-n`):

```sh
rudof mcp --route-name rdfserver
```

### Changing the host address

By default, `rudof` binds the MCP server to `127.0.0.1` (localhost).

To make it accessible from other machines, you can specify a different host address using the `--host` option.

For example, to bind to all interfaces:

```sh
rudof mcp --host 0.0.0.0
```

### Combined example

You can combine all parameters as needed.

For example, to run the MCP server on port 8080, accessible from any host, under the route rdf:

```sh
rudof mcp --host 0.0.0.0 --port 8080 --route-name rdf
```

## Connecting with Claude Desktop (Example MCP Client)

Once you have started rudof as an MCP server, you can configure a `client that supports MCP servers` to it. You can find a list of clients [here](https://modelcontextprotocol.io/clients).

As an example, we'll show how to connect the rudof MCP server to `Claude Desktop`, allowing you to interact with your RDF data directly through Claude.

To do this, ensure the following prerequisites are met:

### Prerequisites

- `rudof MCP server` is running locally (for example, using `rudof mcp`).

> ⚠️ **IMPORTANT** `PlantUML` must be installed before starting the MCP server (see warning above in `Exporting rudof as an MCP server
 section`).

- [Node.js](https://nodejs.org/es/download) is installed on your system (required to use the `mcp-remote` command).

- [Claude Desktop](https://claude.com/download) is installed and configured on your machine.


### Claude Desktop’s Configuration

In Claude Desktop’s configuration file add the following entry under your MCP servers section:

```sh
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

This configuration tells Claude Desktop to connect to your local `rudof MCP server` over HTTP.

Once saved, restart Claude Desktop. You should now be able to query and interact with your RDF data through Claude using `rudof’s MCP tools`.

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
