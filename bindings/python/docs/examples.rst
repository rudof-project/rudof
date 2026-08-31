Examples
========

This page contains validated Python examples that demonstrate pyrudof functionality.
Each example includes executable Python code that can be copied and pasted into a
Jupyter notebook or Python script, along with links to any referenced files.


RDF Data Handling
-----------------

Examples for RDF loading, serialization, and node inspection.


RDF Read and Serialize
^^^^^^^^^^^^^^^^^^^^^^

Read RDF data, merge extra triples, and serialize

**Source**: `rdf_data/rdf_data.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/rdf_data/rdf_data.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data(input="person.ttl", format=RDFFormat.Turtle)
    rudof.read_data(
        input='prefix : <http://example.org/>\n:extra :name "Extra" .\n',
        format=RDFFormat.Turtle,
        merge=True,
    )
    
    serialized = rudof.serialize_data()

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_


Node Info
^^^^^^^^^

Inspect node neighborhood information in loaded RDF data

**Source**: `rdf_data/node_info.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/rdf_data/node_info.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    
    info = rudof.node_info(":alice", [":name"], "outgoing", False, 1)

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_


List Endpoints
^^^^^^^^^^^^^^

List known SPARQL endpoints

**Source**: `rdf_data/list_endpoints.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/rdf_data/list_endpoints.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    endpoints = rudof.list_endpoints()


Dereference a URI
^^^^^^^^^^^^^^^^^

Fetch RDF data over HTTP(S) and merge it into the current graph

**Source**: `rdf_data/dereference.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/rdf_data/dereference.py>`_

**Python Code:**

.. code-block:: python

    """Fetch RDF data over HTTP(S) and merge it into the current graph.
    
    Network-dependent: skipped by default in the test suite (see examples.toml).
    """
    from pyrudof import ReaderMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.dereference("https://www.w3.org/People/Berners-Lee/card", ReaderMode.Lax)
    print(rudof.serialize_data())


SPARQL Queries
--------------

Examples for SELECT, CONSTRUCT and ASK query workflows.


SPARQL SELECT Inline
^^^^^^^^^^^^^^^^^^^^

Run an inline SPARQL SELECT query against loaded RDF data

**Source**: `sparql/sparql_select_inline.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/sparql/sparql_select_inline.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    
    query = """
    PREFIX : <http://example.org/>
    
    SELECT ?person ?name
    WHERE {
      ?person :name ?name .
    }
    """
    
    rudof.read_query(query)
    rudof.run_query()
    results = rudof.serialize_query_results()

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_


SPARQL SELECT File
^^^^^^^^^^^^^^^^^^

Load SPARQL query from file and run it

**Source**: `sparql/sparql_select_file.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/sparql/sparql_select_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_query("person.sparql")
    rudof.run_query()
    results = rudof.serialize_query_results()

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.sparql>`_


SPARQL CONSTRUCT
^^^^^^^^^^^^^^^^

Run a CONSTRUCT query and serialize graph results

**Source**: `sparql/sparql_construct.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/sparql/sparql_construct.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import QueryResultFormat, RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    
    query = """
    PREFIX : <http://example.org/>
    
    CONSTRUCT {
      ?person :name ?name .
    }
    WHERE {
      ?person :name ?name .
    }
    """
    
    rudof.read_query(query)
    rudof.run_query()
    results = rudof.serialize_query_results(QueryResultFormat.Turtle)

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_


SPARQL SELECT Internal
^^^^^^^^^^^^^^^^^^^^^^

Run a SELECT query and serialize results using the default internal format

**Source**: `sparql/sparql_ask.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/sparql/sparql_ask.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    
    query = """
    PREFIX : <http://example.org/>
    
    SELECT ?person ?name
    WHERE {
      ?person :name ?name .
    }
    """
    
    rudof.read_query(query)
    rudof.run_query()
    results = rudof.serialize_query_results()

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_


ShEx Validation
---------------

