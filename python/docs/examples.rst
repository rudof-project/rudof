Examples
========

This page contains practical examples of using ``pyrudof``.


SHACL Validation
----------------

.. code-block:: python

    from pyrudof import *

    rudof = Rudof(RudofConfig())

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

    rudof.read_data_str("""
        prefix : <http://example.org/>
        :ok :name "alice" .
        :ko :name 1 .
    """)

    result = rudof.validate_shacl(ShaclValidationMode(), ShapesGraphSource())
    print(result.show_as_table())


ShEx Validation
---------------

.. code-block:: python

    from pyrudof import *

    rudof = Rudof(RudofConfig())

    rudof.read_shex_str("""
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :Person {
            :name xsd:string ;
            :age xsd:integer ?
        }
    """)

    rudof.read_data_str("""
        prefix : <http://example.org/>
        :alice :name "Alice" ; :age 30 .
        :bob :name "Bob" .
    """)

    shapemap = rudof.read_shapemap_str(":alice@:Person, :bob@:Person")
    result = rudof.validate_shex(shapemap)
    print(result)


DCTAP to ShEx Conversion
-------------------------

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormatter

    rudof = Rudof(RudofConfig())

    rudof.read_dctap_str("""
    shapeId,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
    Person,name,true,false,xsd:string,
    ,birthdate,false,false,xsd:date,
    ,enrolledIn,false,true,,Course
    Course,name,true,false,xsd:string,
    ,student,false,true,,Person
    """)

    rudof.dctap2shex()
    result = rudof.serialize_current_shex(ShExFormatter())
    print(result)


SPARQL CONSTRUCT Query on Wikidata
-----------------------------------

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, QueryResultFormat

    rudof = Rudof(RudofConfig())
    rudof.use_endpoint("https://query.wikidata.org/sparql")

    result = rudof.run_query_construct_str("""
        PREFIX wd:  <http://www.wikidata.org/entity/>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        PREFIX :    <http://example.org/>

        CONSTRUCT {
            ?p a :Person ;
               :name ?person .
        } WHERE {
            ?p wdt:P31 wd:Q5 ;
               rdfs:label ?person .
            FILTER (lang(?person) = "en")
        }
        LIMIT 5
    """, QueryResultFormat.Turtle)

    print(result)


Synthetic Data Generation
-------------------------

.. code-block:: python

    import pyrudof

    config = pyrudof.GeneratorConfig()
    config.set_entity_count(100)
    config.set_output_path("/tmp/synthetic_data.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_seed(42)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)

    generator = pyrudof.DataGenerator(config)
    generator.run("schema.shex")


ShEx to UML Diagram
-------------------

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, UmlGenerationMode

    rudof = Rudof(RudofConfig())

    rudof.read_shex_str("""
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :Person {
            :name xsd:string ;
            :knows @:Person *
        }
    """)

    uml = rudof.shex2uml(UmlGenerationMode())
    print(uml)


More Examples
-------------

Additional examples can be found in the repository:

- `generate_example.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_example.py>`_ — Basic generation examples
- `advanced_generate_example.py <https://github.com/rudof-project/rudof/blob/master/python/examples/advanced_generate_example.py>`_ — Advanced generation patterns
- `config_file_example.py <https://github.com/rudof-project/rudof/blob/master/python/examples/config_file_example.py>`_ — Configuration file usage
- `dctap2shex.py <https://github.com/rudof-project/rudof/blob/master/python/examples/dctap2shex.py>`_ — DCTAP to ShEx conversion
- `construct_query_wikidata.py <https://github.com/rudof-project/rudof/blob/master/python/examples/construct_query_wikidata.py>`_ — SPARQL queries on Wikidata
- `generate_from_schema.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_from_schema.py>`_ — Generate data from a schema file
- `shex2uml.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex2uml.py>`_ — Generate UML diagrams from ShEx
- `shacl_validate.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_validate.py>`_ — SHACL validation examples
- `shex_validate.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex_validate.py>`_ — ShEx validation examples
- `compare_schemas.py <https://github.com/rudof-project/rudof/blob/master/python/examples/compare_schemas.py>`_ — Schema comparison
