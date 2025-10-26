from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.read_shex_str("""
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

#for (node, shape, status) in results.to_list():
#    print(f"Node: {node.show()}")
#    print(f"Shape: {shape.show()}")
#    print(f"Conformant?: {status.is_conformant()}")
#    print(f"Appinfo: {status.as_json()}")
#    print("")


rudof.read_shapemap_str("""
:x@:S, :y@:S
""")
