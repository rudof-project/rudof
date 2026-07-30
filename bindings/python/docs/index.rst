pyrudof |release|
====================

``pyrudof`` is a Python library for Semantic Web operations.

.. important::
   ``pyrudof`` is under active development. APIs may change between versions.

At a high level it supports:

- Loading and serializing **RDF** and **PG** data.
- Loading, checking, serializing and validating **ShEx** schemas.
- Loading, serializing and validating **SHACL** shapes.
- Loading, serializing and validating **PGSchemas**.
- Loading, running and serializing **SPARQL** queries and query results.
- Converting and comparing schemas between supported formats.
- Loading and serializing **DCTAP** and **Service Descriptions**.
- Generating synthetic data from schemas.


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