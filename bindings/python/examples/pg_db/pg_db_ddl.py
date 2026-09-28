"""Derive a property graph schema from RDF data and emit it as Cypher DDL.

Stateless: no database is opened or touched.
"""

from pyrudof import DdlDialect, Rudof

DATA = """
@prefix : <http://example.org/> .
:alice a :Person ;
    :name "Alice" ;
    :knows :bob .
:bob a :Person ;
    :name "Bob" .
"""


def main() -> None:
    with Rudof() as rudof:
        print(rudof.pg_db_ddl(DATA, DdlDialect.Cypher))


if __name__ == "__main__":
    main()
