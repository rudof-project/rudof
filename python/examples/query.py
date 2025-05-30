from pyrudof import Rudof, RudofConfig, QuerySolutions

rudof = Rudof(RudofConfig())
rdf = """prefix : <http://example.org/>
:alice a :Person ;
 :name "Alice"   ;
 :knows :bob     .
:bob a :Person   ;
 :name "Robert"  . 
"""
rudof.read_data_str(rdf)

query = """prefix : <http://example.org/>
select * where { 
  ?x a :Person 
}
"""

results = rudof.run_query_str(query)
for result in iter(results):
    print(result.show())
