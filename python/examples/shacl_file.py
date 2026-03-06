from pyrudof import Rudof, RudofConfig, ShaclFormat, RDFFormat

rudof = Rudof(RudofConfig())

rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
rudof.read_data("timbl.ttl", RDFFormat.Turtle)

result = rudof.validate_shacl()
print(result.show_as_table())
