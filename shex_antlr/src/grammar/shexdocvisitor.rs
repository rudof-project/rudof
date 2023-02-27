#![allow(nonstandard_style)]
// Generated from grammar/ShExDoc.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::shexdocparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ShExDocParser}.
 */
pub trait ShExDocVisitor<'input>: ParseTreeVisitor<'input,ShExDocParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shExDoc}.
	 * @param ctx the parse tree
	 */
	fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#directive}.
	 * @param ctx the parse tree
	 */
	fn visit_directive(&mut self, ctx: &DirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#baseDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_baseDecl(&mut self, ctx: &BaseDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#prefixDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_prefixDecl(&mut self, ctx: &PrefixDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#importDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_importDecl(&mut self, ctx: &ImportDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#notStartAction}.
	 * @param ctx the parse tree
	 */
	fn visit_notStartAction(&mut self, ctx: &NotStartActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#start}.
	 * @param ctx the parse tree
	 */
	fn visit_start(&mut self, ctx: &StartContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#startActions}.
	 * @param ctx the parse tree
	 */
	fn visit_startActions(&mut self, ctx: &StartActionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeExprDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeExprDecl(&mut self, ctx: &ShapeExprDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeExpression(&mut self, ctx: &ShapeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeExpression(&mut self, ctx: &InlineShapeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeOr}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeOr(&mut self, ctx: &ShapeOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeOr}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeOr(&mut self, ctx: &InlineShapeOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeAnd}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeAnd(&mut self, ctx: &ShapeAndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeAnd}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeAnd(&mut self, ctx: &InlineShapeAndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeNot}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeNot(&mut self, ctx: &ShapeNotContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeNot}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeNot(&mut self, ctx: &InlineShapeNotContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#negation}.
	 * @param ctx the parse tree
	 */
	fn visit_negation(&mut self, ctx: &NegationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shapeAtomNonLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeAtomNonLitNodeConstraint(&mut self, ctx: &ShapeAtomNonLitNodeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shapeAtomLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeAtomLitNodeConstraint(&mut self, ctx: &ShapeAtomLitNodeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shapeAtomShapeOrRef}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeAtomShapeOrRef(&mut self, ctx: &ShapeAtomShapeOrRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shapeAtomShapeExpression}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeAtomShapeExpression(&mut self, ctx: &ShapeAtomShapeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shapeAtomAny}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeAtomAny(&mut self, ctx: &ShapeAtomAnyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomNonLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeAtomNonLitNodeConstraint(&mut self, ctx: &InlineShapeAtomNonLitNodeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeAtomLitNodeConstraint(&mut self, ctx: &InlineShapeAtomLitNodeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomShapeOrRef}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeAtomShapeOrRef(&mut self, ctx: &InlineShapeAtomShapeOrRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomShapeExpression}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeAtomShapeExpression(&mut self, ctx: &InlineShapeAtomShapeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomAny}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeAtomAny(&mut self, ctx: &InlineShapeAtomAnyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeOrRef}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeOrRef(&mut self, ctx: &ShapeOrRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeOrRef}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeOrRef(&mut self, ctx: &InlineShapeOrRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeRef}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeRef(&mut self, ctx: &ShapeRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintLiteral}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_nodeConstraintLiteral(&mut self, ctx: &NodeConstraintLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintNonLiteral}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_nodeConstraintNonLiteral(&mut self, ctx: &NodeConstraintNonLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintDatatype}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_nodeConstraintDatatype(&mut self, ctx: &NodeConstraintDatatypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintValueSet}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_nodeConstraintValueSet(&mut self, ctx: &NodeConstraintValueSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintNumericFacet}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_nodeConstraintNumericFacet(&mut self, ctx: &NodeConstraintNumericFacetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#litNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_litNodeConstraint(&mut self, ctx: &LitNodeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code litNodeConstraintLiteral}
	 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_litNodeConstraintLiteral(&mut self, ctx: &LitNodeConstraintLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code litNodeConstraintStringFacet}
	 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_litNodeConstraintStringFacet(&mut self, ctx: &LitNodeConstraintStringFacetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#nonLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_nonLitNodeConstraint(&mut self, ctx: &NonLitNodeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#nonLiteralKind}.
	 * @param ctx the parse tree
	 */
	fn visit_nonLiteralKind(&mut self, ctx: &NonLiteralKindContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#xsFacet}.
	 * @param ctx the parse tree
	 */
	fn visit_xsFacet(&mut self, ctx: &XsFacetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#stringFacet}.
	 * @param ctx the parse tree
	 */
	fn visit_stringFacet(&mut self, ctx: &StringFacetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#stringLength}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLength(&mut self, ctx: &StringLengthContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericFacet}.
	 * @param ctx the parse tree
	 */
	fn visit_numericFacet(&mut self, ctx: &NumericFacetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericRange}.
	 * @param ctx the parse tree
	 */
	fn visit_numericRange(&mut self, ctx: &NumericRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericLength}.
	 * @param ctx the parse tree
	 */
	fn visit_numericLength(&mut self, ctx: &NumericLengthContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#rawNumeric}.
	 * @param ctx the parse tree
	 */
	fn visit_rawNumeric(&mut self, ctx: &RawNumericContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeDefinition(&mut self, ctx: &ShapeDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineShapeDefinition(&mut self, ctx: &InlineShapeDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#qualifier}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifier(&mut self, ctx: &QualifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#extraPropertySet}.
	 * @param ctx the parse tree
	 */
	fn visit_extraPropertySet(&mut self, ctx: &ExtraPropertySetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#tripleExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_tripleExpression(&mut self, ctx: &TripleExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#oneOfTripleExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_oneOfTripleExpr(&mut self, ctx: &OneOfTripleExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#multiElementOneOf}.
	 * @param ctx the parse tree
	 */
	fn visit_multiElementOneOf(&mut self, ctx: &MultiElementOneOfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#groupTripleExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_groupTripleExpr(&mut self, ctx: &GroupTripleExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#singleElementGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_singleElementGroup(&mut self, ctx: &SingleElementGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#multiElementGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_multiElementGroup(&mut self, ctx: &MultiElementGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#unaryTripleExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryTripleExpr(&mut self, ctx: &UnaryTripleExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#bracketedTripleExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_bracketedTripleExpr(&mut self, ctx: &BracketedTripleExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#tripleConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_tripleConstraint(&mut self, ctx: &TripleConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code starCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
	fn visit_starCardinality(&mut self, ctx: &StarCardinalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code plusCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
	fn visit_plusCardinality(&mut self, ctx: &PlusCardinalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code optionalCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
	fn visit_optionalCardinality(&mut self, ctx: &OptionalCardinalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code repeatCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
	fn visit_repeatCardinality(&mut self, ctx: &RepeatCardinalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code exactRange}
	 * labeled alternative in {@link ShExDocParser#repeatRange}.
	 * @param ctx the parse tree
	 */
	fn visit_exactRange(&mut self, ctx: &ExactRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code minMaxRange}
	 * labeled alternative in {@link ShExDocParser#repeatRange}.
	 * @param ctx the parse tree
	 */
	fn visit_minMaxRange(&mut self, ctx: &MinMaxRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#min_range}.
	 * @param ctx the parse tree
	 */
	fn visit_min_range(&mut self, ctx: &Min_rangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#max_range}.
	 * @param ctx the parse tree
	 */
	fn visit_max_range(&mut self, ctx: &Max_rangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_expr(&mut self, ctx: &ExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code equals}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_equals(&mut self, ctx: &EqualsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code notEquals}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_notEquals(&mut self, ctx: &NotEqualsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code gt}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_gt(&mut self, ctx: &GtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code lt}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_lt(&mut self, ctx: &LtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ge}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_ge(&mut self, ctx: &GeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code le}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_le(&mut self, ctx: &LeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code mult}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_mult(&mut self, ctx: &MultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code div}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_div(&mut self, ctx: &DivContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code add}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_add(&mut self, ctx: &AddContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code minus}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
	fn visit_minus(&mut self, ctx: &MinusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#basicExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_basicExpr(&mut self, ctx: &BasicExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#senseFlags}.
	 * @param ctx the parse tree
	 */
	fn visit_senseFlags(&mut self, ctx: &SenseFlagsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#valueSet}.
	 * @param ctx the parse tree
	 */
	fn visit_valueSet(&mut self, ctx: &ValueSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#valueSetValue}.
	 * @param ctx the parse tree
	 */
	fn visit_valueSetValue(&mut self, ctx: &ValueSetValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#iriRange}.
	 * @param ctx the parse tree
	 */
	fn visit_iriRange(&mut self, ctx: &IriRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#iriExclusion}.
	 * @param ctx the parse tree
	 */
	fn visit_iriExclusion(&mut self, ctx: &IriExclusionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#literalRange}.
	 * @param ctx the parse tree
	 */
	fn visit_literalRange(&mut self, ctx: &LiteralRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#literalExclusion}.
	 * @param ctx the parse tree
	 */
	fn visit_literalExclusion(&mut self, ctx: &LiteralExclusionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code languageRangeFull}
	 * labeled alternative in {@link ShExDocParser#languageRange}.
	 * @param ctx the parse tree
	 */
	fn visit_languageRangeFull(&mut self, ctx: &LanguageRangeFullContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code languageRangeAt}
	 * labeled alternative in {@link ShExDocParser#languageRange}.
	 * @param ctx the parse tree
	 */
	fn visit_languageRangeAt(&mut self, ctx: &LanguageRangeAtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#languageExclusion}.
	 * @param ctx the parse tree
	 */
	fn visit_languageExclusion(&mut self, ctx: &LanguageExclusionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#include}.
	 * @param ctx the parse tree
	 */
	fn visit_include(&mut self, ctx: &IncludeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#semanticAction}.
	 * @param ctx the parse tree
	 */
	fn visit_semanticAction(&mut self, ctx: &SemanticActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#rdfType}.
	 * @param ctx the parse tree
	 */
	fn visit_rdfType(&mut self, ctx: &RdfTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#datatype}.
	 * @param ctx the parse tree
	 */
	fn visit_datatype(&mut self, ctx: &DatatypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeExprLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeExprLabel(&mut self, ctx: &ShapeExprLabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#tripleExprLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_tripleExprLabel(&mut self, ctx: &TripleExprLabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#rdfLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_rdfLiteral(&mut self, ctx: &RdfLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#string}.
	 * @param ctx the parse tree
	 */
	fn visit_string(&mut self, ctx: &StringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#iri}.
	 * @param ctx the parse tree
	 */
	fn visit_iri(&mut self, ctx: &IriContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#prefixedName}.
	 * @param ctx the parse tree
	 */
	fn visit_prefixedName(&mut self, ctx: &PrefixedNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#blankNode}.
	 * @param ctx the parse tree
	 */
	fn visit_blankNode(&mut self, ctx: &BlankNodeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#extension}.
	 * @param ctx the parse tree
	 */
	fn visit_extension(&mut self, ctx: &ExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#restriction}.
	 * @param ctx the parse tree
	 */
	fn visit_restriction(&mut self, ctx: &RestrictionContext<'input>) { self.visit_children(ctx) }

}

pub trait ShExDocVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= ShExDocParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shExDoc}.
	 * @param ctx the parse tree
	 */
		fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#directive}.
	 * @param ctx the parse tree
	 */
		fn visit_directive(&mut self, ctx: &DirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#baseDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_baseDecl(&mut self, ctx: &BaseDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#prefixDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_prefixDecl(&mut self, ctx: &PrefixDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#importDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_importDecl(&mut self, ctx: &ImportDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#notStartAction}.
	 * @param ctx the parse tree
	 */
		fn visit_notStartAction(&mut self, ctx: &NotStartActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#start}.
	 * @param ctx the parse tree
	 */
		fn visit_start(&mut self, ctx: &StartContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#startActions}.
	 * @param ctx the parse tree
	 */
		fn visit_startActions(&mut self, ctx: &StartActionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeExprDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeExprDecl(&mut self, ctx: &ShapeExprDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeExpression(&mut self, ctx: &ShapeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeExpression(&mut self, ctx: &InlineShapeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeOr}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeOr(&mut self, ctx: &ShapeOrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeOr}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeOr(&mut self, ctx: &InlineShapeOrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeAnd}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeAnd(&mut self, ctx: &ShapeAndContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeAnd}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeAnd(&mut self, ctx: &InlineShapeAndContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeNot}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeNot(&mut self, ctx: &ShapeNotContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeNot}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeNot(&mut self, ctx: &InlineShapeNotContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#negation}.
	 * @param ctx the parse tree
	 */
		fn visit_negation(&mut self, ctx: &NegationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shapeAtomNonLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeAtomNonLitNodeConstraint(&mut self, ctx: &ShapeAtomNonLitNodeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shapeAtomLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeAtomLitNodeConstraint(&mut self, ctx: &ShapeAtomLitNodeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shapeAtomShapeOrRef}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeAtomShapeOrRef(&mut self, ctx: &ShapeAtomShapeOrRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shapeAtomShapeExpression}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeAtomShapeExpression(&mut self, ctx: &ShapeAtomShapeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shapeAtomAny}
	 * labeled alternative in {@link ShExDocParser#shapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeAtomAny(&mut self, ctx: &ShapeAtomAnyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomNonLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeAtomNonLitNodeConstraint(&mut self, ctx: &InlineShapeAtomNonLitNodeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomLitNodeConstraint}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeAtomLitNodeConstraint(&mut self, ctx: &InlineShapeAtomLitNodeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomShapeOrRef}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeAtomShapeOrRef(&mut self, ctx: &InlineShapeAtomShapeOrRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomShapeExpression}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeAtomShapeExpression(&mut self, ctx: &InlineShapeAtomShapeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineShapeAtomAny}
	 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeAtomAny(&mut self, ctx: &InlineShapeAtomAnyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeOrRef}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeOrRef(&mut self, ctx: &ShapeOrRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeOrRef}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeOrRef(&mut self, ctx: &InlineShapeOrRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeRef}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeRef(&mut self, ctx: &ShapeRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintLiteral}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_nodeConstraintLiteral(&mut self, ctx: &NodeConstraintLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintNonLiteral}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_nodeConstraintNonLiteral(&mut self, ctx: &NodeConstraintNonLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintDatatype}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_nodeConstraintDatatype(&mut self, ctx: &NodeConstraintDatatypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintValueSet}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_nodeConstraintValueSet(&mut self, ctx: &NodeConstraintValueSetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nodeConstraintNumericFacet}
	 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_nodeConstraintNumericFacet(&mut self, ctx: &NodeConstraintNumericFacetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#litNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_litNodeConstraint(&mut self, ctx: &LitNodeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code litNodeConstraintLiteral}
	 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_litNodeConstraintLiteral(&mut self, ctx: &LitNodeConstraintLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code litNodeConstraintStringFacet}
	 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_litNodeConstraintStringFacet(&mut self, ctx: &LitNodeConstraintStringFacetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#nonLitNodeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_nonLitNodeConstraint(&mut self, ctx: &NonLitNodeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#nonLiteralKind}.
	 * @param ctx the parse tree
	 */
		fn visit_nonLiteralKind(&mut self, ctx: &NonLiteralKindContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#xsFacet}.
	 * @param ctx the parse tree
	 */
		fn visit_xsFacet(&mut self, ctx: &XsFacetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#stringFacet}.
	 * @param ctx the parse tree
	 */
		fn visit_stringFacet(&mut self, ctx: &StringFacetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#stringLength}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLength(&mut self, ctx: &StringLengthContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericFacet}.
	 * @param ctx the parse tree
	 */
		fn visit_numericFacet(&mut self, ctx: &NumericFacetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericRange}.
	 * @param ctx the parse tree
	 */
		fn visit_numericRange(&mut self, ctx: &NumericRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericLength}.
	 * @param ctx the parse tree
	 */
		fn visit_numericLength(&mut self, ctx: &NumericLengthContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#rawNumeric}.
	 * @param ctx the parse tree
	 */
		fn visit_rawNumeric(&mut self, ctx: &RawNumericContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeDefinition(&mut self, ctx: &ShapeDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#inlineShapeDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineShapeDefinition(&mut self, ctx: &InlineShapeDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#qualifier}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifier(&mut self, ctx: &QualifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#extraPropertySet}.
	 * @param ctx the parse tree
	 */
		fn visit_extraPropertySet(&mut self, ctx: &ExtraPropertySetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#tripleExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_tripleExpression(&mut self, ctx: &TripleExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#oneOfTripleExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_oneOfTripleExpr(&mut self, ctx: &OneOfTripleExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#multiElementOneOf}.
	 * @param ctx the parse tree
	 */
		fn visit_multiElementOneOf(&mut self, ctx: &MultiElementOneOfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#groupTripleExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_groupTripleExpr(&mut self, ctx: &GroupTripleExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#singleElementGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_singleElementGroup(&mut self, ctx: &SingleElementGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#multiElementGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_multiElementGroup(&mut self, ctx: &MultiElementGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#unaryTripleExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryTripleExpr(&mut self, ctx: &UnaryTripleExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#bracketedTripleExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_bracketedTripleExpr(&mut self, ctx: &BracketedTripleExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#tripleConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_tripleConstraint(&mut self, ctx: &TripleConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code starCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
		fn visit_starCardinality(&mut self, ctx: &StarCardinalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code plusCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
		fn visit_plusCardinality(&mut self, ctx: &PlusCardinalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code optionalCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
		fn visit_optionalCardinality(&mut self, ctx: &OptionalCardinalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code repeatCardinality}
	 * labeled alternative in {@link ShExDocParser#cardinality}.
	 * @param ctx the parse tree
	 */
		fn visit_repeatCardinality(&mut self, ctx: &RepeatCardinalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code exactRange}
	 * labeled alternative in {@link ShExDocParser#repeatRange}.
	 * @param ctx the parse tree
	 */
		fn visit_exactRange(&mut self, ctx: &ExactRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code minMaxRange}
	 * labeled alternative in {@link ShExDocParser#repeatRange}.
	 * @param ctx the parse tree
	 */
		fn visit_minMaxRange(&mut self, ctx: &MinMaxRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#min_range}.
	 * @param ctx the parse tree
	 */
		fn visit_min_range(&mut self, ctx: &Min_rangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#max_range}.
	 * @param ctx the parse tree
	 */
		fn visit_max_range(&mut self, ctx: &Max_rangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code equals}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_equals(&mut self, ctx: &EqualsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code notEquals}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_notEquals(&mut self, ctx: &NotEqualsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code gt}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_gt(&mut self, ctx: &GtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code lt}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_lt(&mut self, ctx: &LtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ge}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_ge(&mut self, ctx: &GeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code le}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_le(&mut self, ctx: &LeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code mult}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_mult(&mut self, ctx: &MultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code div}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_div(&mut self, ctx: &DivContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code add}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_add(&mut self, ctx: &AddContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code minus}
	 * labeled alternative in {@link ShExDocParser#binOp}.
	 * @param ctx the parse tree
	 */
		fn visit_minus(&mut self, ctx: &MinusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#basicExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_basicExpr(&mut self, ctx: &BasicExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#senseFlags}.
	 * @param ctx the parse tree
	 */
		fn visit_senseFlags(&mut self, ctx: &SenseFlagsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#valueSet}.
	 * @param ctx the parse tree
	 */
		fn visit_valueSet(&mut self, ctx: &ValueSetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#valueSetValue}.
	 * @param ctx the parse tree
	 */
		fn visit_valueSetValue(&mut self, ctx: &ValueSetValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#iriRange}.
	 * @param ctx the parse tree
	 */
		fn visit_iriRange(&mut self, ctx: &IriRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#iriExclusion}.
	 * @param ctx the parse tree
	 */
		fn visit_iriExclusion(&mut self, ctx: &IriExclusionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#literalRange}.
	 * @param ctx the parse tree
	 */
		fn visit_literalRange(&mut self, ctx: &LiteralRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#literalExclusion}.
	 * @param ctx the parse tree
	 */
		fn visit_literalExclusion(&mut self, ctx: &LiteralExclusionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code languageRangeFull}
	 * labeled alternative in {@link ShExDocParser#languageRange}.
	 * @param ctx the parse tree
	 */
		fn visit_languageRangeFull(&mut self, ctx: &LanguageRangeFullContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code languageRangeAt}
	 * labeled alternative in {@link ShExDocParser#languageRange}.
	 * @param ctx the parse tree
	 */
		fn visit_languageRangeAt(&mut self, ctx: &LanguageRangeAtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#languageExclusion}.
	 * @param ctx the parse tree
	 */
		fn visit_languageExclusion(&mut self, ctx: &LanguageExclusionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#include}.
	 * @param ctx the parse tree
	 */
		fn visit_include(&mut self, ctx: &IncludeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#semanticAction}.
	 * @param ctx the parse tree
	 */
		fn visit_semanticAction(&mut self, ctx: &SemanticActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#predicate}.
	 * @param ctx the parse tree
	 */
		fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#rdfType}.
	 * @param ctx the parse tree
	 */
		fn visit_rdfType(&mut self, ctx: &RdfTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#datatype}.
	 * @param ctx the parse tree
	 */
		fn visit_datatype(&mut self, ctx: &DatatypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#shapeExprLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_shapeExprLabel(&mut self, ctx: &ShapeExprLabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#tripleExprLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_tripleExprLabel(&mut self, ctx: &TripleExprLabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#numericLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#rdfLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_rdfLiteral(&mut self, ctx: &RdfLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#string}.
	 * @param ctx the parse tree
	 */
		fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#iri}.
	 * @param ctx the parse tree
	 */
		fn visit_iri(&mut self, ctx: &IriContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#prefixedName}.
	 * @param ctx the parse tree
	 */
		fn visit_prefixedName(&mut self, ctx: &PrefixedNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#blankNode}.
	 * @param ctx the parse tree
	 */
		fn visit_blankNode(&mut self, ctx: &BlankNodeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#extension}.
	 * @param ctx the parse tree
	 */
		fn visit_extension(&mut self, ctx: &ExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ShExDocParser#restriction}.
	 * @param ctx the parse tree
	 */
		fn visit_restriction(&mut self, ctx: &RestrictionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> ShExDocVisitor<'input> for T
where
	T: ShExDocVisitorCompat<'input>
{
	fn visit_shExDoc(&mut self, ctx: &ShExDocContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shExDoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_directive(&mut self, ctx: &DirectiveContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_directive(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseDecl(&mut self, ctx: &BaseDeclContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_baseDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_prefixDecl(&mut self, ctx: &PrefixDeclContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_prefixDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importDecl(&mut self, ctx: &ImportDeclContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_importDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notStartAction(&mut self, ctx: &NotStartActionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_notStartAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_start(&mut self, ctx: &StartContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_start(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_startActions(&mut self, ctx: &StartActionsContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_startActions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeExprDecl(&mut self, ctx: &ShapeExprDeclContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeExprDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeExpression(&mut self, ctx: &ShapeExpressionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeExpression(&mut self, ctx: &InlineShapeExpressionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeOr(&mut self, ctx: &ShapeOrContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeOr(&mut self, ctx: &InlineShapeOrContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeAnd(&mut self, ctx: &ShapeAndContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeAnd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeAnd(&mut self, ctx: &InlineShapeAndContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeAnd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeNot(&mut self, ctx: &ShapeNotContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeNot(&mut self, ctx: &InlineShapeNotContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_negation(&mut self, ctx: &NegationContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_negation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeAtomNonLitNodeConstraint(&mut self, ctx: &ShapeAtomNonLitNodeConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeAtomNonLitNodeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeAtomLitNodeConstraint(&mut self, ctx: &ShapeAtomLitNodeConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeAtomLitNodeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeAtomShapeOrRef(&mut self, ctx: &ShapeAtomShapeOrRefContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeAtomShapeOrRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeAtomShapeExpression(&mut self, ctx: &ShapeAtomShapeExpressionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeAtomShapeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeAtomAny(&mut self, ctx: &ShapeAtomAnyContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeAtomAny(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeAtomNonLitNodeConstraint(&mut self, ctx: &InlineShapeAtomNonLitNodeConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeAtomNonLitNodeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeAtomLitNodeConstraint(&mut self, ctx: &InlineShapeAtomLitNodeConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeAtomLitNodeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeAtomShapeOrRef(&mut self, ctx: &InlineShapeAtomShapeOrRefContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeAtomShapeOrRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeAtomShapeExpression(&mut self, ctx: &InlineShapeAtomShapeExpressionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeAtomShapeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeAtomAny(&mut self, ctx: &InlineShapeAtomAnyContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeAtomAny(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeOrRef(&mut self, ctx: &ShapeOrRefContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeOrRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeOrRef(&mut self, ctx: &InlineShapeOrRefContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeOrRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeRef(&mut self, ctx: &ShapeRefContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nodeConstraintLiteral(&mut self, ctx: &NodeConstraintLiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nodeConstraintLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nodeConstraintNonLiteral(&mut self, ctx: &NodeConstraintNonLiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nodeConstraintNonLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nodeConstraintDatatype(&mut self, ctx: &NodeConstraintDatatypeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nodeConstraintDatatype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nodeConstraintValueSet(&mut self, ctx: &NodeConstraintValueSetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nodeConstraintValueSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nodeConstraintNumericFacet(&mut self, ctx: &NodeConstraintNumericFacetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nodeConstraintNumericFacet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_litNodeConstraint(&mut self, ctx: &LitNodeConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_litNodeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_litNodeConstraintLiteral(&mut self, ctx: &LitNodeConstraintLiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_litNodeConstraintLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_litNodeConstraintStringFacet(&mut self, ctx: &LitNodeConstraintStringFacetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_litNodeConstraintStringFacet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonLitNodeConstraint(&mut self, ctx: &NonLitNodeConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nonLitNodeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonLiteralKind(&mut self, ctx: &NonLiteralKindContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_nonLiteralKind(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_xsFacet(&mut self, ctx: &XsFacetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_xsFacet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringFacet(&mut self, ctx: &StringFacetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_stringFacet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLength(&mut self, ctx: &StringLengthContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_stringLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericFacet(&mut self, ctx: &NumericFacetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_numericFacet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericRange(&mut self, ctx: &NumericRangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_numericRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericLength(&mut self, ctx: &NumericLengthContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_numericLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rawNumeric(&mut self, ctx: &RawNumericContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_rawNumeric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeDefinition(&mut self, ctx: &ShapeDefinitionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineShapeDefinition(&mut self, ctx: &InlineShapeDefinitionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_inlineShapeDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifier(&mut self, ctx: &QualifierContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_qualifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_extraPropertySet(&mut self, ctx: &ExtraPropertySetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_extraPropertySet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tripleExpression(&mut self, ctx: &TripleExpressionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_tripleExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_oneOfTripleExpr(&mut self, ctx: &OneOfTripleExprContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_oneOfTripleExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiElementOneOf(&mut self, ctx: &MultiElementOneOfContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_multiElementOneOf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupTripleExpr(&mut self, ctx: &GroupTripleExprContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_groupTripleExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleElementGroup(&mut self, ctx: &SingleElementGroupContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_singleElementGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiElementGroup(&mut self, ctx: &MultiElementGroupContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_multiElementGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryTripleExpr(&mut self, ctx: &UnaryTripleExprContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_unaryTripleExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bracketedTripleExpr(&mut self, ctx: &BracketedTripleExprContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_bracketedTripleExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tripleConstraint(&mut self, ctx: &TripleConstraintContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_tripleConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_starCardinality(&mut self, ctx: &StarCardinalityContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_starCardinality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_plusCardinality(&mut self, ctx: &PlusCardinalityContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_plusCardinality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_optionalCardinality(&mut self, ctx: &OptionalCardinalityContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_optionalCardinality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_repeatCardinality(&mut self, ctx: &RepeatCardinalityContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_repeatCardinality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exactRange(&mut self, ctx: &ExactRangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_exactRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_minMaxRange(&mut self, ctx: &MinMaxRangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_minMaxRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_min_range(&mut self, ctx: &Min_rangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_min_range(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_max_range(&mut self, ctx: &Max_rangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_max_range(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expr(&mut self, ctx: &ExprContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equals(&mut self, ctx: &EqualsContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_equals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notEquals(&mut self, ctx: &NotEqualsContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_notEquals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gt(&mut self, ctx: &GtContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_gt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lt(&mut self, ctx: &LtContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_lt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ge(&mut self, ctx: &GeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_ge(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_le(&mut self, ctx: &LeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_le(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mult(&mut self, ctx: &MultContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_mult(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_div(&mut self, ctx: &DivContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_div(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_add(&mut self, ctx: &AddContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_add(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_minus(&mut self, ctx: &MinusContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_minus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_basicExpr(&mut self, ctx: &BasicExprContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_basicExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_senseFlags(&mut self, ctx: &SenseFlagsContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_senseFlags(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueSet(&mut self, ctx: &ValueSetContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_valueSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueSetValue(&mut self, ctx: &ValueSetValueContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_valueSetValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iriRange(&mut self, ctx: &IriRangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_iriRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iriExclusion(&mut self, ctx: &IriExclusionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_iriExclusion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literalRange(&mut self, ctx: &LiteralRangeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_literalRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literalExclusion(&mut self, ctx: &LiteralExclusionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_literalExclusion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_languageRangeFull(&mut self, ctx: &LanguageRangeFullContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_languageRangeFull(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_languageRangeAt(&mut self, ctx: &LanguageRangeAtContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_languageRangeAt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_languageExclusion(&mut self, ctx: &LanguageExclusionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_languageExclusion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_include(&mut self, ctx: &IncludeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_include(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_semanticAction(&mut self, ctx: &SemanticActionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_semanticAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rdfType(&mut self, ctx: &RdfTypeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_rdfType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_datatype(&mut self, ctx: &DatatypeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_datatype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shapeExprLabel(&mut self, ctx: &ShapeExprLabelContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_shapeExprLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tripleExprLabel(&mut self, ctx: &TripleExprLabelContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_tripleExprLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_numericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rdfLiteral(&mut self, ctx: &RdfLiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_rdfLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_booleanLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_string(&mut self, ctx: &StringContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iri(&mut self, ctx: &IriContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_iri(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_prefixedName(&mut self, ctx: &PrefixedNameContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_prefixedName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blankNode(&mut self, ctx: &BlankNodeContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_blankNode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_extension(&mut self, ctx: &ExtensionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_extension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_restriction(&mut self, ctx: &RestrictionContext<'input>){
		let result = <Self as ShExDocVisitorCompat>::visit_restriction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}