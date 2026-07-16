from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

shapes = """
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
"""

data = """
PREFIX : <http://example.org/>

:alice a :Person ;
  :name "Alice" .
"""

rudof.read_shacl(shapes, ShaclFormat.Turtle)
rudof.read_data(data, RDFFormat.Turtle)
rudof.validate_shacl(ShaclValidationMode.Native)
