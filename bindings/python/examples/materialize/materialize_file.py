"""Materialize an RDF graph from a ShExJ schema file and a MapState file.

Passing ``node`` pins the root subject the materialized triples hang off.
"""

from pathlib import Path

from pyrudof import ResultDataFormat, Rudof, ShExFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shex(HERE / "person_map.shexj", ShExFormat.ShExJ)
        rudof.read_map_state(HERE / "person_map_state.json")

        result = rudof.materialize(
            format=ResultDataFormat.Turtle,
            node="http://example.org/Alice",
        )
        print(result)


if __name__ == "__main__":
    main()
