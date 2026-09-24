"""Load a ShapeMap and serialize it back."""

from pathlib import Path

from pyrudof import RDFFormat, Rudof, ShapeMapFormat, ShExFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_shex(HERE / "person.shex", ShExFormat.ShExC)
        rudof.read_shapemap(HERE / "person.sm", ShapeMapFormat.Compact)

        serialized = rudof.serialize_shapemap(ShapeMapFormat.Compact)
        print(serialized)


if __name__ == "__main__":
    main()
