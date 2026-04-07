from pyrudof import ShaclFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
serialized = rudof.serialize_shacl()

print("SHACL_SERIALIZE_OK")
print(f"Serialized chars: {len(serialized)}")
