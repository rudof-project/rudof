pyrudof |release|
=================

``pyrudof`` is a Python library for Semantic Web operations, wrapping
`rudof <https://github.com/rudof-project/rudof>`_: a **semantic-less tool for the
Semantic Web**.

At a high level it supports:

- Loading, serializing, validating and querying **RDF** data in different formats:

  - **ShEx validation**: Loading, checking, serializing and validating **ShEx** schemas,
    including precompiled schemas and external shape resolvers and support for
    semantic actions.
  - **SHACL validation**: Loading, serializing and validating **SHACL** shapes.
  - **Schema conversion and comparison**: Converting between different schema formats and comparing schemas for equivalence.
  - **SPARQL queries**: Loading, running and serializing SPARQL queries and query results,
    and listing known endpoints.
  - **Neighborhood iteration**: Inspecting the neighborhood of a node.
  - **Prefix map management**: Adding, removing, renaming and copying prefixes.

- Loading, serializing and validating **Property Graphs**:

  - **PGSchemas validation**: Loading, serializing and validating PGSchemas.
  - **Cypher queries**: Connecting to property graph databases, deriving DDL from RDF data,
    loading it, and running **Cypher** queries.

- Loading and serializing **DCTAP**, **Service Descriptions** and **RDF-config**.
- **Generating synthetic data** from schemas.


Installation
------------

``pyrudof`` is available on `PyPI <https://pypi.org/project/pyrudof/>`_ and requires
Python 3.10 or newer:

.. code-block:: bash

    pip install pyrudof


Quickstart
----------

:class:`~pyrudof.Rudof` is a **stateful session**: data, schemas and results stay loaded
across calls, which is what lets a pipeline read once and then validate, query and
serialize against the same graph. Used as a context manager, it resets on exit.

.. code-block:: python

    from pyrudof import Rudof, RDFFormat, ShExFormat, ShapeMapFormat

    schema = """
    PREFIX : <http://example.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

    :Person { :name xsd:string }
    """
    data = """
    PREFIX : <http://example.org/>

    :alice :name "Alice" .
    """

    with Rudof() as rudof:
        rudof.read_shex(schema, ShExFormat.ShExC)
        rudof.read_data(data, RDFFormat.Turtle)
        rudof.read_shapemap(":alice@:Person", ShapeMapFormat.Compact)

        report = rudof.validate_shex()

        print(report.conforms)                  # True
        for entry in report:
            print(entry.node, entry.shape, entry.status)

Failures raise a :class:`~pyrudof.RudofError` or one of its subclasses:

.. code-block:: python

    from pyrudof import InputError, ShExError

    try:
        rudof.read_shex("schema.shex")
    except InputError:
        ...      # the file could not be read
    except ShExError:
        ...      # the file was read but is not a valid schema

.. raw:: html

   <a class="gh-card" href="https://github.com/rudof-project/rudof"
      target="_blank" rel="noopener noreferrer">
     <svg class="gh-card__mark" viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor"
       d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577
       0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633
       17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809
       1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93
       0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267
       1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24
       2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81
       2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24
       12.297c0-6.627-5.373-12-12-12"/></svg>
     <span class="gh-card__body">
       <strong class="gh-card__title">rudof on GitHub</strong>
       <span class="gh-card__sub">Source, issues and releases. Star the repo to follow development.</span>
     </span>
     <span class="gh-card__cta">Open repository<span aria-hidden="true"> &#8594;</span></span>
   </a>


.. toctree::
   :maxdepth: 2
   :caption: User guide
   :hidden:

   library
   generate
   examples
