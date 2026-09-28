API Reference
=============

.. py:currentmodule:: pyrudof

The complete API reference for ``pyrudof``.

Every class on this page is implemented in Rust and re-exported from the ``pyrudof``
package. The descriptions are generated from the Rust doc comments, so this page cannot drift from the implementation.


The session
-----------

.. autoclass:: Rudof
   :no-members:
   :special-members: __init__

The session's methods are grouped below by the domain they belong to.

Session lifecycle
~~~~~~~~~~~~~~~~~

Used as a context manager, the session clears itself on exit.

.. automethod:: Rudof.__enter__
.. automethod:: Rudof.__exit__
.. automethod:: Rudof.update_config
.. automethod:: Rudof.get_version

RDF data
~~~~~~~~

.. automethod:: Rudof.read_data
.. automethod:: Rudof.serialize_data
.. automethod:: Rudof.dereference

ShEx
~~~~

.. automethod:: Rudof.read_shex
.. automethod:: Rudof.check_shex
.. automethod:: Rudof.serialize_current_shex
.. automethod:: Rudof.compile_shex_to_file
.. automethod:: Rudof.read_shex_precompiled
.. automethod:: Rudof.validate_shex
.. automethod:: Rudof.serialize_shex_validation_results
.. automethod:: Rudof.add_external_resolver
.. automethod:: Rudof.clear_external_resolvers
.. automethod:: Rudof.list_external_resolvers
.. automethod:: Rudof.read_shapemap
.. automethod:: Rudof.serialize_shapemap

SHACL
~~~~~

.. automethod:: Rudof.read_shacl
.. automethod:: Rudof.serialize_shacl
.. automethod:: Rudof.validate_shacl
.. automethod:: Rudof.serialize_shacl_validation_results

SPARQL
~~~~~~

.. automethod:: Rudof.read_query
.. automethod:: Rudof.run_query
.. automethod:: Rudof.serialize_query_results
.. automethod:: Rudof.list_endpoints

Property graphs
~~~~~~~~~~~~~~~

.. automethod:: Rudof.read_pgschema
.. automethod:: Rudof.serialize_pgschema
.. automethod:: Rudof.read_typemap
.. automethod:: Rudof.validate_pgschema
.. automethod:: Rudof.serialize_pgschema_validation_results
.. automethod:: Rudof.connect_pg_db
.. automethod:: Rudof.pg_db_ddl
.. automethod:: Rudof.load_pg_db
.. automethod:: Rudof.query_cypher

DCTAP
~~~~~

.. automethod:: Rudof.read_dctap
.. automethod:: Rudof.serialize_dctap

Service descriptions
~~~~~~~~~~~~~~~~~~~~

.. automethod:: Rudof.read_service_description
.. automethod:: Rudof.serialize_service_description

RDF-config
~~~~~~~~~~

.. automethod:: Rudof.read_rdf_config
.. automethod:: Rudof.serialize_rdf_config

Conversion and comparison
~~~~~~~~~~~~~~~~~~~~~~~~~

.. automethod:: Rudof.convert_schemas
.. automethod:: Rudof.compare_schemas

Materialization
~~~~~~~~~~~~~~~

.. automethod:: Rudof.read_map_state
.. automethod:: Rudof.materialize

Prefixes
~~~~~~~~

.. automethod:: Rudof.prefixes
.. automethod:: Rudof.add_prefix
.. automethod:: Rudof.remove_prefix
.. automethod:: Rudof.rename_prefix
.. automethod:: Rudof.copy_prefix

Node inspection
~~~~~~~~~~~~~~~

.. automethod:: Rudof.node_info
.. automethod:: Rudof.node_neighborhood

Resetting state
~~~~~~~~~~~~~~~

Each method clears one piece of session state; :meth:`Rudof.reset_all` clears everything,
which is also what leaving the context manager does.

.. automethod:: Rudof.reset_all
.. automethod:: Rudof.reset_data
.. automethod:: Rudof.reset_shex
.. automethod:: Rudof.reset_shex_schema
.. automethod:: Rudof.reset_shacl
.. automethod:: Rudof.reset_shacl_validation
.. automethod:: Rudof.reset_shapemap
.. automethod:: Rudof.reset_query
.. automethod:: Rudof.reset_query_results
.. automethod:: Rudof.reset_dctap
.. automethod:: Rudof.reset_rdf_config
.. automethod:: Rudof.reset_service_description
.. automethod:: Rudof.reset_pgschema
.. automethod:: Rudof.reset_typemap
.. automethod:: Rudof.reset_pgschema_validation
.. automethod:: Rudof.reset_pg_db_connection
.. automethod:: Rudof.reset_validation_results

Configuration
~~~~~~~~~~~~~

.. autoclass:: RudofConfig
   :members:
   :undoc-members:
   :special-members: __init__, __repr__


Exceptions
----------

Every error raised by ``pyrudof`` is a :class:`RudofError` or a subclass of it, so
``except RudofError`` catches all of them. Catch a specific subclass when you want to
distinguish, for example, a malformed schema from an unreachable endpoint.

This includes the paths ``rudof`` has not implemented yet: they raise
:exc:`InternalError` rather than escaping as a ``BaseException`` that ``except RudofError``
would miss.

.. autoexception:: RudofError
   :members:
   :show-inheritance:

.. autoexception:: ComparisonError
   :show-inheritance:

.. autoexception:: ConfigError
   :show-inheritance:

.. autoexception:: ConversionError
   :show-inheritance:

.. autoexception:: DCTapError
   :show-inheritance:

.. autoexception:: DataError
   :show-inheritance:

.. autoexception:: GenerateError
   :show-inheritance:

.. autoexception:: InputError
   :show-inheritance:

.. autoexception:: InternalError
   :show-inheritance:

