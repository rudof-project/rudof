"""Extract the SHACL shapes graph from the currently loaded RDF data, then validate.

Calling ``read_shacl()`` with no input tells rudof to take the shapes from the
data already in the session, which is how a single file holding both shapes and
instances is validated against itself.
"""

from pyrudof import RDFFormat, Rudof, ShaclValidationMode

SHAPES_AND_DATA = """
PREFIX : <http://example.org/>
PREFIX sh: <http://www.w3.org/ns/shacl#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:property [
    sh:path :name ;
    sh:datatype xsd:string ;
    sh:minCount 1
  ] .

:alice a :Person ;
  :name "Alice" .
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(SHAPES_AND_DATA, RDFFormat.Turtle)
        rudof.read_shacl()  # no input: the shapes come from the loaded data

        report = rudof.validate_shacl(ShaclValidationMode.Native)

        print(f"conforms: {report.conforms}")
        print(f"violations: {len(report)}")


if __name__ == "__main__":
    main()
