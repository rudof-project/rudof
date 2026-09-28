"""Connect to a LadybugDB database, load RDF data into it, and query it with Cypher."""

from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import Rudof

DATA = """
@prefix : <http://example.org/> .
:alice a :Person ;
    :name "Alice" ;
    :knows :bob .
:bob a :Person ;
    :name "Bob" .
"""


def main() -> None:
    with TemporaryDirectory() as tmpdir:
        db_path = Path(tmpdir) / "example.lbug"

        with Rudof() as rudof:
            rudof.connect_pg_db(db_path)

            report = rudof.load_pg_db(DATA, skip_validation=True)
            print(f"loaded: {'Inserted' in report}")

            result = rudof.query_cypher("MATCH (n:Person) RETURN n.name ORDER BY n.name")
            print(result["columns"])
            for row in result["rows"]:
                print(row)


if __name__ == "__main__":
    main()
