from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ReaderMode, ShapeMapFormat

rudof = Rudof(RudofConfig())

rudof.read_shex_str("""
prefix : <http://example.org/>
                    
:S {
 :p .
}
""", ShExFormat.shexc())

rudof.read_data_str("""
prefix : <http://example.org/>
                    
:x :p 1 .
""", RDFFormat.turtle(), None, ReaderMode.lax())

rudof.read_shapemap_str("""
:x@:S
""", ShapeMapFormat.compact())

result = rudof.validate_shex()

print(result.show())
