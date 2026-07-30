//! Tests for `SchemaIR::extend_alternatives`: resolution of extended parents that are
//! references, ShapeAnds and ShapeOrs into selection alternatives.
//! See docs/src/internals/feasibility-model.md §3.

use rudof_iri::iri;
use shex_ast::ShapeLabelIdx;
use shex_ast::ir::external_resolver::ExternalShapeResolverRegistry;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_actions_registry::SemanticActionsRegistry;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::{ResolveMethod, ShExParser};

fn compile(shexc: &str) -> SchemaIR {
    let source = iri!("file:///test/extend_alternatives.shex");
    let schema_json = ShExParser::parse(shexc, None, &source).unwrap();
    let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
    ir.populate_from_schema_json(
        &schema_json,
        &ExternalShapeResolverRegistry::default(),
        &ResolveMethod::default(),
        &None,
    )
    .unwrap();
    ir
}

fn idx(ir: &SchemaIR, iri_str: &str) -> ShapeLabelIdx {
    ir.get_shape_label_idx(&ShapeLabel::iri(iri!(iri_str))).unwrap()
}

#[test]
fn ref_parent_single_alternative() {
    let ir = compile(
        r#"PREFIX : <http://e/>
           :A { :p . }
           :B @:A
           :C EXTENDS @:B { :q . }"#,
    );
    let alts = ir.extend_alternatives(&idx(&ir, "http://e/B"));
    assert_eq!(alts.len(), 1);
    assert_eq!(alts[0].bucket_shapes(), &[idx(&ir, "http://e/A")]);
    assert!(alts[0].constraints().is_empty());
}

#[test]
fn or_parent_one_alternative_per_branch() {
    let ir = compile(
        r#"PREFIX : <http://e/>
           :A1 CLOSED { :p . }
           :A2 CLOSED { :r . }
           :AOr @:A1 OR @:A2
           :C EXTENDS @:AOr { :q . }"#,
    );
    let alts = ir.extend_alternatives(&idx(&ir, "http://e/AOr"));
    assert_eq!(alts.len(), 2);
    let buckets: Vec<_> = alts.iter().map(|a| a.bucket_shapes().to_vec()).collect();
    assert_eq!(buckets[0], vec![idx(&ir, "http://e/A1")]);
    assert_eq!(buckets[1], vec![idx(&ir, "http://e/A2")]);
    assert!(alts.iter().all(|a| a.constraints().is_empty()));
}

const PERSON_EXTENDS: &str = r#"
BASE <http://schema.example/>
PREFIX ex: <http://a.example/ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX schema: <http://schema.org/>

<Item> CLOSED { ex:id IRI ; ex:rolls @<RoleCode>+ }

<RoleCode> [ ex:Manager ex:Engineer ex:Laboror ex:Human ex:Robot ]

<Contact> ( EXTENDS @<Item> CLOSED { foaf:name . ; foaf:mbox IRI }
            OR
            EXTENDS @<Item> CLOSED { schema:name . ; schema:mbox IRI }
          )
      AND EXTRA ex:rolls { ex:rolls [ex:Human] }
      AND
      NOT EXTRA ex:rolls { ex:rolls [ex:Robot] }

<Contact1> @<Contact>

<TBoss>  EXTENDS @<Item>
         CLOSED { ex:sunglassesBrand . ; ex:tieMaterial . }
     AND
         EXTRA ex:rolls { ex:rolls [ex:Manager] }

<TGeek>  EXTENDS @<Item>
         CLOSED { ex:SlideRoolLength . ; ex:PocketProtectorMaterial . }
     AND
         EXTRA ex:rolls { ex:rolls [ex:Engineer] }

<TLabor> EXTENDS @<Item>
         CLOSED { ex:glovesSize . ; ex:bootSize . }
     AND
         EXTRA ex:rolls { ex:rolls [ex:Laboror] }

<Tools> @<TBoss> OR @<TGeek> OR @<TLabor>

<Person> EXTENDS @<Contact1> EXTENDS @<Tools>
         CLOSED { ex:badgeNumber xsd:integer }
"#;

#[test]
fn person_contact_two_alternatives_with_shared_item() {
    let ir = compile(PERSON_EXTENDS);
    let item = idx(&ir, "http://schema.example/Item");
    // Through the reference <Contact1> -> <Contact>: one alternative per OR branch,
    // each bringing the branch shape plus <Item> (via the branch's EXTENDS), and the
    // two AND conjuncts (EXTRA-rolls-Human shape, NOT-Robot) as constraints.
    let alts = ir.extend_alternatives(&idx(&ir, "http://schema.example/Contact1"));
    assert_eq!(alts.len(), 2);
    for alt in &alts {
        assert_eq!(alt.bucket_shapes().len(), 2, "branch shape + Item: {alt:?}");
        assert!(alt.bucket_shapes().contains(&item));
        assert_eq!(alt.constraints().len(), 2, "EXTRA-Human + NOT-Robot: {alt:?}");
    }
    // The two alternatives differ in their branch shape
    assert_ne!(alts[0].bucket_shapes(), alts[1].bucket_shapes());
}

#[test]
fn person_tools_three_alternatives() {
    let ir = compile(PERSON_EXTENDS);
    let item = idx(&ir, "http://schema.example/Item");
    let alts = ir.extend_alternatives(&idx(&ir, "http://schema.example/Tools"));
    assert_eq!(alts.len(), 3);
    for alt in &alts {
        assert_eq!(alt.bucket_shapes().len(), 2, "tool shape + Item: {alt:?}");
        assert!(alt.bucket_shapes().contains(&item));
        assert_eq!(alt.constraints().len(), 1, "EXTRA-role shape: {alt:?}");
    }
}

#[test]
fn person_six_selections_with_item_diamond_deduplicated() {
    let ir = compile(PERSON_EXTENDS);
    let item = idx(&ir, "http://schema.example/Item");
    let person = idx(&ir, "http://schema.example/Person");
    // Person itself: {Person} x alts(Contact1) x alts(Tools) = 2 x 3 = 6 selections.
    // Item is reached through both parents (diamond) and must appear once per selection.
    let alts = ir.extend_alternatives(&person);
    assert_eq!(alts.len(), 6);
    for alt in &alts {
        let item_occurrences = alt.bucket_shapes().iter().filter(|b| **b == item).count();
        assert_eq!(item_occurrences, 1, "diamond Item deduplicated: {alt:?}");
        // Person + contact branch + Item + tool shape
        assert_eq!(alt.bucket_shapes().len(), 4, "{alt:?}");
        // EXTRA-Human + NOT-Robot + EXTRA-role
        assert_eq!(alt.constraints().len(), 3, "{alt:?}");
        assert_eq!(alt.bucket_shapes()[0], person, "own shape is the first bucket");
    }
    // All six selections are distinct
    for i in 0..alts.len() {
        for j in i + 1..alts.len() {
            assert_ne!(alts[i], alts[j]);
        }
    }
}

#[test]
fn plain_shape_parent_is_single_bucket() {
    let ir = compile(
        r#"PREFIX : <http://e/>
           :A { :p . }
           :C EXTENDS @:A { :q . }"#,
    );
    let alts = ir.extend_alternatives(&idx(&ir, "http://e/A"));
    assert_eq!(alts.len(), 1);
    assert_eq!(alts[0].bucket_shapes(), &[idx(&ir, "http://e/A")]);
}
