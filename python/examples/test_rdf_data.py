import unittest
from pyrudof import Rudof, RudofConfig, RDFFormat, ReaderMode

DATA_STR = """prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix : <http://example.org/>

:alice :name "Alice" ;
  :birthdate "1980-03-02"^^xsd:date ;
  :enrolledIn :cs101 ;
  :knows :bob .

:bob :name "Robert" ;
  :birthdate "1981-03-02"^^xsd:date ;
  :enrolledIn :cs101 ;
  :knows :alice .

:cs101 :name "Computer Science 101";
  :student :alice, :bob .
"""


class TestRdfData(unittest.TestCase):
    def setUp(self):
        self.rudof = Rudof(RudofConfig())
        self.fmt = RDFFormat.Turtle
        self.mode = ReaderMode.Lax

    def _load(self):
        self.rudof.read_data_str(DATA_STR, self.fmt, None, self.mode, False)

    def test_read_data_str(self):
        """Reading RDF data from a string should not raise."""
        self._load()

    def test_serialize_ntriples_returns_string(self):
        """Serialising to NTriples should return a non-empty string."""
        self._load()
        result = self.rudof.serialize_data(format=RDFFormat.NTriples)
        self.assertIsNotNone(result)
        self.assertGreater(len(result), 0)

    def test_ntriples_contains_alice(self):
        """The serialised output should contain Alice's IRI."""
        self._load()
        result = self.rudof.serialize_data(format=RDFFormat.NTriples)
        self.assertIn("<http://example.org/alice>", result)

    def test_ntriples_contains_bob(self):
        """The serialised output should contain Bob's IRI."""
        self._load()
        result = self.rudof.serialize_data(format=RDFFormat.NTriples)
        self.assertIn("<http://example.org/bob>", result)

    def test_alice_knows_bob(self):
        """Alice should have a :knows triple pointing to Bob."""
        self._load()
        result = self.rudof.serialize_data(format=RDFFormat.NTriples)
        self.assertIn(
            "<http://example.org/alice> <http://example.org/knows> <http://example.org/bob>",
            result,
        )


if __name__ == "__main__":
    unittest.main()
