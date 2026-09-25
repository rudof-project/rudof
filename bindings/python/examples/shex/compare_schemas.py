"""Compare two ShEx schemas and report the differences between two shapes."""

from pyrudof import ReaderMode, Rudof

SCHEMA1 = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string
}
"""

SCHEMA2 = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
  :name xsd:string ;
  :age xsd:integer ?
}
"""


def main() -> None:
    with Rudof() as rudof:
        comparison = rudof.compare_schemas(
            SCHEMA1,
            SCHEMA2,
            "shex",
            "shex",
            "shexc",
            "shexc",
            None,
            None,
            "http://example.org/Person",
            "http://example.org/Person",
            ReaderMode.Lax,
        )
        print(comparison)


if __name__ == "__main__":
    main()
