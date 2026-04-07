from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data("person.ttl", RDFFormat.Turtle)

info = rudof.node_info(":alice", [":name"], "outgoing", False, 1)

