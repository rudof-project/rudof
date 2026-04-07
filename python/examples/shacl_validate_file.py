from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
rudof.read_data("timbl.ttl", RDFFormat.Turtle)
rudof.validate_shacl(ShaclValidationMode.Native)

print("SHACL_FILE_VALIDATION_OK")
