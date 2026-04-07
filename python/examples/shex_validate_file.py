from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ShapeMapFormat

rudof = Rudof(RudofConfig())

rudof.read_shex("person.shex", ShExFormat.ShExC)
rudof.read_data("person.ttl", RDFFormat.Turtle)
rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)

rudof.validate_shex()

print("SHEX_FILE_VALIDATION_OK")
