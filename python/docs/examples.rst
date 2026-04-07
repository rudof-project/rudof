Examples
========

This page contains validated Python examples that demonstrate pyrudof functionality.
Each example includes executable Python code that can be copied and pasted into a
Jupyter notebook or Python script, along with links to any referenced files.


ShEx Validation
---------------

Examples for reading ShEx schemas, validating data, and serializing schemas/ShapeMaps.


ShEx Validate Inline
^^^^^^^^^^^^^^^^^^^^

Validate inline RDF data against an inline ShEx schema and ShapeMap

**Source**: `shex_validate_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex_validate_inline.py>`_

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
    
    print("SHEX_INLINE_VALIDATION_OK")


ShEx Validate Files
^^^^^^^^^^^^^^^^^^^

Validate RDF data from files against a ShEx schema and ShapeMap

**Source**: `shex_validate_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ShapeMapFormat
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    
    rudof.validate_shex()
    
    print("SHEX_FILE_VALIDATION_OK")

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_


ShEx Serialize
^^^^^^^^^^^^^^

Serialize the currently loaded ShEx schema

**Source**: `shex_serialize.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex_serialize.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    
    serialized = rudof.serialize_current_shex()
    
    print("SHEX_SERIALIZE_OK")
    print(f"Contains Person: {'Person' in serialized}")

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_


ShapeMap Roundtrip
^^^^^^^^^^^^^^^^^^

Load and serialize a ShapeMap

**Source**: `shapemap_roundtrip.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shapemap_roundtrip.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    serialized = rudof.serialize_shapemap()
    
    print("SHAPEMAP_ROUNDTRIP_OK")
    print(f"Serialized chars: {len(serialized)}")

**Referenced Files:**

- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_


Compare Schemas
^^^^^^^^^^^^^^^

Compare two ShEx schemas and print comparison output size

**Source**: `compare_schemas.py <https://github.com/rudof-project/rudof/blob/master/python/examples/compare_schemas.py>`_

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
    
    print("COMPARE_SCHEMAS_OK")
    print(f"Comparison chars: {len(comparison)}")


SHACL Validation
----------------

Examples for SHACL loading, validation, extraction from data, and serialization.


SHACL Validate Inline
^^^^^^^^^^^^^^^^^^^^^

Validate inline RDF data with inline SHACL shapes

**Source**: `shacl_validate_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_validate_inline.py>`_

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
    
    print("SHACL_INLINE_VALIDATION_OK")


SHACL Validate Files
^^^^^^^^^^^^^^^^^^^^

Validate RDF data from files against SHACL shapes

**Source**: `shacl_validate_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, ShaclFormat, ShaclValidationMode, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    rudof.read_data("timbl.ttl", RDFFormat.Turtle)
    rudof.validate_shacl(ShaclValidationMode.Native)
    
    print("SHACL_FILE_VALIDATION_OK")

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl_shapes.ttl>`_
- **Data**: `timbl.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl.ttl>`_


SHACL From Data
^^^^^^^^^^^^^^^

Extract SHACL shapes from current RDF data and validate

**Source**: `shacl_from_data.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_from_data.py>`_

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
    
    print("SHACL_FROM_DATA_OK")


SHACL Serialize
^^^^^^^^^^^^^^^

Serialize the currently loaded SHACL graph

**Source**: `shacl_serialize.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_serialize.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import ShaclFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    serialized = rudof.serialize_shacl()
    
    print("SHACL_SERIALIZE_OK")
    print(f"Serialized chars: {len(serialized)}")

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl_shapes.ttl>`_


RDF Data Handling
-----------------

Examples for RDF loading, serialization, and node inspection.


RDF Read and Serialize
^^^^^^^^^^^^^^^^^^^^^^

Read RDF data, merge extra triples, and serialize

**Source**: `rdf_data.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rdf_data.py>`_

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
    
    print("RDF_SERIALIZE_OK")
    print(f"Serialized chars: {len(serialized)}")

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


Node Info
^^^^^^^^^

Inspect node neighborhood information in loaded RDF data

