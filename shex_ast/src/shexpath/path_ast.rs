//! Abstract syntax of [ShExPath](https://shexspec.github.io/spec/ShExPath).
//!
//! The types follow the nonterminals of the spec's grammar, which are those of the reference
//! implementation's parser, `ShapePathParser.jison`.

use std::fmt;

/// A ShExPath.
///
/// `shapePath ::= sequenceStep ("," sequenceStep)*`
#[derive(Debug, Clone, PartialEq)]
pub enum ShExPath {
    /// `intersectionStep ::= startStep nextStep*`: steps applied left to right, each to every item
    /// the previous one produced.
    Path(Vec<Step>),
}

impl ShExPath {
    /// A path made of `steps`.
    pub fn path(steps: impl IntoIterator<Item = Step>) -> Self {
        ShExPath::Path(steps.into_iter().collect())
    }
}

/// A step of a path: which items are reachable from each item it is applied to, and the filters
/// that keep some of them.
///
/// ```text
/// step ::= "child::"? termType filter*
///        | "child::"? attributeOrAny termType? filter*
///        | nonChildAxis termType? filter*
/// ```
///
/// A term type is kept as the first filter, since it is "exactly the filter `[type = T]`".
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub selector: Selector,
    pub filters: Vec<Filter>,
}

impl Step {
    /// A step without filters.
    pub fn new(selector: Selector) -> Self {
        Step {
            selector,
            filters: Vec::new(),
        }
    }

    /// A `child::` step without filters.
    pub fn child(attribute: AttributeOrAny) -> Self {
        Step::new(Selector::Child(attribute))
    }

    /// A step on `axis` without filters.
    pub fn axis(axis: Axis) -> Self {
        Step::new(Selector::Axis(axis))
    }

    /// This step with `filter` added after its filters.
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }
}

/// Which items a step reaches from each item it is applied to.
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// `child::attribute` or `child::*`.
    Child(AttributeOrAny),
    /// Any other axis.
    Axis(Axis),
}

/// The axes other than `child::`.
///
/// `nonChildAxis ::= "thisShapeExpr::" | "thisTripleExpr::" | "self::" | "parent::" | "ancestor::"`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    /// `thisShapeExpr::`: the item and, if it is a `ShapeAnd`, `ShapeOr` or `ShapeNot`, the shape
    /// expressions it combines, transitively.
    ThisShapeExpr,
    /// `thisTripleExpr::`: the item and, if it is an `EachOf` or `OneOf`, the triple expressions
    /// it groups, transitively, an inclusion among them.
    ThisTripleExpr,
    /// `self::`: the item itself.
    Itself,
    /// `parent::`: the item that has this one as a property value or array member.
    Parent,
    /// `ancestor::`: the parent, its parent, and so on up to the schema.
    Ancestor,
    /// Every item below this one, at any depth. Used by the `$` shortcut; it has no spelling.
    Descendant,
}

impl Axis {
    /// The axes that have a spelling, with it.
    pub const SPELLED: &'static [(Axis, &'static str)] = &[
        (Axis::ThisShapeExpr, "thisShapeExpr::"),
        (Axis::ThisTripleExpr, "thisTripleExpr::"),
        (Axis::Itself, "self::"),
        (Axis::Parent, "parent::"),
        (Axis::Ancestor, "ancestor::"),
    ];
}

/// A filter of a step.
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// A term type: keeps the items whose ShExJ `type` is that one.
    TermType(TermType),
}

/// `attributeOrAny ::= "*" | attribute`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeOrAny {
    /// `*`: every property value of an object, or every member of an array.
    Any,
    /// The value of that ShExJ property.
    Attribute(Attribute),
}

