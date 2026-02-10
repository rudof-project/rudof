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
   :special-members: __init__

RudofConfig
~~~~~~~~~~~

.. autoclass:: RudofConfig
   :members:
   :undoc-members:
   :special-members: __init__


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
   * ``RDFFormat.RDFXML`` - XML-based RDF syntax (.rdf, .owl)
   * ``RDFFormat.TriG`` - Turtle with named graphs (.trig)
   * ``RDFFormat.N3`` - Notation3 (.n3)
   * ``RDFFormat.NQuads`` - N-Triples with named graphs (.nq)
   * ``RDFFormat.JsonLd`` - JSON-LD format (.jsonld)

ShEx Formats
~~~~~~~~~~~~

.. autoclass:: ShExFormat
   :members:
   :undoc-members:

   Supported ShEx schema formats:

   * ``ShExFormat.ShExC`` - ShEx Compact Syntax (human-readable, .shex)
   * ``ShExFormat.ShExJ`` - ShEx JSON format (.json)
   * ``ShExFormat.Turtle`` - ShEx in RDF/Turtle (.ttl)

SHACL Formats
~~~~~~~~~~~~~

.. autoclass:: ShaclFormat
   :members:
   :undoc-members:

   SHACL shapes graph serialization formats (all RDF-based):

   * ``ShaclFormat.Turtle`` - Turtle format (.ttl)
   * ``ShaclFormat.NTriples`` - N-Triples format (.nt)
   * ``ShaclFormat.RDFXML`` - RDF/XML format (.rdf)
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
   * ``ShapeMapFormat.JSON`` - JSON representation

Other Formats
~~~~~~~~~~~~~

.. autoclass:: DCTapFormat
   :members:
   :undoc-members:

   DCTAP (Dublin Core Tabular Application Profiles) formats:

   * ``DCTapFormat.CSV`` - Comma-separated values (.csv)
   * ``DCTapFormat.XLSX`` - Excel spreadsheet (.xlsx)

.. autoclass:: QueryResultFormat
   :members:
   :undoc-members:

   SPARQL query result formats:

   * ``QueryResultFormat.Turtle`` - Turtle format (.ttl)
   * ``QueryResultFormat.NTriples`` - N-Triples format (.nt)
   * ``QueryResultFormat.RDFXML`` - RDF/XML format (.rdf)
   * ``QueryResultFormat.TriG`` - TriG format (.trig)
   * ``QueryResultFormat.N3`` - Notation3 format (.n3)
   * ``QueryResultFormat.NQuads`` - N-Quads format (.nq)
   * ``QueryResultFormat.CSV`` - CSV table format (.csv)

.. autoclass:: ServiceDescriptionFormat
   :members:
   :undoc-members:

   SPARQL Service Description formats:

   * ``ServiceDescriptionFormat.Internal`` - Internal representation
   * ``ServiceDescriptionFormat.Json`` - JSON format
   * ``ServiceDescriptionFormat.Mie`` - MIE specification format

Formatters
----------

ShExFormatter
~~~~~~~~~~~~~

.. autoclass:: ShExFormatter
   :members:
   :undoc-members:
   :special-members: __init__

ShapeMapFormatter
~~~~~~~~~~~~~~~~~

.. autoclass:: ShapeMapFormatter
   :members:
   :undoc-members:
   :special-members: __init__


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

.. autoclass:: ValidationReport
   :members:
   :undoc-members:
   :special-members: __init__

.. autoclass:: ValidationResult
   :members:
   :undoc-members:

ShEx Validation
~~~~~~~~~~~~~~~

.. autoclass:: ResultShapeMap
   :members:
   :undoc-members:
   :special-members: __init__

.. autoclass:: ValidationStatus
   :members:
   :undoc-members:
   :special-members: __init__

.. autoclass:: SortModeResultMap
   :members:
   :undoc-members:

   Sort modes for ResultShapeMap table display:

   * ``SortModeResultMap.Node`` - Sort by focus node
   * ``SortModeResultMap.Shape`` - Sort by shape label
   * ``SortModeResultMap.Status`` - Sort by validation status
   * ``SortModeResultMap.Details`` - Sort by detailed information

Schema Representations
----------------------

.. autoclass:: ShExSchema
   :members:
   :undoc-members:

.. autoclass:: ShaclSchema
   :members:
   :undoc-members:

.. autoclass:: DCTAP
   :members:
   :undoc-members:

.. autoclass:: ServiceDescription
   :members:
   :undoc-members:


Schema Comparison
-----------------

.. autoclass:: CoShaMo
   :members:
   :undoc-members:

.. autoclass:: ShaCo
   :members:
   :undoc-members:

.. autoclass:: CompareSchemaFormat
   :members:
   :undoc-members:

.. autoclass:: CompareSchemaMode
   :members:
   :undoc-members:


Query Results
-------------

.. autoclass:: QueryShapeMap
   :members:
   :undoc-members:

.. autoclass:: QuerySolutions
   :members:
   :undoc-members:
   :special-members: __init__, __iter__

.. autoclass:: QuerySolution
   :members:
   :undoc-members:


Visualization
-------------

.. autoclass:: UmlGenerationMode
   :members:
   :undoc-members:

   UML generation modes for PlantUML exports:

   * ``UmlGenerationMode.all()`` - Generate UML for all shapes in the model
   * ``UmlGenerationMode.neighs(node)`` - Generate UML only for neighbors of specified node


Utilities
---------

.. autoclass:: Mie
   :members:
   :undoc-members:

.. autoclass:: PrefixMap
   :members:
   :undoc-members:

.. autoclass:: Node
   :members:
   :undoc-members:

.. autoclass:: ShapeLabel
   :members:
   :undoc-members:


Exceptions
----------

.. autoclass:: RudofError
   :members:
   :undoc-members:
