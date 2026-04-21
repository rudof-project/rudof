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

**Source**: `rdf_data/rdf_data.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rdf_data/rdf_data.py>`_

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

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


Node Info
^^^^^^^^^

Inspect node neighborhood information in loaded RDF data

**Source**: `rdf_data/node_info.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rdf_data/node_info.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    
    info = rudof.node_info(":alice", [":name"], "outgoing", False, 1)

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


List Endpoints
^^^^^^^^^^^^^^

List known SPARQL endpoints

**Source**: `rdf_data/list_endpoints.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rdf_data/list_endpoints.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    endpoints = rudof.list_endpoints()


SPARQL Queries
--------------

Examples for SELECT, CONSTRUCT and ASK query workflows.


SPARQL SELECT Inline
^^^^^^^^^^^^^^^^^^^^

Run an inline SPARQL SELECT query against loaded RDF data

**Source**: `sparql/sparql_select_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql/sparql_select_inline.py>`_

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

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


SPARQL SELECT File
^^^^^^^^^^^^^^^^^^

Load SPARQL query from file and run it

**Source**: `sparql/sparql_select_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql/sparql_select_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_query("person.sparql")
    rudof.run_query()
    results = rudof.serialize_query_results()

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sparql>`_


SPARQL CONSTRUCT
^^^^^^^^^^^^^^^^

Run a CONSTRUCT query and serialize graph results

**Source**: `sparql/sparql_construct.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql/sparql_construct.py>`_

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

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


SPARQL SELECT Internal
^^^^^^^^^^^^^^^^^^^^^^

Run a SELECT query and serialize results using the default internal format

**Source**: `sparql/sparql_ask.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql/sparql_ask.py>`_

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

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


ShEx Validation
---------------

Examples for reading ShEx schemas, validating data, and serializing schemas/ShapeMaps.


ShEx Validate Inline
^^^^^^^^^^^^^^^^^^^^

Validate inline RDF data against an inline ShEx schema and ShapeMap

**Source**: `shex/shex_validate_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex/shex_validate_inline.py>`_

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

**Source**: `shex/shex_validate_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex/shex_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ShapeMapFormat
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    
    rudof.validate_shex()

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_


ShEx Serialize
^^^^^^^^^^^^^^

Serialize the currently loaded ShEx schema

**Source**: `shex/shex_serialize.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex/shex_serialize.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    
    serialized = rudof.serialize_current_shex()

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_


ShapeMap Roundtrip
^^^^^^^^^^^^^^^^^^

Load and serialize a ShapeMap

**Source**: `shex/shapemap_roundtrip.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex/shapemap_roundtrip.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    serialized = rudof.serialize_shapemap()

**Referenced Files:**

- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_


Compare Schemas
^^^^^^^^^^^^^^^

Compare two ShEx schemas and print comparison output size

**Source**: `shex/compare_schemas.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex/compare_schemas.py>`_

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


Materialize
-----------

Examples for materializing RDF graphs from ShEx schemas with Map semantic actions.

The ``materialize`` operation produces an RDF graph by combining a ShEx schema
(which describes the graph structure via Map semantic actions) with a MapState
(a JSON file that maps each Map-extension IRI key to its concrete RDF node value).


Materialize Inline
^^^^^^^^^^^^^^^^^^

Materialize an RDF graph from an inline ShEx schema and a MapState built in Python

**Source**: `materialize/materialize_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/materialize/materialize_inline.py>`_

**Python Code:**

.. code-block:: python

    import json
    import os
    import tempfile

    from pyrudof import ResultDataFormat, Rudof, RudofConfig, ShExFormat

    rudof = Rudof(RudofConfig())

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

    map_state = {
        "http://example.org/name": {"Iri": "http://example.org/Alice"}
    }

    rudof.read_shex(schema, ShExFormat.ShExJ)

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

**Source**: `materialize/materialize_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/materialize/materialize_file.py>`_

**Python Code:**

.. code-block:: python

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

- **Schema**: `materialize/person_map.shexj <https://github.com/rudof-project/rudof/blob/master/python/examples/materialize/person_map.shexj>`_
- **MapState**: `materialize/person_map_state.json <https://github.com/rudof-project/rudof/blob/master/python/examples/materialize/person_map_state.json>`_


SHACL Validation
----------------

Examples for SHACL loading, validation, extraction from data, and serialization.


SHACL Validate Inline
^^^^^^^^^^^^^^^^^^^^^

Validate inline RDF data with inline SHACL shapes

**Source**: `shacl/shacl_validate_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl/shacl_validate_inline.py>`_

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

**Source**: `shacl/shacl_validate_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl/shacl_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    rudof.read_data("timbl.ttl", RDFFormat.Turtle)
    rudof.validate_shacl(ShaclValidationMode.Native)

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl_shapes.ttl>`_
- **Data**: `timbl.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl.ttl>`_


SHACL From Data
^^^^^^^^^^^^^^^

Extract SHACL shapes from current RDF data and validate

**Source**: `shacl/shacl_from_data.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl/shacl_from_data.py>`_

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

