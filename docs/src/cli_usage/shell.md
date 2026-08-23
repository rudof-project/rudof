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
11 triple(s) loaded
```

For `data`, `shex`, `shacl`, `dctap`, `pgschema` and `service` — the commands that load a resource and, by default, dump it in full — the shell instead prints a short stats line like this when the line loads something new. This keeps a session that chains several such commands readable, since the full dump mostly just scrolls past. The full dump is still available: give `-o`/`--output-file` to write it to a file (see [Redirecting one command's output](#redirecting-one-commands-output)), or call the command bare afterwards to show what's currently loaded in full (see below).

A command that hits an internal bug reports it as an ordinary error (`Error: ...`) rather than crashing the session — the shell keeps running afterwards, with whatever was already loaded still intact.

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
11 triple(s) loaded
rudof> shex examples/user.shex
1 shape(s) loaded
rudof> shex-validate -n :a -l :User
╭──────┬───────┬────────┬───────────────────────╮
│ Node │ Shape │ Status │ Details               │
├──────┼───────┼────────┼───────────────────────┤
│ :a   │ :User │ OK     │ Shape passed :a@:User │
╰──────┴───────┴────────┴───────────────────────╯
```

A bare command with no arguments shows whatever is currently loaded, in full, instead of erroring:

```
rudof> data
@prefix : <http://example.org/> .
...
```

### Loading `data` again: replace, or merge

Unlike the top-level CLI (one `data` call per process), the shell can chain several `data` calls in the same session. A second `data FILE` **replaces** the RDF/PG data currently loaded, rather than merging into it — this is shell-only behavior, chosen because nothing about `data FILE` looks additive:

```
rudof> data examples/user.ttl
11 triple(s) loaded
rudof> data examples/simple.ttl
14 triple(s) loaded
rudof> data
@prefix ... # only examples/simple.ttl's data
```

Add `--merge` to merge into the currently loaded data instead:

```
rudof> data examples/user.ttl
11 triple(s) loaded
rudof> data examples/simple.ttl --merge
25 triple(s) loaded
rudof> data
@prefix ... # both files' data
```

`--merge` only exists in the shell, not in the top-level `rudof data` command.

## Bare resource shorthand

For `shex`, `shacl`, `shapemap`, `pgschema`, `dctap`, `service`, `materialize`, `generate`, `rdf-config`, a single argument with no other flags is shorthand for that command's `-s` flag. `node` works the same way with `-n`, and `sparql` with `-q`. This only applies inside the shell.

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
Registered endpoints: DBpedia, UniProt, Wikidata
```

`endpoint NAME` activates one of the endpoints registered in the [TOML config](../general/configuration.md) for the rest of the session. `NAME` is matched case-insensitively, so `wikidata`, `Wikidata` and `WikiData` all activate the same endpoint — but it's always reported back using its canonical, registered name:

```
rudof> endpoint wikidata
Active endpoint: Wikidata (https://query.wikidata.org/sparql)
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

### Registering a new endpoint from a file

`endpoint FILE.toml` registers a new endpoint from a local TOML file and activates it, instead of activating one already known by name. The file has the same shape as one `[rdf.endpoints.<name>]` table (see the [Config reference](../references/config.md#rdfendpointsname)) — `name`, `query_url`, an optional `update_url`, and an optional `[prefixmap]` — and is registered under its own `name` field:

```
rudof> endpoint my-endpoint.toml
Registered endpoint 'My Endpoint' from my-endpoint.toml
Active endpoint: My Endpoint (https://example.org/sparql)
```

This is session-only, like `config set` — it doesn't get written back to `rudof.toml`. Use `rudof config -o rudof.toml` to save the effective config (now including this endpoint) if you want it to persist.

The endpoints registered by default (`Wikidata`, `DBpedia`, `UniProt`) are themselves just files of this same shape, bundled into `rudof` at compile time from [`rudof_rdf/endpoints/`](https://github.com/rudof-project/rudof/tree/master/rudof_rdf/endpoints) — **every** `*.toml` file in that folder is registered automatically, so adding a new one to the defaults (or changing an existing one) is just adding or editing a file there and opening a pull request; no Rust code change needed.

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

If you want to add the default (empty) prefix, use:

```
rudof> prefixes add "" http://example.org/
Added prefix : <http://example.org/>
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
1 triple(s) loaded
```

### Resolving a resource argument from a prefix

A resource argument — `shex`/`shacl`/`dctap`/`pgschema`/`service`/`rdf-config`/`generate`/`materialize`'s `-s` (or its [bare shorthand](#bare-resource-shorthand)), `sparql`'s `-q`, `shapemap`'s `-m` — can be a prefixed name (`alias:local`) instead of a full path or URL. It's expanded against a known prefix before being loaded, the same way `es` is already bundled with the default Wikidata endpoint (pointing at `https://www.wikidata.org/wiki/Special:EntitySchemaText/`):

```
rudof> endpoint wikidata
Active endpoint: Wikidata (https://query.wikidata.org/sparql)
rudof> shex es:E10
3 shape(s) loaded
```

That fetched `https://www.wikidata.org/wiki/Special:EntitySchemaText/E10` — `es:E10` was expanded using the active endpoint's own prefixes. The session's [default prefixes](#default-prefixes) are checked first; the active endpoint's prefixes (if any endpoint is active) are checked next:

```
rudof> prefixes add ents https://www.wikidata.org/wiki/Special:EntitySchemaText/
Added prefix ents: <https://www.wikidata.org/wiki/Special:EntitySchemaText/>
rudof> shex ents:E10
3 shape(s) loaded
```

A value that's already a URL, an existing file path, `-`, or whose alias isn't registered anywhere is left alone and handled exactly as before. `node`'s identifier argument is deliberately **not** covered by this — it's an RDF term reference resolved by the loaded document's own prefixes, not a resource to fetch.

## Clearing session state

`reset` (no argument, or `reset all`) clears every piece of session state and starts fresh:

```
rudof> data examples/user.ttl
11 triple(s) loaded
rudof> shex examples/user.shex
1 shape(s) loaded
rudof> reset
Reset all session state.
rudof> data
Error: Data error: No data loaded: No data loaded
```

`reset TARGET` clears only one piece of state, leaving the rest of the session untouched. Targets: `data`, `shex`, `shacl`, `pgschema`, `shapemap`, `dctap`, `service`, `query`, `sparql`, `typemap`, `rdf-config`, `endpoint`. `sparql` clears just the loaded query text; `query` clears the loaded query *and* its results. Give more than one to clear several at once:

```
rudof> shex examples/user.shex
1 shape(s) loaded
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

Lines are saved to `~/.rudof_history` between sessions. Tab completes subcommand names; if several match, they're all listed on the first Tab (e.g. `sh` + Tab lists `shex`, `shacl`, `shapemap`, `shell`, ...) instead of silently filling in one. After the first word, Tab completes the `endpoint` command's argument against the endpoint names registered in the [TOML config](../general/configuration.md) — case-insensitively, so `wiki` + Tab matches `Wikidata` — and the `KEY` argument of `config get`/`config set` against every dotted key path in the effective config (see the [Config reference](../references/config.md)) — e.g. `config get shex_validator.` + Tab lists `shex_validator.check_negation`, `shex_validator.width`, `shex_validator.shapemap`, and so on. Every other argument falls back to filenames.

## Exiting

`exit`, `quit`, or Ctrl-D.
