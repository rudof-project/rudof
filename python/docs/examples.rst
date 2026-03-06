Examples
========

This page contains validated Python examples that demonstrate pyrudof functionality.
Each example includes executable Python code that can be copied and pasted into a
Jupyter notebook or Python script, along with links to any referenced files.


ShEx Validation
---------------

ShEx (Shape Expressions) is a language for describing and validating RDF data structures.
These examples show how to use pyrudof for ShEx validation tasks.


Shex Validate
^^^^^^^^^^^^^

Basic ShEx validation with inline schema and data

**Source**: `shex_validate.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex_validate.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex_str("""
    prefix : <http://example.org/>
    prefix xsd: <http://www.w3.org/2001/XMLSchema#>
    
    :S { :p xsd:integer }
    """)
    
    rudof.read_data_str("""
    prefix : <http://example.org/>
    
    :x :p 1 .
    :y :q 2 .
    """)
    
    rudof.read_shapemap_str("""
    :x@:S, :y@:S
    """)
    
    results = rudof.validate_shex()
    print(results.show_as_table())


Shex Validate File
^^^^^^^^^^^^^^^^^^

ShEx validation using external files

**Source**: `shex_validate_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex_validate_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormat, RDFFormat, ShapeMapFormat
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex("person.shex", ShExFormat.ShExC)
    rudof.read_data("person.ttl", RDFFormat.Turtle)
    rudof.read_shapemap("person.sm", ShapeMapFormat.Compact)
    
    result = rudof.validate_shex()
    print(result.show_as_table())

**Referenced Files:**

- **Schema**: `person.shex <https://github.com/rudof-project/rudof/blob/master/python/examples/person.shex>`_
- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Shapemap**: `person.sm <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sm>`_


Compare Schemas
^^^^^^^^^^^^^^^

Compare two ShEx schemas and show the differences as JSON

**Source**: `compare_schemas.py <https://github.com/rudof-project/rudof/blob/master/python/examples/compare_schemas.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormatter
    
    rudof = Rudof(RudofConfig())
    
    schema1 = """
     PREFIX : <http://example.org/>
     PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
     :Person {
        :name xsd:string ;
        :age xsd:integer ;
        :weight xsd:float ;
        :worksFor @:Company
     }
     :Company {
        :name xsd:string ;
        :employee @:Person
     }"""
    
    # rudof.read_data_str(schema1)
    
    schema2 = """
     PREFIX ex: <http://example.org/>
     PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
     ex:Person {
        ex:name xsd:string ;
        ex:birthDate xsd:date ;
        ex:worksFor @ex:Company
    }
    ex:Company {
       ex:name xsd:string
    }
    """
    print("Comparing schemas:");
    result = rudof.compare_schemas_str(
        schema1, schema2,
        "shex", "shex",
        "shexc", "shexc",
        None, None,
        "http://example.org/Person", "http://example.org/Person",
        None,
        )
    
    print(f"Schemas compared: {result.as_json()}")


SHACL Validation
----------------

SHACL (Shapes Constraint Language) is a W3C standard for validating RDF graphs.
These examples demonstrate SHACL validation with pyrudof.


Shacl Validate
^^^^^^^^^^^^^^

Basic SHACL validation with inline shapes and data

**Source**: `shacl_validate.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_validate.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shacl_str("""
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
    """)
    
    rudof.read_data_str("""
    prefix : <http://example.org/>
    
    :ok :name "alice" .
    :ko :name 1 .
    """)
    
    result = rudof.validate_shacl()
    
    print(result.show_as_table())


Shacl Validate File
^^^^^^^^^^^^^^^^^^^

SHACL validation using external files

**Source**: `shacl_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShaclFormat, RDFFormat
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_shacl("timbl_shapes.ttl", ShaclFormat.Turtle)
    rudof.read_data("timbl.ttl", RDFFormat.Turtle)
    
    result = rudof.validate_shacl()
    print(result.show_as_table())

**Referenced Files:**

- **Schema**: `timbl_shapes.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl_shapes.ttl>`_
- **Data**: `timbl.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/timbl.ttl>`_


SHACL Validate (Inline Data)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

SHACL validation where shapes and data are loaded from inline strings

**Source**: `shacl_validate_only_data.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shacl_validate_only_data.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_data_str("""
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
    """)
    
    rudof.read_data_str("""
    prefix : <http://example.org/>
    
    :ok :name "alice" .
    :ko :name 1 .
    """, merge = True)
    
    result = rudof.validate_shacl()
    
    print(result.show_as_table())


RDF Data Handling
-----------------

These examples show how to parse, manipulate, and serialize RDF data using pyrudof.


Rdf Data
^^^^^^^^

RDF data parsing and serialization

**Source**: `rdf_data.py <https://github.com/rudof-project/rudof/blob/master/python/examples/rdf_data.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat
    
    data_str = """prefix xsd: <http://www.w3.org/2001/XMLSchema#>
    prefix : <http://example.org/>
    
    :alice :name "Alice" ;
      :birthdate "1980-03-02"^^xsd:date ;
      :enrolledIn :cs101 ;
      :knows :bob .
    
    :bob :name "Robert" ;
      :birthdate "1981-03-02"^^xsd:date ;
      :enrolledIn :cs101 ;
      :knows :alice .
    
    :cs101 :name "Computer Science 101";
      :student :alice, :bob .
    """
    rudof = Rudof(RudofConfig())
    
    rudof.read_data_str(data_str)
    
    result = rudof.serialize_data(format = RDFFormat.NTriples)
    
    print(result)


Data Visualization
^^^^^^^^^^^^^^^^^^

Convert RDF data into a PlantUML diagram for visualization

**Source**: `data_visualization.py <https://github.com/rudof-project/rudof/blob/master/python/examples/data_visualization.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat
    config = RudofConfig()
    rudof = Rudof(config)
    
    print(config)
    data_str = """
    prefix : <http://example.org/>
    :a :name "Alice" .
    """
    data_str1 = """prefix xsd: <http://www.w3.org/2001/XMLSchema#>
    prefix : <http://example.org/>
    :alice :name "Alice" ;
      :birthdate "1980-03-02"^^xsd:date ;
      :enrolledIn :cs101 ;
      :knows :bob .
    
    :bob :name "Robert" ;
      :birthdate "1981-03-02"^^xsd:date ;
      :enrolledIn :cs101 ;
      :knows :alice .
    
    :cs101 :name "Computer Science 101";
      :student :alice, :bob .
    """
    
    rudof.read_data_str(data_str)
    
    print("RDF Data in PlantUML format:")
    uml = rudof.data2plantuml()
    print("Finished conversion to UML.")
    print(uml)


