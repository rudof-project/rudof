from pyrudof import Rudof, RudofConfig, RDFFormat 
config = RudofConfig()
rudof = Rudof(config)

print(config)
data_str = """
prefix : <http://example.org/>
:a :name "Alice" .
"""
data_str1 = """prefix xsd: <http://www.w3.org/2001/XMLSchema#>
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

rudof.read_data_str(data_str)

print("RDF Data in PlantUML format:")
uml = rudof.data2plantuml()
print("Finished conversion to UML.")
print(uml)