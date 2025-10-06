from pyrudof import Rudof, RudofConfig, RDFFormat, ReaderMode, ServiceDescriptionFormat

rudof = Rudof(RudofConfig())

service = "https://sparql.uniprot.org/sparql"

rudof.read_service_description_url(service, RDFFormat.Turtle, None, ReaderMode.Strict)
service = rudof.get_service_description()
service_str = service.serialize(ServiceDescriptionFormat.Json)
print(f"Service description in JSON:\n{service_str}")

# Converting service description to MIE
mie = service.as_mie()
print(f"Service description in MIE format as YAML:\n{mie.as_yaml()}")