from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.read_data("person.ttl")

results = rudof.run_query_path("person.sparql")
for result in iter(results):
    print(result.show())
