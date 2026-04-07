from pyrudof import RDFFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

schema = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string
}
"""

data = """
PREFIX : <http://example.org/>

:alice :name "Alice" .
"""

shapemap = ":alice@:Person"

rudof.read_shex(schema, ShExFormat.ShExC)
rudof.read_data(data, RDFFormat.Turtle)
rudof.read_shapemap(shapemap, ShapeMapFormat.Compact)
rudof.validate_shex()

print("SHEX_INLINE_VALIDATION_OK")
