"""Read and serialize a SPARQL service description."""

from pyrudof import RDFFormat, ReaderMode, Rudof, ServiceDescriptionFormat

SERVICE_TTL = """
@prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
@prefix : <http://example.org/> .

:svc a sd:Service ;
  sd:endpoint <http://example.org/sparql> ;
  sd:feature sd:BasicFederatedQuery ;
  sd:defaultDataset [ a sd:Dataset ] .
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_service_description(SERVICE_TTL, RDFFormat.Turtle, None, ReaderMode.Lax)

        as_internal = rudof.serialize_service_description(ServiceDescriptionFormat.Internal)
        print(as_internal)

        as_json = rudof.serialize_service_description(ServiceDescriptionFormat.Json)
        print(f"json mentions the endpoint: {'example.org/sparql' in as_json}")


if __name__ == "__main__":
    main()
