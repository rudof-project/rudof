use anyhow::Result;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::OxigraphInMemory;
use shex_ast::ResolveMethod;
use shex_ast::ShExParser;
use shex_ast::Schema;
use shex_ast::ShapeMapParser;
use shex_ast::ir::map_action_extension::MapActionExtension;
use shex_ast::ir::map_state::MapState;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::semantic_actions_registry::SemanticActionsRegistry;
use shex_ast::ir::test_action_extension::TestActionExtension;
use shex_ast::shapemap::QueryShapeMap;
use shex_ast::shapemap::ResultShapeMap;
use shex_validation::Validator;
use shex_validation::ValidatorConfig;

/// Stage 1: Text ShExC Schema to AST Schema.
#[inline(never)]
pub fn parse(src: &str, base: Option<IriS>, source_iri: &IriS) -> Result<Schema> {
    Ok(ShExParser::parse(src, base, source_iri)?)
}

/// Stage 2: AST Schema to IR Schema.
#[inline(never)]
pub fn compile(schema: &Schema, base: Option<IriS>, cfg: &ValidatorConfig) -> Result<SchemaIR> {
    let registry = SemanticActionsRegistry::new().with(
        vec![
            Box::new(TestActionExtension::new()),
            Box::new(MapActionExtension::new(MapState::default())),
        ]
    );
    let mut ir_schema = SchemaIR::new(registry);
    ir_schema.populate_from_schema_json(
        schema,
        cfg.external_resolvers(),
        &ResolveMethod::default(),
        &base,
    )?;
    Ok(ir_schema)
}

/// Stage 3: Negation-cycle check + validator allocation.
#[inline(never)]
pub fn validator_init(ir_schema: &SchemaIR, cfg: &ValidatorConfig) -> Result<Validator> {
    Ok(Validator::new(ir_schema, cfg)?)
}

/// Stage 4: Run validation.
#[inline(never)]
pub fn validate(
    validator: &Validator,
    shapemap: &QueryShapeMap,
    rdf: &OxigraphInMemory,
    ir: &SchemaIR,
) -> Result<ResultShapeMap> {
    Ok(validator.validate_shapemap(shapemap, rdf, ir, &None)?)
}

// -- Helpers used in bench setup (not measured) --

pub fn load_rdf(data_src: &str, base: &IriS) -> Result<OxigraphInMemory> {
    Ok(
        OxigraphInMemory::from_str(
            data_src,
            &RDFFormat::Turtle,
            Some(base.as_str()),
            &rudof_rdf::rdf_impl::ReaderMode::Strict,
        )?
    )
}

pub fn parse_shapemap(src: &str) -> Result<QueryShapeMap> {
    Ok(ShapeMapParser::parse(src, &None, &None, &None, &None)?)
}
