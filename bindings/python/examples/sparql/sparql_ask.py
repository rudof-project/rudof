from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data("person.ttl", RDFFormat.Turtle)

query = """
PREFIX : <http://example.org/>

SELECT ?person ?name
WHERE {
  ?person :name ?name .
}
"""

rudof.read_query(query)
rudof.run_query()
results = rudof.serialize_query_results()
