//! The ShExPath of every shape expression and triple constraint of a compiled schema selects it,
//! and only it, in the ShExJ form of the schema.

use rbe::Component;
use rudof_iri::{IriS, iri};
use shex_ast::ShapeLabelIdx;
use shex_ast::ast::Schema;
use shex_ast::ir::external_resolver::ExternalShapeResolverRegistry;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_actions_registry::SemanticActionsRegistry;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::shexpath::ShExJDocument;
use shex_ast::{ResolveMethod, ShExParser};

const SCHEMA: &str = r#"
PREFIX : <http://a.example/>
:S { :p . ; ( :q . | :p { :r . } ) }
:A IRI AND { :p . }
:N NOT { :p . }
:V { :p @:S OR { :q . } ; :t NOT { :u . } ; :w { :x . } AND { :y . } }
:B { :p [ 1 ] ; ^:p . }
:L { $:lbl :p { :q . } ; :p . }
start = { :p . }
"#;

fn compile(shexc: &str) -> (Schema, SchemaIR) {
    let source = iri!("http://a.example/schema.shex");
    let schema = ShExParser::parse(shexc, None, &source).unwrap();
    let mut ir = SchemaIR::new(SemanticActionsRegistry::default());
    ir.populate_from_schema_json(
        &schema,
        &ExternalShapeResolverRegistry::default(),
        &ResolveMethod::default(),
        &None,
    )
    .unwrap();
    (schema, ir)
}

/// Every index of the schema.
fn indexes(ir: &SchemaIR) -> Vec<ShapeLabelIdx> {
    (0..)
        .map(ShapeLabelIdx::from)
        .take_while(|idx| ir.find_shape_idx(idx).is_some())
        .collect()
}

/// The components of the shape at `idx`, if it is a shape.
fn components(ir: &SchemaIR, idx: &ShapeLabelIdx) -> Vec<Component> {
    match ir.find_shape_idx(idx).map(|info| info.expr()) {
        Some(ShapeExpr::Shape(shape)) => shape.triple_expr().components().map(|(c, _, _)| c).collect(),
        _ => Vec::new(),
    }
}

fn idx(ir: &SchemaIR, local: &str) -> ShapeLabelIdx {
    let label = ShapeLabel::iri(IriS::new_unchecked(&format!("http://a.example/{local}")));
    ir.get_shape_label_idx(&label).unwrap()
}

#[test]
fn every_path_selects_exactly_what_it_names() {
    let (schema, ir) = compile(SCHEMA);
    let doc = ShExJDocument::new(&schema).unwrap();
    let pm = ir.prefixmap();
    let mut checked = 0;
    for idx in indexes(&ir) {
        // `:S`-like indexes and the anonymous ones inside them
        if let Some(path) = ir.path_of(&idx) {
            let shown = path.show_qualified(&pm);
            let items = doc.select(&shown).unwrap_or_else(|e| panic!("{shown}: {e}"));
            assert_eq!(items.len(), 1, "{shown} selects {} items", items.len());
            checked += 1;
        }
        for component in components(&ir, &idx) {
            let path = ir
                .component_path(&idx, &component)
                .unwrap_or_else(|| panic!("no path for {component} of {idx}"));
            let shown = path.show_qualified(&pm);
            let items = doc.select(&shown).unwrap_or_else(|e| panic!("{shown}: {e}"));
            assert_eq!(items.len(), 1, "{shown} selects {} items", items.len());
            assert_eq!(items[0].value()["type"], "TripleConstraint", "{shown}");
            checked += 1;
        }
    }
    assert!(checked > 20, "only {checked} paths checked");
}

#[test]
fn the_paths_of_anonymous_shape_expressions() {
    let (_, ir) = compile(SCHEMA);
    let pm = ir.prefixmap();
    let mut shown: Vec<String> = indexes(&ir)
        .iter()
        .filter(|idx| ir.shape_label_from_idx(idx).is_none())
        .filter_map(|idx| ir.path_of(idx))
        .map(|path| path.show_qualified(&pm))
        .collect();
    shown.sort();
    assert_eq!(
        shown,
        [
            "$:lbl/valueExpr",
            "@:A/shapeExprs/*[0]",
            "@:A/shapeExprs/*[1]",
            "@:N/shapeExpr",
            "@:S~:p/self::[1]/valueExpr",
            "@:V~:p/valueExpr",
            "@:V~:p/valueExpr/shapeExprs/*[0]",
            "@:V~:p/valueExpr/shapeExprs/*[1]",
            "@:V~:t/valueExpr",
            "@:V~:t/valueExpr/shapeExpr",
            "@:V~:w/valueExpr",
            "@:V~:w/valueExpr/shapeExprs/*[0]",
            "@:V~:w/valueExpr/shapeExprs/*[1]",
        ]
    );
}

#[test]
fn constraints_on_a_repeated_predicate_are_numbered() {
    let (_, ir) = compile(SCHEMA);
    let pm = ir.prefixmap();
    let shown = |local: &str| -> Vec<String> {
        let idx = idx(&ir, local);
        components(&ir, &idx)
            .iter()
            .map(|c| ir.component_path(&idx, c).unwrap().show_qualified(&pm))
            .collect()
    };
    assert_eq!(shown("S"), ["@:S~:p/self::[0]", "@:S~:q", "@:S~:p/self::[1]"]);
    // An inverse constraint is on the same predicate, as `~` sees it.
    assert_eq!(shown("B"), ["@:B~:p/self::[0]", "@:B~:p/self::[1]"]);
    // A labelled constraint is named by its label; the other one is still counted after it.
    assert_eq!(shown("L"), ["$:lbl", "@:L~:p/self::[1]"]);
}
