"""Validate inline RDF data against an inline ShEx schema and ShapeMap."""

from pyrudof import RDFFormat, Rudof, ShapeMapFormat, ShExFormat

SCHEMA = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string
}
"""

DATA = """
PREFIX : <http://example.org/>

:alice :name "Alice" .
"""

SHAPEMAP = ":alice@:Person"


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shex(SCHEMA, ShExFormat.ShExC)
        rudof.read_data(DATA, RDFFormat.Turtle)
        rudof.read_shapemap(SHAPEMAP, ShapeMapFormat.Compact)

        report = rudof.validate_shex()

        print(f"conforms: {report.conforms}")
        print(f"entries: {len(report)}")
        for entry in report:
            print(f"{entry.node} @ {entry.shape}: {entry.status}")


if __name__ == "__main__":
    main()
