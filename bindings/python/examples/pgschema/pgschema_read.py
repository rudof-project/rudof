from pyrudof import PgSchemaFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

schema = """
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
"""

rudof.read_pgschema(schema, PgSchemaFormat.PgSchemaC)
print(rudof.serialize_pgschema())
