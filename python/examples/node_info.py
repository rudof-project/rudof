from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

rudof.use_endpoint("dbpedia")
node_info = rudof.node_info("dbr:Oviedo", [])
print(node_info)

rudof.reset_all()

rudof.use_endpoint("wikidata")
node_info = rudof.node_info("wd:Q42", ["wdt:P31", "wdt:P19"])
print(node_info)

endpoints = rudof.list_endpoints()
print(f"Available endpoints: {endpoints}")