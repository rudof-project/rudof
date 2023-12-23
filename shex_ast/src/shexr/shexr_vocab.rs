use const_format::concatcp;
use iri_s::IriS;

pub const SX: &str = "http://www.w3.org/ns/shex#";

// Classes
pub const SX_ANNOTATION: &str = concatcp!(SX, "Annotation");
pub const SX_EACHOF: &str = concatcp!(SX, "EachOf");
pub const SX_IRISTEM: &str = concatcp!(SX, "IriStem");
pub const SX_IRISTEMRANGE: &str = concatcp!(SX, "IriStemRange");
pub const SX_LANGUAGESTEM: &str = concatcp!(SX, "LanguageStem");
pub const SX_LANGUAGESTEMRANGE: &str = concatcp!(SX, "LanguageStemRange");
pub const SX_LITERALSTEM: &str = concatcp!(SX, "LiteralStem");
pub const SX_LITERALSTEMRANGE: &str = concatcp!(SX, "LiteralStemRange");
pub const SX_NODECONSTRAINT: &str = concatcp!(SX, "NodeConstraint");
pub const SX_ONEOF: &str = concatcp!(SX, "OneOf");
pub const SX_SCHEMA: &str = concatcp!(SX, "Schema");
pub const SX_SEMACT: &str = concatcp!(SX, "SemAct");
pub const SX_SHAPE: &str = concatcp!(SX, "Shape");
pub const SX_SHAPE_AND: &str = concatcp!(SX, "ShapeAnd");
pub const SX_SHAPE_EXTERNAL: &str = concatcp!(SX, "ShapeExternal");
pub const SX_SHAPE_NOT: &str = concatcp!(SX, "ShapeNot");
pub const SX_SHAPE_DECL: &str = concatcp!(SX, "ShapeDecl");
pub const SX_SHAPE_OR: &str = concatcp!(SX, "ShapeOr");
pub const SX_TRIPLECONSTRAINT: &str = concatcp!(SX, "TripleConstraint");
pub const SX_WILDCARD: &str = concatcp!(SX, "Wildcard");
pub const SX_INF: &str = concatcp!(SX, "INF");

// Properties
pub const SX_ABSTRACT: &str = concatcp!(SX, "abstract");
pub const SX_ANNOTATION_PROP: &str = concatcp!(SX, "annotation");
pub const SX_BNODE: &str = concatcp!(SX, "bnode");
pub const SX_CODE: &str = concatcp!(SX, "code");
pub const SX_CLOSED: &str = concatcp!(SX, "closed");
pub const SX_DATATYPE: &str = concatcp!(SX, "datatype");
pub const SX_EXTRA: &str = concatcp!(SX, "extra");
pub const SX_EXCLUSION: &str = concatcp!(SX, "exclusion");
pub const SX_EXPRESION: &str = concatcp!(SX, "expresion");
pub const SX_EXPRESIONS: &str = concatcp!(SX, "expresions");
pub const SX_FRACTIONDIGITS: &str = concatcp!(SX, "fractiondigits");
pub const SX_FLAGS: &str = concatcp!(SX, "flags");
pub const SX_IRI: &str = concatcp!(SX, "iri");
pub const SX_INVERSE: &str = concatcp!(SX, "inverse");
pub const SX_LENGTH: &str = concatcp!(SX, "length");
pub const SX_LITERAL: &str = concatcp!(SX, "literal");
pub const SX_MIN: &str = concatcp!(SX, "min");
pub const SX_MININCLUSIVE: &str = concatcp!(SX, "mininclusive");
pub const SX_MINEXCLUSIVE: &str = concatcp!(SX, "minexclusive");
pub const SX_MINLENGTH: &str = concatcp!(SX, "minlength");
pub const SX_MAX: &str = concatcp!(SX, "max");
pub const SX_MAXINCLUSIVE: &str = concatcp!(SX, "maxinclusive");
pub const SX_MAXEXCLUSIVE: &str = concatcp!(SX, "maxexclusive");
pub const SX_MAXLENGTH: &str = concatcp!(SX, "maxlength");
pub const SX_NAME: &str = concatcp!(SX, "name");
pub const SX_NEGATED: &str = concatcp!(SX, "negated");
pub const SX_NODEKIND: &str = concatcp!(SX, "nodeKind");
pub const SX_NONLITERAL: &str = concatcp!(SX, "nonLiteral");
pub const SX_OBJECT: &str = concatcp!(SX, "object");
pub const SX_PATTERN: &str = concatcp!(SX, "pattern");
pub const SX_PREDICATE: &str = concatcp!(SX, "predicate");
pub const SX_SEMACTS: &str = concatcp!(SX, "semActs");
pub const SX_STARTACTS: &str = concatcp!(SX, "startActs");
pub const SX_START: &str = concatcp!(SX, "start");
pub const SX_SHAPES: &str = concatcp!(SX, "shapes");
pub const SX_SHAPE_EXPR: &str = concatcp!(SX, "shapeExpr");
pub const SX_SHAPE_EXPRS: &str = concatcp!(SX, "shapeExprs");
pub const SX_STEM: &str = concatcp!(SX, "stem");
pub const SX_STEMRANGE: &str = concatcp!(SX, "stemRange");
pub const SX_TOTALDIGITS: &str = concatcp!(SX, "totaldigits");
pub const SX_VALUEEXPR: &str = concatcp!(SX, "valueExpr");
pub const SX_VALUES: &str = concatcp!(SX, "values");

pub struct ShExRVocab {}

impl ShExRVocab {
    #[inline]
    pub fn sx_schema() -> IriS {
        IriS::new_unchecked(SX_SCHEMA)
    }

    #[inline]
    pub fn sx_shape_and() -> IriS {
        IriS::new_unchecked(SX_SHAPE_AND)
    }

    #[inline]
    pub fn sx_nodekind() -> IriS {
        IriS::new_unchecked(SX_NODEKIND)
    }

    #[inline]
    pub fn sx_nodeconstraint() -> IriS {
        IriS::new_unchecked(SX_NODECONSTRAINT)
    }

    #[inline]
    pub fn sx_shapes() -> IriS {
        IriS::new_unchecked(SX_SHAPES)
    }

    #[inline]
    pub fn sx_shape_expr() -> IriS {
        IriS::new_unchecked(SX_SHAPE_EXPR)
    }

    #[inline]
    pub fn sx_shape_exprs() -> IriS {
        IriS::new_unchecked(SX_SHAPE_EXPR)
    }

    #[inline]
    pub fn sx_shape() -> IriS {
        IriS::new_unchecked(SX_SHAPE)
    }

    #[inline]
    pub fn sx_iri() -> IriS {
        IriS::new_unchecked(SX_IRI)
    }

    #[inline]
    pub fn sx_literal() -> IriS {
        IriS::new_unchecked(SX_LITERAL)
    }

    #[inline]
    pub fn sx_bnode() -> IriS {
        IriS::new_unchecked(SX_BNODE)
    }

    #[inline]
    pub fn sx_nonliteral() -> IriS {
        IriS::new_unchecked(SX_NONLITERAL)
    }
}
