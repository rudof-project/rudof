use rudof_lib::formats::{
    DCTapFormat, DataFormat, DataReaderMode, GenerationSchemaFormat, InputSpec, NodeInspectionMode, PgSchemaFormat,
    QueryType, RdfConfigFormat, ResultDCTapFormat, ResultDataFormat, ResultPgSchemaValidationFormat, ResultQueryFormat,
    ResultRdfConfigFormat, ResultServiceFormat, ResultShExValidationFormat, ResultShaclValidationFormat, ShExFormat,
    ShExValidationSortByMode, ShaclFormat, ShaclValidationSortByMode, ShapeMapFormat,
};
use rudof_lib::{Rudof, RudofConfig};
use std::path::PathBuf;
use std::str::FromStr;

fn core() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let version = rudof.version().execute();
    println!("Rudof version: {}", version);
    rudof.update_config(RudofConfig::default());
    rudof.reset_all().execute();
}

fn data_rdf() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let rdf_data_input = vec![
        InputSpec::from_str(
            r#"
            @prefix ex: <http://example.org/> .
            ex:alice a ex:Person ;
                ex:age 30 .
        "#,
        )
        .unwrap(),
    ];

    rudof
        .load_data()
        .with_data(&rdf_data_input)
        .with_data_format(&DataFormat::Turtle)
        .with_reader_mode(&DataReaderMode::Lax)
        .with_merge(false)
        .execute()
        .unwrap();

    rudof
        .serialize_data(&mut std::io::stdout())
        .with_result_data_format(&ResultDataFormat::NTriples)
        .execute()
        .unwrap();

    let node = "ex:alice";
    let predicates = vec!["ex:age".to_string()];
    rudof
        .show_node_info(node, &mut std::io::stdout())
        .with_show_node_mode(&NodeInspectionMode::Outgoing)
        .with_predicates(&predicates)
        .with_depth(1)
        .with_show_colors(false)
        .execute()
        .unwrap();

    rudof.reset_data().execute();
}

fn data_pg() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let pg_data_input = vec![
        InputSpec::from_str(
            r#"
            (alice {Person} [ name: "Alice", age: 23, aliases: "Ally" ])
            (bob   {Person} [ name: "Robert", aliases: ["Bob", "Bobby"] ])
        "#,
        )
        .unwrap(),
    ];

    rudof
        .load_data()
        .with_data(&pg_data_input)
        .with_data_format(&DataFormat::Pg)
        .execute()
        .unwrap();

    rudof.serialize_data(&mut std::io::stdout()).execute().unwrap();
}

fn data_endpoint() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let endpoint = "http://example.org/sparql";
    rudof.load_data().with_endpoint(endpoint).execute().unwrap();

    let endpoints = rudof.list_endpoints().execute().unwrap();
    for (name, url) in endpoints {
        println!("{}, {}", name, url);
    }
}

fn data_service_description() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let service_description_input = InputSpec::from_str(
        r#"
            @prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
            @prefix ent: <http://www.w3.org/ns/entailment/> .

            <http://example.org/sparql> a sd:Service ;
                sd:endpoint <http://example.org/sparql> ;
                sd:supportedLanguage sd:SPARQL11Query ;
                sd:defaultEntailmentRegime ent:Simple .
        "#,
    )
    .unwrap();

    let base = "http://example.org/";
    rudof
        .load_service_description(&service_description_input)
        .with_data_format(&DataFormat::Turtle)
        .with_reader_mode(&DataReaderMode::Strict)
        .with_base(base)
        .execute()
        .unwrap();

    rudof
        .serialize_service_description(&mut std::io::stdout())
        .with_result_service_format(&ResultServiceFormat::Json)
        .execute()
        .unwrap();
}

