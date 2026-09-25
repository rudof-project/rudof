"""
Tell the narrow schema-only resets apart from the broad ones.

``reset_shex_schema()`` drops only the schema; ``reset_shex()`` also drops the
ShapeMap, the validator and the results. The SHACL pair works the same way.
Reaching for a schema after it was reset raises the domain's own exception.
"""

from pathlib import Path

from pyrudof import (
    RDFFormat,
    Rudof,
    ShaclError,
    ShaclFormat,
    ShaclValidationMode,
    ShapeMapFormat,
    ShExError,
    ShExFormat,
)

HERE = Path(__file__).resolve().parent.parent

DATA = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
:alice :name "Alice" ; :age 30 .
"""

SHACL_SHAPES = """
PREFIX : <http://example.org/>
PREFIX sh: <http://www.w3.org/ns/shacl#>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person .
"""

SHACL_DATA = 'PREFIX : <http://example.org/>\n:alice a :Person .'


def main() -> None:
    with Rudof() as rudof:
        # --- ShEx ---
        rudof.read_shex(HERE / "person.shex", ShExFormat.ShExC)
        rudof.read_data(DATA, RDFFormat.Turtle)
        rudof.read_shapemap(":alice@:Person", ShapeMapFormat.Compact)
        rudof.validate_shex()

        rudof.reset_shex()
        try:
            rudof.serialize_current_shex()
            print("BUG: ShEx schema survived reset_shex()")
        except ShExError:
            print("reset_shex() cleared the schema as documented")

        # --- SHACL: the narrow reset ---
        rudof.reset_all()
        rudof.read_shacl(SHACL_SHAPES, ShaclFormat.Turtle)
        rudof.read_data(SHACL_DATA, RDFFormat.Turtle)
        rudof.validate_shacl(ShaclValidationMode.Native)

        rudof.reset_shacl()
        try:
            rudof.serialize_shacl()
            print("BUG: SHACL shapes survived reset_shacl()")
        except ShaclError:
            print("reset_shacl() cleared the shapes as documented")

        # --- SHACL: the broad reset ---
        rudof.reset_all()
        rudof.read_shacl(SHACL_SHAPES, ShaclFormat.Turtle)
        rudof.read_data(SHACL_DATA, RDFFormat.Turtle)
        rudof.validate_shacl(ShaclValidationMode.Native)

        rudof.reset_shacl_validation()
        try:
            rudof.serialize_shacl()
            print("BUG: SHACL shapes survived reset_shacl_validation()")
        except ShaclError:
            print("reset_shacl_validation() cleared the shapes as documented")


if __name__ == "__main__":
    main()
