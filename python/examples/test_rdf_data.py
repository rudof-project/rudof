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
        self.rudof.read_data(
            input=DATA_STR,
            format=self.fmt,
            reader_mode=self.mode,
            merge=False,
        )

    def test_read_data(self):
        """Reading RDF data from a string should not raise."""
        self._load()

    def test_serialize_returns_string(self):
        """Serialising data should return a non-empty string."""
        self._load()
        result = self.rudof.serialize_data()
        self.assertIsNotNone(result)
        self.assertGreater(len(result), 0)

    def test_serialized_contains_alice(self):
        """The serialised output should contain Alice's IRI."""
        self._load()
        result = self.rudof.serialize_data()
        self.assertIn("<http://example.org/alice>", result)

    def test_serialized_contains_bob(self):
        """The serialised output should contain Bob's IRI."""
        self._load()
        result = self.rudof.serialize_data()
        self.assertIn("<http://example.org/bob>", result)

    def test_alice_knows_bob(self):
        """Serialized output should contain the knows predicate."""
        self._load()
        result = self.rudof.serialize_data()
        self.assertIn("knows", result)


if __name__ == "__main__":
    unittest.main()
