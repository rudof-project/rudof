from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.add_prefix("ex", "http://example.org/")
rudof.add_prefix("foaf", "http://xmlns.com/foaf/0.1/")

print(sorted(rudof.prefixes()))

rudof.rename_prefix("foaf", "f")
rudof.copy_prefix("ex", "example")
rudof.remove_prefix("f")

print(sorted(rudof.prefixes()))
