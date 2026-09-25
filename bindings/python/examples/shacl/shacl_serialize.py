"""Serialize the currently loaded SHACL shapes graph."""

from pathlib import Path

from pyrudof import Rudof, ShaclFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shacl(HERE / "timbl_shapes.ttl", ShaclFormat.Turtle)

        serialized = rudof.serialize_shacl(ShaclFormat.Turtle)
        print(f"serialized shapes: {len(serialized) > 0}")
        print(f"mentions NodeShape: {'NodeShape' in serialized}")


if __name__ == "__main__":
    main()
