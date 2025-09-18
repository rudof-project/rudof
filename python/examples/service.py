from pyrudof import Rudof, RudofConfig, RDFFormat, ReaderMode

rudof = Rudof(RudofConfig())

service = "https://sparql.uniprot.org/sparql"

rudof.read_service_description_url(service, RDFFormat.Turtle, None, ReaderMode.Strict)
service = rudof.get_service_description()

print(f"Service description:\n{service}")

mie = service.as_mie()
print(f"Service description in MIE format as YAML:\n{mie.as_yaml()}")