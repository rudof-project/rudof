from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

'''
rudof.read_data_str("""
prefix : <http://example.org/>
prefix sh:     <http://www.w3.org/ns/shacl#> 
prefix xsd:    <http://www.w3.org/2001/XMLSchema#> 
prefix rdfs:   <http://www.w3.org/2000/01/rdf-schema#>

:timbl rdfs:label "Tim Berners Lee" ;
       :employer :CERN {| :start "1984" ;
                          :end   "1994" |}
                       {| :start "1980" ;
                          :end   "1980" |} ;
       :award :PA {| :time "2002" ;
                     :togetherWith :vint |} .
:vint  rdfs:label "Vinton Cerf" .
""")
'''

rudof.read_data_str("""
prefix : <http://example.org/>
prefix sh:     <http://www.w3.org/ns/shacl#> 
prefix xsd:    <http://www.w3.org/2001/XMLSchema#> 
prefix rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
prefix rdf:    <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                      
:timbl rdfs:label "Tim Berners Lee" .
_:r rdf:reifies   <<( :timbl :employer :CERN )>> ;
    :start        "1984" ;
    :end          "1994" .
_:s rdf:reifies   <<( :timbl :employer :CERN )>> ;
    :start        "1980" ;
    :end          "1980" .
_:t rdf:reifies   <<( :timbl :employer :CERN )>> ;
    :time         "2002" ;
    :togetherWith :vint  .
:vint rdfs:label  "Vinton Cerf" .
""")

results = rudof.run_query_str("""
prefix : <http://example.org/>
prefix rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
prefix rdf:    <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?person ?employer ?start ?end WHERE {
  ?r rdf:reifies <<( ?person :employer ?employer )>> ;
     :start ?start ;
     :end ?end .
 }
""")
print(results.show())
# print(rudof.node_info(":timbl"))

