"""Validate inline RDF data against inline SHACL shapes.

``:bob`` has no ``:name``, so the report does not conform and carries one
violation describing exactly which constraint failed and where.
"""

from pyrudof import RDFFormat, Rudof, ShaclFormat, ShaclValidationMode

SHAPES = """
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
"""

DATA = """
PREFIX : <http://example.org/>

:alice a :Person ;
  :name "Alice" .

:bob a :Person .
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shacl(SHAPES, ShaclFormat.Turtle)
        rudof.read_data(DATA, RDFFormat.Turtle)

        report = rudof.validate_shacl(ShaclValidationMode.Native)

        print(f"conforms: {report.conforms}")
        print(f"violations: {len(report)}")
        for entry in report:
            print(f"{entry.severity} on {entry.focus_node} (path {entry.path})")
            print(f"  {entry.message}")


if __name__ == "__main__":
    main()
