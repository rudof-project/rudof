"""Materialize an RDF graph from an inline ShEx schema and inline MapState.

The MapState is a dict that maps Map-extension IRI keys to RDF node values.
IRI nodes are represented as ``{"Iri": "<iri-string>"}``.
"""
import json
import os
import tempfile

from pyrudof import ResultDataFormat, Rudof, RudofConfig, ShExFormat

rudof = Rudof(RudofConfig())

# ShEx schema (ShExJ) with Map semantic actions on each triple constraint
schema = json.dumps({
    "@context": "http://www.w3.org/ns/shex.jsonld",
    "type": "Schema",
    "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/PersonShape",
        "shapeExpr": {
            "type": "Shape",
            "expression": {
                "type": "TripleConstraint",
                "predicate": "http://example.org/name",
                "semActs": [{
                    "type": "SemAct",
                    "name": "http://shex.io/extensions/Map/",
                    "code": "<http://example.org/name>"
                }]
            }
        }
    }]
})

# MapState: maps each Map-extension IRI to its concrete RDF node value
map_state = {
    "http://example.org/name": {"Iri": "http://example.org/Alice"}
}

rudof.read_shex(schema, ShExFormat.ShExJ)

# read_map_state requires a file path, so write to a temporary file
with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as tmp:
    json.dump(map_state, tmp)
    map_state_path = tmp.name

try:
    rudof.read_map_state(map_state_path)
    result = rudof.materialize(ResultDataFormat.NTriples)
    print(result)
finally:
    os.unlink(map_state_path)
