"""Count and serialize the triples a ShEx validation read."""

from pyrudof import RDFFormat, Rudof, ShapeMapFormat, ShExFormat

SCHEMA = """
PREFIX : <http://example.org/>

:Person { :name . ; ^:knows . * }
"""

DATA = """
PREFIX : <http://example.org/>

:alice :name "Alice" .
:bob :knows :alice .
:carol :name "Carol" .
"""

SHAPEMAP = ":alice@:Person"


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shex(SCHEMA, ShExFormat.ShExC)
        rudof.read_data(DATA, RDFFormat.Turtle)
        rudof.read_shapemap(SHAPEMAP, ShapeMapFormat.Compact)

        report = rudof.validate_shex(read_triples=True)

        print(f"conforms: {report.conforms}")
        print(f"triples read: {report.triples_read}")
        print(f"distinct: {report.distinct_triples_read}")
        print(report.serialize_read_triples(RDFFormat.NTriples))


if __name__ == "__main__":
    main()
