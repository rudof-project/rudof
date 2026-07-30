from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data("person.ttl", RDFFormat.Turtle)
rudof.read_query("person.sparql")
rudof.run_query()
results = rudof.serialize_query_results()
