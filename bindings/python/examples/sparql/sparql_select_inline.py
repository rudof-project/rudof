"""Run an inline SPARQL SELECT query against loaded RDF data.

``run_query`` returns a ``QueryResults`` object: the bindings are reachable as
data, without re-parsing rudof's own serialized output.
"""

from pathlib import Path

from pyrudof import RDFFormat, Rudof

HERE = Path(__file__).resolve().parent.parent

QUERY = """
PREFIX : <http://example.org/>

SELECT ?person ?name
WHERE {
  ?person :name ?name .
}
"""


def main() -> None:
    with Rudof() as rudof:
        rudof.read_data(HERE / "person.ttl", RDFFormat.Turtle)
        rudof.read_query(QUERY)

        results = rudof.run_query()

        print(f"variables: {results.variables}")
        print(f"rows: {len(results)}")
        for row in results:
            print(row)


if __name__ == "__main__":
    main()
