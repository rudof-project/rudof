"""Validate property-graph data against a PG schema and a typemap.

``n2_wrong`` is 12 years old and the schema requires ``age > 18``, so the report
does not conform and names that node as the single violation.
"""

from pyrudof import PgSchemaFormat, RDFFormat, ResultPgSchemaValidationFormat, Rudof

PG_DATA = """
(n1 {"Student"}["name": "Alice", "age": 23])
(n2_wrong {"Student"}["name": "Bob", "age": 12])
"""

SCHEMA = """
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
"""

TYPEMAP = """
n1: AdultStudentType,
n2_wrong: AdultStudentType
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(PG_DATA, RDFFormat.Pg)
        rudof.read_pgschema(SCHEMA, PgSchemaFormat.PgSchemaC)
        rudof.read_typemap(TYPEMAP)

        report = rudof.validate_pgschema()

        print(f"conforms: {report.conforms}")
        for entry in report.violations:
            print(f"violation: {entry.node_id} as {entry.type_name}")

        print(rudof.serialize_pgschema_validation_results(ResultPgSchemaValidationFormat.Compact))


if __name__ == "__main__":
    main()
