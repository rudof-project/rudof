API Reference
=============

.. py:currentmodule:: pyrudof

This page contains the complete API reference for ``pyrudof``. The library provides Python bindings for performing RDF operations including validation, schema conversion, SPARQL queries, and data generation.


Core Classes
------------

Rudof
~~~~~

.. autoclass:: Rudof
   :members:
   :undoc-members:
   :special-members: __init__, __repr__

RudofConfig
~~~~~~~~~~~

.. autoclass:: RudofConfig
   :members:
   :undoc-members:
   :special-members: __init__, __repr__

RudofError
~~~~~~~~~~

.. autoclass:: RudofError
   :members:
   :undoc-members:
   :special-members: __str__, __repr__


Data Formats
------------

RDF Formats
~~~~~~~~~~~

.. autoclass:: RDFFormat
   :members:
   :undoc-members:

   Supported RDF serialization formats:

   * ``RDFFormat.Turtle`` - Terse RDF Triple Language (.ttl)
   * ``RDFFormat.NTriples`` - Line-based RDF format (.nt)
   * ``RDFFormat.RdfXml`` - XML-based RDF syntax (.rdf, .owl)
   * ``RDFFormat.TriG`` - Turtle with named graphs (.trig)
   * ``RDFFormat.N3`` - Notation3 (.n3)
   * ``RDFFormat.NQuads`` - N-Triples with named graphs (.nq)
   * ``RDFFormat.JsonLd`` - JSON-LD format (.jsonld)

.. autoclass:: ResultDataFormat
   :members:
   :undoc-members:

   Output formats for serialized RDF data:

   * ``ResultDataFormat.Turtle`` - Turtle
   * ``ResultDataFormat.NTriples`` - N-Triples
   * ``ResultDataFormat.RdfXml`` - RDF/XML
   * ``ResultDataFormat.TriG`` - TriG
   * ``ResultDataFormat.N3`` - Notation3
   * ``ResultDataFormat.NQuads`` - N-Quads
   * ``ResultDataFormat.Compact`` - Compact representation (default)
   * ``ResultDataFormat.Json`` - JSON
   * ``ResultDataFormat.PlantUML`` - PlantUML diagram
   * ``ResultDataFormat.Svg`` - SVG image
   * ``ResultDataFormat.Png`` - PNG image

ShEx Formats
~~~~~~~~~~~~

.. autoclass:: ShExFormat
   :members:
   :undoc-members:

   Supported ShEx schema formats:

   * ``ShExFormat.ShExC`` - ShEx Compact Syntax (human-readable, .shex)
   * ``ShExFormat.ShExJ`` - ShEx JSON format (.json)
   * ``ShExFormat.Turtle`` - ShEx in RDF/Turtle (.ttl)

.. autoclass:: ResultShexValidationFormat
   :members:
   :undoc-members:

   Output formats for ShEx validation results:

   * ``ResultShexValidationFormat.Details`` - Human-readable details (default)
   * ``ResultShexValidationFormat.Turtle`` - Turtle
   * ``ResultShexValidationFormat.NTriples`` - N-Triples
   * ``ResultShexValidationFormat.RdfXml`` - RDF/XML
   * ``ResultShexValidationFormat.TriG`` - TriG
   * ``ResultShexValidationFormat.N3`` - Notation3
   * ``ResultShexValidationFormat.NQuads`` - N-Quads
   * ``ResultShexValidationFormat.Compact`` - Compact
   * ``ResultShexValidationFormat.Json`` - JSON
   * ``ResultShexValidationFormat.Csv`` - CSV

SHACL Formats
~~~~~~~~~~~~~

.. autoclass:: ShaclFormat
   :members:
   :undoc-members:

   SHACL shapes graph serialization formats (all RDF-based):

   * ``ShaclFormat.Turtle`` - Turtle format (.ttl)
   * ``ShaclFormat.NTriples`` - N-Triples format (.nt)
   * ``ShaclFormat.RdfXml`` - RDF/XML format (.rdf)
   * ``ShaclFormat.TriG`` - TriG format (.trig)
   * ``ShaclFormat.N3`` - Notation3 format (.n3)
   * ``ShaclFormat.NQuads`` - N-Quads format (.nq)

