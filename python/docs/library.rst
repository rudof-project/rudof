PyRudof Library
===============

``pyrudof`` provides Python bindings for performing rudof operations.
The main class is ``Rudof``, which can be configured using ``RudofConfig``.
The library supports reading and writing RDF data in various formats, validating data against shapes defined in ShEx or SHACL, converting between different shape representation formats, and generating synthetic RDF data.


Core Classes
""""""""""""

.. autoclass:: pyrudof.RudofConfig
    :members:
    :undoc-members:


.. autoclass:: pyrudof.Rudof
    :members:
    :undoc-members:


Data Formats
""""""""""""

.. autoclass:: pyrudof.RDFFormat
    :members:


.. autoclass:: pyrudof.ShExFormat
    :members:


.. autoclass:: pyrudof.ShaclFormat
    :members:

.. autoclass:: pyrudof.ShapeMapFormat
    :members:

.. autoclass:: pyrudof.DCTapFormat
    :members:

.. autoclass:: pyrudof.QueryResultFormat
    :members:

.. autoclass:: pyrudof.ServiceDescriptionFormat
    :members:


Formatters
""""""""""

.. autoclass:: pyrudof.ShExFormatter
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ShapeMapFormatter
    :members:
    :undoc-members:


Reader Configuration
""""""""""""""""""""

.. autoclass:: pyrudof.ReaderMode
    :members:


Validation
""""""""""
.. autoclass:: pyrudof.ShaclValidationMode
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ShapesGraphSource
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ValidationReport
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ValidationStatus
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ResultShapeMap
    :members:
    :undoc-members:

.. autoclass:: pyrudof.SortModeResultMap
    :members:
    :undoc-members:


Schema Representations
""""""""""""""""""""""

.. autoclass:: pyrudof.ShExSchema
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ShaclSchema
    :members:
    :undoc-members:

.. autoclass:: pyrudof.DCTAP
    :members:
    :undoc-members:

.. autoclass:: pyrudof.ServiceDescription
    :members:
    :undoc-members:


Schema Comparison
"""""""""""""""""

.. autoclass:: pyrudof.CompareSchemaFormat
    :members:
    :undoc-members:

.. autoclass:: pyrudof.CompareSchemaMode
    :members:
    :undoc-members:


Query Results
"""""""""""""

.. autoclass:: pyrudof.QueryShapeMap
    :members:
    :undoc-members:

.. autoclass:: pyrudof.QuerySolutions
    :members:
    :undoc-members:

.. autoclass:: pyrudof.QuerySolution
    :members:
    :undoc-members:


Visualization
"""""""""""""

.. autoclass:: pyrudof.UmlGenerationMode
    :members:
    :undoc-members:


Other
"""""

.. autoclass:: pyrudof.Mie
    :members:
    :undoc-members:

.. autoclass:: pyrudof.PrefixMap
    :members:
    :undoc-members:
