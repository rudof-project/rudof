# `rudof_lib`

`rudof_lib` is the centralized, programmatic facade for the entire Rudof ecosystem. It exposes a stable entry point (`Rudof`) that hides the complexity of dealing with numerous specialized underlying subcrates (`shacl_validation`, `shex_validation`, `rudof_rdf`, `pgschema`, etc.).

At a high level, it provides robust, builder-style interfaces for:
- Loading and serializing RDF and PG data.
- Loading, checking, serializing and validating ShEx schemas.
- Loading, serializing and validating SHACL shapes.
- Loading, serializing and validating PGSchemas.
- Loading, running and serializing SPARQL queries and query results.
- Converting and comparing schemas between supported formats.
- Loading and serializing DCTAP and Service Descriptions.
- Generating synthetic data from schemas.

**Primary goals**:
- Provide a stable, ergonomic entry point (`Rudof`) that acts as a facade
over specialized crates.
- Expose operations via builder-style (fluent) interfaces.
- Keep a clear separation between the public API, concrete implementations, and shared types/formats/errors.

## Architecture and Structure

The package structure is organized to make the public API small and stable
while keeping implementations modular and testable. Below is a more detailed description of the main files and folders, what to expect inside them and the conventions used by the crate.

- **`src/lib.rs`**: Core entry, exposing the essential (`Rudof`, `RudofConfig`, errors and formats).
- **`src/rudof.rs`**: Houses the central `Rudof` state container. Contains methods grouped by domain returning configured builder instances.
- **`src/rudof_config.rs`**: Houses `RudofConfig`for configuration handling and defaults.
- **`src/api/`**: The functional brain of the library. Divided by domain (`data`, `shex`, `shacl`, etc.). Each domain contains three subparts:
    - **`builders/`**: The public builder types (`*Builder`) that callers
      instantiate via `Rudof` methods. Builders configure the operation
      and finally call an implementation function when executed.
    - **`implementations/`**: Concrete operation code that performs the
      logic using `Rudof` and the configured inputs.
    - **`trait definition`** (in the domain module): describe the
      operations offered.
- **`src/formats/`**: Enums for input/serialization formats (for example `SchemaFormat`, `InputSpec`, or `ConversionFormat`).
- **`src/types/`**: Shared domain types (for example `Data`, `QueryResult`, `shex_statistics`, and other cross-cutting structs).
- **`src/errors/`**: Domain-specific error types and the unified
`RudofError`. Errors are defined per-domain and composed into a top-level error to provide predictable error handling for callers.
- **`src/utils/`**: Internal utility helpers.

## Operations on the `Rudof` Struct

Every functionality in the library operates by initially calling a method on the `Rudof` instance. This creates a specific operation **builder**. Modifiers chain upon this builder, culminating in an `.execute()` call.

Below is an exhaustive breakdown grouped by domain:

### 1. Core
```rust
use rudof_lib::{Rudof, RudofConfig};

// Initialize Rudof with default config
let mut rudof = Rudof::new(RudofConfig::default());

// Get the current version
let version = rudof.version().execute();
println!("Rudof version: {}", version);

// Update the configuration
rudof.update_config(RudofConfig::default());

// Reset all internal state
rudof.reset_all().execute();
```

### 2. Data

#### 2.1 RDF Data
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{
    InputSpec, DataFormat, DataReaderMode, ResultDataFormat, NodeInspectionMode
    };
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let rdf_data_input = vec![InputSpec::from_str(
    r#"
        prefix ex: <http://example.org/> . 
        ex:alice a ex:Person ;
            ex:age 30 .
    "#
).unwrap()];

// Load RDF data into Rudof's state
rudof.load_data()
    .with_data(&data_input)
    .with_data_format(&DataFormat::Turtle)
    .with_reader_mode(&DataReaderMode::Lax)
    .with_merge(false)
    .execute()
    .unwrap();

// Serialize the loaded data to standard output
rudof.serialize_data(&mut std::io::stdout())
    .with_result_data_format(&ResultDataFormat.NTriples)
    .execute()
    .unwrap();

// Serialize the information of the node "ex:alice" to standard output
let node = "ex:alice";
let predicates = vec!["ex:age".to_string()];
rudof.show_node_info(&node, &mut std::io::stdout())
    .with_show_node_mode(&NodeInspectionMode::Outgoing)
    .with_predicates(&predicates)
    .with_depth(1)
    .with_show_colors(false)
    .execute()
    .unwrap();

