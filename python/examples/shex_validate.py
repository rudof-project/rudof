from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.read_shex_str("""
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:S { :p xsd:integer }
""")

rudof.read_data_str("""
prefix : <http://example.org/>

:x :p 1 .
:y :q 2 .
""")

rudof.read_shapemap_str("""
:x@:S, :y@:S
""")

results = rudof.validate_shex()
print(results.show_as_table())