**Source**: `node_info.py <https://github.com/rudof-project/rudof/blob/master/python/examples/node_info.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    
    info = rudof.node_info(":alice", [":name"], "outgoing", False, 1)
    
    print("NODE_INFO_OK")
    print(info.splitlines()[0])

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


List Endpoints
^^^^^^^^^^^^^^

List known SPARQL endpoints

**Source**: `list_endpoints.py <https://github.com/rudof-project/rudof/blob/master/python/examples/list_endpoints.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    endpoints = rudof.list_endpoints()
    
    print("LIST_ENDPOINTS_OK")
    print(f"ENDPOINTS_COUNT={len(endpoints)}")


DCTAP
-----

Examples for reading DCTAP profiles from inline content or files.


Read DCTAP
^^^^^^^^^^

Read DCTAP from inline CSV and from file

**Source**: `dctap_read.py <https://github.com/rudof-project/rudof/blob/master/python/examples/dctap_read.py>`_

**Python Code:**

.. code-block:: python

    from pathlib import Path
    from tempfile import TemporaryDirectory
    
    from pyrudof import DCTapFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    csv_text = "shapeId,propertyId\n:Person,:name\n"
    rudof.read_dctap(csv_text)
    print("DCTAP_INLINE_OK")
    
    with TemporaryDirectory() as tmpdir:
        csv_path = Path(tmpdir) / "profile.csv"
        csv_path.write_text(csv_text, encoding="utf-8")
        rudof.read_dctap(str(csv_path), DCTapFormat.CSV)
        print("DCTAP_FILE_OK")


SPARQL Queries
--------------

Examples for SELECT, CONSTRUCT and ASK query workflows.


SPARQL SELECT Inline
^^^^^^^^^^^^^^^^^^^^

Run an inline SPARQL SELECT query against loaded RDF data

**Source**: `sparql_select_inline.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql_select_inline.py>`_

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
    
    print("SPARQL_SELECT_INLINE_OK")
    print(f"Result chars: {len(results)}")

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


SPARQL SELECT File
^^^^^^^^^^^^^^^^^^

Load SPARQL query from file and run it

**Source**: `sparql_select_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql_select_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import RDFFormat, Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_query("person.sparql")
    rudof.run_query()
    results = rudof.serialize_query_results()
    
    print("SPARQL_SELECT_FILE_OK")
    print(f"Result chars: {len(results)}")

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sparql>`_


SPARQL CONSTRUCT
^^^^^^^^^^^^^^^^

Run a CONSTRUCT query and serialize graph results

**Source**: `sparql_construct.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql_construct.py>`_

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
    
    print("SPARQL_CONSTRUCT_OK")
    print(f"Result chars: {len(results)}")

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


SPARQL SELECT Internal
^^^^^^^^^^^^^^^^^^^^^^

Run a SELECT query and serialize results using the default internal format

**Source**: `sparql_ask.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql_ask.py>`_

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
    
    print("SPARQL_INTERNAL_SELECT_OK")
    print(f"Result chars: {len(results)}")

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_


Service Description
-------------------

Examples for service description parsing and serialization.


Service Description
^^^^^^^^^^^^^^^^^^^

Read and serialize SPARQL service descriptions

**Source**: `service_description.py <https://github.com/rudof-project/rudof/blob/master/python/examples/service_description.py>`_

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
    
    print("SERVICE_DESCRIPTION_OK")
    print(f"JSON chars: {len(as_json)}")
    print(f"Internal chars: {len(as_internal)}")


Data Generation
---------------

Examples for GeneratorConfig and DataGenerator APIs.


Generator Config Core
^^^^^^^^^^^^^^^^^^^^^

Set and read core generator configuration values

**Source**: `generate_config_core.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_config_core.py>`_

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
    
    print("GEN_CONFIG_CORE_OK")
    print(f"Entity count: {config.get_entity_count()}")
    print(f"Seed: {config.get_seed()}")
    print(f"Output path: {config.get_output_path()}")


Generator Config Parallel
^^^^^^^^^^^^^^^^^^^^^^^^^

Configure and read parallel generation settings

**Source**: `generate_config_parallel.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_config_parallel.py>`_

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
    
    print("GEN_CONFIG_PARALLEL_OK")
    print(f"Compress: {config.get_compress()}")
    print(f"Write stats: {config.get_write_stats()}")
    print(f"Parallel writing: {config.get_parallel_writing()}")
    print(f"Parallel file count: {config.get_parallel_file_count()}")
    print(f"Worker threads: {config.get_worker_threads()}")
    print(f"Batch size: {config.get_batch_size()}")
    print(f"Parallel shapes: {config.get_parallel_shapes()}")
    print(f"Parallel fields: {config.get_parallel_fields()}")


