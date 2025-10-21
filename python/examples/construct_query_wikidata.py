from pyrudof import Rudof, RudofConfig, RDFFormat, QueryResultFormat 

endpoint = "https://query.wikidata.org/sparql"

sparql_query = """
PREFIX wd:  <http://www.wikidata.org/entity/>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX :    <http://example.org/>

CONSTRUCT { 
   ?p a     :Person ;
      :name ?person ;
      :occupation ?occupation
} WHERE {
    ?p wdt:P31 wd:Q5 ;   
          wdt:P106 ?o ;
          rdfs:label ?person ;
          wdt:P19 wd:Q14317 .
  ?o rdfs:label ?occupation 
  FILTER (lang(?person) = "en" && lang(?occupation) = "en")   
}
LIMIT 10
"""
rudof = Rudof(RudofConfig())
rudof.use_endpoint(endpoint)

result = rudof.run_query_construct_str(sparql_query, QueryResultFormat.Turtle)

print(result)