// Reset data state
rudof.reset_data().execute()
```

#### 2.2 Property Graph Data

```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{InputSpec, DataFormat, DataReaderMode, ResultDataFormat};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let pg_data_input = vec![InputSpec::from_str(
    r#"
        (alice {Person} [ name: "Alice", age: 23, aliases: "Ally" ])
        (bob   {Person} [ name: "Robert", aliases: ["Bob", "Bobby"] ])
    "#,
).unwrap()];

// Load Property Graph data into Rudof's state (without merging)
rudof.load_data()
    .with_data(&pg_data_input)
    .with_data_format(&DataFormat::Pg)
    .execute()
    .unwrap();

// Serialize the loaded data to standard output
rudof.serialize_data(&mut std::io::stdout()).execute().unwrap();
```

#### 2.3 SPARQL Endpoint
```rust
use rudof_lib::{Rudof, RudofConfig};

let mut rudof = Rudof::new(RudofConfig::default());

// Lists the registered SPARQL endpoints
let endpoints = rudof.list_endpoints().execute();
for (name, url) in endpoints {
    println!("{}, {}", name, url);
}

// Focus SPARQL endpoint
let endpoint = "http://example.org/sparql";
rudof.load_data()
    .with_endpoint(&endpoint)
    .execute()
    .unwrap();
```

#### 2.4 Service Description
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{
    InputSpec, DataFormat, DataReaderMode, ResultServiceFormat
};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let service_description_input = InputSpec::from_str(
    r#"
        prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
        prefix ent: <http://www.w3.org/ns/entailment/> .
        
        <http://example.org/sparql> a sd:Service ;
            sd:endpoint <http://example.org/sparql> ;
            sd:supportedLanguage sd:SPARQL11Query ;
            sd:defaultEntailmentRegime ent:Simple .
    "#;
    ).unwrap();

// Load a Service Description into Rudof's state
let base = "http://example.org/";
rudof.load_service_description(&service_description)
    .with_data_format(&DataFormat::Turtle)
    .with_reader_mode(&DataReaderMode::Strict)
    .with_base(&base)
    .execute()
    .unwrap();

// Serialize the loaded service description to standard output
rudof.serialize_service_description(&mut std::io::stdout())
    .with_result_service_format(&ResultServiceFormat::Json)
    .execute()
    .unwrap();
```

### 3. ShEx

```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{
    InputSpec, ShExFormat, ShapeMapFormat, ShExValidationSortByMode,

};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let rdf_data_input = vec![InputSpec::from_str(
    r#"
        <alice> <name> "Alice" ;
            <age> 30 .
    "#
    ).unwrap()];
let base_nodes = "http://example.org";

// Load RDF data into Rudof's state
rudof.load_data()
    .with_data(&rdf_data_input)
    .with_base(&base_nodes)
    .execute()
    .unwrap();

let shex_schema_input = InputSpec::from_str(
    r#"
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        <PersonShape> {
            <name> xsd:string ;
            <age> xsd:integer
        }
    "#,
    ).unwrap();
let base_shapes = "http://example.org";

// Check ShEx schema well-formedness and check for negative cycles in the dependency graph,
// serializing the checking results to standard output
rudof.check_shex_schema(&shex_schema_input, &mut std::io::stdout())
    .with_shex_schema_format(&ShExFormat::ShExC)
    .with_base(&base_shapes)
    .execute()
    .unwrap();

// Load ShEx schema into Rudof's state
rudof.load_shex_schema(&shex_schema_input)
    .with_shex_schema_format(&ShExFormat::ShExC)
    .with_base(&base_shapes)
    .execute()
    .unwrap();

// Serialize the loaded ShEx schema to standard output
rudof.serialize_shex_schema(&mut std::io::stdout())
    .with_shape(&"http://example.org/PersonShape")
    .with_show_statistics(true)
    .with_show_dependencies(true)
    .with_show_time(true)
    .with_result_shex_format(&ShExFormat::ShExJ)
    .execute()
    .unwrap();

let shapemap_input = InputSpec::from_str(
    r#"
        <alice>@<PersonShape>
    "#
    ).unwrap();

// Load shapemap into Rudof's state
rudof.load_shex_schema(&shapemap_input)
    .with_shapemap_format(&ShapeMapFormat::Compact)
    .with_base_nodes(&base_nodes)
    .with_base_shapes(&base_shapes)
    .execute()
    .unwrap();

// Serialize the loaded shapemap to standard output
rudof.serialize_shex_schema(&mut std::io::stdout())
    .with_result_shapemap_format(&ShapeMapFormat::Compact)
    .with_show_colors(false)
    .execute()
    .unwrap();

// Run validation storing the result into Rudof´s state
rudof.validate_shex().execute().unwrap();

// Serialize the validation results to standard output
rudof.serialize_shex_validation_results(&mut std::io::stdout())
    .with_shex_validation_sort_order_mode(&ShExValidationSortByMode::Node)
    .with_result_shex_validation_format(&ResultShExValidationFormat::Details)
    .execute()
    .unwrap();

// Reset ShEx schema state
rudof.reset_shex_schema().execute();
// Reset shapemap state
rudof.reset_shapemap().execute();
// Reset ShEx validation state (ShEx schema, shapemap and validation results)
rudof.reset_shex().execute();
```

