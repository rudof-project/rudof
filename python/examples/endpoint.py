from pyrudof import Rudof, RudofConfig, RDFFormat 

endpoint = "https://sparql.uniprot.org/sparql"
rudof = Rudof(RudofConfig())

rudof.add_endpoint(endpoint)

result = rudof.serialize_data(format = RDFFormat.NTriples)

print(result)