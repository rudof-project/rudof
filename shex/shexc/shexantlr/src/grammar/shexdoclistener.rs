#![allow(nonstandard_style)]
// Generated from grammar/ShExDoc.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::shexdocparser::*;

pub trait ShExDocListener<'input> : ParseTreeListener<'input,ShExDocParserContextType>{
/**
 * Enter a parse tree produced by {@link ShExDocParser#shExDoc}.
 * @param ctx the parse tree
 */
fn enter_shExDoc(&mut self, _ctx: &ShExDocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shExDoc}.
 * @param ctx the parse tree
 */
fn exit_shExDoc(&mut self, _ctx: &ShExDocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#directive}.
 * @param ctx the parse tree
 */
fn enter_directive(&mut self, _ctx: &DirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#directive}.
 * @param ctx the parse tree
 */
fn exit_directive(&mut self, _ctx: &DirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#baseDecl}.
 * @param ctx the parse tree
 */
fn enter_baseDecl(&mut self, _ctx: &BaseDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#baseDecl}.
 * @param ctx the parse tree
 */
fn exit_baseDecl(&mut self, _ctx: &BaseDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#prefixDecl}.
 * @param ctx the parse tree
 */
fn enter_prefixDecl(&mut self, _ctx: &PrefixDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#prefixDecl}.
 * @param ctx the parse tree
 */
fn exit_prefixDecl(&mut self, _ctx: &PrefixDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#importDecl}.
 * @param ctx the parse tree
 */
fn enter_importDecl(&mut self, _ctx: &ImportDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#importDecl}.
 * @param ctx the parse tree
 */
fn exit_importDecl(&mut self, _ctx: &ImportDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#notStartAction}.
 * @param ctx the parse tree
 */
fn enter_notStartAction(&mut self, _ctx: &NotStartActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#notStartAction}.
 * @param ctx the parse tree
 */
fn exit_notStartAction(&mut self, _ctx: &NotStartActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#start}.
 * @param ctx the parse tree
 */
fn enter_start(&mut self, _ctx: &StartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#start}.
 * @param ctx the parse tree
 */
fn exit_start(&mut self, _ctx: &StartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#startActions}.
 * @param ctx the parse tree
 */
fn enter_startActions(&mut self, _ctx: &StartActionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#startActions}.
 * @param ctx the parse tree
 */
fn exit_startActions(&mut self, _ctx: &StartActionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeExprDecl}.
 * @param ctx the parse tree
 */
fn enter_shapeExprDecl(&mut self, _ctx: &ShapeExprDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeExprDecl}.
 * @param ctx the parse tree
 */
fn exit_shapeExprDecl(&mut self, _ctx: &ShapeExprDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeExpression}.
 * @param ctx the parse tree
 */
fn enter_shapeExpression(&mut self, _ctx: &ShapeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeExpression}.
 * @param ctx the parse tree
 */
fn exit_shapeExpression(&mut self, _ctx: &ShapeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#inlineShapeExpression}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeExpression(&mut self, _ctx: &InlineShapeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#inlineShapeExpression}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeExpression(&mut self, _ctx: &InlineShapeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeOr}.
 * @param ctx the parse tree
 */
fn enter_shapeOr(&mut self, _ctx: &ShapeOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeOr}.
 * @param ctx the parse tree
 */
fn exit_shapeOr(&mut self, _ctx: &ShapeOrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#inlineShapeOr}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeOr(&mut self, _ctx: &InlineShapeOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#inlineShapeOr}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeOr(&mut self, _ctx: &InlineShapeOrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeAnd}.
 * @param ctx the parse tree
 */
fn enter_shapeAnd(&mut self, _ctx: &ShapeAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeAnd}.
 * @param ctx the parse tree
 */
fn exit_shapeAnd(&mut self, _ctx: &ShapeAndContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#inlineShapeAnd}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeAnd(&mut self, _ctx: &InlineShapeAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#inlineShapeAnd}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeAnd(&mut self, _ctx: &InlineShapeAndContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeNot}.
 * @param ctx the parse tree
 */
fn enter_shapeNot(&mut self, _ctx: &ShapeNotContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeNot}.
 * @param ctx the parse tree
 */
fn exit_shapeNot(&mut self, _ctx: &ShapeNotContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#inlineShapeNot}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeNot(&mut self, _ctx: &InlineShapeNotContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#inlineShapeNot}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeNot(&mut self, _ctx: &InlineShapeNotContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#negation}.
 * @param ctx the parse tree
 */
fn enter_negation(&mut self, _ctx: &NegationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#negation}.
 * @param ctx the parse tree
 */
fn exit_negation(&mut self, _ctx: &NegationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shapeAtomNonLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn enter_shapeAtomNonLitNodeConstraint(&mut self, _ctx: &ShapeAtomNonLitNodeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shapeAtomNonLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn exit_shapeAtomNonLitNodeConstraint(&mut self, _ctx: &ShapeAtomNonLitNodeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shapeAtomLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn enter_shapeAtomLitNodeConstraint(&mut self, _ctx: &ShapeAtomLitNodeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shapeAtomLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn exit_shapeAtomLitNodeConstraint(&mut self, _ctx: &ShapeAtomLitNodeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shapeAtomShapeOrRef}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn enter_shapeAtomShapeOrRef(&mut self, _ctx: &ShapeAtomShapeOrRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shapeAtomShapeOrRef}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn exit_shapeAtomShapeOrRef(&mut self, _ctx: &ShapeAtomShapeOrRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shapeAtomShapeExpression}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn enter_shapeAtomShapeExpression(&mut self, _ctx: &ShapeAtomShapeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shapeAtomShapeExpression}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn exit_shapeAtomShapeExpression(&mut self, _ctx: &ShapeAtomShapeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shapeAtomAny}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn enter_shapeAtomAny(&mut self, _ctx: &ShapeAtomAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shapeAtomAny}
 * labeled alternative in {@link ShExDocParser#shapeAtom}.
 * @param ctx the parse tree
 */
fn exit_shapeAtomAny(&mut self, _ctx: &ShapeAtomAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineShapeAtomNonLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeAtomNonLitNodeConstraint(&mut self, _ctx: &InlineShapeAtomNonLitNodeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineShapeAtomNonLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeAtomNonLitNodeConstraint(&mut self, _ctx: &InlineShapeAtomNonLitNodeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineShapeAtomLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeAtomLitNodeConstraint(&mut self, _ctx: &InlineShapeAtomLitNodeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineShapeAtomLitNodeConstraint}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeAtomLitNodeConstraint(&mut self, _ctx: &InlineShapeAtomLitNodeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineShapeAtomShapeOrRef}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeAtomShapeOrRef(&mut self, _ctx: &InlineShapeAtomShapeOrRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineShapeAtomShapeOrRef}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeAtomShapeOrRef(&mut self, _ctx: &InlineShapeAtomShapeOrRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineShapeAtomShapeExpression}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeAtomShapeExpression(&mut self, _ctx: &InlineShapeAtomShapeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineShapeAtomShapeExpression}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeAtomShapeExpression(&mut self, _ctx: &InlineShapeAtomShapeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineShapeAtomAny}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeAtomAny(&mut self, _ctx: &InlineShapeAtomAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineShapeAtomAny}
 * labeled alternative in {@link ShExDocParser#inlineShapeAtom}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeAtomAny(&mut self, _ctx: &InlineShapeAtomAnyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeOrRef}.
 * @param ctx the parse tree
 */
fn enter_shapeOrRef(&mut self, _ctx: &ShapeOrRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeOrRef}.
 * @param ctx the parse tree
 */
fn exit_shapeOrRef(&mut self, _ctx: &ShapeOrRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#inlineShapeOrRef}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeOrRef(&mut self, _ctx: &InlineShapeOrRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#inlineShapeOrRef}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeOrRef(&mut self, _ctx: &InlineShapeOrRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeRef}.
 * @param ctx the parse tree
 */
fn enter_shapeRef(&mut self, _ctx: &ShapeRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeRef}.
 * @param ctx the parse tree
 */
fn exit_shapeRef(&mut self, _ctx: &ShapeRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nodeConstraintLiteral}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_nodeConstraintLiteral(&mut self, _ctx: &NodeConstraintLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nodeConstraintLiteral}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_nodeConstraintLiteral(&mut self, _ctx: &NodeConstraintLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nodeConstraintNonLiteral}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_nodeConstraintNonLiteral(&mut self, _ctx: &NodeConstraintNonLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nodeConstraintNonLiteral}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_nodeConstraintNonLiteral(&mut self, _ctx: &NodeConstraintNonLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nodeConstraintDatatype}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_nodeConstraintDatatype(&mut self, _ctx: &NodeConstraintDatatypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nodeConstraintDatatype}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_nodeConstraintDatatype(&mut self, _ctx: &NodeConstraintDatatypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nodeConstraintValueSet}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_nodeConstraintValueSet(&mut self, _ctx: &NodeConstraintValueSetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nodeConstraintValueSet}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_nodeConstraintValueSet(&mut self, _ctx: &NodeConstraintValueSetContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nodeConstraintNumericFacet}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_nodeConstraintNumericFacet(&mut self, _ctx: &NodeConstraintNumericFacetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nodeConstraintNumericFacet}
 * labeled alternative in {@link ShExDocParser#inlineLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_nodeConstraintNumericFacet(&mut self, _ctx: &NodeConstraintNumericFacetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#litNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_litNodeConstraint(&mut self, _ctx: &LitNodeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#litNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_litNodeConstraint(&mut self, _ctx: &LitNodeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code litNodeConstraintLiteral}
 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_litNodeConstraintLiteral(&mut self, _ctx: &LitNodeConstraintLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code litNodeConstraintLiteral}
 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_litNodeConstraintLiteral(&mut self, _ctx: &LitNodeConstraintLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code litNodeConstraintStringFacet}
 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_litNodeConstraintStringFacet(&mut self, _ctx: &LitNodeConstraintStringFacetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code litNodeConstraintStringFacet}
 * labeled alternative in {@link ShExDocParser#inlineNonLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_litNodeConstraintStringFacet(&mut self, _ctx: &LitNodeConstraintStringFacetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#nonLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn enter_nonLitNodeConstraint(&mut self, _ctx: &NonLitNodeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#nonLitNodeConstraint}.
 * @param ctx the parse tree
 */
fn exit_nonLitNodeConstraint(&mut self, _ctx: &NonLitNodeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#nonLiteralKind}.
 * @param ctx the parse tree
 */
fn enter_nonLiteralKind(&mut self, _ctx: &NonLiteralKindContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#nonLiteralKind}.
 * @param ctx the parse tree
 */
fn exit_nonLiteralKind(&mut self, _ctx: &NonLiteralKindContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#xsFacet}.
 * @param ctx the parse tree
 */
fn enter_xsFacet(&mut self, _ctx: &XsFacetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#xsFacet}.
 * @param ctx the parse tree
 */
fn exit_xsFacet(&mut self, _ctx: &XsFacetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#stringFacet}.
 * @param ctx the parse tree
 */
fn enter_stringFacet(&mut self, _ctx: &StringFacetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#stringFacet}.
 * @param ctx the parse tree
 */
fn exit_stringFacet(&mut self, _ctx: &StringFacetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#stringLength}.
 * @param ctx the parse tree
 */
fn enter_stringLength(&mut self, _ctx: &StringLengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#stringLength}.
 * @param ctx the parse tree
 */
fn exit_stringLength(&mut self, _ctx: &StringLengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#numericFacet}.
 * @param ctx the parse tree
 */
fn enter_numericFacet(&mut self, _ctx: &NumericFacetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#numericFacet}.
 * @param ctx the parse tree
 */
fn exit_numericFacet(&mut self, _ctx: &NumericFacetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#numericRange}.
 * @param ctx the parse tree
 */
fn enter_numericRange(&mut self, _ctx: &NumericRangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#numericRange}.
 * @param ctx the parse tree
 */
fn exit_numericRange(&mut self, _ctx: &NumericRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#numericLength}.
 * @param ctx the parse tree
 */
fn enter_numericLength(&mut self, _ctx: &NumericLengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#numericLength}.
 * @param ctx the parse tree
 */
fn exit_numericLength(&mut self, _ctx: &NumericLengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#rawNumeric}.
 * @param ctx the parse tree
 */
fn enter_rawNumeric(&mut self, _ctx: &RawNumericContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#rawNumeric}.
 * @param ctx the parse tree
 */
fn exit_rawNumeric(&mut self, _ctx: &RawNumericContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeDefinition}.
 * @param ctx the parse tree
 */
fn enter_shapeDefinition(&mut self, _ctx: &ShapeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeDefinition}.
 * @param ctx the parse tree
 */
fn exit_shapeDefinition(&mut self, _ctx: &ShapeDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#inlineShapeDefinition}.
 * @param ctx the parse tree
 */
fn enter_inlineShapeDefinition(&mut self, _ctx: &InlineShapeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#inlineShapeDefinition}.
 * @param ctx the parse tree
 */
fn exit_inlineShapeDefinition(&mut self, _ctx: &InlineShapeDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#qualifier}.
 * @param ctx the parse tree
 */
fn enter_qualifier(&mut self, _ctx: &QualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#qualifier}.
 * @param ctx the parse tree
 */
fn exit_qualifier(&mut self, _ctx: &QualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#extraPropertySet}.
 * @param ctx the parse tree
 */
fn enter_extraPropertySet(&mut self, _ctx: &ExtraPropertySetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#extraPropertySet}.
 * @param ctx the parse tree
 */
fn exit_extraPropertySet(&mut self, _ctx: &ExtraPropertySetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#tripleExpression}.
 * @param ctx the parse tree
 */
fn enter_tripleExpression(&mut self, _ctx: &TripleExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#tripleExpression}.
 * @param ctx the parse tree
 */
fn exit_tripleExpression(&mut self, _ctx: &TripleExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#oneOfTripleExpr}.
 * @param ctx the parse tree
 */
fn enter_oneOfTripleExpr(&mut self, _ctx: &OneOfTripleExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#oneOfTripleExpr}.
 * @param ctx the parse tree
 */
fn exit_oneOfTripleExpr(&mut self, _ctx: &OneOfTripleExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#multiElementOneOf}.
 * @param ctx the parse tree
 */
fn enter_multiElementOneOf(&mut self, _ctx: &MultiElementOneOfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#multiElementOneOf}.
 * @param ctx the parse tree
 */
fn exit_multiElementOneOf(&mut self, _ctx: &MultiElementOneOfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#groupTripleExpr}.
 * @param ctx the parse tree
 */
fn enter_groupTripleExpr(&mut self, _ctx: &GroupTripleExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#groupTripleExpr}.
 * @param ctx the parse tree
 */
fn exit_groupTripleExpr(&mut self, _ctx: &GroupTripleExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#singleElementGroup}.
 * @param ctx the parse tree
 */
fn enter_singleElementGroup(&mut self, _ctx: &SingleElementGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#singleElementGroup}.
 * @param ctx the parse tree
 */
fn exit_singleElementGroup(&mut self, _ctx: &SingleElementGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#multiElementGroup}.
 * @param ctx the parse tree
 */
fn enter_multiElementGroup(&mut self, _ctx: &MultiElementGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#multiElementGroup}.
 * @param ctx the parse tree
 */
fn exit_multiElementGroup(&mut self, _ctx: &MultiElementGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#unaryTripleExpr}.
 * @param ctx the parse tree
 */
fn enter_unaryTripleExpr(&mut self, _ctx: &UnaryTripleExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#unaryTripleExpr}.
 * @param ctx the parse tree
 */
fn exit_unaryTripleExpr(&mut self, _ctx: &UnaryTripleExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#bracketedTripleExpr}.
 * @param ctx the parse tree
 */
fn enter_bracketedTripleExpr(&mut self, _ctx: &BracketedTripleExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#bracketedTripleExpr}.
 * @param ctx the parse tree
 */
fn exit_bracketedTripleExpr(&mut self, _ctx: &BracketedTripleExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#tripleConstraint}.
 * @param ctx the parse tree
 */
fn enter_tripleConstraint(&mut self, _ctx: &TripleConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#tripleConstraint}.
 * @param ctx the parse tree
 */
fn exit_tripleConstraint(&mut self, _ctx: &TripleConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code starCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn enter_starCardinality(&mut self, _ctx: &StarCardinalityContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code starCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn exit_starCardinality(&mut self, _ctx: &StarCardinalityContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code plusCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn enter_plusCardinality(&mut self, _ctx: &PlusCardinalityContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code plusCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn exit_plusCardinality(&mut self, _ctx: &PlusCardinalityContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code optionalCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn enter_optionalCardinality(&mut self, _ctx: &OptionalCardinalityContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code optionalCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn exit_optionalCardinality(&mut self, _ctx: &OptionalCardinalityContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code repeatCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn enter_repeatCardinality(&mut self, _ctx: &RepeatCardinalityContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code repeatCardinality}
 * labeled alternative in {@link ShExDocParser#cardinality}.
 * @param ctx the parse tree
 */
fn exit_repeatCardinality(&mut self, _ctx: &RepeatCardinalityContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code exactRange}
 * labeled alternative in {@link ShExDocParser#repeatRange}.
 * @param ctx the parse tree
 */
fn enter_exactRange(&mut self, _ctx: &ExactRangeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code exactRange}
 * labeled alternative in {@link ShExDocParser#repeatRange}.
 * @param ctx the parse tree
 */
fn exit_exactRange(&mut self, _ctx: &ExactRangeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code minMaxRange}
 * labeled alternative in {@link ShExDocParser#repeatRange}.
 * @param ctx the parse tree
 */
fn enter_minMaxRange(&mut self, _ctx: &MinMaxRangeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code minMaxRange}
 * labeled alternative in {@link ShExDocParser#repeatRange}.
 * @param ctx the parse tree
 */
fn exit_minMaxRange(&mut self, _ctx: &MinMaxRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#min_range}.
 * @param ctx the parse tree
 */
fn enter_min_range(&mut self, _ctx: &Min_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#min_range}.
 * @param ctx the parse tree
 */
fn exit_min_range(&mut self, _ctx: &Min_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#max_range}.
 * @param ctx the parse tree
 */
fn enter_max_range(&mut self, _ctx: &Max_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#max_range}.
 * @param ctx the parse tree
 */
fn exit_max_range(&mut self, _ctx: &Max_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code equals}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_equals(&mut self, _ctx: &EqualsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code equals}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_equals(&mut self, _ctx: &EqualsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code notEquals}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_notEquals(&mut self, _ctx: &NotEqualsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code notEquals}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_notEquals(&mut self, _ctx: &NotEqualsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code gt}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_gt(&mut self, _ctx: &GtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code gt}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_gt(&mut self, _ctx: &GtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code lt}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_lt(&mut self, _ctx: &LtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code lt}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_lt(&mut self, _ctx: &LtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ge}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_ge(&mut self, _ctx: &GeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ge}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_ge(&mut self, _ctx: &GeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code le}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_le(&mut self, _ctx: &LeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code le}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_le(&mut self, _ctx: &LeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code mult}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_mult(&mut self, _ctx: &MultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code mult}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_mult(&mut self, _ctx: &MultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code div}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_div(&mut self, _ctx: &DivContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code div}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_div(&mut self, _ctx: &DivContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code add}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code add}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code minus}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn enter_minus(&mut self, _ctx: &MinusContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code minus}
 * labeled alternative in {@link ShExDocParser#binOp}.
 * @param ctx the parse tree
 */
fn exit_minus(&mut self, _ctx: &MinusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#basicExpr}.
 * @param ctx the parse tree
 */
fn enter_basicExpr(&mut self, _ctx: &BasicExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#basicExpr}.
 * @param ctx the parse tree
 */
fn exit_basicExpr(&mut self, _ctx: &BasicExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#senseFlags}.
 * @param ctx the parse tree
 */
fn enter_senseFlags(&mut self, _ctx: &SenseFlagsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#senseFlags}.
 * @param ctx the parse tree
 */
fn exit_senseFlags(&mut self, _ctx: &SenseFlagsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#valueSet}.
 * @param ctx the parse tree
 */
fn enter_valueSet(&mut self, _ctx: &ValueSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#valueSet}.
 * @param ctx the parse tree
 */
fn exit_valueSet(&mut self, _ctx: &ValueSetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#valueSetValue}.
 * @param ctx the parse tree
 */
fn enter_valueSetValue(&mut self, _ctx: &ValueSetValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#valueSetValue}.
 * @param ctx the parse tree
 */
fn exit_valueSetValue(&mut self, _ctx: &ValueSetValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#iriRange}.
 * @param ctx the parse tree
 */
fn enter_iriRange(&mut self, _ctx: &IriRangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#iriRange}.
 * @param ctx the parse tree
 */
fn exit_iriRange(&mut self, _ctx: &IriRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#iriExclusion}.
 * @param ctx the parse tree
 */
fn enter_iriExclusion(&mut self, _ctx: &IriExclusionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#iriExclusion}.
 * @param ctx the parse tree
 */
fn exit_iriExclusion(&mut self, _ctx: &IriExclusionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#literalRange}.
 * @param ctx the parse tree
 */
fn enter_literalRange(&mut self, _ctx: &LiteralRangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#literalRange}.
 * @param ctx the parse tree
 */
fn exit_literalRange(&mut self, _ctx: &LiteralRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#literalExclusion}.
 * @param ctx the parse tree
 */
fn enter_literalExclusion(&mut self, _ctx: &LiteralExclusionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#literalExclusion}.
 * @param ctx the parse tree
 */
fn exit_literalExclusion(&mut self, _ctx: &LiteralExclusionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code languageRangeFull}
 * labeled alternative in {@link ShExDocParser#languageRange}.
 * @param ctx the parse tree
 */
fn enter_languageRangeFull(&mut self, _ctx: &LanguageRangeFullContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code languageRangeFull}
 * labeled alternative in {@link ShExDocParser#languageRange}.
 * @param ctx the parse tree
 */
fn exit_languageRangeFull(&mut self, _ctx: &LanguageRangeFullContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code languageRangeAt}
 * labeled alternative in {@link ShExDocParser#languageRange}.
 * @param ctx the parse tree
 */
fn enter_languageRangeAt(&mut self, _ctx: &LanguageRangeAtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code languageRangeAt}
 * labeled alternative in {@link ShExDocParser#languageRange}.
 * @param ctx the parse tree
 */
fn exit_languageRangeAt(&mut self, _ctx: &LanguageRangeAtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#languageExclusion}.
 * @param ctx the parse tree
 */
fn enter_languageExclusion(&mut self, _ctx: &LanguageExclusionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#languageExclusion}.
 * @param ctx the parse tree
 */
fn exit_languageExclusion(&mut self, _ctx: &LanguageExclusionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#include}.
 * @param ctx the parse tree
 */
fn enter_include(&mut self, _ctx: &IncludeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#include}.
 * @param ctx the parse tree
 */
fn exit_include(&mut self, _ctx: &IncludeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#annotation}.
 * @param ctx the parse tree
 */
fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#annotation}.
 * @param ctx the parse tree
 */
fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#semanticAction}.
 * @param ctx the parse tree
 */
fn enter_semanticAction(&mut self, _ctx: &SemanticActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#semanticAction}.
 * @param ctx the parse tree
 */
fn exit_semanticAction(&mut self, _ctx: &SemanticActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#rdfType}.
 * @param ctx the parse tree
 */
fn enter_rdfType(&mut self, _ctx: &RdfTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#rdfType}.
 * @param ctx the parse tree
 */
fn exit_rdfType(&mut self, _ctx: &RdfTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#datatype}.
 * @param ctx the parse tree
 */
fn enter_datatype(&mut self, _ctx: &DatatypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#datatype}.
 * @param ctx the parse tree
 */
fn exit_datatype(&mut self, _ctx: &DatatypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#shapeExprLabel}.
 * @param ctx the parse tree
 */
fn enter_shapeExprLabel(&mut self, _ctx: &ShapeExprLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#shapeExprLabel}.
 * @param ctx the parse tree
 */
fn exit_shapeExprLabel(&mut self, _ctx: &ShapeExprLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#tripleExprLabel}.
 * @param ctx the parse tree
 */
fn enter_tripleExprLabel(&mut self, _ctx: &TripleExprLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#tripleExprLabel}.
 * @param ctx the parse tree
 */
fn exit_tripleExprLabel(&mut self, _ctx: &TripleExprLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#numericLiteral}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#numericLiteral}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#rdfLiteral}.
 * @param ctx the parse tree
 */
fn enter_rdfLiteral(&mut self, _ctx: &RdfLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#rdfLiteral}.
 * @param ctx the parse tree
 */
fn exit_rdfLiteral(&mut self, _ctx: &RdfLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#string}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#string}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#iri}.
 * @param ctx the parse tree
 */
fn enter_iri(&mut self, _ctx: &IriContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#iri}.
 * @param ctx the parse tree
 */
fn exit_iri(&mut self, _ctx: &IriContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#prefixedName}.
 * @param ctx the parse tree
 */
fn enter_prefixedName(&mut self, _ctx: &PrefixedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#prefixedName}.
 * @param ctx the parse tree
 */
fn exit_prefixedName(&mut self, _ctx: &PrefixedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#blankNode}.
 * @param ctx the parse tree
 */
fn enter_blankNode(&mut self, _ctx: &BlankNodeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#blankNode}.
 * @param ctx the parse tree
 */
fn exit_blankNode(&mut self, _ctx: &BlankNodeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#extension}.
 * @param ctx the parse tree
 */
fn enter_extension(&mut self, _ctx: &ExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#extension}.
 * @param ctx the parse tree
 */
fn exit_extension(&mut self, _ctx: &ExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ShExDocParser#restriction}.
 * @param ctx the parse tree
 */
fn enter_restriction(&mut self, _ctx: &RestrictionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ShExDocParser#restriction}.
 * @param ctx the parse tree
 */
fn exit_restriction(&mut self, _ctx: &RestrictionContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : ShExDocListener<'input> }


