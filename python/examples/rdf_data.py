from pyrudof import Rudof, RudofConfig, RDFFormat 

data_str = """prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix : <http://example.org/>

:alice :name "Alice" ; 
  :birthdate "1980-03-02"^^xsd:date ; 
  :enrolledIn :cs101 ;
  :knows :bob .

:bob :name "Robert" ; 
  :birthdate "1981-03-02"^^xsd:date ; 
  :enrolledIn :cs101 ;
  :knows :alice .

:cs101 :name "Computer Science 101"; 
  :student :alice, :bob .
"""
rudof = Rudof(RudofConfig())

rudof.read_data_str(data_str)

result = rudof.serialize_data(format = RDFFormat.NTriples)

print(result)