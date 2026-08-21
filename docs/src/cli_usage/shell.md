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

## Multi-line input

If a line ends with an open quote, the shell doesn't submit it — it keeps reading further lines, shown with a `   ... ` continuation prompt, until the quote closes. This lets a multi-line value (e.g. a SPARQL query passed inline to `-q`) be typed directly at the prompt instead of requiring a separate file:

```
rudof> query -q 'SELECT ?s ?p ?o WHERE {
   ... ?s ?p ?o .
   ... }' examples/user.ttl
```

If input ends (Ctrl-D) while a quote is still open, the whole pending command is discarded with a warning instead of being misparsed.

Within a line, Ctrl-J inserts a literal newline instead of submitting. This matters when recalling a previous command from history (↑) that already has balanced quoting — pressing Enter would run it immediately, so Ctrl-J lets you keep extending it across more lines first. Alt-Enter does the same in terminals that pass it through, but some intercept it for their own use, so Ctrl-J is the one to rely on.

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

## Default prefixes

`prefixes` manages a list of default prefix declarations for the session. When `data` (Turtle), `shacl` (Turtle), `shex` (ShExC) or `query` (SPARQL) is loaded, any alias the source text uses but doesn't declare itself is resolved against these defaults instead of failing to parse. A document's own declarations always win — a default only fills in an alias the document doesn't declare. `node` and `shapemap` need no separate lookup: they resolve prefixed selectors against whatever prefixmap `data`/`shex` already ended up with, so they pick up the same resolution for free.

With no argument, `prefixes` shows the current default prefix declarations:

```
rudof> prefixes
No default prefixes are defined.
```

`prefixes add ALIAS IRI` adds one (overwriting any existing declaration for that alias):

```
rudof> prefixes add rdf http://www.w3.org/1999/02/22-rdf-syntax-ns#
Added prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
rudof> prefixes
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
```

`prefixes rm ALIAS` removes one:

```
rudof> prefixes rm rdf
Removed prefix rdf
```

`prefixes rename OLD NEW` renames an alias, keeping its IRI. `prefixes copy OLD NEW` adds a new alias for the same IRI, keeping the original alias too:

```
rudof> prefixes add rdf http://www.w3.org/1999/02/22-rdf-syntax-ns#
rudof> prefixes rename rdf rdf1
Renamed prefix rdf to rdf1
rudof> prefixes copy rdf1 rdf
Copied prefix rdf1 to rdf
```

For example, Turtle data using a `p:` alias it never declares would normally fail to parse; adding `p` as a default prefix first lets it resolve:

```
rudof> data "p:a p:name \"Alice\" ."
Error: Data error: Failed to parse RDF data from 'string' [...]: The prefix p: has not been declared
rudof> prefixes add p http://example.org/
Added prefix p: <http://example.org/>
rudof> data "p:a p:name \"Alice\" ."
@prefix p: <http://example.org/> .
p:a p:name "Alice" .
```

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
| `prefixes [add\|rm\|rename\|copy ...]` | Show, or manage, the default prefix declarations |

## History and completion

Lines are saved to `~/.rudof_history` between sessions. Tab completes subcommand names, and falls back to filenames after the first word.

## Exiting

`exit`, `quit`, or Ctrl-D.
