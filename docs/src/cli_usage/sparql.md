# SPARQL

`rudof` has some basic support for SPARQL queries, which can be interacted with by means of the `query` command.
It is possible not only to perform SPARQL queries on SPARQL endpoints, but also on local files.

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
?person        ?name          ?status        
     :b        "Bob"          :Waiting
     :a        "Alice"        :Active
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
?person        ?birth_place   ?label         ?birth_place_name
 wd:Q517764         wd:Q16  "Rob Pike"@en          "Canada"@en  
 wd:Q518834         wd:Q16  "André Gingras"@en     "Canada"@en  
 wd:Q535692         wd:Q16  "Michael Reed"@en      "Canada"@en  
 wd:Q574507         wd:Q16  "Elza Kephart"@en      "Canada"@en  
 wd:Q579599         wd:Q16  "Isaiah L. Kenen"@en   "Canada"@en  
 wd:Q601125         wd:Q16  "Jack Wetherall"@en    "Canada"@en  
 wd:Q679597         wd:Q16  "Moishe Postone"@en    "Canada"@en  
 wd:Q2896159        wd:Q16  "Benjamin Lumley"@en   "Canada"@en  
 wd:Q2926838        wd:Q16  "Bruno Hébert"@en      "Canada"@en  
 wd:Q2938960        wd:Q16  "Carl Marotte"@en      "Canada"@en 
```

## Registered endpoints

`rudof` contains a list of registered endpoints like wikidata that can be invoked by their name.
