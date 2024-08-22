from pyrudof import shacl

print("Reading file from examples/simple_shacl.ttl and generating output in target/simple_shacl.rdf")

shacl.parse("examples/simple_shacl.ttl", "target/simple_shacl.rdf")