Examples for reading ShEx schemas, validating data, and serializing schemas/ShapeMaps.


ShEx Validate Inline
^^^^^^^^^^^^^^^^^^^^

Validate inline RDF data against an inline ShEx schema and ShapeMap

**Source**: `shex/shex_validate_inline.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/shex_validate_inline.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    schema = """
    PREFIX : <http://example.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :Person {
      :name xsd:string
    }
    """
    
    data = """
    PREFIX : <http://example.org/>
    
    :alice :name "Alice" .
    """
    
    shapemap = ":alice@:Person"
    
    rudof.read_shex(schema, ShExFormat.ShExC)
    rudof.read_data(data, RDFFormat.Turtle)
    rudof.read_shapemap(shapemap, ShapeMapFormat.Compact)
    rudof.validate_shex()


ShEx Validate Files
^^^^^^^^^^^^^^^^^^^

Validate RDF data from files against a ShEx schema and ShapeMap

**Source**: `shex/shex_validate_file.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/shex_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ShapeMapFormat
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    
    rudof.validate_shex()

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.sm>`_


ShEx Serialize
^^^^^^^^^^^^^^

Serialize the currently loaded ShEx schema

**Source**: `shex/shex_serialize.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/shex_serialize.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    
    serialized = rudof.serialize_current_shex()

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.shex>`_


ShapeMap Roundtrip
^^^^^^^^^^^^^^^^^^

Load and serialize a ShapeMap

**Source**: `shex/shapemap_roundtrip.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/shapemap_roundtrip.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    serialized = rudof.serialize_shapemap()

**Referenced Files:**

- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.sm>`_


Compare Schemas
^^^^^^^^^^^^^^^

Compare two ShEx schemas and print comparison output size

**Source**: `shex/compare_schemas.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/compare_schemas.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ReaderMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    schema1 = """
    PREFIX : <http://example.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :Person {
      :name xsd:string
    }
    """
    
    schema2 = """
    PREFIX : <http://example.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :Person {
      :name xsd:string ;
      :age xsd:integer ?
    }
    """
    
    comparison = rudof.compare_schemas(
        schema1,
        schema2,
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


Check ShEx Schema
^^^^^^^^^^^^^^^^^

Check well-formedness of a valid and an invalid ShEx schema

**Source**: `shex/shex_check.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/shex_check.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    valid_schema = """
    PREFIX ex: <http://example.org/>
    ex:PersonShape {
        ex:name .
    }
    """
    is_valid, message = rudof.check_shex(valid_schema)
    print(is_valid)
    print(message)
    
    invalid_schema = """
    PREFIX ex: <http://example.org/>
    ex:Shape1 {
        ex:prop1 @ex:Shape2
    }
    ex:Shape2 {
        ex:prop2 NOT @ex:Shape1
    }
    """
    is_valid2, message2 = rudof.check_shex(invalid_schema)
    print(is_valid2)
    print(message2)


Precompiled ShEx Schema Cache
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Compile a loaded ShEx schema to a cache file, then validate using the precompiled cache

**Source**: `shex/shex_precompiled.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shex/shex_precompiled.py>`_

**Python Code:**

.. code-block:: python

    from pathlib import Path
    from tempfile import TemporaryDirectory
    
    from pyrudof import RDFFormat, ResultShexValidationFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig
    
    data = """
    PREFIX : <http://example.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :alice :name "Alice" ; :age 30 .
    """
    
    shapemap = ":alice@:Person"
    
    # Compile the schema to a cache file once...
    rudof = Rudof(RudofConfig())
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    
    with TemporaryDirectory() as tmpdir:
        cache_path = Path(tmpdir) / "person.shexcache"
        rudof.compile_shex_to_file(str(cache_path))
    
        # ...then reuse it to skip parsing and AST-to-IR compilation on future runs.
        rudof2 = Rudof(RudofConfig())
        rudof2.read_shex_precompiled(str(cache_path))
        rudof2.read_data(data, RDFFormat.Turtle)
        rudof2.read_shapemap(shapemap, ShapeMapFormat.Compact)
        rudof2.validate_shex()
        print(rudof2.serialize_shex_validation_results(ResultShexValidationFormat.Compact))

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.shex>`_


Materialize
-----------

Examples for materializing RDF graphs from ShEx schemas with Map semantic actions.


Materialize Inline
^^^^^^^^^^^^^^^^^^

Materialize an RDF graph from an inline ShEx schema and a MapState built in Python

**Source**: `materialize/materialize_inline.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/materialize/materialize_inline.py>`_

**Python Code:**

.. code-block:: python

    """Materialize an RDF graph from an inline ShEx schema and inline MapState.
    
    The MapState is a dict that maps Map-extension IRI keys to RDF node values.
    IRI nodes are represented as ``{"Iri": "<iri-string>"}``.
    """
    import json
    import os
    import tempfile
    
    from pyrudof import ResultDataFormat, Rudof, RudofConfig, ShExFormat
    
    rudof = Rudof(RudofConfig())
    
    # ShEx schema (ShExJ) with Map semantic actions on each triple constraint
    schema = json.dumps({
        "@context": "http://www.w3.org/ns/shex.jsonld",
        "type": "Schema",
        "shapes": [{
            "type": "ShapeDecl",
            "id": "http://example.org/PersonShape",
            "shapeExpr": {
                "type": "Shape",
                "expression": {
                    "type": "TripleConstraint",
                    "predicate": "http://example.org/name",
                    "semActs": [{
                        "type": "SemAct",
                        "name": "http://shex.io/extensions/Map/",
                        "code": "<http://example.org/name>"
                    }]
                }
            }
        }]
    })
    
    # MapState: maps each Map-extension IRI to its concrete RDF node value
    map_state = {
        "http://example.org/name": {"Iri": "http://example.org/Alice"}
    }
    
    rudof.read_shex(schema, ShExFormat.ShExJ)
    
    # read_map_state requires a file path, so write to a temporary file
    with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as tmp:
        json.dump(map_state, tmp)
        map_state_path = tmp.name
    
    try:
        rudof.read_map_state(map_state_path)
        result = rudof.materialize(ResultDataFormat.NTriples)
        print(result)
    finally:
        os.unlink(map_state_path)


Materialize from Files
^^^^^^^^^^^^^^^^^^^^^^

Load a ShExJ schema and a MapState file, then materialize with an explicit root subject IRI

**Source**: `materialize/materialize_file.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/materialize/materialize_file.py>`_

**Python Code:**

.. code-block:: python

    """Materialize an RDF graph from ShEx schema and MapState files.
    
    Demonstrates loading a ShExJ schema and a pre-built MapState JSON file, then
    materializing the RDF graph with an explicit root subject IRI.
    """
    from pyrudof import ResultDataFormat, Rudof, RudofConfig, ShExFormat
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex("person_map.shexj", ShExFormat.ShExJ)
    rudof.read_map_state("person_map_state.json")
    
    result = rudof.materialize(
        format=ResultDataFormat.Turtle,
        node="http://example.org/Alice",
    )
    print(result)

**Referenced Files:**

- **Schema**: `person_map.shexj <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person_map.shexj>`_
- **Map_state**: `person_map_state.json <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person_map_state.json>`_


SHACL Validation
----------------

Examples for SHACL loading, validation, extraction from data, and serialization.


SHACL Validate Inline
^^^^^^^^^^^^^^^^^^^^^

Validate inline RDF data with inline SHACL shapes

**Source**: `shacl/shacl_validate_inline.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shacl/shacl_validate_inline.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    shapes = """
    PREFIX : <http://example.org/>
    PREFIX sh: <http://www.w3.org/ns/shacl#>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :PersonShape a sh:NodeShape ;
      sh:targetClass :Person ;
      sh:property [
        sh:path :name ;
        sh:datatype xsd:string ;
        sh:minCount 1
      ] .
    """
    
    data = """
    PREFIX : <http://example.org/>
    
    :alice a :Person ;
      :name "Alice" .
    """
    
    rudof.read_shacl(shapes, ShaclFormat.Turtle)
    rudof.read_data(data, RDFFormat.Turtle)
    rudof.validate_shacl(ShaclValidationMode.Native)


SHACL Validate Files
^^^^^^^^^^^^^^^^^^^^

Validate RDF data from files against SHACL shapes

**Source**: `shacl/shacl_validate_file.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shacl/shacl_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    rudof.read_data("timbl.ttl", RDFFormat.Turtle)
    rudof.validate_shacl(ShaclValidationMode.Native)

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/timbl_shapes.ttl>`_
- **Data**: `timbl.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/timbl.ttl>`_


SHACL From Data
^^^^^^^^^^^^^^^

Extract SHACL shapes from current RDF data and validate

**Source**: `shacl/shacl_from_data.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shacl/shacl_from_data.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShaclValidationMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    shapes_and_data = """
    PREFIX : <http://example.org/>
    PREFIX sh: <http://www.w3.org/ns/shacl#>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :PersonShape a sh:NodeShape ;
      sh:targetClass :Person ;
      sh:property [
        sh:path :name ;
        sh:datatype xsd:string ;
        sh:minCount 1
      ] .
    
    :alice a :Person ;
      :name "Alice" .
    """
    
    rudof.read_data(shapes_and_data, RDFFormat.Turtle)
    rudof.read_shacl()
    rudof.validate_shacl(ShaclValidationMode.Native)


SHACL Serialize
^^^^^^^^^^^^^^^

Serialize the currently loaded SHACL graph

**Source**: `shacl/shacl_serialize.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/shacl/shacl_serialize.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ShaclFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    serialized = rudof.serialize_shacl()

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/timbl_shapes.ttl>`_


DCTAP
-----

Examples for reading DCTAP profiles from inline content or files.


Read DCTAP
^^^^^^^^^^

Read DCTAP from inline CSV and from file

**Source**: `dctap/dctap_read.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/dctap/dctap_read.py>`_

**Python Code:**

.. code-block:: python

    from pathlib import Path
    from tempfile import TemporaryDirectory
    
    from pyrudof import DCTapFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    csv_text = "shapeId,propertyId\n:Person,:name\n"
    rudof.read_dctap(csv_text)
    
    with TemporaryDirectory() as tmpdir:
        csv_path = Path(tmpdir) / "profile.csv"
        csv_path.write_text(csv_text, encoding="utf-8")
        rudof.read_dctap(str(csv_path), DCTapFormat.Csv)


PG Schema Validation
--------------------

Examples for reading Property Graph schemas, loading typemaps, and validating PG data.


Read PG Schema
^^^^^^^^^^^^^^

Read an inline Property Graph schema and serialize it back

**Source**: `pgschema/pgschema_read.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/pgschema/pgschema_read.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import PgSchemaFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    schema = """
    CREATE NODE TYPE ( AdultStudentType: Student {
        name: STRING ,
        age: INTEGER CHECK > 18
    })
    """
    
    rudof.read_pgschema(schema, PgSchemaFormat.PgSchemaC)
    print(rudof.serialize_pgschema())


Validate PG Data
^^^^^^^^^^^^^^^^

Load PG data, a PG schema and a typemap, then validate and serialize the results

**Source**: `pgschema/pgschema_validate.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/pgschema/pgschema_validate.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import PgSchemaFormat, RDFFormat, ResultPgSchemaValidationFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    pg_data = """
    (n1 {"Student"}["name": "Alice", "age": 23])
    (n2_wrong {"Student"}["name": "Bob", "age": 12])
    """
    
    schema = """
    CREATE NODE TYPE ( AdultStudentType: Student {
        name: STRING ,
        age: INTEGER CHECK > 18
    })
    """
    
    typemap = """
    n1: AdultStudentType,
    n2_wrong: AdultStudentType
    """
    
    rudof.read_data(pg_data, RDFFormat.Pg)
    rudof.read_pgschema(schema, PgSchemaFormat.PgSchemaC)
    rudof.read_typemap(typemap)
    rudof.validate_pgschema()
    
    results = rudof.serialize_pgschema_validation_results(ResultPgSchemaValidationFormat.Compact)
    print(results)
    
    rudof.reset_pgschema_validation()
    rudof.reset_typemap()
    rudof.reset_pgschema()


Property Graph Database (LadybugDB)
-----------------------------------

Examples for deriving DDL from RDF data, and connecting to, loading, and querying a LadybugDB property graph database.


Derive Property Graph DDL
^^^^^^^^^^^^^^^^^^^^^^^^^

Derive a property graph schema from RDF data and emit it as Cypher DDL, without opening a database

**Source**: `pg_db/pg_db_ddl.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/pg_db/pg_db_ddl.py>`_

**Python Code:**

.. code-block:: python

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


Connect, Load and Query a LadybugDB Database
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Connect to a LadybugDB database, load RDF data into it, and run a Cypher query against it

**Source**: `pg_db/pg_db_load_and_query.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/pg_db/pg_db_load_and_query.py>`_

**Python Code:**

.. code-block:: python

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


Prefix Management
-----------------

Examples for managing the default prefix map (add, rename, copy, remove).


Manage Prefixes
^^^^^^^^^^^^^^^

Add, rename, copy and remove entries in the default prefix map

**Source**: `prefixes/prefixes_manage.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/prefixes/prefixes_manage.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.add_prefix("ex", "http://example.org/")
    rudof.add_prefix("foaf", "http://xmlns.com/foaf/0.1/")
    
    print(sorted(rudof.prefixes()))
    
    rudof.rename_prefix("foaf", "f")
    rudof.copy_prefix("ex", "example")
    rudof.remove_prefix("f")
    
    print(sorted(rudof.prefixes()))


RDF-config
----------

Examples for reading and serializing RDF-config YAML specifications.


Read RDF-config
^^^^^^^^^^^^^^^

Read an inline RDF-config YAML specification and serialize it back

**Source**: `rdf_config/rdf_config_read.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/rdf_config/rdf_config_read.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RdfConfigFormat, ResultRdfConfigFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    config = """
    - Person ex:person1 ex:person2:
      - a: ex:Person
      - rdfs:label:
        - name: "Alice"
      - ex:age?:
        - age_value: 32
    """
    
    rudof.read_rdf_config(config, RdfConfigFormat.Yaml)
    print(rudof.serialize_rdf_config(ResultRdfConfigFormat.Internal))
    
    rudof.reset_rdf_config()


Service Description
-------------------

Examples for service description parsing and serialization.


Service Description
^^^^^^^^^^^^^^^^^^^

Read and serialize SPARQL service descriptions

**Source**: `endpoint/service_description.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/endpoint/service_description.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ReaderMode, Rudof, RudofConfig, ServiceDescriptionFormat
    
    rudof = Rudof(RudofConfig())
    
    service_ttl = """
    @prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
    @prefix : <http://example.org/> .
    
    :svc a sd:Service ;
      sd:endpoint <http://example.org/sparql> ;
      sd:feature sd:BasicFederatedQuery ;
      sd:defaultDataset [ a sd:Dataset ] .
    """
    
    rudof.read_service_description(service_ttl, RDFFormat.Turtle, None, ReaderMode.Lax)
    as_json = rudof.serialize_service_description(ServiceDescriptionFormat.Json)
    as_internal = rudof.serialize_service_description(ServiceDescriptionFormat.Internal)


Data Generation
---------------

Examples for GeneratorConfig and DataGenerator APIs.


Generator Config Core
^^^^^^^^^^^^^^^^^^^^^

Set and read core generator configuration values

**Source**: `generate/generate_config_core.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/generate/generate_config_core.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import CardinalityStrategy, GeneratorConfig, OutputFormat, SchemaFormat
    
    config = GeneratorConfig()
    config.set_entity_count(5)
    config.set_seed(7)
    config.set_output_path("core_output.ttl")
    config.set_output_format(OutputFormat.Turtle)
    config.set_schema_format(SchemaFormat.ShEx)
    config.set_cardinality_strategy(CardinalityStrategy.Balanced)


Generator Config Parallel
^^^^^^^^^^^^^^^^^^^^^^^^^

Configure and read parallel generation settings

**Source**: `generate/generate_config_parallel.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/generate/generate_config_parallel.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import GeneratorConfig
    
    config = GeneratorConfig()
    
    config.set_compress(True)
    config.set_write_stats(True)
    config.set_parallel_writing(True)
    config.set_parallel_file_count(2)
    config.set_worker_threads(2)
    config.set_batch_size(16)
    config.set_parallel_shapes(True)
    config.set_parallel_fields(True)


Generator Config Quality
^^^^^^^^^^^^^^^^^^^^^^^^

Configure locale, quality and distribution settings

**Source**: `generate/generate_config_quality.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/generate/generate_config_quality.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import DataQuality, EntityDistribution, GeneratorConfig
    
    config = GeneratorConfig()
    config.set_entity_distribution(EntityDistribution.Equal)
    config.set_locale("en")
    config.set_data_quality(DataQuality.Medium)


Generator Config Persistence
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Persist GeneratorConfig to TOML and load from TOML/JSON

**Source**: `generate/generate_config_persistence.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/generate/generate_config_persistence.py>`_

**Python Code:**

.. code-block:: python

    import json
    from pathlib import Path
    from tempfile import TemporaryDirectory
    
    from pyrudof import GeneratorConfig
    
    base = GeneratorConfig()
    base.set_entity_count(3)
    base.set_seed(11)
    base.set_output_path("persist_output.ttl")
    
    with TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
    
        toml_path = tmp_path / "generator.toml"
        base.to_toml_file(str(toml_path))
        loaded_toml = GeneratorConfig.from_toml_file(str(toml_path))
        loaded_toml.validate()
    
        json_path = tmp_path / "generator.json"
        json_path.write_text(
            json.dumps(
                {
                    "generation": {
                        "entity_count": 4,
                        "seed": 5,
                        "schema_format": "ShEx",
                        "cardinality_strategy": "Minimum",
                        "entity_distribution": "Equal",
                    },
                    "output": {
                        "path": str(tmp_path / "out.nt"),
                        "format": "NTriples",
                        "compress": False,
                        "write_stats": False,
                        "parallel_writing": False,
                        "parallel_file_count": 1,
                    },
                    "parallel": {
                        "worker_threads": 1,
                        "batch_size": 8,
                        "parallel_shapes": False,
                        "parallel_fields": False,
                    },
                    "field_generators": {
                        "default": {
                            "locale": "en",
                            "quality": "Low",
                        }
                    },
                }
            ),
            encoding="utf-8",
        )
        loaded_json = GeneratorConfig.from_json_file(str(json_path))


DataGenerator Load Methods
^^^^^^^^^^^^^^^^^^^^^^^^^^

Use load_shex_schema, load_shacl_schema, and load_schema_auto

**Source**: `generate/generate_load_methods.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/generate/generate_load_methods.py>`_

**Python Code:**

.. code-block:: python

    from pathlib import Path
    from tempfile import TemporaryDirectory
    
    from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, SchemaFormat
    
    with TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
    
        shex_out = tmp_path / "from_shex.ttl"
        config_shex = GeneratorConfig()
        config_shex.set_entity_count(1)
        config_shex.set_output_path(str(shex_out))
        config_shex.set_output_format(OutputFormat.Turtle)
        config_shex.set_schema_format(SchemaFormat.ShEx)
        gen_shex = DataGenerator(config_shex)
        gen_shex.load_shex_schema("../../../examples/simple.shex")
        gen_shex.generate()
    
        config_shacl = GeneratorConfig()
        config_shacl.set_entity_count(1)
        config_shacl.set_output_path(str(tmp_path / "from_shacl.ttl"))
        config_shacl.set_output_format(OutputFormat.Turtle)
        config_shacl.set_schema_format(SchemaFormat.Shacl)
        gen_shacl = DataGenerator(config_shacl)
        gen_shacl.load_shacl_schema("../../../examples/simple_shacl.ttl")
    
        config_auto = GeneratorConfig()
        config_auto.set_entity_count(1)
        config_auto.set_output_path(str(tmp_path / "auto.ttl"))
        config_auto.set_output_format(OutputFormat.Turtle)
        gen_auto = DataGenerator(config_auto)
        gen_auto.load_schema_auto("../../../examples/simple.shex")


DataGenerator Run Methods
^^^^^^^^^^^^^^^^^^^^^^^^^

Use run_with_format and run to execute generation

**Source**: `generate/generate_run_methods.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/generate/generate_run_methods.py>`_

**Python Code:**

.. code-block:: python

    from pathlib import Path
    from tempfile import TemporaryDirectory
    
    from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, SchemaFormat
    
    with TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
    
        out_with_format = tmp_path / "run_with_format.ttl"
        config1 = GeneratorConfig()
        config1.set_entity_count(1)
        config1.set_output_path(str(out_with_format))
        config1.set_output_format(OutputFormat.Turtle)
        generator1 = DataGenerator(config1)
        generator1.run_with_format("../../../examples/simple.shex", SchemaFormat.ShEx)
    
        out_auto = tmp_path / "run_auto.ttl"
        config2 = GeneratorConfig()
        config2.set_entity_count(1)
        config2.set_output_path(str(out_auto))
        config2.set_output_format(OutputFormat.Turtle)
        generator2 = DataGenerator(config2)
        generator2.run("../../../examples/simple.shex")


Utility & Introspection
-----------------------

Examples for config loading, resets, versioning, and module introspection.


RudofConfig From Path
^^^^^^^^^^^^^^^^^^^^^

Create RudofConfig from a TOML file and initialize Rudof

**Source**: `utility/rudof_config_from_path.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/rudof_config_from_path.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    config = RudofConfig.from_path("example.toml")
    rudof = Rudof(config)
    
    print("RUDOF_CONFIG_FROM_PATH_OK")
    print(type(rudof).__name__)

**Referenced Files:**

- **Config**: `example.toml <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/example.toml>`_


Rudof Update Config
^^^^^^^^^^^^^^^^^^^

Update the configuration of an existing Rudof instance

**Source**: `utility/rudof_update_config.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/rudof_update_config.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    initial = RudofConfig()
    updated = RudofConfig.from_path("example.toml")
    
    rudof = Rudof(initial)
    rudof.update_config(updated)

**Referenced Files:**

- **Config**: `example.toml <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/example.toml>`_


Rudof Reset Methods
^^^^^^^^^^^^^^^^^^^

Call all reset methods exposed by Rudof

**Source**: `utility/rudof_resets.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/rudof_resets.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShaclFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    rudof.read_query("person.sparql")
    
    rudof.reset_data()
    rudof.reset_shex_schema()
    rudof.reset_shex()
    rudof.reset_shacl()
    rudof.reset_shacl_validation()
    rudof.reset_shapemap()
    rudof.reset_query()
    rudof.reset_validation_results()
    rudof.reset_pgschema()
    rudof.reset_typemap()
    rudof.reset_pgschema_validation()
    rudof.reset_all()

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.sm>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.sparql>`_


Rudof Version
^^^^^^^^^^^^^

Get and print the current Rudof version

**Source**: `utility/rudof_version.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/rudof_version.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    print(f"Version: {rudof.get_version()}")


Module Info
^^^^^^^^^^^

Print the installed pyrudof module file path

**Source**: `utility/module_info.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/module_info.py>`_

**Python Code:**

.. code-block:: python

    import pyrudof
    
    print(pyrudof.__file__)


Reset Semantics
^^^^^^^^^^^^^^^

Distinguish the narrow schema-only resets from the broader resets that also clear validation state

**Source**: `utility/reset_semantics.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/reset_semantics.py>`_

**Python Code:**

.. code-block:: python

    """Demonstrates the distinction between the narrow "clear the schema only"
    resets and the broader "reset validation" resets, which also unload the
    schema/shapes graph along with the validation results and ShapeMap.
    """
    from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, ShExFormat, ShapeMapFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    # --- ShEx: reset_shex_schema() clears only the schema ---
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_data(
        'PREFIX : <http://example.org/>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n:alice :name "Alice" ; :age 30 .',
        RDFFormat.Turtle,
    )
    rudof.read_shapemap(":alice@:Person", ShapeMapFormat.Compact)
    rudof.validate_shex()
    
    # reset_shex() is the broad reset: schema, ShapeMap, validator and results all go
    rudof.reset_shex()
    try:
        rudof.serialize_current_shex()
        print("BUG: ShEx schema survived reset_shex()")
    except ValueError:
        print("reset_shex() cleared the schema as documented")
    
    # --- SHACL: reset_shacl() clears only the shapes graph, leaving results alone ---
    shapes = """
    PREFIX : <http://example.org/>
    PREFIX sh: <http://www.w3.org/ns/shacl#>
    
    :PersonShape a sh:NodeShape ;
      sh:targetClass :Person .
    """
    rudof.reset_all()
    rudof.read_shacl(shapes, ShaclFormat.Turtle)
    rudof.read_data("PREFIX : <http://example.org/>\n:alice a :Person .", RDFFormat.Turtle)
    rudof.validate_shacl(ShaclValidationMode.Native)
    
    rudof.reset_shacl()
    try:
        rudof.serialize_shacl()
        print("BUG: SHACL shapes survived reset_shacl()")
    except ValueError:
        print("reset_shacl() cleared the shapes as documented")
    
    # reset_shacl_validation() is the broad reset: shapes and validation results both go
    rudof.reset_all()
    rudof.read_shacl(shapes, ShaclFormat.Turtle)
    rudof.read_data("PREFIX : <http://example.org/>\n:alice a :Person .", RDFFormat.Turtle)
    rudof.validate_shacl(ShaclValidationMode.Native)
    
    rudof.reset_shacl_validation()
    try:
        rudof.serialize_shacl()
        print("BUG: SHACL shapes survived reset_shacl_validation()")
    except ValueError:
        print("reset_shacl_validation() cleared the shapes as documented")

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/person.shex>`_


Error Handling
^^^^^^^^^^^^^^

Catching exceptions raised by rudof operations

**Source**: `utility/error_handling.py <https://github.com/rudof-project/rudof/blob/master/bindings/python/examples/utility/error_handling.py>`_

**Python Code:**

.. code-block:: python

    """Demonstrate catching RudofError exceptions."""
    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    # Trying to parse invalid NTriples raises a ValueError (wrapping RudofError).
    try:
        rudof.read_data("this is not valid RDF at all!!!", RDFFormat.NTriples)
    except Exception as e:
        msg = str(e)
        print(f"Caught RudofError: {msg[:60]}")
    
    # A second attempt with valid data succeeds normally.
    rudof2 = Rudof(RudofConfig())
    rudof2.read_data(
        "<http://example.org/alice> <http://example.org/name> \"Alice\" .\n",
        RDFFormat.NTriples,
    )

