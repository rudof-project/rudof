"""
Call every reset method the session exposes.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof, ShaclFormat, ShapeMapFormat, ShExFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_shex(HERE / "person.shex", ShExFormat.ShExC)
        rudof.read_shacl(HERE / "timbl_shapes.ttl", ShaclFormat.Turtle)
        rudof.read_shapemap(HERE / "person.sm", ShapeMapFormat.Compact)
        rudof.read_query(HERE / "person.sparql")

        rudof.reset_data()
        rudof.reset_shex_schema()
        rudof.reset_shex()
        rudof.reset_shacl()
        rudof.reset_shacl_validation()
        rudof.reset_shapemap()
        rudof.reset_query()
        rudof.reset_query_results()
        rudof.reset_dctap()
        rudof.reset_rdf_config()
        rudof.reset_service_description()
        rudof.reset_validation_results()
        rudof.reset_pgschema()
        rudof.reset_typemap()
        rudof.reset_pgschema_validation()
        rudof.reset_pg_db_connection()
        rudof.reset_all()

        print("all resets completed")


if __name__ == "__main__":
    main()