fn shex() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let rdf_data_input = vec![
        InputSpec::from_str(
            r#"
            <http://example.org/alice> <http://example.org/name> "Alice" ;
                <http://example.org/age> 30 .
        "#,
        )
        .unwrap(),
    ];
    let base_nodes = "http://example.org/";

    rudof
        .load_data()
        .with_data(&rdf_data_input)
        .with_base(base_nodes)
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
    )
    .unwrap();
    let base_shapes = "http://example.org/";

    rudof
        .check_shex_schema(&shex_schema_input, &mut std::io::stdout())
        .with_shex_schema_format(&ShExFormat::ShExC)
        .with_base(base_shapes)
        .execute()
        .unwrap();

    rudof
        .load_shex_schema(&shex_schema_input)
        .with_shex_schema_format(&ShExFormat::ShExC)
        .with_base(base_shapes)
        .execute()
        .unwrap();

    rudof
        .serialize_shex_schema(&mut std::io::stdout())
        .with_shape("<http://example.org/PersonShape>")
        .with_show_statistics(true)
        .with_show_dependencies(true)
        .with_show_time(true)
        .with_result_shex_format(&ShExFormat::ShExJ)
        .execute()
        .unwrap();

    let shapemap_input = InputSpec::from_str("<http://example.org/alice>@<PersonShape>").unwrap();

    rudof
        .load_shapemap(&shapemap_input)
        .with_shapemap_format(&ShapeMapFormat::Compact)
        .with_base_nodes(base_nodes)
        .with_base_shapes(base_shapes)
        .execute()
        .unwrap();

    rudof
        .serialize_shapemap(&mut std::io::stdout())
        .with_result_shapemap_format(&ShapeMapFormat::Compact)
        .with_show_colors(false)
        .execute()
        .unwrap();

    rudof.validate_shex().execute().unwrap();

    rudof
        .serialize_shex_validation_results(&mut std::io::stdout())
        .with_shex_validation_sort_order_mode(&ShExValidationSortByMode::Node)
        .with_result_shex_validation_format(&ResultShExValidationFormat::Details)
        .execute()
        .unwrap();

    rudof.reset_shex_schema().execute();
    rudof.reset_shapemap().execute();
    rudof.reset_shex().execute();
}

fn shacl() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let data_with_shapes_input = InputSpec::from_str(
        r#"
            @prefix ex: <http://example.org/> .
            @prefix sh: <http://www.w3.org/ns/shacl#> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

            ex:PersonShape
                a sh:NodeShape ;
                sh:targetClass ex:Person ;
                sh:property [
                    sh:path ex:name ;
                    sh:datatype xsd:string ;
                    sh:minCount 1 ;
                ] .

            ex:Alice
                a ex:Person ;
                ex:name "Alice Smith" ;
                ex:age 30 .

            ex:Bob
                a ex:Person ;
                ex:name "Bob Jones" ;
                ex:age 25 .
        "#,
    )
    .unwrap();

    rudof
        .load_data()
        .with_data(&[data_with_shapes_input])
        .with_data_format(&DataFormat::Turtle)
        .execute()
        .unwrap();

    rudof.load_shacl_shapes().execute().unwrap();

    rudof
        .serialize_shacl_shapes(&mut std::io::stdout())
        .with_shacl_result_format(&ShaclFormat::NTriples)
        .execute()
        .unwrap();

    rudof.validate_shacl().execute().unwrap();

    rudof
        .serialize_shacl_validation_results(&mut std::io::stdout())
        .with_shacl_validation_sort_order_mode(&ShaclValidationSortByMode::Node)
        .with_result_shacl_validation_format(&ResultShaclValidationFormat::Details)
        .execute()
        .unwrap();

    rudof.reset_shacl_shapes().execute();
    rudof.reset_shacl().execute();
}

fn pgschema() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let pg_data_input = vec![
        InputSpec::from_str(
            r#"
            (n1 {"Student"}["name": "Alice", "age": 23])
            (n2_wrong {"Student"}["name": "Bob", "age": 12])
        "#,
        )
        .unwrap(),
    ];

    rudof
        .load_data()
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
        "#,
    )
    .unwrap();

    rudof
        .load_pg_schema(&pg_schema_input)
        .with_pg_schema_format(&PgSchemaFormat::PgSchemaC)
        .execute()
        .unwrap();

    rudof
        .serialize_pg_schema(&mut std::io::stdout())
        .with_result_pg_schema_format(&PgSchemaFormat::PgSchemaC)
        .execute()
        .unwrap();

    let typemap_input = InputSpec::from_str(
        r#"
            n1: AdultStudentType,
            n2_wrong: AdultStudentType
        "#,
    )
    .unwrap();

    rudof.load_typemap(&typemap_input).execute().unwrap();

    rudof.validate_pgschema().execute().unwrap();

    rudof
        .serialize_pgschema_validation_results(&mut std::io::stdout())
        .with_result_pg_schema_validation_format(&ResultPgSchemaValidationFormat::Compact)
        .with_show_colors(false)
        .execute()
        .unwrap();

    rudof.reset_pg_schema().execute();
    rudof.reset_typemap().execute();
    rudof.reset_pg_schema_validation().execute();
}

