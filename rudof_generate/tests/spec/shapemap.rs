//! The generator records which shape produced each node, as a ShEx ShapeMap.
//!
//! Validation targets derived from `rdf:type` over-check: a node typed with a
//! subclass and its superclass is also checked against the superclass shape,
//! which a shape that names `rdf:type` without EXTRA rejects. The exact map
//! checks each node against the shape it was generated from, which is what
//! "the output conforms to the schema" means for a generator.

use crate::common::*;
use rudof_generate::GeneratorConfig;

const SCHEMA: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:StudentShape { a [ex:Student] ; ex:name xsd:string }
ex:AssistantShape { a [ex:Student] ; a [ex:Assistant] ; ex:name xsd:string ; ex:salary xsd:integer }
"#;

async fn generate_with_shapemap(entities: usize) -> (String, String) {
    let fixture = Fixture::shex(SCHEMA);
    let mut cfg = config(&fixture.output(), entities, 5);
    let shapemap_path = fixture.dir.path().join("out.shapemap");
    cfg.output.shapemap = Some(shapemap_path.clone());
    let data = generate(&fixture, cfg).await;
    let shapemap = std::fs::read_to_string(shapemap_path).expect("shapemap should be written");
    (data, shapemap)
}

fn associations(shapemap: &str) -> Vec<(String, String)> {
    shapemap
        .split(',')
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(|a| {
            let (node, shape) = a.split_once('@').unwrap_or_else(|| panic!("not an association: {a}"));
            (node.to_string(), shape.to_string())
        })
        .collect()
}

#[tokio::test]
async fn the_shapemap_lists_every_generated_entity_once_with_its_shape() {
    let (_, shapemap) = generate_with_shapemap(12).await;
    let assoc = associations(&shapemap);
    assert_eq!(assoc.len(), 12, "{shapemap}");
    let mut nodes: Vec<_> = assoc.iter().map(|(n, _)| n.clone()).collect();
    nodes.sort();
    nodes.dedup();
    assert_eq!(nodes.len(), 12, "each node appears once\n{shapemap}");
    for (node, shape) in &assoc {
        let shape_iri = shape.trim_matches(['<', '>']);
        assert!(
            node.trim_matches(['<', '>']).starts_with(shape_iri),
            "{node} was minted by {shape}"
        );
    }
}

#[tokio::test]
async fn the_exact_shapemap_validates_where_type_based_targets_overcheck() {
    let (data, shapemap) = generate_with_shapemap(12).await;

    let exact = validate_shex(&data, SCHEMA, &shapemap);
    assert!(
        exact.conforms() && exact.conformant == 12,
        "every node conforms to the shape it came from\n{}",
        exact.report
    );

    // Assistants are also Students; checking them against StudentShape fails
    // because that shape names rdf:type without EXTRA. This is the over-check
    // the exact map avoids (see docs/research/phase0-conformance.md).
    let by_type = validate_shex(
        &data,
        SCHEMA,
        "{FOCUS a <http://example.org/Student>}@<http://example.org/StudentShape>",
    );
    assert!(by_type.violations > 0, "{}", by_type.report);
}

#[test]
fn v2_output_shapemap_sets_the_path() {
    let config = GeneratorConfig::from_toml_str("version = 2\n[output]\nshapemap = \"run.shapemap\"\n").unwrap();
    assert_eq!(
        config.output.shapemap.as_deref(),
        Some(std::path::Path::new("run.shapemap"))
    );
}

#[tokio::test]
async fn no_shapemap_is_written_unless_asked() {
    let fixture = Fixture::shex(SCHEMA);
    let cfg = config(&fixture.output(), 4, 5);
    generate(&fixture, cfg).await;
    let leftovers: Vec<_> = std::fs::read_dir(fixture.dir.path())
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "shapemap"))
        .collect();
    assert!(leftovers.is_empty());
}
