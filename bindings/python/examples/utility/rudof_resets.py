from pyrudof import RDFFormat, ShaclFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.read_data("person.ttl", RDFFormat.Turtle)
rudof.read_shex("person.shex", ShExFormat.ShExC)
rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
rudof.read_query("person.sparql")

rudof.reset_data()
rudof.reset_shex()
rudof.reset_shacl()
rudof.reset_shapemap()
rudof.reset_query()
rudof.reset_validation_results()
rudof.reset_all()
