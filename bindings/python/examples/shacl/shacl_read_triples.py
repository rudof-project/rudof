"""Count and serialize the triples a SHACL validation read."""

from pyrudof import RDFFormat, Rudof, ShaclFormat

SHAPES = """
PREFIX : <http://example.org/>
PREFIX sh: <http://www.w3.org/ns/shacl#>

:PersonShape a sh:NodeShape ;
  sh:targetNode :alice ;
  sh:property [ sh:path :name ; sh:minCount 1 ] .
"""

DATA = """
PREFIX : <http://example.org/>

:alice :name "Alice" ; :age 30 .
:carol :name "Carol" .
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shacl(SHAPES, ShaclFormat.Turtle)
        rudof.read_data(DATA, RDFFormat.Turtle)

        report = rudof.validate_shacl(read_triples=True)

        print(f"conforms: {report.conforms}")
        print(f"triples read: {report.triples_read}")
        print(f"distinct: {report.distinct_triples_read}")
        print(report.serialize_read_triples(RDFFormat.NTriples))


if __name__ == "__main__":
    main()
