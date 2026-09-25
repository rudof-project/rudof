"""Validate RDF data from files against a SHACL shapes graph."""

from pathlib import Path

from pyrudof import RDFFormat, Rudof, ShaclFormat, ShaclValidationMode

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shacl(HERE / "timbl_shapes.ttl", ShaclFormat.Turtle)
        rudof.read_data(HERE / "timbl.ttl", RDFFormat.Turtle)

        report = rudof.validate_shacl(ShaclValidationMode.Native)

        print(f"conforms: {report.conforms}")
        print(f"violations: {len(report)}")


if __name__ == "__main__":
    main()
