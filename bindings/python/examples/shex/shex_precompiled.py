"""Compile a ShEx schema to a cache file, then validate using the cache.

Precompiling skips both parsing and AST-to-IR compilation on subsequent runs,
which is what makes a large schema cheap to reuse across processes.
"""

from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import RDFFormat, Rudof, ShapeMapFormat, ShExFormat

HERE = Path(__file__).resolve().parent.parent

DATA = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:alice :name "Alice" ; :age 30 .
"""

SHAPEMAP = ":alice@:Person"


def main() -> None:
    with TemporaryDirectory() as tmpdir:
        cache_path = Path(tmpdir) / "person.shexcache"

        # Compile the schema to a cache file once...
        with Rudof() as rudof:
            rudof.read_shex(HERE / "person.shex", ShExFormat.ShExC)
            rudof.compile_shex_to_file(cache_path)
            print(f"cache written: {cache_path.exists()}")

        # ...then reuse it in a fresh session.
        with Rudof() as rudof:
            rudof.read_shex_precompiled(cache_path)
            rudof.read_data(DATA, RDFFormat.Turtle)
            rudof.read_shapemap(SHAPEMAP, ShapeMapFormat.Compact)

            report = rudof.validate_shex()
            print(f"conforms: {report.conforms}")
            for entry in report:
                print(f"{entry.node} @ {entry.shape}: {entry.status}")


if __name__ == "__main__":
    main()
