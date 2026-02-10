pyrudof |release|
====================

``pyrudof`` is a Python library for working with RDF data that implements `Shape Expressions <https://shex.io/>`_, `SHACL <https://www.w3.org/TR/shacl/>`_, `DCTAP <https://www.dublincore.org/specifications/dctap/>`_, and other technologies in the RDF ecosystem.

.. important::
   ``pyrudof`` is under active development. APIs may change between versions.

Key features:

* **Validation**: Validate RDF data against ShEx and SHACL schemas
* **Schema Conversion**: Convert between ShEx, SHACL, and DCTAP formats
* **SPARQL Queries**: Execute queries against local data or remote endpoints
* **Visualization**: Generate UML-like diagrams from schemas and data
* **Data Generation**: Generate synthetic RDF data from schemas


Installation
------------

``pyrudof`` is available on `PyPI <https://pypi.org/project/pyrudof/>`_ and can be installed using pip:

.. code-block:: bash

    pip install pyrudof


Quick Links
-----------

* :doc:`library` - Complete API reference
* :doc:`generate` - Data generation guide
* :doc:`examples` - Practical examples
* `GitHub Repository <https://github.com/rudof-project/rudof>`_
* `Issue Tracker <https://github.com/rudof-project/rudof/issues>`_


Documentation Contents
----------------------

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   library
   generate
   examples


Quick Start Examples
--------------------

Below are a few quick examples to get you started with ``pyrudof``. For more comprehensive examples and use cases, see the :doc:`examples` section.

SHACL Validation
~~~~~~~~~~~~~~~~

Validate RDF data against SHACL shapes:

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShaclValidationMode, ShapesGraphSource

    rudof = Rudof(RudofConfig())

    # Load SHACL shapes
    rudof.read_shacl_str("""
        prefix : <http://example.org/>
        prefix sh: <http://www.w3.org/ns/shacl#>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :Person a sh:NodeShape;
            sh:targetNode :ok, :ko ;
            sh:property [
                sh:path :name ;
                sh:minCount 1;
                sh:maxCount 1;
                sh:datatype xsd:string ;
            ] .
    """)

    # Load RDF data
    rudof.read_data_str("""
        prefix : <http://example.org/>
        :ok :name "alice" .
        :ko :name 1 .
    """)

    # Validate and display results
    result = rudof.validate_shacl(
        ShaclValidationMode.Native,
        ShapesGraphSource.CurrentSchema
    )
    print(result.show_as_table())


Synthetic Data Generation
~~~~~~~~~~~~~~~~~~~~~~~~~~

Generate synthetic RDF data from schemas:

.. code-block:: python

    from pyrudof import GeneratorConfig, DataGenerator, OutputFormat, CardinalityStrategy

    # Configure the generator
    config = GeneratorConfig()
    config.set_entity_count(100)
    config.set_output_path("output.ttl")
    config.set_output_format(OutputFormat.Turtle)
    config.set_seed(42)  # For reproducible results
    config.set_cardinality_strategy(CardinalityStrategy.Balanced)

    # Generate data from schema
    generator = DataGenerator(config)
    generator.run("schema.shex")


.. seealso::
   See :doc:`generate` for advanced configuration options and parallel processing.


SPARQL Queries
~~~~~~~~~~~~~~

Execute SPARQL queries against remote endpoints:

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, QueryResultFormat

    rudof = Rudof(RudofConfig())
    rudof.use_endpoint("https://query.wikidata.org/sparql")

    result = rudof.run_query_construct_str("""
        PREFIX wd: <http://www.wikidata.org/entity/>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

        CONSTRUCT {
            ?p a :Person ;
               :name ?personLabel .
        }
        WHERE {
            ?p wdt:P31 wd:Q5 ;
               rdfs:label ?personLabel .
            FILTER (lang(?personLabel) = "en")
        }
        LIMIT 5
    """, QueryResultFormat.Turtle)

    print(result)


Schema Conversion
~~~~~~~~~~~~~~~~~

Convert between different schema formats:

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormatter

    rudof = Rudof(RudofConfig())

    # Read DCTAP schema
    rudof.read_dctap_str("""
    shapeId,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
    Person,name,true,false,xsd:string,
    ,birthdate,false,false,xsd:date,
    ,enrolledIn,false,true,,Course
    Course,name,true,false,xsd:string,
    ,student,false,true,,Person
    """)

    # Convert to ShEx
    rudof.dctap2shex()
    result = rudof.serialize_current_shex(ShExFormatter())
    print(result)