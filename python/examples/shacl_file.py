from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.read_shacl("examples/timbl_shapes.ttl")
rudof.read_data("examples/timbl.ttl")
result = rudof.validate_shacl()
print(result.show())
