"""Load a SPARQL query from a file and run it."""

from pathlib import Path

from pyrudof import RDFFormat, Rudof

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_query(HERE / "person.sparql")

        results = rudof.run_query()

        print(f"variables: {results.variables}")
        for row in results.rows:
            print(row)


if __name__ == "__main__":
    main()
