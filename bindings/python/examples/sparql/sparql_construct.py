"""Run a CONSTRUCT query and serialize the resulting graph.

A CONSTRUCT result is a graph, not a table: ``is_graph`` says so, and ``graph``
carries the serialized triples.
"""

from pathlib import Path

from pyrudof import QueryResultFormat, RDFFormat, Rudof

HERE = Path(__file__).resolve().parent.parent

QUERY = """
PREFIX : <http://example.org/>

CONSTRUCT {
  ?person :name ?name .
}
WHERE {
  ?person :name ?name .
}
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_query(QUERY)

        results = rudof.run_query()
        print(f"is_graph: {results.is_graph}")

        serialized = rudof.serialize_query_results(QueryResultFormat.Turtle)
        print(f"has alice: {'alice' in serialized}")


if __name__ == "__main__":
    main()
