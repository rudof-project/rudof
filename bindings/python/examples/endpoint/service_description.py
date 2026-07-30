from pyrudof import RDFFormat, ReaderMode, Rudof, RudofConfig, ServiceDescriptionFormat

rudof = Rudof(RudofConfig())

service_ttl = """
@prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
@prefix : <http://example.org/> .

:svc a sd:Service ;
  sd:endpoint <http://example.org/sparql> ;
  sd:feature sd:BasicFederatedQuery ;
  sd:defaultDataset [ a sd:Dataset ] .
"""

rudof.read_service_description(service_ttl, RDFFormat.Turtle, None, ReaderMode.Lax)
as_json = rudof.serialize_service_description(ServiceDescriptionFormat.Json)
as_internal = rudof.serialize_service_description(ServiceDescriptionFormat.Internal)

