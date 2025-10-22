from pyrudof import Rudof, RudofConfig, RDFFormat 

query = """
PREFIX wd: <http://www.wikidata.org/entity/>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
SELECT ?person ?occupation WHERE {
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
rudof.use_endpoint("wikidata")
results = rudof.run_query_str(query)
print(results.show())