.. autoexception:: IriError
   :show-inheritance:

.. autoexception:: MapStateError
   :show-inheritance:

.. autoexception:: MaterializeError
   :show-inheritance:

.. autoexception:: NodeInspectionError
   :show-inheritance:

.. autoexception:: PgDbError
   :show-inheritance:

.. autoexception:: PgSchemaError
   :show-inheritance:

.. autoexception:: PrefixesError
   :show-inheritance:

.. autoexception:: QueryError
   :show-inheritance:

.. autoexception:: RdfConfigError
   :show-inheritance:

.. autoexception:: ServiceError
   :show-inheritance:

.. autoexception:: ShExError
   :show-inheritance:

.. autoexception:: ShaclError
   :show-inheritance:

.. autoexception:: ShapeMapError
   :show-inheritance:

.. autoexception:: UnsupportedOperationError
   :show-inheritance:

.. autoexception:: ValidationError
   :show-inheritance:


Results
-------

The ``validate_*`` and ``run_query`` methods return a result object you can inspect
directly. The matching ``serialize_*`` methods render the same result as text, for when
you want to print or store it rather than act on it.

Validation reports
~~~~~~~~~~~~~~~~~~

.. autoclass:: ShExValidationReport
   :members:
   :undoc-members:
   :special-members: __bool__, __len__, __iter__, __repr__

.. autoclass:: ShExValidationEntry
   :members:
   :undoc-members:
   :special-members: __repr__

.. autoclass:: ShExValidationEntryIterator
   :members:
   :undoc-members:
   :special-members: __iter__, __next__

.. autoclass:: ShaclValidationReport
   :members:
   :undoc-members:
   :special-members: __bool__, __len__, __iter__, __repr__

.. autoclass:: ShaclValidationEntry
   :members:
   :undoc-members:
   :special-members: __repr__

.. autoclass:: ShaclValidationEntryIterator
   :members:
   :undoc-members:
   :special-members: __iter__, __next__

.. autoclass:: PgSchemaValidationReport
   :members:
   :undoc-members:
   :special-members: __bool__, __len__, __iter__, __repr__

.. autoclass:: PgSchemaValidationEntry
   :members:
   :undoc-members:
   :special-members: __repr__

.. autoclass:: PgSchemaValidationEntryIterator
   :members:
   :undoc-members:
   :special-members: __iter__, __next__

Query results
~~~~~~~~~~~~~

.. autoclass:: QueryResults
   :members:
   :undoc-members:
   :special-members: __len__, __iter__, __repr__

.. autoclass:: QueryRowIterator
   :members:
   :undoc-members:
   :special-members: __iter__, __next__

Node neighborhood
~~~~~~~~~~~~~~~~~

.. autoclass:: NeighborArc
   :members:
   :undoc-members:
   :special-members: __repr__

.. autoclass:: NeighborArcIterator
   :members:
   :undoc-members:
   :special-members: __iter__, __next__

.. autoclass:: ArcDirection
   :members:
   :undoc-members:


Formats
-------

Every format is a unit enum. All of them support ``==``, ``hash()``, ``str()`` and the
``all()`` classmethod; those that can be parsed from text also provide ``from_str``.
Calling the class with no arguments returns its default variant.

RDF data
~~~~~~~~

.. autoclass:: RDFFormat
   :members:
   :undoc-members:

.. autoclass:: ResultDataFormat
   :members:
   :undoc-members:

.. autoclass:: ReaderMode
   :members:
   :undoc-members:

ShEx
~~~~

.. autoclass:: ShExFormat
   :members:
   :undoc-members:

.. autoclass:: ResultShexValidationFormat
   :members:
   :undoc-members:

.. autoclass:: ShexValidationSortMode
   :members:
   :undoc-members:

.. autoclass:: ShapeMapFormat
   :members:
   :undoc-members:

SHACL
~~~~~

.. autoclass:: ShaclFormat
   :members:
   :undoc-members:

.. autoclass:: ShaclValidationMode
   :members:
   :undoc-members:

.. autoclass:: ShaclValidationSortMode
   :members:
   :undoc-members:

.. autoclass:: ResultShaclValidationFormat
   :members:
   :undoc-members:

.. autoclass:: ShapesGraphSource
   :members:
   :undoc-members:

DCTAP
~~~~~

.. autoclass:: DCTapFormat
   :members:
   :undoc-members:

.. autoclass:: ResultDCTapFormat
   :members:
   :undoc-members:

Property graphs
~~~~~~~~~~~~~~~

.. autoclass:: PgSchemaFormat
   :members:
   :undoc-members:

.. autoclass:: ResultPgSchemaValidationFormat
   :members:
   :undoc-members:

.. autoclass:: DbEngine
   :members:
   :undoc-members:

.. autoclass:: DdlDialect
   :members:
   :undoc-members:

SPARQL
~~~~~~

.. autoclass:: QueryType
   :members:
   :undoc-members:

.. autoclass:: QueryResultFormat
   :members:
   :undoc-members:

Conversion
~~~~~~~~~~

.. autoclass:: ConversionMode
   :members:
   :undoc-members:

.. autoclass:: ResultConversionMode
   :members:
   :undoc-members:

.. autoclass:: ConversionFormat
   :members:
   :undoc-members:

.. autoclass:: ResultConversionFormat
   :members:
   :undoc-members:

Service descriptions and RDF-config
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. autoclass:: ServiceDescriptionFormat
   :members:
   :undoc-members:

.. autoclass:: RdfConfigFormat
   :members:
   :undoc-members:

.. autoclass:: ResultRdfConfigFormat
   :members:
   :undoc-members:


Data generation
---------------

For ``GeneratorConfig``, ``DataGenerator`` and the generator enums, see :doc:`generate`.