fn query() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let data_input = InputSpec::from_str(
        r#"
            @prefix : <http://example.org/> .
            @prefix schema: <http://schema.org/> .

            :a schema:name  "Alice" ;
               :status      :Active ;
               schema:knows :a, :b  .

            :b schema:name  "Bob"    ;
               :status      :Waiting ;
               schema:knows :c       .

            :c schema:name  "Carol"  .
        "#,
    )
    .unwrap();

    rudof
        .load_data()
        .with_data(&[data_input])
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

    rudof
        .load_sparql_query(&query_input)
        .with_query_type(&QueryType::Select)
        .execute()
        .unwrap();

    rudof.serialize_sparql_query(&mut std::io::stdout()).execute().unwrap();

    rudof.run_query().execute().unwrap();

    rudof
        .serialize_query_results(&mut std::io::stdout())
        .with_result_query_format(&ResultQueryFormat::Csv)
        .execute()
        .unwrap();

    rudof.reset_sparql_query().execute();
    rudof.reset_query_results().execute();
}

fn dctap() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let dctap_input = InputSpec::from_str(
        r#"shapeID,propertyID,mandatory,repeatable,valueDataType
:Person,rdf:type,true,false,
:Person,schema:name,true,false,xsd:string
:Person,schema:age,false,false,xsd:integer
"#,
    )
    .unwrap();

    rudof
        .load_dctap(&dctap_input)
        .with_dctap_format(&DCTapFormat::Csv)
        .execute()
        .unwrap();

    rudof
        .serialize_dctap(&mut std::io::stdout())
        .with_result_dctap_format(&ResultDCTapFormat::Json)
        .execute()
        .unwrap();

    rudof.reset_dctap().execute();
}

fn rdf_config() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let config_input = InputSpec::from_str(
        r#"
- Person ex:person1 ex:person2:
  - a: ex:Person
  - rdfs:label:
      - name: "Alice"
  - ex:age?:
      - age_value: 32
"#,
    )
    .unwrap();

    rudof
        .load_rdf_config(&config_input)
        .with_rdf_config_format(&RdfConfigFormat::Yaml)
        .execute()
        .unwrap();

    rudof
        .serialize_rdf_config(&mut std::io::stdout())
        .with_result_rdf_config_format(&ResultRdfConfigFormat::Yaml)
        .execute()
        .unwrap();

    rudof.reset_rdf_config().execute();
}

async fn generation() {
    let rudof = Rudof::new(RudofConfig::default());
    let schema_input = InputSpec::Path(PathBuf::from("examples/user.shex"));

    rudof
        .generate_data(&schema_input, &GenerationSchemaFormat::ShEx, Some(10))
        .with_result_generation_format(&DataFormat::Turtle)
        .with_seed(42)
        .with_parallel(4)
        .execute()
        .await
        .unwrap();
}

fn prefixes() {
    let mut rudof = Rudof::new(RudofConfig::default());

    rudof
        .add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")
        .execute()
        .unwrap();
    let pm = rudof.prefixes().execute();
    println!("{pm}");

    rudof.rename_prefix("rdf", "rdf1").execute().unwrap();
    rudof.copy_prefix("rdf1", "rdf").execute().unwrap();
    rudof.remove_prefix("rdf1").execute().unwrap();
}

#[tokio::main]
async fn main() {
    core();
    data_rdf();
    data_pg();
    data_endpoint();
    data_service_description();
    shex();
    shacl();
    pgschema();
    query();
    dctap();
    rdf_config();
    generation().await;
    prefixes();
    println!("ALL OK");
}
