pyrudof |release|
====================

``pyrudof`` is a Python-based RDF library that implements `Shape Expressions <https://shex.io/>`_, `SHACL <https://www.w3.org/TR/shacl/>`_, `DCTAP <https://www.dublincore.org/specifications/dctap/>`_ and other technologies in the RDF ecosystem.
It can be used to validate RDF data both locally or through an endpoint, for converting between different data modeling languges (like ShEx, SHACL and DCTAP), and for generating UML-like visualizations and HTML views.


Installation
-----------------

``pyrudof`` is available on `PyPI <https://pypi.org/project/pyrudof/>`_ and can be installed by running the usual ``pip install pyrudof``, which will install the latest version currently available.

.. code-block:: python
    :caption: Basic example of knowledge graph validation using SHACL
    
    from pyrudof import *

    rudof = Rudof(RudofConfig())

    # we read the data graph
    rudof.read_data_str(
        """
        prefix : <http://example.org/>
        prefix sh:     <http://www.w3.org/ns/shacl#>
        prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

        :Person a sh:NodeShape;
            sh:targetNode :ok, :ko ;
            sh:property [
            sh:path     :name ;
            sh:minCount 1;
            sh:maxCount 1;
            sh:datatype xsd:string ;
            ] .
        """
    )

    # we read the shapes graph
    rudof.read_data_str(
        """
        prefix : <http://example.org/>

        :ok :name "alice" .
        :ko :name 1 .
        """
    )

    result = rudof.validate_shacl(ShaclValidationMode(), ShapesGraphSource())
    print(result.show_as_table())
    

Table of Contents
-----------------

.. toctree::
    :maxdepth: 2

    library