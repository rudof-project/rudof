"""Validate RDF data from files against a ShEx schema and a ShapeMap.

Every "content, path or URL" parameter accepts a ``str`` or any ``os.PathLike``,
so ``pathlib.Path`` objects work directly.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof, ShapeMapFormat, ShExFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shex(HERE / "person.shex", ShExFormat.ShExC)
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_shapemap(HERE / "person.sm", ShapeMapFormat.Compact)

        report = rudof.validate_shex()

        print(f"conforms: {report.conforms}")
        print(f"violations: {len(report.violations)}")
        for entry in report:
            print(f"{entry.node} @ {entry.shape}: {entry.status}")


if __name__ == "__main__":
    main()
