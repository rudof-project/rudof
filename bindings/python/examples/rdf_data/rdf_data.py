"""Read RDF data, add more triples to it, then replace the graph outright.

Reading data merges into the graph already loaded — ``merge`` defaults to true.
Pass ``merge=False`` to replace the graph instead.
"""

from pathlib import Path

from pyrudof import RDFFormat, ResultDataFormat, Rudof

HERE = Path(__file__).resolve().parent.parent

EXTRA = """
prefix : <http://example.org/>
:extra :name "Extra" .
"""

REPLACEMENT = """
prefix : <http://example.org/>
:carol :name "Carol" .
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(input=HERE / "person.ttl", format=RDFFormat.Turtle)
        rudof.read_data(input=EXTRA, format=RDFFormat.Turtle)

        merged = rudof.serialize_data(ResultDataFormat.Turtle)
        print(f"merged has alice: {'alice' in merged}")
        print(f"merged has extra: {'extra' in merged}")

        rudof.read_data(input=REPLACEMENT, format=RDFFormat.Turtle, merge=False)

        replaced = rudof.serialize_data(ResultDataFormat.Turtle)
        print(f"replaced has carol: {'carol' in replaced}")
        print(f"replaced still has alice: {'alice' in replaced}")


if __name__ == "__main__":
    main()
