from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data("person.ttl", RDFFormat.Turtle)
endpoints = rudof.list_endpoints()
