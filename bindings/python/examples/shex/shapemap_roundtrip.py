from pyrudof import RDFFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data("person.ttl", RDFFormat.Turtle)
rudof.read_shex("person.shex", ShExFormat.ShExC)
rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
serialized = rudof.serialize_shapemap()
