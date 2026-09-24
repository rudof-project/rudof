"""Read a DCTAP profile from inline CSV and from a file, then serialize it."""

from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DCTapFormat, ResultDCTapFormat, Rudof

CSV_TEXT = "shapeId,propertyId\n:Person,:name\n"


def main() -> None:
    with Rudof() as rudof:
        rudof.read_dctap(CSV_TEXT)
        print(rudof.serialize_dctap(ResultDCTapFormat.Internal))

        with TemporaryDirectory() as tmpdir:
            csv_path = Path(tmpdir) / "profile.csv"
            csv_path.write_text(CSV_TEXT, encoding="utf-8")

            rudof.read_dctap(csv_path, DCTapFormat.Csv)
            print(f"read from file: {':Person' in rudof.serialize_dctap()}")


if __name__ == "__main__":
    main()
