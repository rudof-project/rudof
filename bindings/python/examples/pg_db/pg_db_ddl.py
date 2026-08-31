from pyrudof import DdlDialect, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

data = """
@prefix : <http://example.org/> .
:alice a :Person ;
    :name "Alice" ;
    :knows :bob .
:bob a :Person ;
    :name "Bob" .
"""

# Stateless: derives a property graph schema from the data and emits DDL,
# without opening or touching any database.
print(rudof.pg_db_ddl(data, DdlDialect.Cypher))
