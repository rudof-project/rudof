Data Generation
=====================================

.. py:currentmodule:: pyrudof

``pyrudof`` includes bindings for ``rudof_generate``, a module that generates
synthetic RDF data from ShEx or SHACL schemas. This is useful for testing,
benchmarking, and creating sample datasets.


Basic Usage
-----------

.. code-block:: python

    import pyrudof

    # 1. Configure
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(100)
    config.set_output_path("output.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)

    # 2. Create generator
    generator = pyrudof.DataGenerator(config)

    # 3. Load schema and generate
    generator.run("schema.shex")

You can also load schemas step by step:

.. code-block:: python

    generator.load_shex_schema("schema.shex")
    # or
    generator.load_shacl_schema("shapes.ttl")
    # or auto-detect
    generator.load_schema_auto("schema_file")

    # Then generate
    generator.generate()


Configuration from Files
------------------------

.. code-block:: python

    # Load from TOML
    config = pyrudof.GeneratorConfig.from_toml_file("config.toml")

    # Load from JSON
    config = pyrudof.GeneratorConfig.from_json_file("config.json")

    # Save configuration for later use
    config.to_toml_file("saved_config.toml")


Reproducible Generation
-----------------------

Use a seed for reproducible results:

.. code-block:: python

    config = pyrudof.GeneratorConfig()
    config.set_seed(42)
    config.set_entity_count(50)


Cardinality Strategies
----------------------

Control how cardinalities are handled when generating relationships:

- ``CardinalityStrategy.Minimum`` — Generate the minimum number of relationships
- ``CardinalityStrategy.Maximum`` — Generate the maximum number of relationships
- ``CardinalityStrategy.Random`` — Random number within the valid range
- ``CardinalityStrategy.Balanced`` — Balanced distribution (default)

.. code-block:: python

    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)


Data Quality Levels
-------------------

- ``DataQuality.Low`` — Fast, simple data
- ``DataQuality.Medium`` — Realistic patterns
- ``DataQuality.High`` — Complex, correlated data

.. code-block:: python

    config.set_data_quality(pyrudof.DataQuality.High)


Parallel Processing
-------------------

.. code-block:: python

    config.set_worker_threads(4)
    config.set_batch_size(100)
    config.set_parallel_writing(True)
    config.set_parallel_file_count(4)


API Reference
-------------

.. autoclass:: GeneratorConfig
    :members:
    :undoc-members:

.. autoclass:: DataGenerator
    :members:
    :undoc-members:

.. autoclass:: SchemaFormat
    :members:

.. autoclass:: OutputFormat
    :members:

.. autoclass:: CardinalityStrategy
    :members:

.. autoclass:: DataQuality
    :members:

.. autoclass:: EntityDistribution
    :members:
