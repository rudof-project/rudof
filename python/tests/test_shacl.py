import unittest

from pyrudof import Rudof, RudofConfig

class TestShacl(unittest.TestCase):
    def test_ok(self) -> None:
        rudof = Rudof(RudofConfig())
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
        rudof.read_data_str(data)
        result = rudof.validate_shacl()
        print(result.show())
        self.assertTrue(result.conforms())

    def test_ko(self) -> None:
        rudof = Rudof(RudofConfig())
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
        rudof.read_data_str(data)
        result = rudof.validate_shacl()
        print(result.show())
        self.assertFalse(result.conforms())


if __name__ == "__main__":
    unittest.main()    