"""Serialize the currently loaded ShEx schema."""

from pathlib import Path

from pyrudof import Rudof, ShExFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_shex(HERE / "person.shex", ShExFormat.ShExC)

        as_shexc = rudof.serialize_current_shex(format=ShExFormat.ShExC)
        print(as_shexc)

        as_shexj = rudof.serialize_current_shex(format=ShExFormat.ShExJ)
        print(f"ShExJ length: {len(as_shexj) > 0}")


if __name__ == "__main__":
    main()