ShapeMap Formats
~~~~~~~~~~~~~~~~

.. autoclass:: ShapeMapFormat
   :members:
   :undoc-members:

   ShapeMap serialization formats:

   * ``ShapeMapFormat.Compact`` - Compact ShapeMap syntax (human-readable)
   * ``ShapeMapFormat.Json`` - JSON representation

Other Formats
~~~~~~~~~~~~~

.. autoclass:: DCTapFormat
   :members:
   :undoc-members:

   DCTAP (Dublin Core Tabular Application Profiles) formats:

   * ``DCTapFormat.Csv`` - Comma-separated values (.csv)
   * ``DCTapFormat.Xlsx`` - Excel spreadsheet (.xlsx)

.. autoclass:: QueryResultFormat
   :members:
   :undoc-members:

   SPARQL query result formats:

   * ``QueryResultFormat.Turtle`` - Turtle format (.ttl)
   * ``QueryResultFormat.NTriples`` - N-Triples format (.nt)
   * ``QueryResultFormat.RdfXml`` - RDF/XML format (.rdf)
   * ``QueryResultFormat.TriG`` - TriG format (.trig)
   * ``QueryResultFormat.N3`` - Notation3 format (.n3)
   * ``QueryResultFormat.NQuads`` - N-Quads format (.nq)
   * ``QueryResultFormat.Csv`` - CSV table format (.csv)

.. autoclass:: QueryType
   :members:
   :undoc-members:

   SPARQL query type:

   * ``QueryType.Select`` - SELECT query
   * ``QueryType.Construct`` - CONSTRUCT query
   * ``QueryType.Ask`` - ASK query
   * ``QueryType.Describe`` - DESCRIBE query

.. autoclass:: ServiceDescriptionFormat
   :members:
   :undoc-members:

   SPARQL Service Description formats:

   * ``ServiceDescriptionFormat.Internal`` - Internal representation
   * ``ServiceDescriptionFormat.Json`` - JSON format
   * ``ServiceDescriptionFormat.Mie`` - MIE specification format


Reader Configuration
--------------------

.. autoclass:: ReaderMode
   :members:
   :undoc-members:

   Controls error handling during parsing:

   * ``ReaderMode.Lax`` - Ignore non-fatal errors and continue (default, recommended for real-world data)
   * ``ReaderMode.Strict`` - Fail immediately on first error (useful for strict validation)

Validation
----------

SHACL Validation
~~~~~~~~~~~~~~~~

.. autoclass:: ShaclValidationMode
   :members:
   :undoc-members:

   SHACL validation engines:

   * ``ShaclValidationMode.Native`` - Native SHACL validation engine (faster, recommended)
   * ``ShaclValidationMode.Sparql`` - SPARQL-based validation (slower, useful for debugging)

.. autoclass:: ShapesGraphSource
   :members:
   :undoc-members:

   Source of SHACL shapes for validation:

   * ``ShapesGraphSource.CurrentData`` - Extract shapes from the current RDF data graph
   * ``ShapesGraphSource.CurrentSchema`` - Use the currently loaded SHACL schema

ShEx Validation
~~~~~~~~~~~~~~~

.. autoclass:: SortModeResultMap
   :members:
   :undoc-members:

   Sort modes for validation result table display:

   * ``SortModeResultMap.Node`` - Sort by focus node
   * ``SortModeResultMap.Shape`` - Sort by shape label
   * ``SortModeResultMap.Status`` - Sort by validation status
   * ``SortModeResultMap.Details`` - Sort by detailed information


Data Generation
---------------

For the complete data generation API reference (``GeneratorConfig``,
``DataGenerator``, ``SchemaFormat``, ``OutputFormat``, ``CardinalityStrategy``,
``EntityDistribution``, ``DataQuality``), see :doc:`generate`.
