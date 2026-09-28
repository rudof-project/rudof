"""Run an ASK query and read the boolean answer off the result object."""

from pathlib import Path

from pyrudof import RDFFormat, Rudof

HERE = Path(__file__).resolve().parent.parent

QUERY = """
PREFIX : <http://example.org/>

ASK {
  ?person :name "Alice" .
}
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_query(QUERY)

        results = rudof.run_query()

        print(f"is_boolean: {results.is_boolean}")
        print(f"answer: {results.boolean}")


if __name__ == "__main__":
    main()
