from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ReaderMode, ShapeMapFormat

rudof = Rudof(RudofConfig())

rudof.read_shex_str("""
prefix : <http://example.org/>
                    
:S {
 :p .
}
""")

rudof.read_data_str("""
prefix : <http://example.org/>
                    
:x :p 1 .
""")

rudof.read_shapemap_str("""
:x@:S
""")

result = rudof.validate_shex()

print(result.show())