/// Declares an enum of the keywords of a nonterminal of the grammar, with conversions to and from
/// the keyword.
macro_rules! keywords {
    ($(#[$meta:meta])* $name:ident { $($variant:ident => $kw:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                #[doc = concat!("`", $kw, "`")]
                $variant,
            )+
        }

        impl $name {
            /// Every keyword, in the order of the grammar.
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The value spelled `keyword`, if it is one. Keywords are case sensitive.
            pub fn from_keyword(keyword: &str) -> Option<Self> {
                match keyword {
                    $($kw => Some($name::$variant),)+
                    _ => None,
                }
            }

            /// The keyword, which is also the ShExJ name.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $($name::$variant => $kw,)+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

keywords! {
    /// A ShExJ `type` a step can test for. A term type is a keyword, not an IRI.
    ///
    /// ```text
    /// termType       ::= shapeExprType | tripleExprType | valueType
    ///                  | "Schema" | "SemAct" | "Annotation"
    /// shapeExprType  ::= "ShapeAnd" | "ShapeOr" | "ShapeNot"
    ///                  | "NodeConstraint" | "Shape" | "ShapeExternal"
    /// tripleExprType ::= "EachOf" | "OneOf" | "TripleConstraint"
    /// valueType      ::= "IriStem" | "IriStemRange"
    ///                  | "LiteralStem" | "LiteralStemRange"
    ///                  | "Language" | "LanguageStem" | "LanguageStemRange"
    ///                  | "Wildcard"
    /// ```
    TermType {
        ShapeAnd => "ShapeAnd",
        ShapeOr => "ShapeOr",
        ShapeNot => "ShapeNot",
        NodeConstraint => "NodeConstraint",
        Shape => "Shape",
        ShapeExternal => "ShapeExternal",
        EachOf => "EachOf",
        OneOf => "OneOf",
        TripleConstraint => "TripleConstraint",
        IriStem => "IriStem",
        IriStemRange => "IriStemRange",
        LiteralStem => "LiteralStem",
        LiteralStemRange => "LiteralStemRange",
        Language => "Language",
        LanguageStem => "LanguageStem",
        LanguageStemRange => "LanguageStemRange",
        Wildcard => "Wildcard",
        Schema => "Schema",
        SemAct => "SemAct",
        Annotation => "Annotation",
    }
}

keywords! {
    /// A ShExJ property a child step can name.
    ///
    /// ```text
    /// attribute            ::= "type" | "id" | "semActs" | "annotations" | "predicate"
    ///                        | schemaAttr | shapeExprAttr | tripleExprAttr
    ///                        | valueSetValueAttr | semActAttr | annotationAttr
    /// schemaAttr           ::= "@context" | "startActs" | "start" | "imports" | "shapes"
    /// shapeExprAttr        ::= "shapeExprs" | "shapeExpr" | nodeConstraintAttr | shapeAttr
    /// nodeConstraintAttr   ::= "nodeKind" | "datatype" | xsFacetAttr | "values"
    /// xsFacetAttr          ::= stringFacetAttr | numericFacetAttr
    /// stringFacetAttr      ::= "length" | "minlength" | "maxlength" | "pattern" | "flags"
    /// numericFacetAttr     ::= "mininclusive" | "minexclusive"
    ///                        | "maxinclusive" | "maxexclusive"
    ///                        | "totaldigits" | "fractiondigits"
    /// valueSetValueAttr    ::= "value" | "language" | "stem" | "exclusions" | "languageTag"
    /// shapeAttr            ::= "closed" | "extra" | "extends" | "expression"
    /// tripleExprAttr       ::= "expressions" | "min" | "max" | tripleConstraintAttr
    /// tripleConstraintAttr ::= "inverse" | "valueExpr"
    /// semActAttr           ::= "name" | "code"
    /// annotationAttr       ::= "object"
    /// ```
    Attribute {
        Type => "type",
        Id => "id",
        SemActs => "semActs",
        Annotations => "annotations",
        Predicate => "predicate",
        Context => "@context",
        StartActs => "startActs",
        Start => "start",
        Imports => "imports",
        Shapes => "shapes",
        ShapeExprs => "shapeExprs",
        ShapeExpr => "shapeExpr",
        NodeKind => "nodeKind",
        Datatype => "datatype",
        Length => "length",
        MinLength => "minlength",
        MaxLength => "maxlength",
        Pattern => "pattern",
        Flags => "flags",
        MinInclusive => "mininclusive",
        MinExclusive => "minexclusive",
        MaxInclusive => "maxinclusive",
        MaxExclusive => "maxexclusive",
        TotalDigits => "totaldigits",
        FractionDigits => "fractiondigits",
        Values => "values",
        Value => "value",
        Language => "language",
        Stem => "stem",
        Exclusions => "exclusions",
        LanguageTag => "languageTag",
        Closed => "closed",
        Extra => "extra",
        Extends => "extends",
        Expression => "expression",
        Expressions => "expressions",
        Min => "min",
        Max => "max",
        Inverse => "inverse",
        ValueExpr => "valueExpr",
        Name => "name",
        Code => "code",
        Object => "object",
    }
}
