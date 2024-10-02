# SPARQL

Rudof has some basic support for SPARQL queries.

The `query` command can be used to run SPARQL queries on RDF data.

```sh
rudof query -q examples/user.sparql examples/user.ttl
?person    ?name   ?status        
     :b    "Bob"  :Waiting
     :a  "Alice"   :Active
```

It is also possible to run a SPARQL query that obtains its data from a SPARQL endpoint. For example, the following command:

```sh
$ rudof query -q examples/wikidata.sparql -e wikidata
?person        ?birth_place               ?label   ?birth_place_name
  wd:Q448941         wd:Q16      "Lucy Walker"@en        "Canada"@en  
  wd:Q450015         wd:Q16     "Makyla Smith"@en        "Canada"@en  
  wd:Q495818         wd:Q16  "Andreas Apergis"@en        "Canada"@en  
  wd:Q517764         wd:Q16         "Rob Pike"@en        "Canada"@en  
 wd:Q2926838         wd:Q16     "Bruno Hébert"@en        "Canada"@en  
 wd:Q2938960         wd:Q16     "Carl Marotte"@en        "Canada"@en  
 wd:Q2978165         wd:Q16    "Claude Soucie"@en        "Canada"@en  
 wd:Q3014451         wd:Q16   "Daniel Meurois"@en        "Canada"@en  
 wd:Q3015785         wd:Q16     "Dany Gelinas"@en        "Canada"@en  
 wd:Q3018685         wd:Q16      "David Rigby"@en        "Canada"@en  
```
