from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.read_data_str("""
prefix : <http://example.org/>
prefix sh:     <http://www.w3.org/ns/shacl#> 
prefix xsd:    <http://www.w3.org/2001/XMLSchema#> 
                    
:Person a sh:NodeShape;
   sh:targetNode :ok, :ko ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] .
""")

rudof.read_data_str("""
prefix : <http://example.org/>

:ok :name "alice" .                 
:ko :name 1 .
""", merge = True)

result = rudof.validate_shacl()

print(result.show_as_table())
