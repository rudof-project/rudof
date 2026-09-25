"""Read an inline Property Graph schema and serialize it back."""

from pyrudof import PgSchemaFormat, Rudof

SCHEMA = """
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_pgschema(SCHEMA, PgSchemaFormat.PgSchemaC)
        print(rudof.serialize_pgschema())


if __name__ == "__main__":
    main()
