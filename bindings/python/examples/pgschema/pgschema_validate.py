from pyrudof import PgSchemaFormat, RDFFormat, ResultPgSchemaValidationFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

pg_data = """
(n1 {"Student"}["name": "Alice", "age": 23])
(n2_wrong {"Student"}["name": "Bob", "age": 12])
"""

schema = """
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
"""

typemap = """
n1: AdultStudentType,
n2_wrong: AdultStudentType
"""

rudof.read_data(pg_data, RDFFormat.Pg)
rudof.read_pgschema(schema, PgSchemaFormat.PgSchemaC)
rudof.read_typemap(typemap)
rudof.validate_pgschema()

results = rudof.serialize_pgschema_validation_results(ResultPgSchemaValidationFormat.Compact)
print(results)

rudof.reset_pgschema_validation()
rudof.reset_typemap()
rudof.reset_pgschema()
