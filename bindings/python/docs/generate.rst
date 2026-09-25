Data generation
===============

.. py:currentmodule:: pyrudof

``pyrudof`` includes bindings for ``rudof_generate``, which produces synthetic RDF data
from a ShEx or SHACL schema.

The module offers:

* **Schema-driven generation** — data that conforms to your ShEx or SHACL schemas.
* **Reproducible results** — seed the generator for deterministic output.
* **Parallel processing** — generate large datasets efficiently.
* **Quality control** — from fast random filler to realistic correlated data.
* **Flexible output** — Turtle and N-Triples, optionally compressed.

Basic usage
-----------

.. code-block:: python

    import pyrudof

    # 1. Configure
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(100)
    config.set_output_path("output.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)

    # 2. Create the generator
    generator = pyrudof.DataGenerator(config)

    # 3. Load a schema and generate
    generator.run("schema.shex")

Anything that goes wrong raises :class:`GenerateError`:

.. code-block:: python

    from pyrudof import GenerateError

    try:
        generator.run("schema.shex")
    except GenerateError as e:
        print(f"generation failed: {e}")

Step by step
~~~~~~~~~~~~

Loading and generating can also be separate steps:

.. code-block:: python

    generator = pyrudof.DataGenerator(config)

    # Load the schema (choose one)
    generator.load_shex_schema("schema.shex")
    generator.load_shacl_schema("shapes.ttl")
    generator.load_schema_auto("schema_file")   # detect the format

    # Then generate
    generator.generate()


Configuration
-------------

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
    config.set_locale("en")

    # Cardinality handling
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)

    # Performance
    config.set_worker_threads(4)
    config.set_batch_size(100)
    config.set_parallel_writing(True)

A configuration can also be loaded from a file with
:meth:`GeneratorConfig.from_toml_file` or :meth:`GeneratorConfig.from_json_file`, and
written back out with :meth:`GeneratorConfig.to_toml_file`.
:meth:`GeneratorConfig.validate` checks it before use, and :meth:`GeneratorConfig.show`
renders it for logging.


Reproducible generation
-----------------------

.. code-block:: python

    config = pyrudof.GeneratorConfig()
    config.set_seed(42)
    config.set_entity_count(50)

    generator = pyrudof.DataGenerator(config)
    generator.run("schema.shex")

    # Running again with the same seed produces identical output.

.. note::
   A seed makes the same configuration always generate the same data, which is what makes
   generated data usable as a test fixture or a benchmark input.


Cardinality strategies
----------------------

:class:`CardinalityStrategy` controls how many relationships are produced when a shape
allows a range:

.. code-block:: python

    # Minimum relationships: faster, smaller output
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Minimum)

    # Maximum relationships: slower, larger output, exercises edge cases
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Maximum)

    # Balanced distribution across the range (the default)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)


Data quality levels
-------------------

:class:`DataQuality` trades generation speed against realism:

.. list-table::
   :header-rows: 1
   :widths: 15 40 45

   * - Level
     - Characteristics
     - Use case
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

    config.set_data_quality(pyrudof.DataQuality.High)
    config.set_locale("es")

.. tip::
   Use ``DataQuality.Low`` for performance testing and ``DataQuality.High`` when the data
   has to look plausible to a person.


Parallel processing
-------------------

.. code-block:: python

    config = pyrudof.GeneratorConfig()
    config.set_entity_count(10000)

    config.set_worker_threads(4)       # use 4 CPU cores
    config.set_batch_size(100)         # 100 entities per batch
    config.set_parallel_shapes(True)   # parallel shape processing
    config.set_parallel_fields(True)   # parallel field generation
    config.set_parallel_writing(True)  # parallel output writing
    config.set_parallel_file_count(4)  # write 4 files at once

    generator = pyrudof.DataGenerator(config)
    generator.run("large_schema.shex")

.. warning::
   Parallel writing produces multiple output files. Merge them yourself if you need one.

The generator runs on a process-wide async runtime shared by every
:class:`DataGenerator`, rather than one runtime per instance, so creating several
generators does not multiply the thread pools. Generation releases the GIL for its
duration, so it does not block other Python threads.


Output formats
--------------

.. code-block:: python

    # Turtle: human-readable, compact (the default)
    config.set_output_format(pyrudof.OutputFormat.Turtle)

    # N-Triples: line-based, good for streaming
    config.set_output_format(pyrudof.OutputFormat.NTriples)

    config.set_compress(True)     # write .ttl.gz / .nt.gz
    config.set_write_stats(True)  # write output.stats.json


A complete example
------------------

.. code-block:: python

    import pyrudof

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

    config.validate()

    generator = pyrudof.DataGenerator(config)
    generator.run_with_format("schema.shex", pyrudof.SchemaFormat.ShEx)

    print("Generation complete!")
    print(f"Configuration: {config.show()}")


API reference
-------------

Generator configuration
~~~~~~~~~~~~~~~~~~~~~~~

.. autoclass:: GeneratorConfig
   :members:
   :undoc-members:
   :special-members: __init__

Data generator
~~~~~~~~~~~~~~

.. autoclass:: DataGenerator
   :members:
   :undoc-members:
   :special-members: __init__

Enums
~~~~~

.. autoclass:: SchemaFormat
   :members:
   :undoc-members:

.. autoclass:: OutputFormat
   :members:
   :undoc-members:

.. autoclass:: CardinalityStrategy
   :members:
   :undoc-members:

.. autoclass:: DataQuality
   :members:
   :undoc-members:

.. autoclass:: EntityDistribution
   :members:
   :undoc-members:
