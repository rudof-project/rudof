"""Demonstrate catching RudofError exceptions."""
from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

# Trying to parse invalid NTriples raises a ValueError (wrapping RudofError).
try:
    rudof.read_data("this is not valid RDF at all!!!", RDFFormat.NTriples)
except Exception as e:
    msg = str(e)
    print(f"Caught RudofError: {msg[:60]}")

# A second attempt with valid data succeeds normally.
rudof2 = Rudof(RudofConfig())
rudof2.read_data(
    "<http://example.org/alice> <http://example.org/name> \"Alice\" .\n",
    RDFFormat.NTriples,
)
