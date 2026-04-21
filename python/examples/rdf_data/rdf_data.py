from pyrudof import RDFFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_data(input="person.ttl", format=RDFFormat.Turtle)
rudof.read_data(
    input='prefix : <http://example.org/>\n:extra :name "Extra" .\n',
    format=RDFFormat.Turtle,
    merge=True,
)

serialized = rudof.serialize_data()
