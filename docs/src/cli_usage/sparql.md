# SPARQL

`rudof` has some basic support for SPARQL queries, which can be interacted with by means of the `sparql` and `query` commands.
It is possible not only to perform SPARQL queries on SPARQL endpoints, but also on local files.

## Showing a loaded query

`sparql` is the read-only counterpart to `query` — it loads a SPARQL query and prints it back without running it, the same way `shex`/`shacl` show a loaded schema instead of validating with it.

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

## Querying over local files

For querying over local files you need both a SPARQL file and a data graph.
The idea is that we will apply the SPARQL query over the provided graph.
For us to do so you just need to use the `query` command.

Let's assume you have the following content in a file called `user.sparql`:

```sparql
prefix : <http://example.org/>
prefix schema: <http://schema.org/>

select ?person ?name ?status where {
  ?person schema:name ?name ;
          :status ?status .
}
```

and the following content in a file called `user.ttl`:

```turtle
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

Then you can run the instruction below.

```sh
rudof query -q user.sparql user.ttl
╭───┬─────────┬─────────┬──────────╮
│   │ ?person │ ?name   │ ?status  │
├───┼─────────┼─────────┼──────────┤
│ 1 │ :b      │ "Bob"   │ :Waiting │
├───┼─────────┼─────────┼──────────┤
│ 2 │ :a      │ "Alice" │ :Active  │
╰───┴─────────┴─────────┴──────────╯
```

## Querying over SPARQL endpoints

It is also possible to run a SPARQL query that obtains its data from a SPARQL endpoint.
However, the idea is the exact same as in the case above, with the difference that the SPARQL endpoint should be provided by means of the `-e` argument.

Assuming you have the following content in a file called `wikidata_query.sparql`:

```sparql
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX wd: <http://www.wikidata.org/entity/>
select * where {
  ?person wdt:P31 wd:Q5 ;
          wdt:P19 ?birth_place ;
       rdfs:label ?label .
  ?birth_place rdfs:label ?birth_place_name
  Filter(Lang(?label)='en' && Lang(?birth_place_name)='en')
} limit 10
```

You can run the following command to run the SPARQL query:

```sh
rudof query -q wikidata.sparql -e wikidata
╭────┬──────────────┬───────────────────┬───────────────────────┬─────────────╮
│    │ ?birth_place │ ?birth_place_name │ ?label                │ ?person     │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 1  │ wd:Q16       │ "Canada"@en       │ "Barry Pederson"@en   │ wd:Q358189  │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 2  │ wd:Q16       │ "Canada"@en       │ "Reuben Epp"@en       │ wd:Q360020  │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 3  │ wd:Q16       │ "Canada"@en       │ "Wendy McElroy"@en    │ wd:Q434376  │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 4  │ wd:Q16       │ "Canada"@en       │ "Terri Tatchell"@en   │ wd:Q435705  │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 5  │ wd:Q16       │ "Canada"@en       │ "John Ince"@en        │ wd:Q5822999 │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 6  │ wd:Q16       │ "Canada"@en       │ "Harry Strachan"@en   │ wd:Q5833843 │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 7  │ wd:Q16       │ "Canada"@en       │ "Howard M. Resh"@en   │ wd:Q5920263 │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 8  │ wd:Q16       │ "Canada"@en       │ "Hulda Crooks"@en     │ wd:Q5935600 │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 9  │ wd:Q16       │ "Canada"@en       │ "Huntly D. Millar"@en │ wd:Q5945697 │
├────┼──────────────┼───────────────────┼───────────────────────┼─────────────┤
│ 10 │ wd:Q16       │ "Canada"@en       │ "Graeme Manson"@en    │ wd:Q5975317 │
╰────┴──────────────┴───────────────────┴───────────────────────┴─────────────╯
```

## Registered endpoints

`rudof` registers `Wikidata`, `DBpedia` and `UniProt` by default; `-e <NAME>` looks them up by name instead of a full URL, matched case-insensitively (`wikidata`/`Wikidata`/`WikiData` all work). Run `rudof config` to see the full list, including any endpoints added in your own TOML config. Inside `rudof shell`, `endpoint <NAME>` activates one for the rest of the session, see [shell](./shell.md).

## Selecting the RDF backend

By default, local files are loaded into an in-process `memory` graph. Use `--backend` to load data into a QLever Docker container or to route queries to a remote SPARQL endpoint:

```sh
rudof query -q my.sparql --backend qlever data.ttl
rudof query -q my.sparql --endpoint https://my.sparql.server/sparql
```

See the [RDF backend (`--backend`) reference](./backend.md) for full documentation.