DCTAP Conversion
----------------

DCTAP (Dublin Core Tabular Application Profiles) is a simple tabular format for
describing data models. These examples show how to convert DCTAP profiles to ShEx schemas.


Dctap2Shex
^^^^^^^^^^

Convert DCTAP application profile to ShEx schema

**Source**: `dctap2shex.py <https://github.com/rudof-project/rudof/blob/master/python/examples/dctap2shex.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormatter
    
    rudof = Rudof(RudofConfig())
    
    
    dctap_str = """
    shapeId,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
    Person,name,true,false,xsd:string,
    ,birthdate,false,false,xsd:date,
    ,enrolledIn,false,true,,Course
    Course,name,true,false,xsd:string,
    ,student,false,true,,Person
    """
    
    rudof.read_dctap_str(dctap_str)
    
    dctap = rudof.get_dctap()
    
    print(f"DCTAP\n{dctap}")
    
    rudof.dctap2shex()
    result = rudof.serialize_current_shex(ShExFormatter())
    print(f"DCTAP converted to ShEx\n{result}")


DCTAP Parse
^^^^^^^^^^^

Read a DCTAP (Dublin Core Tabular Application Profile) from a CSV string

**Source**: `dctap.py <https://github.com/rudof-project/rudof/blob/master/python/examples/dctap.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    dctap = """
    shapeID,propertyID,propertyLabel,mandatory,repeatable,valueNodeType,valueDataType,valueConstraint,valueConstraintType,valueShape,note,severity
    BookShape,dct:title,Title,TRUE,FALSE,Literal,rdf:langString,,,,,Violation
    BookShape,dct:creator,Author,FALSE,TRUE,IRI BNODE,,,,AuthorShape,,Warning
    BookShape,sdo:isbn,ISBN-13,FALSE,FALSE,Literal,xsd:string,,,,"Just the 13 numbers, no spaces or separators.",Violation
    BookShape,rdf:type,Type,TRUE,FALSE,IRI,,sdo:Book,,,,Warning
    AuthorShape,rdf:type,Type,TRUE,TRUE,IRI,,foaf:Person,,,,Warning
    AuthorShape,foaf:givenName,Given name,FALSE,TRUE,Literal,xsd:string,,,,,
    AuthorShape,foaf:familyName,Family name,FALSE,TRUE,Literal,xsd:string,,,,,
    """
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_dctap_str(dctap)
    
    dctap_read = rudof.get_dctap()
    
    print(f"DCTAP read: {dctap_read}")


DCTAP to UML
^^^^^^^^^^^^

Convert a DCTAP profile to ShEx and then to a PlantUML diagram

**Source**: `dctap2uml.py <https://github.com/rudof-project/rudof/blob/master/python/examples/dctap2uml.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, ShExFormatter, UmlGenerationMode
    
    rudof = Rudof(RudofConfig())
    dctap_str = """shapeId,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
    Person,name,true,false,xsd:string,
    ,birthdate,false,false,xsd:date,
    ,enrolledIn,false,true,,Course
    Course,name,true,false,xsd:string,
    ,student,false,true,,Person
    """
    rudof.read_dctap_str(dctap_str)
    
    dctap = rudof.get_dctap()
    print(f"DCTAP\n{dctap}")
    
    rudof.dctap2shex()
    result = rudof.serialize_current_shex(ShExFormatter())
    print(f"DCTAP converted to ShEx\n{result}")
    
    uml = rudof.shex2plantuml(UmlGenerationMode.all())
    print(f"DCTAP converted to UML\n{uml}")


SPARQL Queries
--------------

SPARQL is the standard query language for RDF. These examples demonstrate
how to execute SPARQL queries using pyrudof.


Query
^^^^^

SPARQL SELECT query execution

**Source**: `query.py <https://github.com/rudof-project/rudof/blob/master/python/examples/query.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    rdf = """prefix : <http://example.org/>
    :alice a :Person ;
     :name "Alice"   ;
     :knows :bob     .
    :bob a :Person   ;
     :name "Robert"  .
    """
    rudof.read_data_str(rdf)
    
    query = """prefix : <http://example.org/>
    select * where {
      ?x a :Person
    }
    """
    
    results = rudof.run_query_str(query)
    for result in iter(results):
        print(result.show())


Sparql File
^^^^^^^^^^^

SPARQL query from external file

**Source**: `sparql_file.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql_file.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.read_data("person.ttl")
    
    results = rudof.run_query_path("person.sparql")
    for result in iter(results):
        print(result.show())

**Referenced Files:**

- **Data**: `person.ttl <https://github.com/rudof-project/rudof/blob/master/python/examples/person.ttl>`_
- **Query**: `person.sparql <https://github.com/rudof-project/rudof/blob/master/python/examples/person.sparql>`_


SPARQL Inline
^^^^^^^^^^^^^

Run a SPARQL SELECT query against inline RDF data

**Source**: `sparql.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat
    
    data_str = """prefix xsd: <http://www.w3.org/2001/XMLSchema#>
    prefix : <http://example.org/>
    
    :alice :name "Alice" ;
      :birthdate "1980-03-02"^^xsd:date ;
      :enrolledIn :cs101 ;
      :knows :bob .
    
    :bob :name "Robert" ;
      :birthdate "1981-03-02"^^xsd:date ;
      :enrolledIn :cs101 ;
      :knows :alice .
    
    :cs101 :name "Computer Science 101";
      :student :alice, :bob .
    """
    rudof = Rudof(RudofConfig())
    
    rudof.read_data_str(data_str)
    
    results = rudof.run_query_str("""
    PREFIX : <http://example.org/>
    SELECT ?person ?name WHERE {
      ?person :name ?name .
    }
    """)
    
    print(results.show())


SPARQL 1.2 (RDF Reification)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Query RDF 1.2 reified triples using SPARQL

**Source**: `sparql12.py <https://github.com/rudof-project/rudof/blob/master/python/examples/sparql12.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    '''
    rudof.read_data_str("""
    prefix : <http://example.org/>
    prefix sh:     <http://www.w3.org/ns/shacl#>
    prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
    prefix rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
    
    :timbl rdfs:label "Tim Berners Lee" ;
           :employer :CERN {| :start "1984" ;
                              :end   "1994" |}
                           {| :start "1980" ;
                              :end   "1980" |} ;
           :award :PA {| :time "2002" ;
                         :togetherWith :vint |} .
    :vint  rdfs:label "Vinton Cerf" .
    """)
    '''
    
    rudof.read_data_str("""
    prefix : <http://example.org/>
    prefix sh:     <http://www.w3.org/ns/shacl#>
    prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
    prefix rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
    prefix rdf:    <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    
    :timbl rdfs:label "Tim Berners Lee" .
    _:r rdf:reifies   <<( :timbl :employer :CERN )>> ;
        :start        "1984" ;
        :end          "1994" .
    _:s rdf:reifies   <<( :timbl :employer :CERN )>> ;
        :start        "1980" ;
        :end          "1980" .
    _:t rdf:reifies   <<( :timbl :employer :CERN )>> ;
        :time         "2002" ;
        :togetherWith :vint  .
    :vint rdfs:label  "Vinton Cerf" .
    """)
    
    results = rudof.run_query_str("""
    prefix : <http://example.org/>
    prefix rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
    prefix rdf:    <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    
    SELECT ?person ?employer ?start ?end WHERE {
      ?r rdf:reifies <<( ?person :employer ?employer )>> ;
         :start ?start ;
         :end ?end .
     }
    """)
    print(results.show())
    # print(rudof.node_info(":timbl"))


Endpoints & Service Descriptions
--------------------------------

These examples demonstrate how to interact with remote SPARQL endpoints,
retrieve service descriptions, and query node information.
They require network access and are not executed during automated testing.


Endpoint CONSTRUCT Query
^^^^^^^^^^^^^^^^^^^^^^^^

Run a SPARQL CONSTRUCT query against a remote endpoint and display results as Turtle (requires network)

**Source**: `endpoint.py <https://github.com/rudof-project/rudof/blob/master/python/examples/endpoint.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat, QueryResultFormat
    
    endpoint = "https://plod.dbcls.jp/repositories/RDFPortal_VoID"
    
    sparql_query = """
    PREFIX void: <http://rdfs.org/ns/void#>
    PREFIX sd: <http://www.w3.org/ns/sparql-service-description#>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
    
    CONSTRUCT WHERE {
      [
        a sd:Service ;
        sd:defaultDataset [
           a sd:Dataset ;
           sd:namedGraph [
             sd:name <http://sparql.uniprot.org/uniprot> ;
             a sd:NamedGraph ;
             sd:endpoint ?ep_url ;
             sd:graph [
               a void:Dataset ;
               void:triples ?total_count ;
               void:classes ?class_count ;
               void:properties ?property_count ;
               void:distinctObjects ?uniq_object_count ;
               void:distinctSubjects ?uniq_subject_count ;
               void:classPartition [
                 void:class ?class_name ;
                 void:entities ?class_triple_count
               ] ;
               void:propertyPartition [
                 void:property ?property_name ;
                 void:triples ?property_triple_count
               ]
             ]
           ]
         ]
      ] .
    }
    """
    rudof = Rudof(RudofConfig())
    rudof.use_endpoint(endpoint)
    
    result = rudof.run_query_construct_str(sparql_query, QueryResultFormat.Turtle)
    
    print(result)


Service Description to MIE
^^^^^^^^^^^^^^^^^^^^^^^^^^

Read a SPARQL service description and convert it to MIE format (requires network)

**Source**: `service.py <https://github.com/rudof-project/rudof/blob/master/python/examples/service.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat, ReaderMode, ServiceDescriptionFormat
    
    rudof = Rudof(RudofConfig())
    
    service = "https://sparql.uniprot.org/sparql"
    
    rudof.read_service_description(service, RDFFormat.Turtle, None, ReaderMode.Strict)
    service = rudof.get_service_description()
    service_str = service.serialize(ServiceDescriptionFormat.Json)
    print(f"Service description in JSON:\n{service_str}")
    
    # Converting service description to MIE
    mie = service.as_mie()
    print(f"Service description in MIE format as YAML:\n{mie.as_yaml()}")


Node Info
^^^^^^^^^

Retrieve information about specific nodes from SPARQL endpoints (requires network)

**Source**: `node_info.py <https://github.com/rudof-project/rudof/blob/master/python/examples/node_info.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig
    
    rudof = Rudof(RudofConfig())
    
    rudof.use_endpoint("dbpedia")
    node_info = rudof.node_info("dbr:Oviedo", [])
    print(node_info)
    
    rudof.reset_all()
    
    rudof.use_endpoint("wikidata")
    node_info = rudof.node_info("wd:Q42", ["wdt:P31", "wdt:P19"])
    print(node_info)
    
    endpoints = rudof.list_endpoints()
    print(f"Available endpoints: {endpoints}")


Wikidata SELECT
^^^^^^^^^^^^^^^

Run a SELECT query against the Wikidata SPARQL endpoint (requires network)

**Source**: `wikidata.py <https://github.com/rudof-project/rudof/blob/master/python/examples/wikidata.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat
    
    query = """
    PREFIX wd: <http://www.wikidata.org/entity/>
    PREFIX wdt: <http://www.wikidata.org/prop/direct/>
    SELECT ?person ?occupation WHERE {
        ?p wdt:P31 wd:Q5 ;
              wdt:P106 ?o ;
              rdfs:label ?person ;
              wdt:P19 wd:Q14317 .
      ?o rdfs:label ?occupation
      FILTER (lang(?person) = "en" && lang(?occupation) = "en")
    }
    LIMIT 10
    """
    rudof = Rudof(RudofConfig())
    rudof.use_endpoint("wikidata")
    results = rudof.run_query_str(query)
    print(results.show())


CONSTRUCT Query
^^^^^^^^^^^^^^^

Run a SPARQL CONSTRUCT query against a remote endpoint (requires network)

**Source**: `construct_query.py <https://github.com/rudof-project/rudof/blob/master/python/examples/construct_query.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat, QueryResultFormat
    
    endpoint = "https://plod.dbcls.jp/repositories/RDFPortal_VoID"
    
    sparql_query = """
    PREFIX void: <http://rdfs.org/ns/void#>
    PREFIX sd: <http://www.w3.org/ns/sparql-service-description#>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
    
    CONSTRUCT WHERE {
      [
        a sd:Service ;
        sd:defaultDataset [
           a sd:Dataset ;
           sd:namedGraph [
             sd:name <http://sparql.uniprot.org/uniprot> ;
             a sd:NamedGraph ;
             sd:endpoint ?ep_url ;
             sd:graph [
               a void:Dataset ;
               void:triples ?total_count ;
               void:classes ?class_count ;
               void:properties ?property_count ;
               void:distinctObjects ?uniq_object_count ;
               void:distinctSubjects ?uniq_subject_count ;
               void:classPartition [
                 void:class ?class_name ;
                 void:entities ?class_triple_count
               ] ;
               void:propertyPartition [
                 void:property ?property_name ;
                 void:triples ?property_triple_count
               ]
             ]
           ]
         ]
      ] .
    }
    """
    rudof = Rudof(RudofConfig())
    rudof.use_endpoint(endpoint)
    
    result = rudof.run_query_construct_str(sparql_query, QueryResultFormat.Turtle)
    
    print(result)


CONSTRUCT Query (Wikidata)
^^^^^^^^^^^^^^^^^^^^^^^^^^

Run a SPARQL CONSTRUCT query against the Wikidata endpoint (requires network)

**Source**: `construct_query_wikidata.py <https://github.com/rudof-project/rudof/blob/master/python/examples/construct_query_wikidata.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, RDFFormat, QueryResultFormat
    
    endpoint = "https://query.wikidata.org/sparql"
    
    sparql_query = """
    PREFIX wd:  <http://www.wikidata.org/entity/>
    PREFIX wdt: <http://www.wikidata.org/prop/direct/>
    PREFIX :    <http://example.org/>
    
    CONSTRUCT {
       ?p a     :Person ;
          :name ?person ;
          :occupation ?occupation
    } WHERE {
        ?p wdt:P31 wd:Q5 ;
              wdt:P106 ?o ;
              rdfs:label ?person ;
              wdt:P19 wd:Q14317 .
      ?o rdfs:label ?occupation
      FILTER (lang(?person) = "en" && lang(?occupation) = "en")
    }
    LIMIT 10
    """
    rudof = Rudof(RudofConfig())
    rudof.use_endpoint(endpoint)
    
    result = rudof.run_query_construct_str(sparql_query, QueryResultFormat.Turtle)
    
    print(result)


Data Generation
---------------

These examples show how to configure and use rudof_generate to create
synthetic RDF data from ShEx or SHACL schemas.


Generate Config
^^^^^^^^^^^^^^^

Configure data generator with various options

**Source**: `generate_example.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_example.py>`_

**Python Code:**

.. code-block:: python

    import pyrudof
    
    # Create configuration
    config = pyrudof.GeneratorConfig()
    
    # Configure generation parameters
    config.set_entity_count(50)
    config.set_seed(12345)  # For reproducible results
    
    # Configure output
    config.set_output_path("generated_data.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_write_stats(True)
    
    # Configure schema
    config.set_schema_format(pyrudof.SchemaFormat.ShEx)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    
    # Configure parallelism
    config.set_worker_threads(4)
    config.set_batch_size(10)
    
    # Create generator
    generator = pyrudof.DataGenerator(config)
    print(f"Generator configured: {config.get_entity_count()} entities")


Advanced Generate
^^^^^^^^^^^^^^^^^

Advanced data generator configuration: parallel processing, cardinality strategies, output formats, and error handling

**Source**: `advanced_generate_example.py <https://github.com/rudof-project/rudof/blob/master/python/examples/advanced_generate_example.py>`_

**Python Code:**

.. code-block:: python

    #!/usr/bin/env python3
    """
    Advanced example: Using rudof_generate Python bindings
    
    This example demonstrates advanced usage patterns for the rudof_generate
    Python bindings, including:
    - Configuration from files
    - Different schema formats (ShEx and SHACL)
    - Parallel processing
    - Error handling
    """
    
    import pyrudof
    import tempfile
    import os
    
    
    def example_basic_generation():
        """Most basic example - minimal configuration"""
        print("\n" + "=" * 70)
        print("Example 1: Basic Data Generation")
        print("=" * 70)
    
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(10)
        config.set_output_path("/tmp/basic_output.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
    
        generator = pyrudof.DataGenerator(config)
        print("✓ Created basic generator configuration")
        print(f"  Entities: {config.get_entity_count()}")
        print(f"  Output: {config.get_output_path()}")
    
    
    def example_reproducible_generation():
        """Example with random seed for reproducible results"""
        print("\n" + "=" * 70)
        print("Example 2: Reproducible Generation")
        print("=" * 70)
    
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(100)
        config.set_seed(42)  # Fixed seed for reproducibility
        config.set_output_path("/tmp/reproducible_output.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
    
        generator = pyrudof.DataGenerator(config)
        print("✓ Generator with fixed seed for reproducible results")
        print(f"  Seed: {config.get_seed()}")
        print("  Running this configuration multiple times will produce identical output")
    
    
    def example_parallel_generation():
        """Example using parallel processing"""
        print("\n" + "=" * 70)
        print("Example 3: Parallel Generation")
        print("=" * 70)
    
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(1000)
    
        # Configure parallel processing
        config.set_worker_threads(4)
        config.set_batch_size(100)
        config.set_parallel_writing(True)
        config.set_parallel_file_count(4)
    
        config.set_output_path("/tmp/parallel_output")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
    
        generator = pyrudof.DataGenerator(config)
        print("✓ Generator configured for parallel processing")
        print(f"  Worker threads: 4")
        print(f"  Batch size: 100")
        print(f"  Parallel files: 4")
        print("  This will generate 4 separate output files")
    
    
    def example_with_stats():
        """Example that writes generation statistics"""
        print("\n" + "=" * 70)
        print("Example 4: Generation with Statistics")
        print("=" * 70)
    
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(50)
        config.set_output_path("/tmp/output_with_stats.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        config.set_write_stats(True)
    
        generator = pyrudof.DataGenerator(config)
        print("✓ Generator will write statistics file")
        print(f"  Data output: {config.get_output_path()}")
        print(f"  Stats output: {config.get_output_path().replace('.ttl', '_stats.json')}")
    
    
    def example_cardinality_strategies():
        """Example showing different cardinality strategies"""
        print("\n" + "=" * 70)
        print("Example 5: Cardinality Strategies")
        print("=" * 70)
    
        strategies = {
            "Minimum": pyrudof.CardinalityStrategy.Minimum,
            "Maximum": pyrudof.CardinalityStrategy.Maximum,
            "Random": pyrudof.CardinalityStrategy.Random,
            "Balanced": pyrudof.CardinalityStrategy.Balanced,
        }
    
        for name, strategy in strategies.items():
            config = pyrudof.GeneratorConfig()
            config.set_entity_count(20)
            config.set_cardinality_strategy(strategy)
            config.set_output_path(f"/tmp/output_{name.lower()}.ttl")
            config.set_output_format(pyrudof.OutputFormat.Turtle)
    
            generator = pyrudof.DataGenerator(config)
            print(f"  ✓ {name:10} - Uses {name.lower()} cardinality for relationships")
    
    
    def example_different_output_formats():
        """Example showing different output formats"""
        print("\n" + "=" * 70)
        print("Example 6: Output Formats")
        print("=" * 70)
    
        # Turtle format (more readable)
        config_turtle = pyrudof.GeneratorConfig()
        config_turtle.set_entity_count(10)
        config_turtle.set_output_path("/tmp/output.ttl")
        config_turtle.set_output_format(pyrudof.OutputFormat.Turtle)
        gen1 = pyrudof.DataGenerator(config_turtle)
        print("  ✓ Turtle format - Human-readable, with prefixes")
    
        # NTriples format (more compact)
        config_ntriples = pyrudof.GeneratorConfig()
        config_ntriples.set_entity_count(10)
        config_ntriples.set_output_path("/tmp/output.nt")
        config_ntriples.set_output_format(pyrudof.OutputFormat.NTriples)
        gen2 = pyrudof.DataGenerator(config_ntriples)
        print("  ✓ NTriples format - Simple, one triple per line")
    
    
    def example_schema_formats():
        """Example showing different schema formats"""
        print("\n" + "=" * 70)
        print("Example 7: Schema Formats")
        print("=" * 70)
    
        # ShEx schema
        config_shex = pyrudof.GeneratorConfig()
        config_shex.set_schema_format(pyrudof.SchemaFormat.ShEx)
        config_shex.set_entity_count(20)
        config_shex.set_output_path("/tmp/shex_output.ttl")
        gen1 = pyrudof.DataGenerator(config_shex)
        print("  ✓ ShEx schema format")
        print("    Use with: generator.load_shex_schema('schema.shex')")
    
        # SHACL schema
        config_shacl = pyrudof.GeneratorConfig()
        config_shacl.set_schema_format(pyrudof.SchemaFormat.Shacl)
        config_shacl.set_entity_count(20)
        config_shacl.set_output_path("/tmp/shacl_output.ttl")
        gen2 = pyrudof.DataGenerator(config_shacl)
        print("  ✓ SHACL schema format")
        print("    Use with: generator.load_shacl_schema('shapes.ttl')")
    
    
    def example_config_persistence():
        """Example showing configuration save/load"""
        print("\n" + "=" * 70)
        print("Example 8: Configuration Persistence")
        print("=" * 70)
    
        # Create a configuration
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(100)
        config.set_seed(42)
        config.set_output_path("/tmp/saved_config_output.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
        config.set_write_stats(True)
    
        # Save to TOML file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
            config_path = f.name
    
        try:
            config.to_toml_file(config_path)
            print(f"✓ Configuration saved to: {config_path}")
    
            # Load it back
            loaded_config = pyrudof.GeneratorConfig.from_toml_file(config_path)
            print(f"✓ Configuration loaded from file")
            print(f"  Entity count: {loaded_config.get_entity_count()}")
            print(f"  Random seed: {loaded_config.get_seed()}")
            print(f"  Output path: {loaded_config.get_output_path()}")
    
            # Can also load from JSON (if you create a JSON config)
            print("\n  Supported formats:")
            print("    - TOML: GeneratorConfig.from_toml_file(path)")
            print("    - JSON: GeneratorConfig.from_json_file(path)")
        finally:
            if os.path.exists(config_path):
                os.unlink(config_path)
    
    
    def example_error_handling():
        """Example showing proper error handling"""
        print("\n" + "=" * 70)
        print("Example 9: Error Handling")
        print("=" * 70)
    
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(10)
        config.set_output_path("/tmp/error_test.ttl")
    
        generator = pyrudof.DataGenerator(config)
    
        # Try to load a non-existent schema
        try:
            generator.load_shex_schema("/nonexistent/schema.shex")
            print("  This shouldn't print")
        except Exception as e:
            print(f"✓ Caught expected error when loading non-existent schema:")
            print(f"  Error type: {type(e).__name__}")
            print(f"  Error message: {str(e)[:60]}...")
    
        # Try to load invalid config file
        try:
            config = pyrudof.GeneratorConfig.from_toml_file("/nonexistent/config.toml")
            print("  This shouldn't print")
        except Exception as e:
            print(f"✓ Caught expected error when loading non-existent config:")
            print(f"  Error type: {type(e).__name__}")
    
    
    def example_complete_workflow():
        """Example of a complete workflow (conceptual)"""
        print("\n" + "=" * 70)
        print("Example 10: Complete Workflow (Conceptual)")
        print("=" * 70)
    
        print("""
    A complete workflow would look like:
    
    1. Create and configure:
       config = pyrudof.GeneratorConfig()
       config.set_entity_count(1000)
       config.set_seed(42)
       config.set_output_path("output.ttl")
       config.set_output_format(pyrudof.OutputFormat.Turtle)
       config.set_schema_format(pyrudof.SchemaFormat.ShEx)
       config.set_write_stats(True)
    
    2. Create generator:
       generator = pyrudof.DataGenerator(config)
    
    3. Load schema and generate (option A - separate steps):
       generator.load_shex_schema("schema.shex")
       generator.generate()
    
    4. OR use the convenience method (option B - one step):
       generator.run("schema.shex")
    
    5. Check output:
       - Data file: output.ttl
       - Stats file: output_stats.json
    
    Note: This example shows the API structure. To actually run it,
          you need a valid ShEx or SHACL schema file.
        """)
    
    
    def main():
        """Run all examples"""
        print("\n" + "=" * 70)
        print(" RUDOF_GENERATE Python Bindings - Advanced Examples")
        print("=" * 70)
        print("\nThese examples demonstrate the Python API for rudof_generate.")
        print("They show configuration patterns without actually generating data.")
    
        try:
            example_basic_generation()
            example_reproducible_generation()
            example_parallel_generation()
            example_with_stats()
            example_cardinality_strategies()
            example_different_output_formats()
            example_schema_formats()
            example_config_persistence()
            example_error_handling()
            example_complete_workflow()
    
            print("\n" + "=" * 70)
            print("✓ All examples completed successfully!")
            print("=" * 70)
            print("\nFor actual data generation, you need:")
            print("  1. A valid ShEx schema (.shex file)")
            print("  2. OR a valid SHACL schema (.ttl file with SHACL shapes)")
            print("\nSee the repository examples/ directory for sample schemas.")
            print("\n")
    
        except Exception as e:
            print(f"\n✗ Error running examples: {e}")
            import traceback
            traceback.print_exc()
            return 1
    
        return 0
    
    
    if __name__ == "__main__":
        exit(main())


Config File
^^^^^^^^^^^

Use TOML and JSON configuration files with the data generator

**Source**: `config_file_example.py <https://github.com/rudof-project/rudof/blob/master/python/examples/config_file_example.py>`_

**Python Code:**

.. code-block:: python

    #!/usr/bin/env python3
    """
    Example: Using configuration files with rudof_generate Python bindings
    
    This example demonstrates how to use TOML and JSON configuration files,
    similar to the CLI interface.
    """
    
    import pyrudof
    import tempfile
    import os
    import json
    
    
    def example_toml_config():
        """Example creating and using a TOML configuration file"""
        print("=" * 70)
        print("Example 1: TOML Configuration File")
        print("=" * 70)
    
        # Create a comprehensive configuration
        config = pyrudof.GeneratorConfig()
    
        # Generation settings
        config.set_entity_count(1000)
        config.set_seed(42)
        config.set_entity_distribution(pyrudof.EntityDistribution.Equal)
        config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
        config.set_schema_format(pyrudof.SchemaFormat.ShEx)
    
        # Field generation settings
        config.set_locale("en")
        config.set_data_quality(pyrudof.DataQuality.High)
    
        # Output settings
        config.set_output_path("output/data.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        config.set_compress(False)
        config.set_write_stats(True)
        config.set_parallel_writing(True)
        config.set_parallel_file_count(4)
    
        # Parallel processing settings
        config.set_worker_threads(8)
        config.set_batch_size(100)
        config.set_parallel_shapes(True)
        config.set_parallel_fields(True)
    
        # Save to TOML file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
            toml_path = f.name
    
        try:
            config.to_toml_file(toml_path)
            print(f"✓ Configuration saved to: {toml_path}\n")
    
            # Show the TOML content
            with open(toml_path, 'r') as f:
                content = f.read()
                print("TOML Configuration:")
                print("-" * 70)
                print(content)
                print("-" * 70)
    
            # Load the configuration back
            loaded_config = pyrudof.GeneratorConfig.from_toml_file(toml_path)
            print(f"\n✓ Configuration loaded from TOML file")
            print(f"  Entity count: {loaded_config.get_entity_count()}")
            print(f"  Seed: {loaded_config.get_seed()}")
            print(f"  Locale: {loaded_config.get_locale()}")
            print(f"  Worker threads: {loaded_config.get_worker_threads()}")
    
            # Use it to create a generator
            generator = pyrudof.DataGenerator(loaded_config)
            print(f"✓ DataGenerator created from loaded config\n")
    
        finally:
            if os.path.exists(toml_path):
                os.unlink(toml_path)
    
    
    def example_json_config():
        """Example creating and using a JSON configuration file"""
        print("\n" + "=" * 70)
        print("Example 2: JSON Configuration File (Manual)")
        print("=" * 70)
    
        # Create a JSON configuration manually
        json_config = {
            "generation": {
                "entity_count": 500,
                "seed": 12345,
                "entity_distribution": "Equal",
                "cardinality_strategy": "Random",
                "schema_format": "ShEx"
            },
            "field_generators": {
                "default": {
                    "locale": "es",
                    "quality": "Medium"
                },
                "datatypes": {},
                "properties": {}
            },
            "output": {
                "path": "output/spanish_data.ttl",
                "format": "Turtle",
                "compress": False,
                "write_stats": True,
                "parallel_writing": False,
                "parallel_file_count": 0
            },
            "parallel": {
                "worker_threads": None,
                "batch_size": 50,
                "parallel_shapes": True,
                "parallel_fields": True
            }
        }
    
        # Save to JSON file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
            json_path = f.name
            json.dump(json_config, f, indent=2)
    
        try:
            print(f"✓ JSON configuration created: {json_path}\n")
    
            # Show the JSON content
            with open(json_path, 'r') as f:
                content = f.read()
                print("JSON Configuration:")
                print("-" * 70)
                print(content)
                print("-" * 70)
    
            # Load the configuration
            loaded_config = pyrudof.GeneratorConfig.from_json_file(json_path)
            print(f"\n✓ Configuration loaded from JSON file")
            print(f"  Entity count: {loaded_config.get_entity_count()}")
            print(f"  Seed: {loaded_config.get_seed()}")
            print(f"  Locale: {loaded_config.get_locale()}")
            print(f"  Batch size: {loaded_config.get_batch_size()}")
    
            # Validate
            loaded_config.validate()
            print(f"✓ Configuration validated successfully")
    
            # Use it to create a generator
            generator = pyrudof.DataGenerator(loaded_config)
            print(f"✓ DataGenerator created from loaded config\n")
    
        finally:
            if os.path.exists(json_path):
                os.unlink(json_path)
    
    
    def example_cli_like_workflow():
        """Example simulating CLI-like workflow with config file + overrides"""
        print("\n" + "=" * 70)
        print("Example 3: CLI-like Workflow (Config File + Overrides)")
        print("=" * 70)
    
        # Create a base configuration file (like --config in CLI)
        base_config = pyrudof.GeneratorConfig()
        base_config.set_locale("en")
        base_config.set_data_quality(pyrudof.DataQuality.High)
        base_config.set_batch_size(100)
        base_config.set_parallel_shapes(True)
        base_config.set_parallel_fields(True)
    
        with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
            config_path = f.name
    
        try:
            base_config.to_toml_file(config_path)
            print(f"✓ Base configuration saved to: {config_path}")
    
            # Load base configuration (like --config myconfig.toml)
            config = pyrudof.GeneratorConfig.from_toml_file(config_path)
            print(f"✓ Base configuration loaded")
    
            # Apply command-line style overrides
            config.set_entity_count(2000)  # Like --entities 2000
            config.set_output_path("/tmp/custom_output.ttl")  # Like --output /tmp/custom_output.ttl
            config.set_seed(99999)  # Like --seed 99999
            config.set_worker_threads(16)  # Like --parallel 16
    
            print(f"\n  Applied overrides:")
            print(f"    --entities {config.get_entity_count()}")
            print(f"    --output {config.get_output_path()}")
            print(f"    --seed {config.get_seed()}")
            print(f"    --parallel {config.get_worker_threads()}")
    
            # Validate merged configuration
            config.validate()
            print(f"\n✓ Merged configuration validated")
    
            # Create generator (like running the CLI)
            generator = pyrudof.DataGenerator(config)
            print(f"✓ DataGenerator ready to run")
            print(f"\n  To generate data, you would call:")
            print(f"    generator.run('schema.shex')")
            print(f"  Or:")
            print(f"    generator.load_shex_schema('schema.shex')")
            print(f"    generator.generate()\n")
    
        finally:
            if os.path.exists(config_path):
                os.unlink(config_path)
    
    
    def example_different_locales():
        """Example showing different locale configurations"""
        print("\n" + "=" * 70)
        print("Example 4: Different Locale Configurations")
        print("=" * 70)
    
        locales = ["en", "es", "fr", "de", "it"]
    
        for locale in locales:
            config = pyrudof.GeneratorConfig()
            config.set_entity_count(100)
            config.set_locale(locale)
            config.set_output_path(f"output/data_{locale}.ttl")
    
            generator = pyrudof.DataGenerator(config)
            print(f"  ✓ Generator configured for locale '{locale}' -> {config.get_output_path()}")
    
    
    def example_quality_levels():
        """Example showing different data quality levels"""
        print("\n" + "=" * 70)
        print("Example 5: Data Quality Levels")
        print("=" * 70)
    
        qualities = [
            (pyrudof.DataQuality.Low, "Fast, simple data"),
            (pyrudof.DataQuality.Medium, "Realistic patterns"),
            (pyrudof.DataQuality.High, "Complex, correlated data"),
        ]
    
        for quality, description in qualities:
            config = pyrudof.GeneratorConfig()
            config.set_entity_count(100)
            config.set_data_quality(quality)
    
            generator = pyrudof.DataGenerator(config)
            print(f"  ✓ {quality}: {description}")
    
    
    def main():
        """Run all configuration file examples"""
        print("\n" + "=" * 70)
        print(" RUDOF_GENERATE - Configuration File Examples")
        print("=" * 70)
        print("\nThese examples show how to use configuration files,")
        print("similar to the CLI interface.\n")
    
        try:
            example_toml_config()
            example_json_config()
            example_cli_like_workflow()
            example_different_locales()
            example_quality_levels()
    
            print("\n" + "=" * 70)
            print("✓ All configuration file examples completed!")
            print("=" * 70)
            print("\nKey takeaways:")
            print("  1. Save configurations to TOML/JSON files for reuse")
            print("  2. Load configurations from files: from_toml_file() / from_json_file()")
            print("  3. Override specific settings after loading (like CLI --options)")
            print("  4. Use validate() to check configuration before running")
            print("  5. Set locale and quality for customized data generation")
            print("\n")
    
        except Exception as e:
            print(f"\n✗ Error running examples: {e}")
            import traceback
            traceback.print_exc()
            return 1
    
        return 0
    
    
    if __name__ == "__main__":
        exit(main())


Generate From Schema
^^^^^^^^^^^^^^^^^^^^

Generate synthetic RDF data from a ShEx schema file

**Source**: `generate_from_schema.py <https://github.com/rudof-project/rudof/blob/master/python/examples/generate_from_schema.py>`_

**Python Code:**

.. code-block:: python

    #!/usr/bin/env python3
    """
    Practical example: Generate synthetic data from a ShEx schema
    
    This example demonstrates how to generate synthetic RDF data from
    the example ShEx schema in the repository.
    """
    
    import pyrudof
    import os
    import sys
    
    def generate_from_schema():
        """Generate data from the simple.shex example schema"""
    
        # Path to example schema
        schema_path = "../../examples/simple.shex"
    
        # Check if schema exists
        if not os.path.exists(schema_path):
            print(f"Warning: Schema file not found at {schema_path}")
            print("This is a demonstration of the API, even without the actual file.")
            schema_exists = False
        else:
            schema_exists = True
            print(f"✓ Found schema: {schema_path}")
    
        # Create configuration
        config = pyrudof.GeneratorConfig()
    
        # Configure generation
        config.set_entity_count(20)
        config.set_seed(42)  # For reproducible results
    
        # Configure output
        output_dir = "/tmp/pyrudof_generate"
        os.makedirs(output_dir, exist_ok=True)
        output_file = os.path.join(output_dir, "generated_simple.ttl")
        stats_file = os.path.join(output_dir, "generated_simple_stats.json")
    
        config.set_output_path(output_file)
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        config.set_write_stats(True)
        config.set_compress(False)
    
        # Configure schema
        config.set_schema_format(pyrudof.SchemaFormat.ShEx)
        config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    
        print("\nConfiguration:")
        print(f"  Entities to generate: {config.get_entity_count()}")
        print(f"  Random seed: {config.get_seed()}")
        print(f"  Output file: {config.get_output_path()}")
        print(f"  Statistics file: {stats_file}")
    
        # Create generator
        print("\n✓ Creating DataGenerator...")
        generator = pyrudof.DataGenerator(config)
        print("✓ DataGenerator created successfully")
    
        if schema_exists:
            try:
                # Load schema and generate
                print(f"\nLoading schema from: {schema_path}")
                generator.run(schema_path)
    
                print(f"\n✓ Data generation completed!")
                print(f"  Output written to: {output_file}")
    
                if os.path.exists(stats_file):
                    print(f"  Statistics written to: {stats_file}")
    
                # Show file size
                if os.path.exists(output_file):
                    size = os.path.getsize(output_file)
                    print(f"  Generated file size: {size} bytes")
    
                    # Show first few lines
                    print(f"\nFirst 10 lines of generated data:")
                    print("-" * 60)
                    with open(output_file, 'r') as f:
                        for i, line in enumerate(f):
                            if i >= 10:
                                break
                            print(line.rstrip())
                    print("-" * 60)
    
            except Exception as e:
                print(f"\n✗ Error during generation: {e}")
                import traceback
                traceback.print_exc()
                return 1
        else:
            print("\nSkipping actual generation (schema file not found)")
            print("To run with a real schema, provide a valid ShEx or SHACL file.")
    
        return 0
    
    if __name__ == "__main__":
        print("=" * 60)
        print("Practical Example: Generate Synthetic RDF Data")
        print("=" * 60)
    
        result = generate_from_schema()
    
        print("\n" + "=" * 60)
        if result == 0:
            print("Example completed successfully!")
        else:
            print("Example completed with errors.")
        print("=" * 60)
    
        sys.exit(result)


UML Visualization
-----------------

These examples show how to convert RDF data, ShEx schemas, and DCTAP
profiles into PlantUML diagrams for visualization.
They require plantuml jar and are not executed during automated testing.


ShEx to UML
^^^^^^^^^^^

Convert a ShEx schema into a PlantUML class diagram

**Source**: `shex2uml.py <https://github.com/rudof-project/rudof/blob/master/python/examples/shex2uml.py>`_

**Python Code:**

.. code-block:: python

    from pyrudof import Rudof, RudofConfig, UmlGenerationMode, ShExFormat
    rudof = Rudof(RudofConfig())
    
    rudof.read_shex_str("""
    prefix : <http://example.org/>
    prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
    
    :Person {
     :name xsd:string  ;
     :knows @:Person * ;
     :worksFor @:Company
    }
    
    :Company {
      :name xsd:string     ;
      :employee @:Person * ;
    }
    """)
    
    uml_str = rudof.shex2plantuml(UmlGenerationMode.all())
    
    print(uml_str)


Utility & Introspection
-----------------------

Miscellaneous utilities: inspecting the pyrudof module and listing available
classes.


Module Info
^^^^^^^^^^^

Display the file path of the pyrudof module

**Source**: `module_info.py <https://github.com/rudof-project/rudof/blob/master/python/examples/module_info.py>`_

**Python Code:**

.. code-block:: python

    import pyrudof
    
    print(pyrudof.__file__)


Show Pyrudof Classes
^^^^^^^^^^^^^^^^^^^^

List all classes available in the pyrudof package

**Source**: `show_pyrydof_classes.py <https://github.com/rudof-project/rudof/blob/master/python/examples/show_pyrydof_classes.py>`_

**Python Code:**

.. code-block:: python

    import pyrudof
    import inspect
    
    classes = [cls_name for cls_name, cls_obj in inspect.getmembers(pyrudof)
              if inspect.isclass(cls_obj)]
    print(classes)