### 4. SHACL
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{
    InputSpec, DataFormat, ShaclFormat, ShaclValidationSortByMode
};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let data_with_shapes_input = InputSpec::from_str(r#"
        prefix ex: <http://example.org/> .
        prefix sh: <http://www.w3.org/ns/shacl#> .
        prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        # SHACL Shape
        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .

        # Instance data
        ex:Alice
            a ex:Person ;
            ex:name "Alice Smith" ;
            ex:age 30 .

        ex:Bob
            a ex:Person ;
            ex:name "Bob Jones" ;
            ex:age 25 .
    "#).unwrap();

// Load RDF data containing SHACL shapes
rudof.load_data()
    .with_data(&[data_with_shapes_input])
    .with_data_format(&DataFormat::Turtle)
    .execute()
    .unwrap();

// Extract SHACL shapes from the loaded data and load them into Rudof's state (you can also separate data and shapes into two fields and use ".with_shacl_schema()")
rudof.load_shacl_shapes().execute().unwrap();

// Serialize the loaded shapes to standard output
rudof.serialize_shacl_shapes(&mut std::io::stdout())
    .with_shacl_result_format(&ShaclFormat::NTriples)
    .execute()
    .unwrap();

// Run validation storing the result into Rudof´s state
rudof.validate_shacl().execute().unwrap();

// Serialize the validation results to standard output
rudof.serialize_shacl_validation_results(&mut std::io::stdout())
    .with_shacl_validation_sort_order_mode(&ShaclValidationSortByMode::Node)
    .with_result_shacl_validation_format(&ResultShaclValidationFormat::Details)
    .execute()
    .unwrap();

// Reset SHACL shapes state
rudof.reset_shacl_shapes().execute();
// Reset SHACL validation state (SHACL schema and validation results)
rudof.reset_shacl().execute();
```

### 5. Property Graph Schema
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{InputSpec, DataFormat, PgSchemaFormat, ResultPgSchemaValidationFormat};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let pg_data_input = vec![InputSpec::from_str(
    r#"
        (n1 {"Student"}["name": "Alice", "age": 23])
        (n2_wrong {"Student"}["name": "Bob", "age": 12])
        (n3_wrong {"Student"}["name": "Carol", "age": "unknown"])
    "#,
).unwrap()];

// Load Property Graph data into Rudof's state
rudof.load_data()
    .with_data(&pg_data_input)
    .with_data_format(&DataFormat::Pg)
    .execute()
    .unwrap();

let pg_schema_input = InputSpec::from_str(
    r#"
        CREATE NODE TYPE ( AdultStudentType: Student {
            name: STRING ,
            age: INTEGER CHECK > 18
        })
    "#).unwrap();

// Load Property Graph Schema into Rudof's state
rudof.load_pgschema(&pg_schema_input)
    .with_pg_schema_format(&PgSchemaFormat::PgSchemaC)
    .execute()
    .unwrap();

// Serialize the loaded Property Graph Schema to standard output
rudof.serialize_pgschema(&mut std::io::stdout())
    .with_result_pg_schema_format(&PgSchemaFormat::PgSchemaC)
    .execute()
    .unwrap();

let typemap_input = InputSpec::from_str(
    r#"
        n1: AdultStudentType,
        n2_wrong: AdultStudentType,
        n3_wrong: AdultStudentType
    "#,).unwrap();

// Load typemap into Rudof's state
rudof.load_typemap(&typemap_input).execute().unwrap();

// Run validation storing the result into Rudof´s state
rudof.validate_pgschema().execute().unwrap();

// Serialize the validation results to standard output
rudof.serialize_pgschema_validation_results(&mut std::io::stdout())
    .with_result_pg_schema_validation_format(&ResultPgSchemaValidationFormat::Details)
    .with_show_colors(false)
    .execute()
    .unwrap()

// Reset Property Graph schema shapes state
rudof.reset_pgschema().execute()
// Reset typemap state
rudof.reset_typemap().execute()
// Reset Property Graph schema validation state (PG schema, typemap and validation results)
rudof.reset_pgschema_validation().execute()
```

### 6. Query
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{
    InputSpec, DataFormat, ResultQueryFormat, QueryType
};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let data_input = InputSpec::from_str(r#"
        prefix : <http://example.org/>
        prefix schema: <http://schema.org/>

        :a schema:name  "Alice" ;
        :status      :Active ;
        schema:knows :a, :b  .

        :b schema:name  "Bob"    ;
        :status      :Waiting ;
        schema:knows :c       .

        :c schema:name  "Carol"  .

        :d schema:name  23      .  # Should fail

        :e schema:name  "Emily" ;  # Should fail
        schema:knows :d      .
    "#).unwrap();

// Load RDF data
rudof.load_data()
    .with_data(&[data_with_shapes_input])
    .with_data_format(&DataFormat::Turtle)
    .execute()
    .unwrap();

let query_input = InputSpec::from_str(
        r#"
            prefix : <http://example.org/>
            prefix schema: <http://schema.org/>

            select ?person ?name ?status where {
            ?person schema:name ?name ;
                    :status ?status .
            }
        "#,
    )
    .unwrap();

// Load query into Rudof's state
rudof.load_query(&query_input)
    .with_query_type(&QueryType::Select)
    .execute()
    .unwrap(); 

// Serialize the query to standard output
rudof.serialize_query(&mut std::io::stdout()).execute().unwrap();

// Run the query
rudof.run_query().execute();

// Serialize the query results to standard output
rudof.serialize_query_result(&mut std::io::stdout())
    .with_result_query_format(&ResultQueryFormat::Csv)
    .execute()
    .unwrap();

// Reset query state
rudof.reset_query().execute()
// Reset query execution state (query and query results)
rudof.reset_query_results().execute()

```

### 7. DCTAP
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{InputSpec, DCTapFormat, ResultDCTapFormat};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let dctap_input = InputSpec::from_str(
    r#"
        shapeID,propertyID,mandatory,repeatable,valueDataType
        :Person,rdf:type,true,false,
        :Person,schema:name,true,false,xsd:string
        :Person,schema:age,false,false,xsd:integer
        :Person,schema:memberOf,false,true,:Organization
        :Organization,rdf:type,true,false,
        :Organization,schema:name,true,false,xsd:string
        :Organization,schema:location,false,false,xsd:string
    "#
).unwrap();

// Load a DCTAP into Rudof's state
rudof.load_dctap(&dctap_input)
    .with_dctap_format(&DCTapFormat::Csv)
    .execute()
    .unwrap();

// Serialize the loaded DCTAP to standard output
rudof.serialize_dctap(&mut std::io::stdout())
    .with_result_dctap_format(&ResultDCTapFormat::Json)
    .execute()
    .unwrap();

// Reset DCTAP state
rudof.reset_dctap().execute();
```

### 8. RDF_Config (DBCLS)
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{InputSpec, RdfConfigFormat, ResultRdfConfigFormat};
use std::str::FromStr;

let mut rudof = Rudof::new(RudofConfig::default());
let config_input = InputSpec::from_str(
    r#"
        - Person ex:person1 ex:person2:
        - a: ex:Person
        - rdfs:label:
            - name: "Alice"
        - ex:age?:
            - age_value: 32
        - ex:memberOf:
            - organization: Organization
        - Organization ex:org1:
        - a: ex:Organization
        - rdfs:label:
            - org_name: "Example Org"
        - ex:location:
            - city: "Oviedo" 
    "#).unwrap();

// Load RDF config
rudof.load_rdf_config(&config_input)
    .with_rdf_config_format(&RdfConfigFormat::Yaml)
    .execute()
    .unwrap();

// Serialize the loaded RDF config to standard output
rudof.serialize_rdf_config(&mut std::io::stdout())
    .with_rdf_config_format(&ResultRdfConfigFormat::Yaml)
    .execute()
    .unwrap();

// Reset RDF config state
rudof.reset_rdf_config().execute();
```

### 9. Generation
```rust
use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::formats::{DataFormat, InputSpec, GenerationSchemaFormat};
use std::str::FromStr;

let rudof = Rudof::new(RudofConfig::default());
let schema_input = InputSpec::from_str(
    r#"
        prefix : <http://example.org/>
        :User { :name . }
    "#).unwrap();

// Generate synthetic data asynchronously
// Note: Requires a Tokio runtime or similar async environment
rudof.generate_data(&schema_input, &GenerationSchemaFormat::ShEx, 10)
    .with_output(&mut std::io::stdout())
    .with_parallel(4)
    .with_result_generation_format(&DataFormat::Turtle)
    .with_seed(42)
    .execute()
    .await
    .unwrap();
```

