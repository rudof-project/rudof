import os
import tempfile

from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())

data = """
@prefix : <http://example.org/> .
:alice a :Person ;
    :name "Alice" ;
    :knows :bob .
:bob a :Person ;
    :name "Bob" .
"""

# A fresh, throwaway LadybugDB database for this example run.
db_path = os.path.join(tempfile.mkdtemp(), "example.lbug")
rudof.connect_pg_db(db_path)

# Skips SHACL validation for brevity; see the `load` CLI docs for the
# validated flow. `load_pg_db` returns progress text describing what was
# derived/inserted.
print(rudof.load_pg_db(data, skip_validation=True))

# `query_cypher` reuses the database connected above; no path needed here.
result = rudof.query_cypher("MATCH (n:Person) RETURN n.name ORDER BY n.name")
print(result)
