from pyrudof import Rudof, RudofConfig, RDFFormat 

rudof = Rudof(RudofConfig())

rudof.use_endpoint("dbpedia")
node_info = rudof.node_info("dbr:Oviedo", [])
print(node_info)