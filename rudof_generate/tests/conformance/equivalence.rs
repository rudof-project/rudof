//! Equivalence by generation. The same constructs written in ShEx, SHACL and
//! PG-Schema must give the same graph from the same seed, triple for triple, and that
//! graph must be valid in the three languages. The RDF schemas are written over the
//! RDF encoding of the property graph (keys, labels and edges as IRIs under
//! `http://rudof.example/pg/`), which is how a property graph and an RDF graph are
//! compared at all. Each group is one layer of the paper: L1 is the common fragment of
//! Ahmetaj et al. (WWW 2025), L2 the constructs outside it that the three languages share.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::OutputFormat;
use std::collections::BTreeSet;

const RDF_PREFIXES: &str = "PREFIX t: <http://rudof.example/pg/type/>
PREFIX k: <http://rudof.example/pg/key/>
PREFIX l: <http://rudof.example/pg/label/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
";

const SHACL_PREFIXES: &str = "@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix t: <http://rudof.example/pg/type/> .
@prefix k: <http://rudof.example/pg/key/> .
@prefix l: <http://rudof.example/pg/label/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
";

/// One construct group in the three languages.
struct Equivalent {
    pgs: &'static str,
    shex: &'static str,
    shacl: &'static str,
}

/// L1: required, optional and multi-valued keys of four value types.
const KEYS: Equivalent = Equivalent {
    pgs: "CREATE NODE TYPE ( PersonType : Person {
    name: STRING, OPTIONAL nickname: STRING, tags: STRING {1,3}, birthDate: DATE, active: BOOL })",
    shex: "t:PersonType CLOSED { a [ l:Person ] ; k:name xsd:string ; k:nickname xsd:string ? ;
  k:tags xsd:string {1,3} ; k:birthDate xsd:date ; k:active xsd:boolean }",
    shacl:
        "t:PersonType a sh:NodeShape ; sh:targetClass l:Person ; sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:nickname ; sh:datatype xsd:string ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:tags ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 3 ] ;
  sh:property [ sh:path k:birthDate ; sh:datatype xsd:date ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:active ; sh:datatype xsd:boolean ; sh:minCount 1 ; sh:maxCount 1 ] .",
};

/// L1: a pattern and inclusive and exclusive integer ranges.
const FACETS: Equivalent = Equivalent {
    pgs: "CREATE NODE TYPE ( ItemType : Item {
    code: STRING CHECK REGEX \"^[A-Z]{3}[0-9]{2}$\",
    price: INTEGER CHECK >= 1 AND <= 500,
    stock: INTEGER CHECK > 0 AND < 100 })",
    shex: "t:ItemType CLOSED { a [ l:Item ] ; k:code xsd:string /^[A-Z]{3}[0-9]{2}$/ ;
  k:price xsd:integer MinInclusive 1 MaxInclusive 500 ; k:stock xsd:integer MinExclusive 0 MaxExclusive 100 }",
    shacl: "t:ItemType a sh:NodeShape ; sh:targetClass l:Item ; sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:code ; sh:datatype xsd:string ; sh:pattern \"^[A-Z]{3}[0-9]{2}$\" ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:price ; sh:datatype xsd:integer ; sh:minInclusive 1 ; sh:maxInclusive 500 ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:stock ; sh:datatype xsd:integer ; sh:minExclusive 0 ; sh:maxExclusive 100 ; sh:minCount 1 ; sh:maxCount 1 ] .",
};

/// L2: a node type with two labels.
const LABELS: Equivalent = Equivalent {
    pgs: "CREATE NODE TYPE ( StaffType : Person & Employee { name: STRING })",
    shex: "t:StaffType CLOSED { a [ l:Person ] ; a [ l:Employee ] ; k:name xsd:string }",
    shacl: "t:StaffType a sh:NodeShape ; sh:targetClass l:Person ; sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path rdf:type ; sh:hasValue l:Employee ] ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .",
};

/// L2: references to node types over two steps.
const REFERENCES: Equivalent = Equivalent {
    pgs: "CREATE NODE TYPE ( PersonType : Person { name: STRING, OPTIONAL age: INTEGER CHECK > 17 }) ;
CREATE NODE TYPE ( OrgType : Org { label: STRING }) ;
CREATE NODE TYPE ( CityType : City { name: STRING }) ;
CREATE EDGE TYPE (:@PersonType) -[WorksForType : worksFor]-> (:@OrgType) ;
CREATE EDGE TYPE (:@OrgType) -[LocatedInType : locatedIn]-> (:@CityType)",
    shex: "t:PersonType CLOSED { a [ l:Person ] ; k:name xsd:string ; k:age xsd:integer MinExclusive 17 ? ;
  <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> @t:OrgType * }
