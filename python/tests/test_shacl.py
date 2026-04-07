import unittest

from pyrudof import RDFFormat, Rudof, RudofConfig, ShaclFormat, ShaclValidationMode


class TestShacl(unittest.TestCase):
    def setUp(self) -> None:
        self.rudof = Rudof(RudofConfig())

    def test_validate_shacl_native_runs(self) -> None:
        self.rudof.read_shacl("../examples/timbl_shapes.ttl", ShaclFormat.Turtle)
        self.rudof.read_data("../examples/timbl.ttl", RDFFormat.Turtle)

        self.rudof.validate_shacl(ShaclValidationMode.Native)

    def test_validate_shacl_sparql_runs(self) -> None:
        self.rudof.read_shacl("../examples/timbl_shapes.ttl", ShaclFormat.Turtle)
        self.rudof.read_data("../examples/timbl.ttl", RDFFormat.Turtle)

        self.rudof.validate_shacl(ShaclValidationMode.Sparql)

    def test_read_shacl_from_current_data(self) -> None:
        graph = """
        PREFIX : <http://example.org/>
        PREFIX sh: <http://www.w3.org/ns/shacl#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        :PersonShape a sh:NodeShape ;
          sh:targetClass :Person ;
          sh:property [
            sh:path :name ;
            sh:datatype xsd:string ;
            sh:minCount 1
          ] .

        :alice a :Person ;
          :name "Alice" .
        """
        self.rudof.read_data(graph, RDFFormat.Turtle)
        self.rudof.read_shacl()

        self.rudof.validate_shacl(ShaclValidationMode.Native)

    def test_serialize_shacl_not_empty(self) -> None:
        self.rudof.read_shacl("../examples/timbl_shapes.ttl", ShaclFormat.Turtle)
        serialized = self.rudof.serialize_shacl()

        self.assertTrue(len(serialized.strip()) > 0)


if __name__ == "__main__":
    unittest.main(verbosity=2)
