from pyrudof import Rudof, RudofConfig, RDFFormat 

rudof = Rudof(RudofConfig())
rudof.read_data("examples/person.ttl")
results = rudof.run_query_path("examples/person.sparql")

print(results.show())