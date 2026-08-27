from pyrudof import RdfConfigFormat, ResultRdfConfigFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

config = """
- Person ex:person1 ex:person2:
  - a: ex:Person
  - rdfs:label:
    - name: "Alice"
  - ex:age?:
    - age_value: 32
"""

rudof.read_rdf_config(config, RdfConfigFormat.Yaml)
print(rudof.serialize_rdf_config(ResultRdfConfigFormat.Internal))

rudof.reset_rdf_config()
