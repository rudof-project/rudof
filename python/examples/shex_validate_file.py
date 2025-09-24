from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ReaderMode, ShapeMapFormat

rudof = Rudof(RudofConfig())

rudof.read_shex("examples/person.shex", ShExFormat.ShExC)
rudof.read_data("examples/person.ttl", RDFFormat.Turtle)
rudof.read_shapemap("examples/person.sm", ShapeMapFormat.Compact)

result = rudof.validate_shex()

print(result.show())
