import unittest
from pyrudof import (
    Rudof, 
    RudofConfig, 
    RDFFormat, 
    ReaderMode, 
    ShaclFormat, 
    ShaclValidationMode, 
    ShapesGraphSource
)

class TestShacl(unittest.TestCase):
    def setUp(self) -> None:
        self.config = RudofConfig()
        self.rudof = Rudof(self.config)
        
        self.rdf_format = RDFFormat.Turtle
        self.shacl_format = ShaclFormat.Turtle
        self.reader_mode = ReaderMode.Lax
        self.validation_mode = ShaclValidationMode.Native
        self.shapes_source = ShapesGraphSource.CurrentData

    def test_ok(self) -> None:
        data = """
        prefix : <http://example.org/>
        prefix sh:     <http://www.w3.org/ns/shacl#>
        prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

        :Person a sh:NodeShape; sh:targetNode :ok ;
         sh:property [
          sh:path     :name ;
           sh:minCount 1;
           sh:maxCount 1;
           sh:datatype xsd:string ;
        ] .
        :ok :name "alice" .
        """
        self.rudof.read_data_str(data, self.rdf_format, None, self.reader_mode, False)
        
        result = self.rudof.validate_shacl(self.validation_mode, self.shapes_source)

        self.assertTrue(result.conforms())

    def test_ko(self) -> None:
        data = """
        prefix : <http://example.org/>
        prefix sh:     <http://www.w3.org/ns/shacl#>
        prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

        :Person a sh:NodeShape; sh:targetNode :ko ;
         sh:property [
          sh:path     :name ;
           sh:minCount 1;
           sh:maxCount 1;
           sh:datatype xsd:string ;
        ] .
        :ko :name 23 .
        """
        self.rudof.read_data_str(data, self.rdf_format, None, self.reader_mode, False)
        
        result = self.rudof.validate_shacl(self.validation_mode, self.shapes_source)
        
        self.assertFalse(result.conforms())

if __name__ == "__main__":
    unittest.main()
