"""Fetch RDF data over HTTP(S) and merge it into the current graph.

Network-dependent: skipped by default in the test suite (see examples.toml).
"""
from pyrudof import ReaderMode, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.dereference("https://www.w3.org/People/Berners-Lee/card", ReaderMode.Lax)
print(rudof.serialize_data())
