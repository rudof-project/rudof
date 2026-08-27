"""Demonstrates the distinction between the narrow "clear the schema only"
resets and the broader "reset validation" resets, which also unload the
schema/shapes graph along with the validation results and ShapeMap.
"""
from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, ShExFormat, ShapeMapFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

# --- ShEx: reset_shex_schema() clears only the schema ---
rudof.read_shex("person.shex", ShExFormat.ShExC)
rudof.read_data(
    'PREFIX : <http://example.org/>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n:alice :name "Alice" ; :age 30 .',
    RDFFormat.Turtle,
)
rudof.read_shapemap(":alice@:Person", ShapeMapFormat.Compact)
rudof.validate_shex()

# reset_shex() is the broad reset: schema, ShapeMap, validator and results all go
rudof.reset_shex()
try:
    rudof.serialize_current_shex()
    print("BUG: ShEx schema survived reset_shex()")
except ValueError:
    print("reset_shex() cleared the schema as documented")

# --- SHACL: reset_shacl() clears only the shapes graph, leaving results alone ---
shapes = """
PREFIX : <http://example.org/>
PREFIX sh: <http://www.w3.org/ns/shacl#>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person .
"""
rudof.reset_all()
rudof.read_shacl(shapes, ShaclFormat.Turtle)
rudof.read_data("PREFIX : <http://example.org/>\n:alice a :Person .", RDFFormat.Turtle)
rudof.validate_shacl(ShaclValidationMode.Native)

rudof.reset_shacl()
try:
    rudof.serialize_shacl()
    print("BUG: SHACL shapes survived reset_shacl()")
except ValueError:
    print("reset_shacl() cleared the shapes as documented")

# reset_shacl_validation() is the broad reset: shapes and validation results both go
rudof.reset_all()
rudof.read_shacl(shapes, ShaclFormat.Turtle)
rudof.read_data("PREFIX : <http://example.org/>\n:alice a :Person .", RDFFormat.Turtle)
rudof.validate_shacl(ShaclValidationMode.Native)

rudof.reset_shacl_validation()
try:
    rudof.serialize_shacl()
    print("BUG: SHACL shapes survived reset_shacl_validation()")
except ValueError:
    print("reset_shacl_validation() cleared the shapes as documented")
