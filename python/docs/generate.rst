Data Generation
=====================================

.. py:currentmodule:: pyrudof

``pyrudof`` includes bindings for ``rudof_generate``, a module that generates
synthetic RDF data from ShEx or SHACL schemas. This is useful for testing,
benchmarking, and creating sample datasets.


Overview
--------

The data generation module provides:

* **Schema-driven generation**: Create data that conforms to your ShEx or SHACL schemas
* **Reproducible results**: Use seeds for deterministic generation
* **Parallel processing**: Generate large datasets efficiently
* **Quality control**: Configure data quality from simple to complex
* **Flexible output**: Support for Turtle and N-Triples formats


Basic Usage
-----------

The simplest way to generate data:

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


Step-by-Step Generation
~~~~~~~~~~~~~~~~~~~~~~~~

You can also load schemas and generate data in separate steps:

.. code-block:: python

    generator = pyrudof.DataGenerator(config)

    # Load schema (choose one method)
    generator.load_shex_schema("schema.shex")
    # OR
    generator.load_shacl_schema("shapes.ttl")
    # OR auto-detect format
    generator.load_schema_auto("schema_file")

    # Then generate
    generator.generate()


Configuration
-------------

Configuration from Python
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    config = pyrudof.GeneratorConfig()

    # Basic settings
    config.set_entity_count(1000)
    config.set_output_path("/tmp/generated_data.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)

    # Reproducibility
    config.set_seed(42)

    # Data quality
    config.set_data_quality(pyrudof.DataQuality.High)
    config.set_locale("en")  # Use English locale for generated text

    # Cardinality handling
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)

    # Performance
    config.set_worker_threads(4)
    config.set_batch_size(100)
    config.set_parallel_writing(True)


Reproducible Generation
-----------------------

Use a seed for reproducible results:

.. code-block:: python

    config = pyrudof.GeneratorConfig()
    config.set_seed(42)
    config.set_entity_count(50)

    generator = pyrudof.DataGenerator(config)
    generator.run("schema.shex")

    # Running again with the same seed produces identical output


.. note::
   Setting a seed ensures that the same configuration always generates the same data,
   which is essential for reproducible testing and benchmarking.


Cardinality Strategies
----------------------

Control how cardinalities are handled when generating relationships:

.. list-table::
   :header-rows: 1
   :widths: 20 80

   * - Strategy
     - Description
   * - ``Minimum``
     - Generate the minimum number of relationships allowed
   * - ``Maximum``
     - Generate the maximum number of relationships allowed
   * - ``Random``
     - Generate a random number within the valid range
   * - ``Balanced``
     - Use a balanced distribution (default, recommended)

.. code-block:: python

    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)


Example with different strategies:

.. code-block:: python

    # Minimum relationships (faster, smaller output)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Minimum)

    # Maximum relationships (slower, larger output, tests edge cases)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Maximum)


Data Quality Levels
-------------------

Configure the realism and complexity of generated data:

.. list-table::
   :header-rows: 1
   :widths: 15 40 45

   * - Level
     - Characteristics
     - Use Case
   * - ``Low``
     - Fast, simple random data
     - Quick testing, performance benchmarks
   * - ``Medium``
     - Realistic patterns
     - Integration testing, demos
   * - ``High``
     - Complex, correlated data
     - Production-like testing, presentations

.. code-block:: python

    # High-quality data with correlations
    config.set_data_quality(pyrudof.DataQuality.High)
    config.set_locale("es")  # Spanish locale


.. tip::
   Use ``DataQuality.Low`` for performance testing and ``DataQuality.High``
   when you need realistic data for demonstrations or integration testing.


Parallel Processing
-------------------

Enable parallel processing for faster generation of large datasets:

.. code-block:: python

    config = pyrudof.GeneratorConfig()
    config.set_entity_count(10000)

    # Enable parallelization
    config.set_worker_threads(4)  # Use 4 CPU cores
    config.set_batch_size(100)    # Process 100 entities per batch
    config.set_parallel_shapes(True)   # Parallel shape processing
    config.set_parallel_fields(True)   # Parallel field generation
    config.set_parallel_writing(True)  # Parallel output writing
    config.set_parallel_file_count(4)  # Write to 4 files simultaneously

    generator = pyrudof.DataGenerator(config)
    generator.run("large_schema.shex")