**Source**: `shacl/shacl_serialize.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl/shacl_serialize.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ShaclFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    serialized = rudof.serialize_shacl()

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl_shapes.ttl>`_


DCTAP
-----

Examples for reading DCTAP profiles from inline content or files.


Read DCTAP
^^^^^^^^^^

Read DCTAP from inline CSV and from file

**Source**: `dctap/dctap_read.py <https://github.com/rudof-project/rudof/blob/master/python/examples/dctap/dctap_read.py>`_

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


Service Description
-------------------

Examples for service description parsing and serialization.


Service Description
^^^^^^^^^^^^^^^^^^^

Read and serialize SPARQL service descriptions

**Source**: `endpoint/service_description.py <https://github.com/rudof-project/rudof/blob/master/python/examples/endpoint/service_description.py>`_

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

**Source**: `generate/generate_config_core.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate/generate_config_core.py>`_

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

**Source**: `generate/generate_config_parallel.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate/generate_config_parallel.py>`_

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

**Source**: `generate/generate_config_quality.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate/generate_config_quality.py>`_

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

**Source**: `generate/generate_config_persistence.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate/generate_config_persistence.py>`_

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

**Source**: `generate/generate_load_methods.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate/generate_load_methods.py>`_

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
        gen_shex.load_shex_schema("../../examples/simple.shex")
        gen_shex.generate()
    
        config_shacl = GeneratorConfig()
        config_shacl.set_entity_count(1)
        config_shacl.set_output_path(str(tmp_path / "from_shacl.ttl"))
        config_shacl.set_output_format(OutputFormat.Turtle)
        config_shacl.set_schema_format(SchemaFormat.Shacl)
        gen_shacl = DataGenerator(config_shacl)
        gen_shacl.load_shacl_schema("../../examples/simple_shacl.ttl")
    
        config_auto = GeneratorConfig()
        config_auto.set_entity_count(1)
        config_auto.set_output_path(str(tmp_path / "auto.ttl"))
        config_auto.set_output_format(OutputFormat.Turtle)
        gen_auto = DataGenerator(config_auto)
        gen_auto.load_schema_auto("../../examples/simple.shex")


DataGenerator Run Methods
^^^^^^^^^^^^^^^^^^^^^^^^^

Use run_with_format and run to execute generation

**Source**: `generate/generate_run_methods.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate/generate_run_methods.py>`_

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
        generator1.run_with_format("../../examples/simple.shex", SchemaFormat.ShEx)
    
        out_auto = tmp_path / "run_auto.ttl"
        config2 = GeneratorConfig()
        config2.set_entity_count(1)
        config2.set_output_path(str(out_auto))
        config2.set_output_format(OutputFormat.Turtle)
        generator2 = DataGenerator(config2)
        generator2.run("../../examples/simple.shex")


Utility & Introspection
-----------------------

Examples for config loading, resets, versioning, and module introspection.


RudofConfig From Path
^^^^^^^^^^^^^^^^^^^^^

Create RudofConfig from a TOML file and initialize Rudof

**Source**: `utility/rudof_config_from_path.py <https://github.com/rudof-project/rudof/blob/master/python/examples/utility/rudof_config_from_path.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    config = RudofConfig.from_path("../../rudof_lib/src/default_config.toml")
    rudof = Rudof(config)
    
    print("RUDOF_CONFIG_FROM_PATH_OK")
    print(type(rudof).__name__)


Rudof Update Config
^^^^^^^^^^^^^^^^^^^

Update the configuration of an existing Rudof instance

**Source**: `utility/rudof_update_config.py <https://github.com/rudof-project/rudof/blob/master/python/examples/utility/rudof_update_config.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    initial = RudofConfig()
    updated = RudofConfig.from_path("../../rudof_lib/src/default_config.toml")
    
    rudof = Rudof(initial)
    rudof.update_config(updated)


Rudof Reset Methods
^^^^^^^^^^^^^^^^^^^

Call all reset methods exposed by Rudof

**Source**: `utility/rudof_resets.py <https://github.com/rudof-project/rudof/blob/master/python/examples/utility/rudof_resets.py>`_

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
    rudof.reset_shex()
    rudof.reset_shacl()
    rudof.reset_shapemap()
    rudof.reset_query()
    rudof.reset_validation_results()
    rudof.reset_all()

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sparql>`_


Rudof Version
^^^^^^^^^^^^^

Get and print the current Rudof version

**Source**: `utility/rudof_version.py <https://github.com/rudof-project/rudof/blob/master/python/examples/utility/rudof_version.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    print(f"Version: {rudof.get_version()}")


Module Info
^^^^^^^^^^^

Print the installed pyrudof module file path

**Source**: `utility/module_info.py <https://github.com/rudof-project/rudof/blob/master/python/examples/utility/module_info.py>`_

**Python Code:**

.. code-block:: python

    import pyrudof
    
    print(pyrudof.__file__)


Error Handling
^^^^^^^^^^^^^^

Catching exceptions raised by rudof operations

**Source**: `utility/error_handling.py <https://github.com/rudof-project/rudof/blob/master/python/examples/utility/error_handling.py>`_

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