Generator Config Quality
^^^^^^^^^^^^^^^^^^^^^^^^

Configure locale, quality and distribution settings

**Source**: `generate_config_quality.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_config_quality.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import DataQuality, EntityDistribution, GeneratorConfig
    
    config = GeneratorConfig()
    config.set_entity_distribution(EntityDistribution.Equal)
    config.set_locale("en")
    config.set_data_quality(DataQuality.Medium)
    
    print("GEN_CONFIG_QUALITY_OK")
    print(f"Locale: {config.get_locale()}")


Generator Config Persistence
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Persist GeneratorConfig to TOML and load from TOML/JSON

**Source**: `generate_config_persistence.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_config_persistence.py>`_

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
    
        print("GEN_CONFIG_PERSISTENCE_OK")
        print(f"TOML entity count: {loaded_toml.get_entity_count()}")
        print(f"JSON entity count: {loaded_json.get_entity_count()}")
        print(f"Show has GeneratorConfig: {'GeneratorConfig' in base.show()}")


DataGenerator Load Methods
^^^^^^^^^^^^^^^^^^^^^^^^^^

Use load_shex_schema, load_shacl_schema, and load_schema_auto

**Source**: `generate_load_methods.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_load_methods.py>`_

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
    
        print("GEN_LOAD_METHODS_OK")
        print(f"Generated shex output exists: {shex_out.exists()}")
        print("Loaded SHACL schema")
        print("Loaded schema with auto-detection")


DataGenerator Run Methods
^^^^^^^^^^^^^^^^^^^^^^^^^

Use run_with_format and run to execute generation

**Source**: `generate_run_methods.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_run_methods.py>`_

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
    
        print("GEN_RUN_METHODS_OK")
        print(f"run_with_format output exists: {out_with_format.exists()}")
        print(f"run output exists: {out_auto.exists()}")


Utility & Introspection
-----------------------

Examples for config loading, resets, versioning, and module introspection.


RudofConfig From Path
^^^^^^^^^^^^^^^^^^^^^

Create RudofConfig from a TOML file and initialize Rudof

**Source**: `rudof_config_from_path.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rudof_config_from_path.py>`_

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

**Source**: `rudof_update_config.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rudof_update_config.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    initial = RudofConfig()
    updated = RudofConfig.from_path("../../rudof_lib/src/default_config.toml")
    
    rudof = Rudof(initial)
    rudof.update_config(updated)
    
    print("RUDOF_UPDATE_CONFIG_OK")


Rudof Reset Methods
^^^^^^^^^^^^^^^^^^^

Call all reset methods exposed by Rudof

**Source**: `rudof_resets.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rudof_resets.py>`_

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
    
    print("RUDOF_RESETS_OK")

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sparql>`_


Rudof Version
^^^^^^^^^^^^^

Get and print the current Rudof version

**Source**: `rudof_version.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rudof_version.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    version = rudof.get_version()
    
    print("RUDOF_VERSION_OK")
    print(f"Version: {version}")


Module Info
^^^^^^^^^^^

Print the installed pyrudof module file path

**Source**: `module_info.py <https://github.com/rudof-project/rudof/blob/master/python/examples/module_info.py>`_

**Python Code:**

.. code-block:: python

    import pyrudof
    
    print("MODULE_FILE")
    print(pyrudof.__file__)


Show pyrudof Classes
^^^^^^^^^^^^^^^^^^^^

List classes available in pyrudof

**Source**: `show_pyrudof_classes.py <https://github.com/rudof-project/rudof/blob/master/python/examples/show_pyrudof_classes.py>`_

**Python Code:**

.. code-block:: python

    import inspect
    
    import pyrudof
    
    classes = sorted(name for name, obj in inspect.getmembers(pyrudof, inspect.isclass))
    
    print("PYRUDOF_CLASSES_OK")
    print(f"Contains Rudof: {'Rudof' in classes}")
    print(f"Contains RudofConfig: {'RudofConfig' in classes}")

