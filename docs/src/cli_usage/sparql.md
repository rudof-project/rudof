# SPARQL: show a loaded query

`rudof sparql` loads a SPARQL query and prints it back without running it — the read-only counterpart to [`query`](./query.md), the same way `shex`/`shacl` show a loaded schema instead of validating with it.

```sh
rudof sparql -q user.sparql
```

```sparql
prefix : <http://example.org/>
prefix schema: <http://schema.org/>

select ?person ?name ?status where {
  ?person schema:name ?name ;
          :status ?status .
}
```

With no `-q`, it re-shows whatever query is currently loaded, instead of erroring — useful in `rudof shell`, where session state persists across commands. `sparql` and `query` load into the same state, so a query loaded by one is visible to the other: `sparql -q FILE` loads and shows it, and a later bare `query` (no `-q`) runs that same loaded query.

## Related commands

- [`rudof query`](./query.md): run the loaded (or a freshly given) SPARQL query, over local files or a remote endpoint
