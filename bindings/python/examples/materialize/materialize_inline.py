"""Materialize an RDF graph from an inline ShEx schema and an inline MapState.

The MapState maps each Map-extension IRI declared by a semantic action to the
concrete RDF node it stands for. IRI nodes are written ``{"Iri": "<iri>"}``.
"""

import json
from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import ResultDataFormat, Rudof, ShExFormat

SCHEMA = json.dumps(
    {
        "@context": "http://www.w3.org/ns/shex.jsonld",
        "type": "Schema",
        "shapes": [
            {
                "type": "ShapeDecl",
                "id": "http://example.org/PersonShape",
                "shapeExpr": {
                    "type": "Shape",
                    "expression": {
                        "type": "TripleConstraint",
                        "predicate": "http://example.org/name",
                        "semActs": [
                            {
                                "type": "SemAct",
                                "name": "http://shex.io/extensions/Map/",
                                "code": "<http://example.org/name>",
                            }
                        ],
                    },
                },
            }
        ],
    }
)

MAP_STATE = {"http://example.org/name": {"Iri": "http://example.org/Alice"}}


def main() -> None:
    with TemporaryDirectory() as tmpdir:
        # read_map_state takes a path, so the state is written out first.
        map_state_path = Path(tmpdir) / "map_state.json"
        map_state_path.write_text(json.dumps(MAP_STATE), encoding="utf-8")

        with Rudof() as rudof:
            rudof.read_shex(SCHEMA, ShExFormat.ShExJ)
            rudof.read_map_state(map_state_path)

            result = rudof.materialize(ResultDataFormat.NTriples)
            print(result)


if __name__ == "__main__":
    main()
