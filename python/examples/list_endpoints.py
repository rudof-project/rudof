from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data("person.ttl", RDFFormat.Turtle)
endpoints = rudof.list_endpoints()

print("LIST_ENDPOINTS_OK")
print(f"ENDPOINTS_COUNT={len(endpoints)}")