.. warning::
   Using parallel writing creates multiple output files. You'll need to merge them manually if you need a single file.


Output Formats
--------------

Supported output formats:

* **Turtle** (``OutputFormat.Turtle``) - Human-readable, compact (default)
* **N-Triples** (``OutputFormat.NTriples``) - Line-based, simple format

.. code-block:: python

    # Turtle format (default)
    config.set_output_format(pyrudof.OutputFormat.Turtle)

    # N-Triples format (useful for streaming processing)
    config.set_output_format(pyrudof.OutputFormat.NTriples)

    # Enable compression
    config.set_compress(True)  # Creates .ttl.gz or .nt.gz

    # Generate statistics file
    config.set_write_stats(True)  # Creates output.stats.json


Advanced Example
----------------

Complete example with all features:

.. code-block:: python

    import pyrudof

    # Create configuration
    config = pyrudof.GeneratorConfig()

    # Generation settings
    config.set_entity_count(5000)
    config.set_seed(42)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    config.set_data_quality(pyrudof.DataQuality.High)
    config.set_locale("en")

    # Output settings
    config.set_output_path("./output/data.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_compress(True)
    config.set_write_stats(True)

    # Performance settings
    config.set_worker_threads(8)
    config.set_batch_size(500)
    config.set_parallel_shapes(True)
    config.set_parallel_fields(True)

    # Validate configuration
    config.validate()

    # Create generator and run
    generator = pyrudof.DataGenerator(config)
    generator.run_with_format("schema.shex", pyrudof.SchemaFormat.ShEx)

    print("Generation complete!")
    print(f"Configuration: {config.show()}")


API Reference
-------------

Generator Configuration
~~~~~~~~~~~~~~~~~~~~~~~

.. autoclass:: GeneratorConfig
   :members:
   :undoc-members:
   :special-members: __init__

Data Generator
~~~~~~~~~~~~~~

.. autoclass:: DataGenerator
   :members:
   :undoc-members:
   :special-members: __init__

Formats
^^^^^^^^^^^^^

Schema Format
^^^^^^^^^^^^^

.. autoclass:: SchemaFormat
   :members:
   :undoc-members:

   Schema formats supported by the generator:

   * ``SchemaFormat.ShEx`` - Shape Expressions schema
   * ``SchemaFormat.SHACL`` - SHACL (Shapes Constraint Language) schema

Output Format
^^^^^^^^^^^^^

.. autoclass:: OutputFormat
   :members:
   :undoc-members:

   RDF serialization formats for generated output:

   * ``OutputFormat.Turtle`` - Turtle/Terse RDF Triple Language (.ttl) - Human-readable, compact (default)
   * ``OutputFormat.NTriples`` - N-Triples (.nt) - Line-based, simple format for streaming

Cardinality Strategy
^^^^^^^^^^^^^^^^^^^^

.. autoclass:: CardinalityStrategy
   :members:
   :undoc-members:

   Strategies for handling cardinalities when generating relationships:

   * ``CardinalityStrategy.Minimum`` - Always use minimum cardinality (fastest, smallest output)
   * ``CardinalityStrategy.Maximum`` - Always use maximum cardinality (slowest, largest output, tests edge cases)
   * ``CardinalityStrategy.Random`` - Random value within valid range (unpredictable distribution)
   * ``CardinalityStrategy.Balanced`` - Balanced distribution across range (default, recommended)

Data Quality
^^^^^^^^^^^^

.. autoclass:: DataQuality
   :members:
   :undoc-members:

   Data quality levels controlling realism and complexity:

   * ``DataQuality.Low`` - Simple random data (fastest generation, minimal realism)
   * ``DataQuality.Medium`` - Realistic patterns (moderate speed, good for demos)
   * ``DataQuality.High`` - Complex realistic data with correlations (slower, production-like)

Entity Distribution
^^^^^^^^^^^^^^^^^^^

.. autoclass:: EntityDistribution
   :members:
   :undoc-members:

   Entity distribution strategies across shapes:

   * ``EntityDistribution.Equal`` - Equal distribution of entities across all shapes

   .. note::
      Currently only ``Equal`` distribution is supported.
