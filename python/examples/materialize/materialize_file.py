"""Materialize an RDF graph from ShEx schema and MapState files.

Demonstrates loading a ShExJ schema and a pre-built MapState JSON file, then
materializing the RDF graph with an explicit root subject IRI.
"""
from pyrudof import ResultDataFormat, Rudof, RudofConfig, ShExFormat

rudof = Rudof(RudofConfig())

rudof.read_shex("person_map.shexj", ShExFormat.ShExJ)
rudof.read_map_state("person_map_state.json")

result = rudof.materialize(
    format=ResultDataFormat.Turtle,
    node="http://example.org/Alice",
)
print(result)