t:OrgType CLOSED { a [ l:Org ] ; k:label xsd:string ;
  <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> @t:CityType * }
t:CityType CLOSED { a [ l:City ] ; k:name xsd:string }",
    shacl: "t:PersonType a sh:NodeShape ; sh:targetClass l:Person ; sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path k:age ; sh:datatype xsd:integer ; sh:minExclusive 17 ; sh:maxCount 1 ] ;
  sh:property [ sh:path <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> ; sh:node t:OrgType ] .
t:OrgType a sh:NodeShape ; sh:targetClass l:Org ; sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:label ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] ;
  sh:property [ sh:path <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> ; sh:node t:CityType ] .
t:CityType a sh:NodeShape ; sh:targetClass l:City ; sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .",
};

const ENTITIES: usize = 120;
const SEED: u64 = 13;

/// The triples of a Turtle document, as N-Triples lines.
fn triples(turtle: &str) -> BTreeSet<String> {
    oxttl::TurtleParser::new()
        .for_slice(turtle.as_bytes())
        .map(|t| t.expect("generated Turtle parses").to_string())
        .collect()
}

/// Generate from a ShEx or SHACL fixture; return the Turtle and the ShapeMap written.
async fn generate_rdf(fixture: &Fixture) -> (String, String) {
    let map = fixture.dir.path().join("out.shapemap");
    let mut cfg = config(&fixture.output(), ENTITIES, SEED);
    cfg.output.shapemap = Some(map.clone());
    let data = generate(fixture, cfg).await;
    (data, std::fs::read_to_string(map).unwrap())
}

/// Check one group: the same graph from the three schemas, valid in the three languages.
async fn assert_equivalent(group: &Equivalent) {
    let shex_schema = format!("{RDF_PREFIXES}{}", group.shex);
    let shacl_schema = format!("{SHACL_PREFIXES}{}", group.shacl);

    // PG-Schema: the property graph, its type map, and the same graph as RDF.
    let pg_fixture = Fixture::pgschema(group.pgs);
    let dir = pg_fixture.dir.path();
    let mut cfg = GeneratorConfig::default();
    cfg.generation.entity_count = ENTITIES;
    cfg.generation.seed = Some(SEED);
    cfg.output.path = dir.join("out.pg");
    cfg.output.format = OutputFormat::Pg;
    cfg.output.write_stats = false;
    cfg.output.shapemap = Some(dir.join("out.map"));
    cfg.output.rdf = Some(dir.join("out.ttl"));
    cfg.output.rdf_shapemap = Some(dir.join("out.shapemap"));
    let pg_graph = generate(&pg_fixture, cfg).await;
    let type_map = std::fs::read_to_string(dir.join("out.map")).unwrap();
    let pg_rdf = std::fs::read_to_string(dir.join("out.ttl")).unwrap();
    let pg_shapemap = std::fs::read_to_string(dir.join("out.shapemap")).unwrap();

    let (shex_rdf, shex_map) = generate_rdf(&Fixture::shex(&shex_schema)).await;
    let (shacl_rdf, _) = generate_rdf(&Fixture::shacl(&shacl_schema)).await;

    // The same data from the three languages.
    let base = triples(&pg_rdf);
    assert!(base.len() > ENTITIES, "too few triples: {}", base.len());
    assert_eq!(triples(&shex_rdf), base, "ShEx and PG-Schema generate different graphs");
    assert_eq!(
        triples(&shacl_rdf),
        base,
        "SHACL and PG-Schema generate different graphs"
    );
    assert_eq!(shex_map.lines().count(), pg_shapemap.lines().count());

    // Valid in the three languages.
    let pg = validate_pgschema(&pg_graph, group.pgs, &type_map);
    assert!(pg.failing.is_empty() && pg.conformant > 0, "PG-Schema: {}", pg.report);
    let shex = validate_shex(&pg_rdf, &shex_schema, &pg_shapemap);
    assert!(shex.failing.is_empty() && shex.conformant > 0, "ShEx: {}", shex.report);
    let shacl = validate_shacl(&pg_rdf, &shacl_schema);
    assert!(shacl.failing.is_empty(), "SHACL: {}", shacl.report);
}

#[tokio::test]
async fn l1_keys_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent(&KEYS).await;
}

#[tokio::test]
async fn l1_facets_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent(&FACETS).await;
}

#[tokio::test]
async fn l2_labels_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent(&LABELS).await;
}

#[tokio::test]
async fn l2_references_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent(&REFERENCES).await;
}
