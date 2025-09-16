# SHACL Data Generation Example

This example demonstrates generating synthetic RDF data from a SHACL schema.

## SHACL Schema

The SHACL schema defines constraints for Person and Course entities:

```turtle
@prefix :       <http://example.org/> .
@prefix sh:     <http://www.w3.org/ns/shacl#> .
@prefix xsd:    <http://www.w3.org/2001/XMLSchema#> .
        
:Person a sh:NodeShape ;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] ;
  sh:property [                   
   sh:path     :birthDate ; 
   sh:maxCount 1; 
   sh:datatype xsd:date ;
  ] ;
  sh:property [                   
   sh:path     :enrolledIn ; 
   sh:node    :Course ;
  ] .
:Course a sh:NodeShape;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] .
```

## Usage

Generate data using the CLI:

```bash
# Generate 5 entities from SHACL schema
cargo run -p data_generator -- --schema ../../examples/simple_shacl.ttl --output shacl_output.ttl --entities 5

# Or using the root directory
cd ../../
./target/debug/data_generator --schema examples/simple_shacl.ttl --output data_generator/examples/shacl_output.ttl --entities 5
```

## Generated Output

The tool generates realistic RDF data conforming to the SHACL constraints:

```turtle
<http://example.org/Person-1> <http://example.org/name> "George Brown" ;
	<http://example.org/enrolledIn> <http://example.org/Course-1> ;
	<http://example.org/birthDate> "1975-06-01"^^<http://www.w3.org/2001/XMLSchema#date> ;
	a <http://example.org/Person> .
<http://example.org/Course-1> <http://example.org/name> "Advanced Mathematics" ;
	a <http://example.org/Course> .
<http://example.org/Person-2> <http://example.org/name> "Ian Garcia" ;
	<http://example.org/enrolledIn> <http://example.org/Course-1> ;
	<http://example.org/birthDate> "1999-08-01"^^<http://www.w3.org/2001/XMLSchema#date> ;
	a <http://example.org/Person> .
```

## Features Demonstrated

- **Property constraints**: Required `name` properties with string datatypes
- **Cardinality constraints**: `minCount`/`maxCount` properly enforced
- **Datatype constraints**: Date values for `birthDate`, strings for `name`
- **Node references**: `enrolledIn` properties link to generated Course entities
- **Closed shapes**: Only properties defined in the schema are generated
