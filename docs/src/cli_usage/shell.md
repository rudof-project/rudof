# shell

`rudof shell` starts an interactive REPL. Each line is one of the regular `rudof` subcommands, without the leading `rudof`. State loaded by one command (RDF data, a schema, a shapemap, an active endpoint, ...) stays loaded for the next command in the same session.

## Starting

```sh
rudof shell
```

```
                 _        ___
                | |      / __)
  ____ _   _  _ | | ___ | |__
 / ___) | | |/ || |/ _ \|  __)
| |   | |_| ( (_| | |_| | |
|_|    \____|\____|\___/|_|
Type 'help' for available commands, 'exit' to quit.
rudof>
```

## Running commands

Same syntax as the CLI, minus the `rudof` prefix:

```
rudof> data examples/user.ttl
```

## State persists across commands

Load data and a schema in separate lines, then validate without repeating either:

```
rudof> data examples/user.ttl
rudof> shex examples/user.shex
rudof> shex-validate -n :a -l :User
╭──────┬───────┬────────┬───────────────────────╮
│ Node │ Shape │ Status │ Details               │
├──────┼───────┼────────┼───────────────────────┤
│ :a   │ :User │ OK     │ Shape passed :a@:User │
╰──────┴───────┴────────┴───────────────────────╯
```

A bare command with no arguments shows whatever is currently loaded, instead of erroring:

```
rudof> data
```

## Bare resource shorthand

For `shex`, `shacl`, `shapemap`, `pgschema`, `dctap`, `service`, `materialize`, `generate`, `rdf-config`, a single argument with no other flags is shorthand for that command's `-s` flag. `node` works the same way with `-n`. This only applies inside the shell.

```
rudof> shex examples/user.shex
```

is the same as:

```
rudof> shex -s examples/user.shex
```

```
rudof> node :a
```

is the same as:

```
rudof> node -n :a
```

## Redirecting one command's output

`-o`/`--output-file` on a single line writes only that command's output to a file, the rest of the session still prints to the terminal:

```
rudof> data examples/user.ttl -o out.ttl
Output saved in out.ttl
```

## Selecting a SPARQL endpoint

`endpoint` shows the endpoint activated in the session, if any:

```
rudof> endpoint
No endpoint is currently active.
Use 'endpoint NAME' to activate one.
Registered endpoints: dbpedia, uniprot, wikidata
```

`endpoint NAME` activates one of the endpoints registered in the [TOML config](../general/configuration.md) for the rest of the session:

```
rudof> endpoint wikidata
Active endpoint: wikidata (https://query.wikidata.org/sparql)
```

Commands that query RDF data reuse it without needing `--endpoint`/`-e`:

```
rudof> query -q "select ?label where { <http://www.wikidata.org/entity/Q80> <http://www.w3.org/2000/01/rdf-schema#label> ?label . filter(lang(?label) = 'en') } limit 1"
╭───┬──────────────────────╮
│   │ ?label               │
├───┼──────────────────────┤
│ 1 │ "Tim Berners-Lee"@en │
╰───┴──────────────────────╯
```

See the [RDF backend (`--backend`) reference](./backend.md) for how named endpoints are registered.

## Clearing session state

`reset` (no argument, or `reset all`) clears every piece of session state and starts fresh:

```
rudof> data examples/user.ttl
rudof> shex examples/user.shex
rudof> reset
Reset all session state.
rudof> data
Error: Data error: No data loaded: No data loaded
```

`reset TARGET` clears only one piece of state, leaving the rest of the session untouched. Targets: `data`, `shex`, `shacl`, `pgschema`, `shapemap`, `dctap`, `service`, `query`, `typemap`, `rdf-config`, `endpoint`. Give more than one to clear several at once:

```
rudof> shex examples/user.shex
rudof> reset shex
Reset: shex.
rudof> shex
Error: ShEx error: No ShEx schema loaded: No ShEx schema loaded
```

Data loaded earlier in the same session is untouched by `reset shex`. Give more than one target to clear several at once, e.g. `reset data shex`.

## Running system shell commands

A line starting with `!` runs the rest of the line in the system shell:

```
rudof> !ls examples
```

## Built-in commands

| Command | Effect |
|---|---|
| `help`, `?` | List the available subcommands |
| `exit`, `quit` | Exit the shell |
| `!<command>` | Run `<command>` in the system shell |
| `endpoint [NAME]` | Show the active endpoint, or activate a registered one |
| `reset [TARGET...]` | Clear session state, one or more targets, or everything with no argument |

## History and completion

Lines are saved to `~/.rudof_history` between sessions. Tab completes subcommand names, and falls back to filenames after the first word.

## Exiting

`exit`, `quit`, or Ctrl-D.
