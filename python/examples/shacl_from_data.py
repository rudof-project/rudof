from pyrudof import RDFFormat, ShaclValidationMode, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

shapes_and_data = """
PREFIX : <http://example.org/>
PREFIX sh: <http://www.w3.org/ns/shacl#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:property [
    sh:path :name ;
    sh:datatype xsd:string ;
    sh:minCount 1
  ] .

:alice a :Person ;
  :name "Alice" .
"""

rudof.read_data(shapes_and_data, RDFFormat.Turtle)
rudof.read_shacl()
rudof.validate_shacl(ShaclValidationMode.Native)

print("SHACL_FROM_DATA_OK")
