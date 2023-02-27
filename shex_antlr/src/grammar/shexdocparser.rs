// Generated from grammar/ShExDoc.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::shexdoclistener::*;
use super::shexdocvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const KW_ABSTRACT:isize=29; 
		pub const KW_AS:isize=30; 
		pub const KW_BASE:isize=31; 
		pub const KW_EXTENDS:isize=32; 
		pub const KW_IMPORT:isize=33; 
		pub const KW_RESTRICTS:isize=34; 
		pub const KW_EXTERNAL:isize=35; 
		pub const KW_PREFIX:isize=36; 
		pub const KW_START:isize=37; 
		pub const KW_VIRTUAL:isize=38; 
		pub const KW_CLOSED:isize=39; 
		pub const KW_EXTRA:isize=40; 
		pub const KW_LITERAL:isize=41; 
		pub const KW_IRI:isize=42; 
		pub const KW_NONLITERAL:isize=43; 
		pub const KW_BNODE:isize=44; 
		pub const KW_AND:isize=45; 
		pub const KW_OR:isize=46; 
		pub const KW_MININCLUSIVE:isize=47; 
		pub const KW_MINEXCLUSIVE:isize=48; 
		pub const KW_MAXINCLUSIVE:isize=49; 
		pub const KW_MAXEXCLUSIVE:isize=50; 
		pub const KW_LENGTH:isize=51; 
		pub const KW_MINLENGTH:isize=52; 
		pub const KW_MAXLENGTH:isize=53; 
		pub const KW_TOTALDIGITS:isize=54; 
		pub const KW_FRACTIONDIGITS:isize=55; 
		pub const KW_NOT:isize=56; 
		pub const KW_TRUE:isize=57; 
		pub const KW_FALSE:isize=58; 
		pub const SKIP_:isize=59; 
		pub const CODE:isize=60; 
		pub const RDF_TYPE:isize=61; 
		pub const IRIREF:isize=62; 
		pub const PNAME_NS:isize=63; 
		pub const PNAME_LN:isize=64; 
		pub const ATPNAME_NS:isize=65; 
		pub const ATPNAME_LN:isize=66; 
		pub const REGEXP:isize=67; 
		pub const REGEXP_FLAGS:isize=68; 
		pub const BLANK_NODE_LABEL:isize=69; 
		pub const LANGTAG:isize=70; 
		pub const INTEGER:isize=71; 
		pub const DECIMAL:isize=72; 
		pub const DOUBLE:isize=73; 
		pub const STEM_MARK:isize=74; 
		pub const UNBOUNDED:isize=75; 
		pub const STRING_LITERAL1:isize=76; 
		pub const STRING_LITERAL2:isize=77; 
		pub const STRING_LITERAL_LONG1:isize=78; 
		pub const STRING_LITERAL_LONG2:isize=79;
	pub const RULE_shExDoc:usize = 0; 
	pub const RULE_directive:usize = 1; 
	pub const RULE_baseDecl:usize = 2; 
	pub const RULE_prefixDecl:usize = 3; 
	pub const RULE_importDecl:usize = 4; 
	pub const RULE_notStartAction:usize = 5; 
	pub const RULE_start:usize = 6; 
	pub const RULE_startActions:usize = 7; 
	pub const RULE_statement:usize = 8; 
	pub const RULE_shapeExprDecl:usize = 9; 
	pub const RULE_shapeExpression:usize = 10; 
	pub const RULE_inlineShapeExpression:usize = 11; 
	pub const RULE_shapeOr:usize = 12; 
	pub const RULE_inlineShapeOr:usize = 13; 
	pub const RULE_shapeAnd:usize = 14; 
	pub const RULE_inlineShapeAnd:usize = 15; 
	pub const RULE_shapeNot:usize = 16; 
	pub const RULE_inlineShapeNot:usize = 17; 
	pub const RULE_negation:usize = 18; 
	pub const RULE_shapeAtom:usize = 19; 
	pub const RULE_inlineShapeAtom:usize = 20; 
	pub const RULE_shapeOrRef:usize = 21; 
	pub const RULE_inlineShapeOrRef:usize = 22; 
	pub const RULE_shapeRef:usize = 23; 
	pub const RULE_inlineLitNodeConstraint:usize = 24; 
	pub const RULE_litNodeConstraint:usize = 25; 
	pub const RULE_inlineNonLitNodeConstraint:usize = 26; 
	pub const RULE_nonLitNodeConstraint:usize = 27; 
	pub const RULE_nonLiteralKind:usize = 28; 
	pub const RULE_xsFacet:usize = 29; 
	pub const RULE_stringFacet:usize = 30; 
	pub const RULE_stringLength:usize = 31; 
	pub const RULE_numericFacet:usize = 32; 
	pub const RULE_numericRange:usize = 33; 
	pub const RULE_numericLength:usize = 34; 
	pub const RULE_rawNumeric:usize = 35; 
	pub const RULE_shapeDefinition:usize = 36; 
	pub const RULE_inlineShapeDefinition:usize = 37; 
	pub const RULE_qualifier:usize = 38; 
	pub const RULE_extraPropertySet:usize = 39; 
	pub const RULE_tripleExpression:usize = 40; 
	pub const RULE_oneOfTripleExpr:usize = 41; 
	pub const RULE_multiElementOneOf:usize = 42; 
	pub const RULE_groupTripleExpr:usize = 43; 
	pub const RULE_singleElementGroup:usize = 44; 
	pub const RULE_multiElementGroup:usize = 45; 
	pub const RULE_unaryTripleExpr:usize = 46; 
	pub const RULE_bracketedTripleExpr:usize = 47; 
	pub const RULE_tripleConstraint:usize = 48; 
	pub const RULE_cardinality:usize = 49; 
	pub const RULE_repeatRange:usize = 50; 
	pub const RULE_min_range:usize = 51; 
	pub const RULE_max_range:usize = 52; 
	pub const RULE_expr:usize = 53; 
	pub const RULE_binOp:usize = 54; 
	pub const RULE_basicExpr:usize = 55; 
	pub const RULE_senseFlags:usize = 56; 
	pub const RULE_valueSet:usize = 57; 
	pub const RULE_valueSetValue:usize = 58; 
	pub const RULE_iriRange:usize = 59; 
	pub const RULE_iriExclusion:usize = 60; 
	pub const RULE_literalRange:usize = 61; 
	pub const RULE_literalExclusion:usize = 62; 
	pub const RULE_languageRange:usize = 63; 
	pub const RULE_languageExclusion:usize = 64; 
	pub const RULE_include:usize = 65; 
	pub const RULE_annotation:usize = 66; 
	pub const RULE_semanticAction:usize = 67; 
	pub const RULE_literal:usize = 68; 
	pub const RULE_predicate:usize = 69; 
	pub const RULE_rdfType:usize = 70; 
	pub const RULE_datatype:usize = 71; 
	pub const RULE_shapeExprLabel:usize = 72; 
	pub const RULE_tripleExprLabel:usize = 73; 
	pub const RULE_numericLiteral:usize = 74; 
	pub const RULE_rdfLiteral:usize = 75; 
	pub const RULE_booleanLiteral:usize = 76; 
	pub const RULE_string:usize = 77; 
	pub const RULE_iri:usize = 78; 
	pub const RULE_prefixedName:usize = 79; 
	pub const RULE_blankNode:usize = 80; 
	pub const RULE_extension:usize = 81; 
	pub const RULE_restriction:usize = 82;
	pub const ruleNames: [&'static str; 83] =  [
		"shExDoc", "directive", "baseDecl", "prefixDecl", "importDecl", "notStartAction", 
		"start", "startActions", "statement", "shapeExprDecl", "shapeExpression", 
		"inlineShapeExpression", "shapeOr", "inlineShapeOr", "shapeAnd", "inlineShapeAnd", 
		"shapeNot", "inlineShapeNot", "negation", "shapeAtom", "inlineShapeAtom", 
		"shapeOrRef", "inlineShapeOrRef", "shapeRef", "inlineLitNodeConstraint", 
		"litNodeConstraint", "inlineNonLitNodeConstraint", "nonLitNodeConstraint", 
		"nonLiteralKind", "xsFacet", "stringFacet", "stringLength", "numericFacet", 
		"numericRange", "numericLength", "rawNumeric", "shapeDefinition", "inlineShapeDefinition", 
		"qualifier", "extraPropertySet", "tripleExpression", "oneOfTripleExpr", 
		"multiElementOneOf", "groupTripleExpr", "singleElementGroup", "multiElementGroup", 
		"unaryTripleExpr", "bracketedTripleExpr", "tripleConstraint", "cardinality", 
		"repeatRange", "min_range", "max_range", "expr", "binOp", "basicExpr", 
		"senseFlags", "valueSet", "valueSetValue", "iriRange", "iriExclusion", 
		"literalRange", "literalExclusion", "languageRange", "languageExclusion", 
		"include", "annotation", "semanticAction", "literal", "predicate", "rdfType", 
		"datatype", "shapeExprLabel", "tripleExprLabel", "numericLiteral", "rdfLiteral", 
		"booleanLiteral", "string", "iri", "prefixedName", "blankNode", "extension", 
		"restriction"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;76] = [
		None, Some("'='"), Some("'!'"), Some("'('"), Some("')'"), Some("'.'"), 
		Some("'@'"), Some("'{'"), Some("'}'"), Some("'|'"), Some("';'"), Some("'$'"), 
		Some("'+'"), Some("'?'"), Some("','"), Some("'!='"), Some("'>'"), Some("'<'"), 
		Some("'>='"), Some("'<='"), Some("'/'"), Some("'-'"), Some("'^'"), Some("'['"), 
		Some("']'"), Some("'&'"), Some("'//'"), Some("'%'"), Some("'^^'"), None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, Some("'true'"), Some("'false'"), None, None, Some("'a'"), 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		Some("'~'"), Some("'*'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;80]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, Some("KW_ABSTRACT"), Some("KW_AS"), Some("KW_BASE"), 
		Some("KW_EXTENDS"), Some("KW_IMPORT"), Some("KW_RESTRICTS"), Some("KW_EXTERNAL"), 
		Some("KW_PREFIX"), Some("KW_START"), Some("KW_VIRTUAL"), Some("KW_CLOSED"), 
		Some("KW_EXTRA"), Some("KW_LITERAL"), Some("KW_IRI"), Some("KW_NONLITERAL"), 
		Some("KW_BNODE"), Some("KW_AND"), Some("KW_OR"), Some("KW_MININCLUSIVE"), 
		Some("KW_MINEXCLUSIVE"), Some("KW_MAXINCLUSIVE"), Some("KW_MAXEXCLUSIVE"), 
		Some("KW_LENGTH"), Some("KW_MINLENGTH"), Some("KW_MAXLENGTH"), Some("KW_TOTALDIGITS"), 
		Some("KW_FRACTIONDIGITS"), Some("KW_NOT"), Some("KW_TRUE"), Some("KW_FALSE"), 
		Some("SKIP_"), Some("CODE"), Some("RDF_TYPE"), Some("IRIREF"), Some("PNAME_NS"), 
		Some("PNAME_LN"), Some("ATPNAME_NS"), Some("ATPNAME_LN"), Some("REGEXP"), 
		Some("REGEXP_FLAGS"), Some("BLANK_NODE_LABEL"), Some("LANGTAG"), Some("INTEGER"), 
		Some("DECIMAL"), Some("DOUBLE"), Some("STEM_MARK"), Some("UNBOUNDED"), 
		Some("STRING_LITERAL1"), Some("STRING_LITERAL2"), Some("STRING_LITERAL_LONG1"), 
		Some("STRING_LITERAL_LONG2")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,ShExDocParserExt<'input>, I, ShExDocParserContextType , dyn ShExDocListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ShExDocTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, ShExDocParserContextType , dyn ShExDocListener<'input> + 'a>;

/// Parser for ShExDoc grammar
pub struct ShExDocParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				ShExDocParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> ShExDocParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ShExDocParser<'input, I, DefaultErrorStrategy<'input,ShExDocParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ShExDocParser
pub trait ShExDocParserContext<'input>:
	for<'x> Listenable<dyn ShExDocListener<'input> + 'x > + 
	for<'x> Visitable<dyn ShExDocVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ShExDocParserContextType>
{}

antlr_rust::coerce_from!{ 'input : ShExDocParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn ShExDocParserContext<'input> + 'input
where
    T: ShExDocVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn ShExDocVisitor<'input> + 'x))
    }
}

impl<'input> ShExDocParserContext<'input> for TerminalNode<'input,ShExDocParserContextType> {}
impl<'input> ShExDocParserContext<'input> for ErrorNode<'input,ShExDocParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ShExDocParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ShExDocListener<'input> + 'input }

pub struct ShExDocParserContextType;
antlr_rust::tid!{ShExDocParserContextType}

impl<'input> ParserNodeType<'input> for ShExDocParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn ShExDocParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ShExDocParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> ShExDocParserExt<'input>{
}
antlr_rust::tid! { ShExDocParserExt<'a> }

impl<'input> TokenAware<'input> for ShExDocParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ShExDocParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ShExDocParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "ShExDoc.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn ShExDocParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					53 => ShExDocParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> ShExDocParser<'input, I, DefaultErrorStrategy<'input,ShExDocParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- shExDoc ----------------
pub type ShExDocContextAll<'input> = ShExDocContext<'input>;


pub type ShExDocContext<'input> = BaseParserRuleContext<'input,ShExDocContextExt<'input>>;

#[derive(Clone)]
pub struct ShExDocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShExDocContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShExDocContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shExDoc(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shExDoc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShExDocContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shExDoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShExDocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shExDoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shExDoc }
}
antlr_rust::tid!{ShExDocContextExt<'a>}

impl<'input> ShExDocContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShExDocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShExDocContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShExDocContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShExDocContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn directive_all(&self) ->  Vec<Rc<DirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn directive(&self, i: usize) -> Option<Rc<DirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn notStartAction(&self) -> Option<Rc<NotStartActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn startActions(&self) -> Option<Rc<StartActionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ShExDocContextAttrs<'input> for ShExDocContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shExDoc(&mut self,)
	-> Result<Rc<ShExDocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShExDocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_shExDoc);
        let mut _localctx: Rc<ShExDocContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(169);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 31)) & !0x3f) == 0 && ((1usize << (_la - 31)) & ((1usize << (KW_BASE - 31)) | (1usize << (KW_IMPORT - 31)) | (1usize << (KW_PREFIX - 31)))) != 0) {
				{
				{
				/*InvokeRule directive*/
				recog.base.set_state(166);
				recog.directive()?;

				}
				}
				recog.base.set_state(171);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(182);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 27)) & !0x3f) == 0 && ((1usize << (_la - 27)) & ((1usize << (T__26 - 27)) | (1usize << (KW_ABSTRACT - 27)) | (1usize << (KW_START - 27)))) != 0) || ((((_la - 62)) & !0x3f) == 0 && ((1usize << (_la - 62)) & ((1usize << (IRIREF - 62)) | (1usize << (PNAME_NS - 62)) | (1usize << (PNAME_LN - 62)) | (1usize << (BLANK_NODE_LABEL - 62)))) != 0) {
				{
				recog.base.set_state(174);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 KW_ABSTRACT | KW_START | IRIREF | PNAME_NS | PNAME_LN | BLANK_NODE_LABEL 
					=> {
						{
						/*InvokeRule notStartAction*/
						recog.base.set_state(172);
						recog.notStartAction()?;

						}
					}

				 T__26 
					=> {
						{
						/*InvokeRule startActions*/
						recog.base.set_state(173);
						recog.startActions()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(179);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while ((((_la - 29)) & !0x3f) == 0 && ((1usize << (_la - 29)) & ((1usize << (KW_ABSTRACT - 29)) | (1usize << (KW_BASE - 29)) | (1usize << (KW_IMPORT - 29)) | (1usize << (KW_PREFIX - 29)) | (1usize << (KW_START - 29)))) != 0) || ((((_la - 62)) & !0x3f) == 0 && ((1usize << (_la - 62)) & ((1usize << (IRIREF - 62)) | (1usize << (PNAME_NS - 62)) | (1usize << (PNAME_LN - 62)) | (1usize << (BLANK_NODE_LABEL - 62)))) != 0) {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(176);
					recog.statement()?;

					}
					}
					recog.base.set_state(181);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(184);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- directive ----------------
pub type DirectiveContextAll<'input> = DirectiveContext<'input>;


pub type DirectiveContext<'input> = BaseParserRuleContext<'input,DirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct DirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for DirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for DirectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_directive(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_directive(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for DirectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_directive(self);
	}
}

impl<'input> CustomRuleContext<'input> for DirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_directive }
	//fn type_rule_index() -> usize where Self: Sized { RULE_directive }
}
antlr_rust::tid!{DirectiveContextExt<'a>}

impl<'input> DirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DirectiveContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<DirectiveContextExt<'input>>{

fn baseDecl(&self) -> Option<Rc<BaseDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn prefixDecl(&self) -> Option<Rc<PrefixDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn importDecl(&self) -> Option<Rc<ImportDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DirectiveContextAttrs<'input> for DirectiveContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn directive(&mut self,)
	-> Result<Rc<DirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_directive);
        let mut _localctx: Rc<DirectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(189);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_BASE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule baseDecl*/
					recog.base.set_state(186);
					recog.baseDecl()?;

					}
				}

			 KW_PREFIX 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule prefixDecl*/
					recog.base.set_state(187);
					recog.prefixDecl()?;

					}
				}

			 KW_IMPORT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule importDecl*/
					recog.base.set_state(188);
					recog.importDecl()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- baseDecl ----------------
pub type BaseDeclContextAll<'input> = BaseDeclContext<'input>;


pub type BaseDeclContext<'input> = BaseParserRuleContext<'input,BaseDeclContextExt<'input>>;

#[derive(Clone)]
pub struct BaseDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for BaseDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BaseDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_baseDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_baseDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BaseDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_baseDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for BaseDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_baseDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_baseDecl }
}
antlr_rust::tid!{BaseDeclContextExt<'a>}

impl<'input> BaseDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BaseDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BaseDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BaseDeclContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<BaseDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_BASE
/// Returns `None` if there is no child corresponding to token KW_BASE
fn KW_BASE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_BASE, 0)
}
/// Retrieves first TerminalNode corresponding to token IRIREF
/// Returns `None` if there is no child corresponding to token IRIREF
fn IRIREF(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(IRIREF, 0)
}

}

impl<'input> BaseDeclContextAttrs<'input> for BaseDeclContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn baseDecl(&mut self,)
	-> Result<Rc<BaseDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BaseDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_baseDecl);
        let mut _localctx: Rc<BaseDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(191);
			recog.base.match_token(KW_BASE,&mut recog.err_handler)?;

			recog.base.set_state(192);
			recog.base.match_token(IRIREF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- prefixDecl ----------------
pub type PrefixDeclContextAll<'input> = PrefixDeclContext<'input>;


pub type PrefixDeclContext<'input> = BaseParserRuleContext<'input,PrefixDeclContextExt<'input>>;

#[derive(Clone)]
pub struct PrefixDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for PrefixDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for PrefixDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prefixDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_prefixDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for PrefixDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_prefixDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrefixDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prefixDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prefixDecl }
}
antlr_rust::tid!{PrefixDeclContextExt<'a>}

impl<'input> PrefixDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrefixDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrefixDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrefixDeclContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<PrefixDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_PREFIX
/// Returns `None` if there is no child corresponding to token KW_PREFIX
fn KW_PREFIX(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_PREFIX, 0)
}
/// Retrieves first TerminalNode corresponding to token PNAME_NS
/// Returns `None` if there is no child corresponding to token PNAME_NS
fn PNAME_NS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(PNAME_NS, 0)
}
/// Retrieves first TerminalNode corresponding to token IRIREF
/// Returns `None` if there is no child corresponding to token IRIREF
fn IRIREF(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(IRIREF, 0)
}

}

impl<'input> PrefixDeclContextAttrs<'input> for PrefixDeclContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prefixDecl(&mut self,)
	-> Result<Rc<PrefixDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrefixDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_prefixDecl);
        let mut _localctx: Rc<PrefixDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(194);
			recog.base.match_token(KW_PREFIX,&mut recog.err_handler)?;

			recog.base.set_state(195);
			recog.base.match_token(PNAME_NS,&mut recog.err_handler)?;

			recog.base.set_state(196);
			recog.base.match_token(IRIREF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importDecl ----------------
pub type ImportDeclContextAll<'input> = ImportDeclContext<'input>;


pub type ImportDeclContext<'input> = BaseParserRuleContext<'input,ImportDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ImportDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ImportDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ImportDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_importDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ImportDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_importDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importDecl }
}
antlr_rust::tid!{ImportDeclContextExt<'a>}

impl<'input> ImportDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportDeclContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ImportDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_IMPORT
/// Returns `None` if there is no child corresponding to token KW_IMPORT
fn KW_IMPORT(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_IMPORT, 0)
}
fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportDeclContextAttrs<'input> for ImportDeclContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importDecl(&mut self,)
	-> Result<Rc<ImportDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_importDecl);
        let mut _localctx: Rc<ImportDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(198);
			recog.base.match_token(KW_IMPORT,&mut recog.err_handler)?;

			/*InvokeRule iri*/
			recog.base.set_state(199);
			recog.iri()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- notStartAction ----------------
pub type NotStartActionContextAll<'input> = NotStartActionContext<'input>;


pub type NotStartActionContext<'input> = BaseParserRuleContext<'input,NotStartActionContextExt<'input>>;

#[derive(Clone)]
pub struct NotStartActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NotStartActionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NotStartActionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_notStartAction(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_notStartAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NotStartActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_notStartAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for NotStartActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_notStartAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_notStartAction }
}
antlr_rust::tid!{NotStartActionContextExt<'a>}

impl<'input> NotStartActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NotStartActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NotStartActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NotStartActionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NotStartActionContextExt<'input>>{

fn start(&self) -> Option<Rc<StartContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn shapeExprDecl(&self) -> Option<Rc<ShapeExprDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NotStartActionContextAttrs<'input> for NotStartActionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn notStartAction(&mut self,)
	-> Result<Rc<NotStartActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NotStartActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_notStartAction);
        let mut _localctx: Rc<NotStartActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(203);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule start*/
					recog.base.set_state(201);
					recog.start()?;

					}
				}

			 KW_ABSTRACT | IRIREF | PNAME_NS | PNAME_LN | BLANK_NODE_LABEL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule shapeExprDecl*/
					recog.base.set_state(202);
					recog.shapeExprDecl()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start ----------------
pub type StartContextAll<'input> = StartContext<'input>;


pub type StartContext<'input> = BaseParserRuleContext<'input,StartContextExt<'input>>;

#[derive(Clone)]
pub struct StartContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for StartContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StartContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_start(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StartContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_start(self);
	}
}

impl<'input> CustomRuleContext<'input> for StartContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start }
}
antlr_rust::tid!{StartContextExt<'a>}

impl<'input> StartContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StartContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StartContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<StartContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_START
/// Returns `None` if there is no child corresponding to token KW_START
fn KW_START(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_START, 0)
}
fn shapeExpression(&self) -> Option<Rc<ShapeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StartContextAttrs<'input> for StartContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start(&mut self,)
	-> Result<Rc<StartContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_start);
        let mut _localctx: Rc<StartContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(205);
			recog.base.match_token(KW_START,&mut recog.err_handler)?;

			recog.base.set_state(206);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule shapeExpression*/
			recog.base.set_state(207);
			recog.shapeExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- startActions ----------------
pub type StartActionsContextAll<'input> = StartActionsContext<'input>;


pub type StartActionsContext<'input> = BaseParserRuleContext<'input,StartActionsContextExt<'input>>;

#[derive(Clone)]
pub struct StartActionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for StartActionsContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StartActionsContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_startActions(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_startActions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StartActionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_startActions(self);
	}
}

impl<'input> CustomRuleContext<'input> for StartActionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startActions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startActions }
}
antlr_rust::tid!{StartActionsContextExt<'a>}

impl<'input> StartActionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StartActionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartActionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StartActionsContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<StartActionsContextExt<'input>>{

fn semanticAction_all(&self) ->  Vec<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn semanticAction(&self, i: usize) -> Option<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StartActionsContextAttrs<'input> for StartActionsContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn startActions(&mut self,)
	-> Result<Rc<StartActionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartActionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_startActions);
        let mut _localctx: Rc<StartActionsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(210); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule semanticAction*/
				recog.base.set_state(209);
				recog.semanticAction()?;

				}
				}
				recog.base.set_state(212); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__26) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn directive(&self) -> Option<Rc<DirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn notStartAction(&self) -> Option<Rc<NotStartActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(216);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_BASE | KW_IMPORT | KW_PREFIX 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule directive*/
					recog.base.set_state(214);
					recog.directive()?;

					}
				}

			 KW_ABSTRACT | KW_START | IRIREF | PNAME_NS | PNAME_LN | BLANK_NODE_LABEL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule notStartAction*/
					recog.base.set_state(215);
					recog.notStartAction()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeExprDecl ----------------
pub type ShapeExprDeclContextAll<'input> = ShapeExprDeclContext<'input>;


pub type ShapeExprDeclContext<'input> = BaseParserRuleContext<'input,ShapeExprDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeExprDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeExprDeclContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeExprDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeExprDecl(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeExprDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeExprDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeExprDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeExprDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeExprDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeExprDecl }
}
antlr_rust::tid!{ShapeExprDeclContextExt<'a>}

impl<'input> ShapeExprDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeExprDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeExprDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeExprDeclContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeExprDeclContextExt<'input>>{

fn shapeExprLabel(&self) -> Option<Rc<ShapeExprLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn shapeExpression(&self) -> Option<Rc<ShapeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KW_EXTERNAL
/// Returns `None` if there is no child corresponding to token KW_EXTERNAL
fn KW_EXTERNAL(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_EXTERNAL, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_ABSTRACT
/// Returns `None` if there is no child corresponding to token KW_ABSTRACT
fn KW_ABSTRACT(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_ABSTRACT, 0)
}

}

impl<'input> ShapeExprDeclContextAttrs<'input> for ShapeExprDeclContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeExprDecl(&mut self,)
	-> Result<Rc<ShapeExprDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeExprDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_shapeExprDecl);
        let mut _localctx: Rc<ShapeExprDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(219);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==KW_ABSTRACT {
				{
				recog.base.set_state(218);
				recog.base.match_token(KW_ABSTRACT,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule shapeExprLabel*/
			recog.base.set_state(221);
			recog.shapeExprLabel()?;

			recog.base.set_state(224);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__1 | T__2 | T__4 | T__5 | T__6 | T__20 | T__22 | T__24 | KW_EXTENDS |
			 KW_RESTRICTS | KW_CLOSED | KW_EXTRA | KW_LITERAL | KW_IRI | KW_NONLITERAL |
			 KW_BNODE | KW_MININCLUSIVE | KW_MINEXCLUSIVE | KW_MAXINCLUSIVE | KW_MAXEXCLUSIVE |
			 KW_LENGTH | KW_MINLENGTH | KW_MAXLENGTH | KW_TOTALDIGITS | KW_FRACTIONDIGITS |
			 KW_NOT | IRIREF | PNAME_NS | PNAME_LN | ATPNAME_NS | ATPNAME_LN | REGEXP 
				=> {
					{
					/*InvokeRule shapeExpression*/
					recog.base.set_state(222);
					recog.shapeExpression()?;

					}
				}

			 KW_EXTERNAL 
				=> {
					{
					recog.base.set_state(223);
					recog.base.match_token(KW_EXTERNAL,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeExpression ----------------
pub type ShapeExpressionContextAll<'input> = ShapeExpressionContext<'input>;


pub type ShapeExpressionContext<'input> = BaseParserRuleContext<'input,ShapeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeExpression(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeExpression }
}
antlr_rust::tid!{ShapeExpressionContextExt<'a>}

impl<'input> ShapeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeExpressionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeExpressionContextExt<'input>>{

fn shapeOr(&self) -> Option<Rc<ShapeOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShapeExpressionContextAttrs<'input> for ShapeExpressionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeExpression(&mut self,)
	-> Result<Rc<ShapeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_shapeExpression);
        let mut _localctx: Rc<ShapeExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule shapeOr*/
			recog.base.set_state(226);
			recog.shapeOr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeExpression ----------------
pub type InlineShapeExpressionContextAll<'input> = InlineShapeExpressionContext<'input>;


pub type InlineShapeExpressionContext<'input> = BaseParserRuleContext<'input,InlineShapeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineShapeExpression(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_inlineShapeExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeExpression }
}
antlr_rust::tid!{InlineShapeExpressionContextExt<'a>}

impl<'input> InlineShapeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineShapeExpressionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeExpressionContextExt<'input>>{

fn inlineShapeOr(&self) -> Option<Rc<InlineShapeOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InlineShapeExpressionContextAttrs<'input> for InlineShapeExpressionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeExpression(&mut self,)
	-> Result<Rc<InlineShapeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_inlineShapeExpression);
        let mut _localctx: Rc<InlineShapeExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inlineShapeOr*/
			recog.base.set_state(228);
			recog.inlineShapeOr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeOr ----------------
pub type ShapeOrContextAll<'input> = ShapeOrContext<'input>;


pub type ShapeOrContext<'input> = BaseParserRuleContext<'input,ShapeOrContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeOrContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeOrContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeOr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeOrContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeOr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeOr }
}
antlr_rust::tid!{ShapeOrContextExt<'a>}

impl<'input> ShapeOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeOrContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeOrContextExt<'input>>{

fn shapeAnd_all(&self) ->  Vec<Rc<ShapeAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shapeAnd(&self, i: usize) -> Option<Rc<ShapeAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token KW_OR in current rule
fn KW_OR_all(&self) -> Vec<Rc<TerminalNode<'input,ShExDocParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token KW_OR, starting from 0.
/// Returns `None` if number of children corresponding to token KW_OR is less or equal than `i`.
fn KW_OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_OR, i)
}

}

impl<'input> ShapeOrContextAttrs<'input> for ShapeOrContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeOr(&mut self,)
	-> Result<Rc<ShapeOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_shapeOr);
        let mut _localctx: Rc<ShapeOrContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule shapeAnd*/
			recog.base.set_state(230);
			recog.shapeAnd()?;

			recog.base.set_state(235);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==KW_OR {
				{
				{
				recog.base.set_state(231);
				recog.base.match_token(KW_OR,&mut recog.err_handler)?;

				/*InvokeRule shapeAnd*/
				recog.base.set_state(232);
				recog.shapeAnd()?;

				}
				}
				recog.base.set_state(237);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeOr ----------------
pub type InlineShapeOrContextAll<'input> = InlineShapeOrContext<'input>;


pub type InlineShapeOrContext<'input> = BaseParserRuleContext<'input,InlineShapeOrContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeOrContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeOrContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineShapeOr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_inlineShapeOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeOrContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeOr(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeOr }
}
antlr_rust::tid!{InlineShapeOrContextExt<'a>}

impl<'input> InlineShapeOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineShapeOrContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeOrContextExt<'input>>{

fn inlineShapeAnd_all(&self) ->  Vec<Rc<InlineShapeAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inlineShapeAnd(&self, i: usize) -> Option<Rc<InlineShapeAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token KW_OR in current rule
fn KW_OR_all(&self) -> Vec<Rc<TerminalNode<'input,ShExDocParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token KW_OR, starting from 0.
/// Returns `None` if number of children corresponding to token KW_OR is less or equal than `i`.
fn KW_OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_OR, i)
}

}

impl<'input> InlineShapeOrContextAttrs<'input> for InlineShapeOrContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeOr(&mut self,)
	-> Result<Rc<InlineShapeOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_inlineShapeOr);
        let mut _localctx: Rc<InlineShapeOrContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inlineShapeAnd*/
			recog.base.set_state(238);
			recog.inlineShapeAnd()?;

			recog.base.set_state(243);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==KW_OR {
				{
				{
				recog.base.set_state(239);
				recog.base.match_token(KW_OR,&mut recog.err_handler)?;

				/*InvokeRule inlineShapeAnd*/
				recog.base.set_state(240);
				recog.inlineShapeAnd()?;

				}
				}
				recog.base.set_state(245);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeAnd ----------------
pub type ShapeAndContextAll<'input> = ShapeAndContext<'input>;


pub type ShapeAndContext<'input> = BaseParserRuleContext<'input,ShapeAndContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeAndContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAndContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeAnd(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeAnd(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAndContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeAnd(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAnd }
}
antlr_rust::tid!{ShapeAndContextExt<'a>}

impl<'input> ShapeAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeAndContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeAndContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeAndContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeAndContextExt<'input>>{

fn shapeNot_all(&self) ->  Vec<Rc<ShapeNotContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shapeNot(&self, i: usize) -> Option<Rc<ShapeNotContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token KW_AND in current rule
fn KW_AND_all(&self) -> Vec<Rc<TerminalNode<'input,ShExDocParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token KW_AND, starting from 0.
/// Returns `None` if number of children corresponding to token KW_AND is less or equal than `i`.
fn KW_AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_AND, i)
}

}

impl<'input> ShapeAndContextAttrs<'input> for ShapeAndContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeAnd(&mut self,)
	-> Result<Rc<ShapeAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_shapeAnd);
        let mut _localctx: Rc<ShapeAndContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule shapeNot*/
			recog.base.set_state(246);
			recog.shapeNot()?;

			recog.base.set_state(251);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==KW_AND {
				{
				{
				recog.base.set_state(247);
				recog.base.match_token(KW_AND,&mut recog.err_handler)?;

				/*InvokeRule shapeNot*/
				recog.base.set_state(248);
				recog.shapeNot()?;

				}
				}
				recog.base.set_state(253);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeAnd ----------------
pub type InlineShapeAndContextAll<'input> = InlineShapeAndContext<'input>;


pub type InlineShapeAndContext<'input> = BaseParserRuleContext<'input,InlineShapeAndContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeAndContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAndContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineShapeAnd(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_inlineShapeAnd(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAndContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeAnd(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAnd }
}
antlr_rust::tid!{InlineShapeAndContextExt<'a>}

impl<'input> InlineShapeAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeAndContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeAndContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineShapeAndContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeAndContextExt<'input>>{

fn inlineShapeNot_all(&self) ->  Vec<Rc<InlineShapeNotContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inlineShapeNot(&self, i: usize) -> Option<Rc<InlineShapeNotContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token KW_AND in current rule
fn KW_AND_all(&self) -> Vec<Rc<TerminalNode<'input,ShExDocParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token KW_AND, starting from 0.
/// Returns `None` if number of children corresponding to token KW_AND is less or equal than `i`.
fn KW_AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_AND, i)
}

}

impl<'input> InlineShapeAndContextAttrs<'input> for InlineShapeAndContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeAnd(&mut self,)
	-> Result<Rc<InlineShapeAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_inlineShapeAnd);
        let mut _localctx: Rc<InlineShapeAndContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inlineShapeNot*/
			recog.base.set_state(254);
			recog.inlineShapeNot()?;

			recog.base.set_state(259);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==KW_AND {
				{
				{
				recog.base.set_state(255);
				recog.base.match_token(KW_AND,&mut recog.err_handler)?;

				/*InvokeRule inlineShapeNot*/
				recog.base.set_state(256);
				recog.inlineShapeNot()?;

				}
				}
				recog.base.set_state(261);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeNot ----------------
pub type ShapeNotContextAll<'input> = ShapeNotContext<'input>;


pub type ShapeNotContext<'input> = BaseParserRuleContext<'input,ShapeNotContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeNotContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeNotContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeNotContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeNot(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeNot(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeNotContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeNot(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeNotContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeNot }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeNot }
}
antlr_rust::tid!{ShapeNotContextExt<'a>}

impl<'input> ShapeNotContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeNotContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeNotContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeNotContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeNotContextExt<'input>>{

fn shapeAtom(&self) -> Option<Rc<ShapeAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn negation(&self) -> Option<Rc<NegationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShapeNotContextAttrs<'input> for ShapeNotContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeNot(&mut self,)
	-> Result<Rc<ShapeNotContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeNotContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_shapeNot);
        let mut _localctx: Rc<ShapeNotContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(263);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 || _la==KW_NOT {
				{
				/*InvokeRule negation*/
				recog.base.set_state(262);
				recog.negation()?;

				}
			}

			/*InvokeRule shapeAtom*/
			recog.base.set_state(265);
			recog.shapeAtom()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeNot ----------------
pub type InlineShapeNotContextAll<'input> = InlineShapeNotContext<'input>;


pub type InlineShapeNotContext<'input> = BaseParserRuleContext<'input,InlineShapeNotContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeNotContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeNotContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeNotContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineShapeNot(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_inlineShapeNot(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeNotContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeNot(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeNotContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeNot }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeNot }
}
antlr_rust::tid!{InlineShapeNotContextExt<'a>}

impl<'input> InlineShapeNotContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeNotContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeNotContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineShapeNotContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeNotContextExt<'input>>{

fn inlineShapeAtom(&self) -> Option<Rc<InlineShapeAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn negation(&self) -> Option<Rc<NegationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InlineShapeNotContextAttrs<'input> for InlineShapeNotContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeNot(&mut self,)
	-> Result<Rc<InlineShapeNotContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeNotContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_inlineShapeNot);
        let mut _localctx: Rc<InlineShapeNotContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 || _la==KW_NOT {
				{
				/*InvokeRule negation*/
				recog.base.set_state(267);
				recog.negation()?;

				}
			}

			/*InvokeRule inlineShapeAtom*/
			recog.base.set_state(270);
			recog.inlineShapeAtom()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- negation ----------------
pub type NegationContextAll<'input> = NegationContext<'input>;


pub type NegationContext<'input> = BaseParserRuleContext<'input,NegationContextExt<'input>>;

#[derive(Clone)]
pub struct NegationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NegationContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NegationContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_negation(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_negation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NegationContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_negation(self);
	}
}

impl<'input> CustomRuleContext<'input> for NegationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_negation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_negation }
}
antlr_rust::tid!{NegationContextExt<'a>}

impl<'input> NegationContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NegationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NegationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NegationContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NegationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_NOT
/// Returns `None` if there is no child corresponding to token KW_NOT
fn KW_NOT(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_NOT, 0)
}

}

impl<'input> NegationContextAttrs<'input> for NegationContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn negation(&mut self,)
	-> Result<Rc<NegationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NegationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_negation);
        let mut _localctx: Rc<NegationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(272);
			_la = recog.base.input.la(1);
			if { !(_la==T__1 || _la==KW_NOT) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeAtom ----------------
#[derive(Debug)]
pub enum ShapeAtomContextAll<'input>{
	ShapeAtomShapeOrRefContext(ShapeAtomShapeOrRefContext<'input>),
	ShapeAtomNonLitNodeConstraintContext(ShapeAtomNonLitNodeConstraintContext<'input>),
	ShapeAtomLitNodeConstraintContext(ShapeAtomLitNodeConstraintContext<'input>),
	ShapeAtomShapeExpressionContext(ShapeAtomShapeExpressionContext<'input>),
	ShapeAtomAnyContext(ShapeAtomAnyContext<'input>),
Error(ShapeAtomContext<'input>)
}
antlr_rust::tid!{ShapeAtomContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ShapeAtomContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for ShapeAtomContextAll<'input>{}

impl<'input> Deref for ShapeAtomContextAll<'input>{
	type Target = dyn ShapeAtomContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ShapeAtomContextAll::*;
		match self{
			ShapeAtomShapeOrRefContext(inner) => inner,
			ShapeAtomNonLitNodeConstraintContext(inner) => inner,
			ShapeAtomLitNodeConstraintContext(inner) => inner,
			ShapeAtomShapeExpressionContext(inner) => inner,
			ShapeAtomAnyContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ShapeAtomContext<'input> = BaseParserRuleContext<'input,ShapeAtomContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeAtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeAtomContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ShapeAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAtom }
}
antlr_rust::tid!{ShapeAtomContextExt<'a>}

impl<'input> ShapeAtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeAtomContextAll<'input>> {
		Rc::new(
		ShapeAtomContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeAtomContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ShapeAtomContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeAtomContextExt<'input>>{


}

impl<'input> ShapeAtomContextAttrs<'input> for ShapeAtomContext<'input>{}

pub type ShapeAtomShapeOrRefContext<'input> = BaseParserRuleContext<'input,ShapeAtomShapeOrRefContextExt<'input>>;

pub trait ShapeAtomShapeOrRefContextAttrs<'input>: ShExDocParserContext<'input>{
	fn shapeOrRef(&self) -> Option<Rc<ShapeOrRefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn nonLitNodeConstraint(&self) -> Option<Rc<NonLitNodeConstraintContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ShapeAtomShapeOrRefContextAttrs<'input> for ShapeAtomShapeOrRefContext<'input>{}

pub struct ShapeAtomShapeOrRefContextExt<'input>{
	base:ShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ShapeAtomShapeOrRefContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for ShapeAtomShapeOrRefContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomShapeOrRefContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeAtomShapeOrRef(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_shapeAtomShapeOrRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomShapeOrRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeAtomShapeOrRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeAtomShapeOrRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAtom }
}

impl<'input> Borrow<ShapeAtomContextExt<'input>> for ShapeAtomShapeOrRefContext<'input>{
	fn borrow(&self) -> &ShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ShapeAtomContextExt<'input>> for ShapeAtomShapeOrRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut ShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> ShapeAtomContextAttrs<'input> for ShapeAtomShapeOrRefContext<'input> {}

impl<'input> ShapeAtomShapeOrRefContextExt<'input>{
	fn new(ctx: &dyn ShapeAtomContextAttrs<'input>) -> Rc<ShapeAtomContextAll<'input>>  {
		Rc::new(
			ShapeAtomContextAll::ShapeAtomShapeOrRefContext(
				BaseParserRuleContext::copy_from(ctx,ShapeAtomShapeOrRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ShapeAtomNonLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,ShapeAtomNonLitNodeConstraintContextExt<'input>>;

pub trait ShapeAtomNonLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input>{
	fn nonLitNodeConstraint(&self) -> Option<Rc<NonLitNodeConstraintContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn shapeOrRef(&self) -> Option<Rc<ShapeOrRefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ShapeAtomNonLitNodeConstraintContextAttrs<'input> for ShapeAtomNonLitNodeConstraintContext<'input>{}

pub struct ShapeAtomNonLitNodeConstraintContextExt<'input>{
	base:ShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ShapeAtomNonLitNodeConstraintContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for ShapeAtomNonLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomNonLitNodeConstraintContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeAtomNonLitNodeConstraint(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_shapeAtomNonLitNodeConstraint(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomNonLitNodeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeAtomNonLitNodeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeAtomNonLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAtom }
}

impl<'input> Borrow<ShapeAtomContextExt<'input>> for ShapeAtomNonLitNodeConstraintContext<'input>{
	fn borrow(&self) -> &ShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ShapeAtomContextExt<'input>> for ShapeAtomNonLitNodeConstraintContext<'input>{
	fn borrow_mut(&mut self) -> &mut ShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> ShapeAtomContextAttrs<'input> for ShapeAtomNonLitNodeConstraintContext<'input> {}

impl<'input> ShapeAtomNonLitNodeConstraintContextExt<'input>{
	fn new(ctx: &dyn ShapeAtomContextAttrs<'input>) -> Rc<ShapeAtomContextAll<'input>>  {
		Rc::new(
			ShapeAtomContextAll::ShapeAtomNonLitNodeConstraintContext(
				BaseParserRuleContext::copy_from(ctx,ShapeAtomNonLitNodeConstraintContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ShapeAtomLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,ShapeAtomLitNodeConstraintContextExt<'input>>;

pub trait ShapeAtomLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input>{
	fn litNodeConstraint(&self) -> Option<Rc<LitNodeConstraintContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ShapeAtomLitNodeConstraintContextAttrs<'input> for ShapeAtomLitNodeConstraintContext<'input>{}

pub struct ShapeAtomLitNodeConstraintContextExt<'input>{
	base:ShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ShapeAtomLitNodeConstraintContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for ShapeAtomLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomLitNodeConstraintContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeAtomLitNodeConstraint(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_shapeAtomLitNodeConstraint(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomLitNodeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeAtomLitNodeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeAtomLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAtom }
}

impl<'input> Borrow<ShapeAtomContextExt<'input>> for ShapeAtomLitNodeConstraintContext<'input>{
	fn borrow(&self) -> &ShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ShapeAtomContextExt<'input>> for ShapeAtomLitNodeConstraintContext<'input>{
	fn borrow_mut(&mut self) -> &mut ShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> ShapeAtomContextAttrs<'input> for ShapeAtomLitNodeConstraintContext<'input> {}

impl<'input> ShapeAtomLitNodeConstraintContextExt<'input>{
	fn new(ctx: &dyn ShapeAtomContextAttrs<'input>) -> Rc<ShapeAtomContextAll<'input>>  {
		Rc::new(
			ShapeAtomContextAll::ShapeAtomLitNodeConstraintContext(
				BaseParserRuleContext::copy_from(ctx,ShapeAtomLitNodeConstraintContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ShapeAtomShapeExpressionContext<'input> = BaseParserRuleContext<'input,ShapeAtomShapeExpressionContextExt<'input>>;

pub trait ShapeAtomShapeExpressionContextAttrs<'input>: ShExDocParserContext<'input>{
	fn shapeExpression(&self) -> Option<Rc<ShapeExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ShapeAtomShapeExpressionContextAttrs<'input> for ShapeAtomShapeExpressionContext<'input>{}

pub struct ShapeAtomShapeExpressionContextExt<'input>{
	base:ShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ShapeAtomShapeExpressionContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for ShapeAtomShapeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomShapeExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeAtomShapeExpression(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_shapeAtomShapeExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomShapeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeAtomShapeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeAtomShapeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAtom }
}

impl<'input> Borrow<ShapeAtomContextExt<'input>> for ShapeAtomShapeExpressionContext<'input>{
	fn borrow(&self) -> &ShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ShapeAtomContextExt<'input>> for ShapeAtomShapeExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> ShapeAtomContextAttrs<'input> for ShapeAtomShapeExpressionContext<'input> {}

impl<'input> ShapeAtomShapeExpressionContextExt<'input>{
	fn new(ctx: &dyn ShapeAtomContextAttrs<'input>) -> Rc<ShapeAtomContextAll<'input>>  {
		Rc::new(
			ShapeAtomContextAll::ShapeAtomShapeExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ShapeAtomShapeExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ShapeAtomAnyContext<'input> = BaseParserRuleContext<'input,ShapeAtomAnyContextExt<'input>>;

pub trait ShapeAtomAnyContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> ShapeAtomAnyContextAttrs<'input> for ShapeAtomAnyContext<'input>{}

pub struct ShapeAtomAnyContextExt<'input>{
	base:ShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ShapeAtomAnyContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for ShapeAtomAnyContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeAtomAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeAtomAny(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_shapeAtomAny(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeAtomAnyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeAtomAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeAtomAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeAtom }
}

impl<'input> Borrow<ShapeAtomContextExt<'input>> for ShapeAtomAnyContext<'input>{
	fn borrow(&self) -> &ShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ShapeAtomContextExt<'input>> for ShapeAtomAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut ShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> ShapeAtomContextAttrs<'input> for ShapeAtomAnyContext<'input> {}

impl<'input> ShapeAtomAnyContextExt<'input>{
	fn new(ctx: &dyn ShapeAtomContextAttrs<'input>) -> Rc<ShapeAtomContextAll<'input>>  {
		Rc::new(
			ShapeAtomContextAll::ShapeAtomAnyContext(
				BaseParserRuleContext::copy_from(ctx,ShapeAtomAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeAtom(&mut self,)
	-> Result<Rc<ShapeAtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeAtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_shapeAtom);
        let mut _localctx: Rc<ShapeAtomContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(288);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				1 =>{
					let tmp = ShapeAtomNonLitNodeConstraintContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule nonLitNodeConstraint*/
					recog.base.set_state(274);
					recog.nonLitNodeConstraint()?;

					recog.base.set_state(276);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__6 - 6)) | (1usize << (T__20 - 6)) | (1usize << (T__24 - 6)) | (1usize << (KW_EXTENDS - 6)) | (1usize << (KW_RESTRICTS - 6)))) != 0) || ((((_la - 39)) & !0x3f) == 0 && ((1usize << (_la - 39)) & ((1usize << (KW_CLOSED - 39)) | (1usize << (KW_EXTRA - 39)) | (1usize << (ATPNAME_NS - 39)) | (1usize << (ATPNAME_LN - 39)))) != 0) {
						{
						/*InvokeRule shapeOrRef*/
						recog.base.set_state(275);
						recog.shapeOrRef()?;

						}
					}

					}
				}
			,
				2 =>{
					let tmp = ShapeAtomLitNodeConstraintContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule litNodeConstraint*/
					recog.base.set_state(278);
					recog.litNodeConstraint()?;

					}
				}
			,
				3 =>{
					let tmp = ShapeAtomShapeOrRefContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule shapeOrRef*/
					recog.base.set_state(279);
					recog.shapeOrRef()?;

					recog.base.set_state(281);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 42)) & !0x3f) == 0 && ((1usize << (_la - 42)) & ((1usize << (KW_IRI - 42)) | (1usize << (KW_NONLITERAL - 42)) | (1usize << (KW_BNODE - 42)) | (1usize << (KW_LENGTH - 42)) | (1usize << (KW_MINLENGTH - 42)) | (1usize << (KW_MAXLENGTH - 42)) | (1usize << (REGEXP - 42)))) != 0) {
						{
						/*InvokeRule nonLitNodeConstraint*/
						recog.base.set_state(280);
						recog.nonLitNodeConstraint()?;

						}
					}

					}
				}
			,
				4 =>{
					let tmp = ShapeAtomShapeExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(283);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule shapeExpression*/
					recog.base.set_state(284);
					recog.shapeExpression()?;

					recog.base.set_state(285);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = ShapeAtomAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(287);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeAtom ----------------
#[derive(Debug)]
pub enum InlineShapeAtomContextAll<'input>{
	InlineShapeAtomShapeExpressionContext(InlineShapeAtomShapeExpressionContext<'input>),
	InlineShapeAtomLitNodeConstraintContext(InlineShapeAtomLitNodeConstraintContext<'input>),
	InlineShapeAtomShapeOrRefContext(InlineShapeAtomShapeOrRefContext<'input>),
	InlineShapeAtomAnyContext(InlineShapeAtomAnyContext<'input>),
	InlineShapeAtomNonLitNodeConstraintContext(InlineShapeAtomNonLitNodeConstraintContext<'input>),
Error(InlineShapeAtomContext<'input>)
}
antlr_rust::tid!{InlineShapeAtomContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for InlineShapeAtomContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomContextAll<'input>{}

impl<'input> Deref for InlineShapeAtomContextAll<'input>{
	type Target = dyn InlineShapeAtomContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use InlineShapeAtomContextAll::*;
		match self{
			InlineShapeAtomShapeExpressionContext(inner) => inner,
			InlineShapeAtomLitNodeConstraintContext(inner) => inner,
			InlineShapeAtomShapeOrRefContext(inner) => inner,
			InlineShapeAtomAnyContext(inner) => inner,
			InlineShapeAtomNonLitNodeConstraintContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type InlineShapeAtomContext<'input> = BaseParserRuleContext<'input,InlineShapeAtomContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeAtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomContext<'input>{
}

impl<'input> CustomRuleContext<'input> for InlineShapeAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAtom }
}
antlr_rust::tid!{InlineShapeAtomContextExt<'a>}

impl<'input> InlineShapeAtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeAtomContextAll<'input>> {
		Rc::new(
		InlineShapeAtomContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeAtomContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait InlineShapeAtomContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeAtomContextExt<'input>>{


}

impl<'input> InlineShapeAtomContextAttrs<'input> for InlineShapeAtomContext<'input>{}

pub type InlineShapeAtomShapeExpressionContext<'input> = BaseParserRuleContext<'input,InlineShapeAtomShapeExpressionContextExt<'input>>;

pub trait InlineShapeAtomShapeExpressionContextAttrs<'input>: ShExDocParserContext<'input>{
	fn shapeExpression(&self) -> Option<Rc<ShapeExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InlineShapeAtomShapeExpressionContextAttrs<'input> for InlineShapeAtomShapeExpressionContext<'input>{}

pub struct InlineShapeAtomShapeExpressionContextExt<'input>{
	base:InlineShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InlineShapeAtomShapeExpressionContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomShapeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomShapeExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineShapeAtomShapeExpression(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_inlineShapeAtomShapeExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomShapeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeAtomShapeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeAtomShapeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAtom }
}

impl<'input> Borrow<InlineShapeAtomContextExt<'input>> for InlineShapeAtomShapeExpressionContext<'input>{
	fn borrow(&self) -> &InlineShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineShapeAtomContextExt<'input>> for InlineShapeAtomShapeExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> InlineShapeAtomContextAttrs<'input> for InlineShapeAtomShapeExpressionContext<'input> {}

impl<'input> InlineShapeAtomShapeExpressionContextExt<'input>{
	fn new(ctx: &dyn InlineShapeAtomContextAttrs<'input>) -> Rc<InlineShapeAtomContextAll<'input>>  {
		Rc::new(
			InlineShapeAtomContextAll::InlineShapeAtomShapeExpressionContext(
				BaseParserRuleContext::copy_from(ctx,InlineShapeAtomShapeExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type InlineShapeAtomLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,InlineShapeAtomLitNodeConstraintContextExt<'input>>;

pub trait InlineShapeAtomLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input>{
	fn inlineLitNodeConstraint(&self) -> Option<Rc<InlineLitNodeConstraintContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InlineShapeAtomLitNodeConstraintContextAttrs<'input> for InlineShapeAtomLitNodeConstraintContext<'input>{}

pub struct InlineShapeAtomLitNodeConstraintContextExt<'input>{
	base:InlineShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InlineShapeAtomLitNodeConstraintContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomLitNodeConstraintContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineShapeAtomLitNodeConstraint(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_inlineShapeAtomLitNodeConstraint(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomLitNodeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeAtomLitNodeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeAtomLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAtom }
}

impl<'input> Borrow<InlineShapeAtomContextExt<'input>> for InlineShapeAtomLitNodeConstraintContext<'input>{
	fn borrow(&self) -> &InlineShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineShapeAtomContextExt<'input>> for InlineShapeAtomLitNodeConstraintContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> InlineShapeAtomContextAttrs<'input> for InlineShapeAtomLitNodeConstraintContext<'input> {}

impl<'input> InlineShapeAtomLitNodeConstraintContextExt<'input>{
	fn new(ctx: &dyn InlineShapeAtomContextAttrs<'input>) -> Rc<InlineShapeAtomContextAll<'input>>  {
		Rc::new(
			InlineShapeAtomContextAll::InlineShapeAtomLitNodeConstraintContext(
				BaseParserRuleContext::copy_from(ctx,InlineShapeAtomLitNodeConstraintContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type InlineShapeAtomShapeOrRefContext<'input> = BaseParserRuleContext<'input,InlineShapeAtomShapeOrRefContextExt<'input>>;

pub trait InlineShapeAtomShapeOrRefContextAttrs<'input>: ShExDocParserContext<'input>{
	fn inlineShapeOrRef(&self) -> Option<Rc<InlineShapeOrRefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn inlineNonLitNodeConstraint(&self) -> Option<Rc<InlineNonLitNodeConstraintContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InlineShapeAtomShapeOrRefContextAttrs<'input> for InlineShapeAtomShapeOrRefContext<'input>{}

pub struct InlineShapeAtomShapeOrRefContextExt<'input>{
	base:InlineShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InlineShapeAtomShapeOrRefContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomShapeOrRefContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomShapeOrRefContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineShapeAtomShapeOrRef(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_inlineShapeAtomShapeOrRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomShapeOrRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeAtomShapeOrRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeAtomShapeOrRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAtom }
}

impl<'input> Borrow<InlineShapeAtomContextExt<'input>> for InlineShapeAtomShapeOrRefContext<'input>{
	fn borrow(&self) -> &InlineShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineShapeAtomContextExt<'input>> for InlineShapeAtomShapeOrRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> InlineShapeAtomContextAttrs<'input> for InlineShapeAtomShapeOrRefContext<'input> {}

impl<'input> InlineShapeAtomShapeOrRefContextExt<'input>{
	fn new(ctx: &dyn InlineShapeAtomContextAttrs<'input>) -> Rc<InlineShapeAtomContextAll<'input>>  {
		Rc::new(
			InlineShapeAtomContextAll::InlineShapeAtomShapeOrRefContext(
				BaseParserRuleContext::copy_from(ctx,InlineShapeAtomShapeOrRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type InlineShapeAtomAnyContext<'input> = BaseParserRuleContext<'input,InlineShapeAtomAnyContextExt<'input>>;

pub trait InlineShapeAtomAnyContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> InlineShapeAtomAnyContextAttrs<'input> for InlineShapeAtomAnyContext<'input>{}

pub struct InlineShapeAtomAnyContextExt<'input>{
	base:InlineShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InlineShapeAtomAnyContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomAnyContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineShapeAtomAny(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_inlineShapeAtomAny(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomAnyContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeAtomAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeAtomAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAtom }
}

impl<'input> Borrow<InlineShapeAtomContextExt<'input>> for InlineShapeAtomAnyContext<'input>{
	fn borrow(&self) -> &InlineShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineShapeAtomContextExt<'input>> for InlineShapeAtomAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> InlineShapeAtomContextAttrs<'input> for InlineShapeAtomAnyContext<'input> {}

impl<'input> InlineShapeAtomAnyContextExt<'input>{
	fn new(ctx: &dyn InlineShapeAtomContextAttrs<'input>) -> Rc<InlineShapeAtomContextAll<'input>>  {
		Rc::new(
			InlineShapeAtomContextAll::InlineShapeAtomAnyContext(
				BaseParserRuleContext::copy_from(ctx,InlineShapeAtomAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type InlineShapeAtomNonLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,InlineShapeAtomNonLitNodeConstraintContextExt<'input>>;

pub trait InlineShapeAtomNonLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input>{
	fn inlineNonLitNodeConstraint(&self) -> Option<Rc<InlineNonLitNodeConstraintContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn inlineShapeOrRef(&self) -> Option<Rc<InlineShapeOrRefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InlineShapeAtomNonLitNodeConstraintContextAttrs<'input> for InlineShapeAtomNonLitNodeConstraintContext<'input>{}

pub struct InlineShapeAtomNonLitNodeConstraintContextExt<'input>{
	base:InlineShapeAtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InlineShapeAtomNonLitNodeConstraintContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for InlineShapeAtomNonLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeAtomNonLitNodeConstraintContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineShapeAtomNonLitNodeConstraint(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_inlineShapeAtomNonLitNodeConstraint(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeAtomNonLitNodeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeAtomNonLitNodeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeAtomNonLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeAtom }
}

impl<'input> Borrow<InlineShapeAtomContextExt<'input>> for InlineShapeAtomNonLitNodeConstraintContext<'input>{
	fn borrow(&self) -> &InlineShapeAtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineShapeAtomContextExt<'input>> for InlineShapeAtomNonLitNodeConstraintContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineShapeAtomContextExt<'input> { &mut self.base }
}

impl<'input> InlineShapeAtomContextAttrs<'input> for InlineShapeAtomNonLitNodeConstraintContext<'input> {}

impl<'input> InlineShapeAtomNonLitNodeConstraintContextExt<'input>{
	fn new(ctx: &dyn InlineShapeAtomContextAttrs<'input>) -> Rc<InlineShapeAtomContextAll<'input>>  {
		Rc::new(
			InlineShapeAtomContextAll::InlineShapeAtomNonLitNodeConstraintContext(
				BaseParserRuleContext::copy_from(ctx,InlineShapeAtomNonLitNodeConstraintContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeAtom(&mut self,)
	-> Result<Rc<InlineShapeAtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeAtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_inlineShapeAtom);
        let mut _localctx: Rc<InlineShapeAtomContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(304);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				1 =>{
					let tmp = InlineShapeAtomNonLitNodeConstraintContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule inlineNonLitNodeConstraint*/
					recog.base.set_state(290);
					recog.inlineNonLitNodeConstraint()?;

					recog.base.set_state(292);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule inlineShapeOrRef*/
							recog.base.set_state(291);
							recog.inlineShapeOrRef()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					let tmp = InlineShapeAtomLitNodeConstraintContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule inlineLitNodeConstraint*/
					recog.base.set_state(294);
					recog.inlineLitNodeConstraint()?;

					}
				}
			,
				3 =>{
					let tmp = InlineShapeAtomShapeOrRefContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule inlineShapeOrRef*/
					recog.base.set_state(295);
					recog.inlineShapeOrRef()?;

					recog.base.set_state(297);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 42)) & !0x3f) == 0 && ((1usize << (_la - 42)) & ((1usize << (KW_IRI - 42)) | (1usize << (KW_NONLITERAL - 42)) | (1usize << (KW_BNODE - 42)) | (1usize << (KW_LENGTH - 42)) | (1usize << (KW_MINLENGTH - 42)) | (1usize << (KW_MAXLENGTH - 42)) | (1usize << (REGEXP - 42)))) != 0) {
						{
						/*InvokeRule inlineNonLitNodeConstraint*/
						recog.base.set_state(296);
						recog.inlineNonLitNodeConstraint()?;

						}
					}

					}
				}
			,
				4 =>{
					let tmp = InlineShapeAtomShapeExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(299);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule shapeExpression*/
					recog.base.set_state(300);
					recog.shapeExpression()?;

					recog.base.set_state(301);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = InlineShapeAtomAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(303);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeOrRef ----------------
pub type ShapeOrRefContextAll<'input> = ShapeOrRefContext<'input>;


pub type ShapeOrRefContext<'input> = BaseParserRuleContext<'input,ShapeOrRefContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeOrRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeOrRefContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeOrRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeOrRef(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeOrRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeOrRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeOrRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeOrRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeOrRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeOrRef }
}
antlr_rust::tid!{ShapeOrRefContextExt<'a>}

impl<'input> ShapeOrRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeOrRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeOrRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeOrRefContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeOrRefContextExt<'input>>{

fn shapeDefinition(&self) -> Option<Rc<ShapeDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn shapeRef(&self) -> Option<Rc<ShapeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShapeOrRefContextAttrs<'input> for ShapeOrRefContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeOrRef(&mut self,)
	-> Result<Rc<ShapeOrRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeOrRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_shapeOrRef);
        let mut _localctx: Rc<ShapeOrRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(308);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__6 | T__20 | T__24 | KW_EXTENDS | KW_RESTRICTS | KW_CLOSED | KW_EXTRA 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule shapeDefinition*/
					recog.base.set_state(306);
					recog.shapeDefinition()?;

					}
				}

			 T__5 | ATPNAME_NS | ATPNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule shapeRef*/
					recog.base.set_state(307);
					recog.shapeRef()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeOrRef ----------------
pub type InlineShapeOrRefContextAll<'input> = InlineShapeOrRefContext<'input>;


pub type InlineShapeOrRefContext<'input> = BaseParserRuleContext<'input,InlineShapeOrRefContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeOrRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeOrRefContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeOrRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineShapeOrRef(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_inlineShapeOrRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeOrRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeOrRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeOrRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeOrRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeOrRef }
}
antlr_rust::tid!{InlineShapeOrRefContextExt<'a>}

impl<'input> InlineShapeOrRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeOrRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeOrRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineShapeOrRefContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeOrRefContextExt<'input>>{

fn inlineShapeDefinition(&self) -> Option<Rc<InlineShapeDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn shapeRef(&self) -> Option<Rc<ShapeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InlineShapeOrRefContextAttrs<'input> for InlineShapeOrRefContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeOrRef(&mut self,)
	-> Result<Rc<InlineShapeOrRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeOrRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_inlineShapeOrRef);
        let mut _localctx: Rc<InlineShapeOrRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(312);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__6 | T__20 | T__24 | KW_EXTENDS | KW_RESTRICTS | KW_CLOSED | KW_EXTRA 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule inlineShapeDefinition*/
					recog.base.set_state(310);
					recog.inlineShapeDefinition()?;

					}
				}

			 T__5 | ATPNAME_NS | ATPNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule shapeRef*/
					recog.base.set_state(311);
					recog.shapeRef()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeRef ----------------
pub type ShapeRefContextAll<'input> = ShapeRefContext<'input>;


pub type ShapeRefContext<'input> = BaseParserRuleContext<'input,ShapeRefContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeRefContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeRefContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeRef(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeRef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeRef }
}
antlr_rust::tid!{ShapeRefContextExt<'a>}

impl<'input> ShapeRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeRefContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeRefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ATPNAME_LN
/// Returns `None` if there is no child corresponding to token ATPNAME_LN
fn ATPNAME_LN(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(ATPNAME_LN, 0)
}
/// Retrieves first TerminalNode corresponding to token ATPNAME_NS
/// Returns `None` if there is no child corresponding to token ATPNAME_NS
fn ATPNAME_NS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(ATPNAME_NS, 0)
}
fn shapeExprLabel(&self) -> Option<Rc<ShapeExprLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShapeRefContextAttrs<'input> for ShapeRefContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeRef(&mut self,)
	-> Result<Rc<ShapeRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_shapeRef);
        let mut _localctx: Rc<ShapeRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ATPNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(314);
					recog.base.match_token(ATPNAME_LN,&mut recog.err_handler)?;

					}
				}

			 ATPNAME_NS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(315);
					recog.base.match_token(ATPNAME_NS,&mut recog.err_handler)?;

					}
				}

			 T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(316);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule shapeExprLabel*/
					recog.base.set_state(317);
					recog.shapeExprLabel()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineLitNodeConstraint ----------------
#[derive(Debug)]
pub enum InlineLitNodeConstraintContextAll<'input>{
	NodeConstraintNumericFacetContext(NodeConstraintNumericFacetContext<'input>),
	NodeConstraintLiteralContext(NodeConstraintLiteralContext<'input>),
	NodeConstraintNonLiteralContext(NodeConstraintNonLiteralContext<'input>),
	NodeConstraintDatatypeContext(NodeConstraintDatatypeContext<'input>),
	NodeConstraintValueSetContext(NodeConstraintValueSetContext<'input>),
Error(InlineLitNodeConstraintContext<'input>)
}
antlr_rust::tid!{InlineLitNodeConstraintContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for InlineLitNodeConstraintContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for InlineLitNodeConstraintContextAll<'input>{}

impl<'input> Deref for InlineLitNodeConstraintContextAll<'input>{
	type Target = dyn InlineLitNodeConstraintContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use InlineLitNodeConstraintContextAll::*;
		match self{
			NodeConstraintNumericFacetContext(inner) => inner,
			NodeConstraintLiteralContext(inner) => inner,
			NodeConstraintNonLiteralContext(inner) => inner,
			NodeConstraintDatatypeContext(inner) => inner,
			NodeConstraintValueSetContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineLitNodeConstraintContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineLitNodeConstraintContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type InlineLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,InlineLitNodeConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct InlineLitNodeConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineLitNodeConstraintContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineLitNodeConstraintContext<'input>{
}

impl<'input> CustomRuleContext<'input> for InlineLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineLitNodeConstraint }
}
antlr_rust::tid!{InlineLitNodeConstraintContextExt<'a>}

impl<'input> InlineLitNodeConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineLitNodeConstraintContextAll<'input>> {
		Rc::new(
		InlineLitNodeConstraintContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineLitNodeConstraintContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait InlineLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineLitNodeConstraintContextExt<'input>>{


}

impl<'input> InlineLitNodeConstraintContextAttrs<'input> for InlineLitNodeConstraintContext<'input>{}

pub type NodeConstraintNumericFacetContext<'input> = BaseParserRuleContext<'input,NodeConstraintNumericFacetContextExt<'input>>;

pub trait NodeConstraintNumericFacetContextAttrs<'input>: ShExDocParserContext<'input>{
	fn numericFacet_all(&self) ->  Vec<Rc<NumericFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn numericFacet(&self, i: usize) -> Option<Rc<NumericFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NodeConstraintNumericFacetContextAttrs<'input> for NodeConstraintNumericFacetContext<'input>{}

pub struct NodeConstraintNumericFacetContextExt<'input>{
	base:InlineLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NodeConstraintNumericFacetContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for NodeConstraintNumericFacetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NodeConstraintNumericFacetContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nodeConstraintNumericFacet(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_nodeConstraintNumericFacet(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NodeConstraintNumericFacetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nodeConstraintNumericFacet(self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeConstraintNumericFacetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineLitNodeConstraint }
}

impl<'input> Borrow<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintNumericFacetContext<'input>{
	fn borrow(&self) -> &InlineLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintNumericFacetContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineLitNodeConstraintContextAttrs<'input> for NodeConstraintNumericFacetContext<'input> {}

impl<'input> NodeConstraintNumericFacetContextExt<'input>{
	fn new(ctx: &dyn InlineLitNodeConstraintContextAttrs<'input>) -> Rc<InlineLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineLitNodeConstraintContextAll::NodeConstraintNumericFacetContext(
				BaseParserRuleContext::copy_from(ctx,NodeConstraintNumericFacetContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NodeConstraintLiteralContext<'input> = BaseParserRuleContext<'input,NodeConstraintLiteralContextExt<'input>>;

pub trait NodeConstraintLiteralContextAttrs<'input>: ShExDocParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KW_LITERAL
	/// Returns `None` if there is no child corresponding to token KW_LITERAL
	fn KW_LITERAL(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(KW_LITERAL, 0)
	}
	fn xsFacet_all(&self) ->  Vec<Rc<XsFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn xsFacet(&self, i: usize) -> Option<Rc<XsFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NodeConstraintLiteralContextAttrs<'input> for NodeConstraintLiteralContext<'input>{}

pub struct NodeConstraintLiteralContextExt<'input>{
	base:InlineLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NodeConstraintLiteralContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for NodeConstraintLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NodeConstraintLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nodeConstraintLiteral(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_nodeConstraintLiteral(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NodeConstraintLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nodeConstraintLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeConstraintLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineLitNodeConstraint }
}

impl<'input> Borrow<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintLiteralContext<'input>{
	fn borrow(&self) -> &InlineLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineLitNodeConstraintContextAttrs<'input> for NodeConstraintLiteralContext<'input> {}

impl<'input> NodeConstraintLiteralContextExt<'input>{
	fn new(ctx: &dyn InlineLitNodeConstraintContextAttrs<'input>) -> Rc<InlineLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineLitNodeConstraintContextAll::NodeConstraintLiteralContext(
				BaseParserRuleContext::copy_from(ctx,NodeConstraintLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NodeConstraintNonLiteralContext<'input> = BaseParserRuleContext<'input,NodeConstraintNonLiteralContextExt<'input>>;

pub trait NodeConstraintNonLiteralContextAttrs<'input>: ShExDocParserContext<'input>{
	fn nonLiteralKind(&self) -> Option<Rc<NonLiteralKindContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn stringFacet_all(&self) ->  Vec<Rc<StringFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stringFacet(&self, i: usize) -> Option<Rc<StringFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NodeConstraintNonLiteralContextAttrs<'input> for NodeConstraintNonLiteralContext<'input>{}

pub struct NodeConstraintNonLiteralContextExt<'input>{
	base:InlineLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NodeConstraintNonLiteralContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for NodeConstraintNonLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NodeConstraintNonLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nodeConstraintNonLiteral(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_nodeConstraintNonLiteral(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NodeConstraintNonLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nodeConstraintNonLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeConstraintNonLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineLitNodeConstraint }
}

impl<'input> Borrow<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintNonLiteralContext<'input>{
	fn borrow(&self) -> &InlineLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintNonLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineLitNodeConstraintContextAttrs<'input> for NodeConstraintNonLiteralContext<'input> {}

impl<'input> NodeConstraintNonLiteralContextExt<'input>{
	fn new(ctx: &dyn InlineLitNodeConstraintContextAttrs<'input>) -> Rc<InlineLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineLitNodeConstraintContextAll::NodeConstraintNonLiteralContext(
				BaseParserRuleContext::copy_from(ctx,NodeConstraintNonLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NodeConstraintDatatypeContext<'input> = BaseParserRuleContext<'input,NodeConstraintDatatypeContextExt<'input>>;

pub trait NodeConstraintDatatypeContextAttrs<'input>: ShExDocParserContext<'input>{
	fn datatype(&self) -> Option<Rc<DatatypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn xsFacet_all(&self) ->  Vec<Rc<XsFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn xsFacet(&self, i: usize) -> Option<Rc<XsFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NodeConstraintDatatypeContextAttrs<'input> for NodeConstraintDatatypeContext<'input>{}

pub struct NodeConstraintDatatypeContextExt<'input>{
	base:InlineLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NodeConstraintDatatypeContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for NodeConstraintDatatypeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NodeConstraintDatatypeContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nodeConstraintDatatype(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_nodeConstraintDatatype(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NodeConstraintDatatypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nodeConstraintDatatype(self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeConstraintDatatypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineLitNodeConstraint }
}

impl<'input> Borrow<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintDatatypeContext<'input>{
	fn borrow(&self) -> &InlineLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintDatatypeContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineLitNodeConstraintContextAttrs<'input> for NodeConstraintDatatypeContext<'input> {}

impl<'input> NodeConstraintDatatypeContextExt<'input>{
	fn new(ctx: &dyn InlineLitNodeConstraintContextAttrs<'input>) -> Rc<InlineLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineLitNodeConstraintContextAll::NodeConstraintDatatypeContext(
				BaseParserRuleContext::copy_from(ctx,NodeConstraintDatatypeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NodeConstraintValueSetContext<'input> = BaseParserRuleContext<'input,NodeConstraintValueSetContextExt<'input>>;

pub trait NodeConstraintValueSetContextAttrs<'input>: ShExDocParserContext<'input>{
	fn valueSet(&self) -> Option<Rc<ValueSetContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn xsFacet_all(&self) ->  Vec<Rc<XsFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn xsFacet(&self, i: usize) -> Option<Rc<XsFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NodeConstraintValueSetContextAttrs<'input> for NodeConstraintValueSetContext<'input>{}

pub struct NodeConstraintValueSetContextExt<'input>{
	base:InlineLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NodeConstraintValueSetContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for NodeConstraintValueSetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NodeConstraintValueSetContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nodeConstraintValueSet(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_nodeConstraintValueSet(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NodeConstraintValueSetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nodeConstraintValueSet(self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeConstraintValueSetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineLitNodeConstraint }
}

impl<'input> Borrow<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintValueSetContext<'input>{
	fn borrow(&self) -> &InlineLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineLitNodeConstraintContextExt<'input>> for NodeConstraintValueSetContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineLitNodeConstraintContextAttrs<'input> for NodeConstraintValueSetContext<'input> {}

impl<'input> NodeConstraintValueSetContextExt<'input>{
	fn new(ctx: &dyn InlineLitNodeConstraintContextAttrs<'input>) -> Rc<InlineLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineLitNodeConstraintContextAll::NodeConstraintValueSetContext(
				BaseParserRuleContext::copy_from(ctx,NodeConstraintValueSetContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineLitNodeConstraint(&mut self,)
	-> Result<Rc<InlineLitNodeConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineLitNodeConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_inlineLitNodeConstraint);
        let mut _localctx: Rc<InlineLitNodeConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(353);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_LITERAL 
				=> {
					let tmp = NodeConstraintLiteralContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(320);
					recog.base.match_token(KW_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(324);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (KW_MININCLUSIVE - 47)) | (1usize << (KW_MINEXCLUSIVE - 47)) | (1usize << (KW_MAXINCLUSIVE - 47)) | (1usize << (KW_MAXEXCLUSIVE - 47)) | (1usize << (KW_LENGTH - 47)) | (1usize << (KW_MINLENGTH - 47)) | (1usize << (KW_MAXLENGTH - 47)) | (1usize << (KW_TOTALDIGITS - 47)) | (1usize << (KW_FRACTIONDIGITS - 47)) | (1usize << (REGEXP - 47)))) != 0) {
						{
						{
						/*InvokeRule xsFacet*/
						recog.base.set_state(321);
						recog.xsFacet()?;

						}
						}
						recog.base.set_state(326);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 KW_IRI | KW_NONLITERAL | KW_BNODE 
				=> {
					let tmp = NodeConstraintNonLiteralContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule nonLiteralKind*/
					recog.base.set_state(327);
					recog.nonLiteralKind()?;

					recog.base.set_state(331);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (KW_LENGTH - 51)) | (1usize << (KW_MINLENGTH - 51)) | (1usize << (KW_MAXLENGTH - 51)) | (1usize << (REGEXP - 51)))) != 0) {
						{
						{
						/*InvokeRule stringFacet*/
						recog.base.set_state(328);
						recog.stringFacet()?;

						}
						}
						recog.base.set_state(333);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					let tmp = NodeConstraintDatatypeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule datatype*/
					recog.base.set_state(334);
					recog.datatype()?;

					recog.base.set_state(338);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (KW_MININCLUSIVE - 47)) | (1usize << (KW_MINEXCLUSIVE - 47)) | (1usize << (KW_MAXINCLUSIVE - 47)) | (1usize << (KW_MAXEXCLUSIVE - 47)) | (1usize << (KW_LENGTH - 47)) | (1usize << (KW_MINLENGTH - 47)) | (1usize << (KW_MAXLENGTH - 47)) | (1usize << (KW_TOTALDIGITS - 47)) | (1usize << (KW_FRACTIONDIGITS - 47)) | (1usize << (REGEXP - 47)))) != 0) {
						{
						{
						/*InvokeRule xsFacet*/
						recog.base.set_state(335);
						recog.xsFacet()?;

						}
						}
						recog.base.set_state(340);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 T__22 
				=> {
					let tmp = NodeConstraintValueSetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					/*InvokeRule valueSet*/
					recog.base.set_state(341);
					recog.valueSet()?;

					recog.base.set_state(345);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (KW_MININCLUSIVE - 47)) | (1usize << (KW_MINEXCLUSIVE - 47)) | (1usize << (KW_MAXINCLUSIVE - 47)) | (1usize << (KW_MAXEXCLUSIVE - 47)) | (1usize << (KW_LENGTH - 47)) | (1usize << (KW_MINLENGTH - 47)) | (1usize << (KW_MAXLENGTH - 47)) | (1usize << (KW_TOTALDIGITS - 47)) | (1usize << (KW_FRACTIONDIGITS - 47)) | (1usize << (REGEXP - 47)))) != 0) {
						{
						{
						/*InvokeRule xsFacet*/
						recog.base.set_state(342);
						recog.xsFacet()?;

						}
						}
						recog.base.set_state(347);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 KW_MININCLUSIVE | KW_MINEXCLUSIVE | KW_MAXINCLUSIVE | KW_MAXEXCLUSIVE |
			 KW_TOTALDIGITS | KW_FRACTIONDIGITS 
				=> {
					let tmp = NodeConstraintNumericFacetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(349); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule numericFacet*/
						recog.base.set_state(348);
						recog.numericFacet()?;

						}
						}
						recog.base.set_state(351); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (KW_MININCLUSIVE - 47)) | (1usize << (KW_MINEXCLUSIVE - 47)) | (1usize << (KW_MAXINCLUSIVE - 47)) | (1usize << (KW_MAXEXCLUSIVE - 47)) | (1usize << (KW_TOTALDIGITS - 47)) | (1usize << (KW_FRACTIONDIGITS - 47)))) != 0)) {break}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- litNodeConstraint ----------------
pub type LitNodeConstraintContextAll<'input> = LitNodeConstraintContext<'input>;


pub type LitNodeConstraintContext<'input> = BaseParserRuleContext<'input,LitNodeConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct LitNodeConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for LitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LitNodeConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_litNodeConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_litNodeConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LitNodeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_litNodeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for LitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_litNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_litNodeConstraint }
}
antlr_rust::tid!{LitNodeConstraintContextExt<'a>}

impl<'input> LitNodeConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LitNodeConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LitNodeConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<LitNodeConstraintContextExt<'input>>{

fn inlineLitNodeConstraint(&self) -> Option<Rc<InlineLitNodeConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn semanticAction_all(&self) ->  Vec<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn semanticAction(&self, i: usize) -> Option<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LitNodeConstraintContextAttrs<'input> for LitNodeConstraintContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn litNodeConstraint(&mut self,)
	-> Result<Rc<LitNodeConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LitNodeConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_litNodeConstraint);
        let mut _localctx: Rc<LitNodeConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inlineLitNodeConstraint*/
			recog.base.set_state(355);
			recog.inlineLitNodeConstraint()?;

			recog.base.set_state(359);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(356);
				recog.annotation()?;

				}
				}
				recog.base.set_state(361);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(365);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__26 {
				{
				{
				/*InvokeRule semanticAction*/
				recog.base.set_state(362);
				recog.semanticAction()?;

				}
				}
				recog.base.set_state(367);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineNonLitNodeConstraint ----------------
#[derive(Debug)]
pub enum InlineNonLitNodeConstraintContextAll<'input>{
	LitNodeConstraintStringFacetContext(LitNodeConstraintStringFacetContext<'input>),
	LitNodeConstraintLiteralContext(LitNodeConstraintLiteralContext<'input>),
Error(InlineNonLitNodeConstraintContext<'input>)
}
antlr_rust::tid!{InlineNonLitNodeConstraintContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for InlineNonLitNodeConstraintContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for InlineNonLitNodeConstraintContextAll<'input>{}

impl<'input> Deref for InlineNonLitNodeConstraintContextAll<'input>{
	type Target = dyn InlineNonLitNodeConstraintContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use InlineNonLitNodeConstraintContextAll::*;
		match self{
			LitNodeConstraintStringFacetContext(inner) => inner,
			LitNodeConstraintLiteralContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineNonLitNodeConstraintContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineNonLitNodeConstraintContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type InlineNonLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,InlineNonLitNodeConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct InlineNonLitNodeConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineNonLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineNonLitNodeConstraintContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineNonLitNodeConstraintContext<'input>{
}

impl<'input> CustomRuleContext<'input> for InlineNonLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineNonLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineNonLitNodeConstraint }
}
antlr_rust::tid!{InlineNonLitNodeConstraintContextExt<'a>}

impl<'input> InlineNonLitNodeConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineNonLitNodeConstraintContextAll<'input>> {
		Rc::new(
		InlineNonLitNodeConstraintContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineNonLitNodeConstraintContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait InlineNonLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineNonLitNodeConstraintContextExt<'input>>{


}

impl<'input> InlineNonLitNodeConstraintContextAttrs<'input> for InlineNonLitNodeConstraintContext<'input>{}

pub type LitNodeConstraintStringFacetContext<'input> = BaseParserRuleContext<'input,LitNodeConstraintStringFacetContextExt<'input>>;

pub trait LitNodeConstraintStringFacetContextAttrs<'input>: ShExDocParserContext<'input>{
	fn stringFacet_all(&self) ->  Vec<Rc<StringFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stringFacet(&self, i: usize) -> Option<Rc<StringFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LitNodeConstraintStringFacetContextAttrs<'input> for LitNodeConstraintStringFacetContext<'input>{}

pub struct LitNodeConstraintStringFacetContextExt<'input>{
	base:InlineNonLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LitNodeConstraintStringFacetContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for LitNodeConstraintStringFacetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LitNodeConstraintStringFacetContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_litNodeConstraintStringFacet(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_litNodeConstraintStringFacet(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LitNodeConstraintStringFacetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_litNodeConstraintStringFacet(self);
	}
}

impl<'input> CustomRuleContext<'input> for LitNodeConstraintStringFacetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineNonLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineNonLitNodeConstraint }
}

impl<'input> Borrow<InlineNonLitNodeConstraintContextExt<'input>> for LitNodeConstraintStringFacetContext<'input>{
	fn borrow(&self) -> &InlineNonLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineNonLitNodeConstraintContextExt<'input>> for LitNodeConstraintStringFacetContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineNonLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineNonLitNodeConstraintContextAttrs<'input> for LitNodeConstraintStringFacetContext<'input> {}

impl<'input> LitNodeConstraintStringFacetContextExt<'input>{
	fn new(ctx: &dyn InlineNonLitNodeConstraintContextAttrs<'input>) -> Rc<InlineNonLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineNonLitNodeConstraintContextAll::LitNodeConstraintStringFacetContext(
				BaseParserRuleContext::copy_from(ctx,LitNodeConstraintStringFacetContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LitNodeConstraintLiteralContext<'input> = BaseParserRuleContext<'input,LitNodeConstraintLiteralContextExt<'input>>;

pub trait LitNodeConstraintLiteralContextAttrs<'input>: ShExDocParserContext<'input>{
	fn nonLiteralKind(&self) -> Option<Rc<NonLiteralKindContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn stringFacet_all(&self) ->  Vec<Rc<StringFacetContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stringFacet(&self, i: usize) -> Option<Rc<StringFacetContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LitNodeConstraintLiteralContextAttrs<'input> for LitNodeConstraintLiteralContext<'input>{}

pub struct LitNodeConstraintLiteralContextExt<'input>{
	base:InlineNonLitNodeConstraintContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LitNodeConstraintLiteralContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for LitNodeConstraintLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LitNodeConstraintLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_litNodeConstraintLiteral(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_litNodeConstraintLiteral(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LitNodeConstraintLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_litNodeConstraintLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for LitNodeConstraintLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineNonLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineNonLitNodeConstraint }
}

impl<'input> Borrow<InlineNonLitNodeConstraintContextExt<'input>> for LitNodeConstraintLiteralContext<'input>{
	fn borrow(&self) -> &InlineNonLitNodeConstraintContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineNonLitNodeConstraintContextExt<'input>> for LitNodeConstraintLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineNonLitNodeConstraintContextExt<'input> { &mut self.base }
}

impl<'input> InlineNonLitNodeConstraintContextAttrs<'input> for LitNodeConstraintLiteralContext<'input> {}

impl<'input> LitNodeConstraintLiteralContextExt<'input>{
	fn new(ctx: &dyn InlineNonLitNodeConstraintContextAttrs<'input>) -> Rc<InlineNonLitNodeConstraintContextAll<'input>>  {
		Rc::new(
			InlineNonLitNodeConstraintContextAll::LitNodeConstraintLiteralContext(
				BaseParserRuleContext::copy_from(ctx,LitNodeConstraintLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineNonLitNodeConstraint(&mut self,)
	-> Result<Rc<InlineNonLitNodeConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineNonLitNodeConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_inlineNonLitNodeConstraint);
        let mut _localctx: Rc<InlineNonLitNodeConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(380);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_IRI | KW_NONLITERAL | KW_BNODE 
				=> {
					let tmp = LitNodeConstraintLiteralContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule nonLiteralKind*/
					recog.base.set_state(368);
					recog.nonLiteralKind()?;

					recog.base.set_state(372);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (KW_LENGTH - 51)) | (1usize << (KW_MINLENGTH - 51)) | (1usize << (KW_MAXLENGTH - 51)) | (1usize << (REGEXP - 51)))) != 0) {
						{
						{
						/*InvokeRule stringFacet*/
						recog.base.set_state(369);
						recog.stringFacet()?;

						}
						}
						recog.base.set_state(374);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 KW_LENGTH | KW_MINLENGTH | KW_MAXLENGTH | REGEXP 
				=> {
					let tmp = LitNodeConstraintStringFacetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(376); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule stringFacet*/
						recog.base.set_state(375);
						recog.stringFacet()?;

						}
						}
						recog.base.set_state(378); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (KW_LENGTH - 51)) | (1usize << (KW_MINLENGTH - 51)) | (1usize << (KW_MAXLENGTH - 51)) | (1usize << (REGEXP - 51)))) != 0)) {break}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nonLitNodeConstraint ----------------
pub type NonLitNodeConstraintContextAll<'input> = NonLitNodeConstraintContext<'input>;


pub type NonLitNodeConstraintContext<'input> = BaseParserRuleContext<'input,NonLitNodeConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct NonLitNodeConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NonLitNodeConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NonLitNodeConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nonLitNodeConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_nonLitNodeConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NonLitNodeConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nonLitNodeConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for NonLitNodeConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nonLitNodeConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nonLitNodeConstraint }
}
antlr_rust::tid!{NonLitNodeConstraintContextExt<'a>}

impl<'input> NonLitNodeConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NonLitNodeConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NonLitNodeConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NonLitNodeConstraintContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NonLitNodeConstraintContextExt<'input>>{

fn inlineNonLitNodeConstraint(&self) -> Option<Rc<InlineNonLitNodeConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn semanticAction_all(&self) ->  Vec<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn semanticAction(&self, i: usize) -> Option<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> NonLitNodeConstraintContextAttrs<'input> for NonLitNodeConstraintContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nonLitNodeConstraint(&mut self,)
	-> Result<Rc<NonLitNodeConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NonLitNodeConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_nonLitNodeConstraint);
        let mut _localctx: Rc<NonLitNodeConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inlineNonLitNodeConstraint*/
			recog.base.set_state(382);
			recog.inlineNonLitNodeConstraint()?;

			recog.base.set_state(386);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(383);
				recog.annotation()?;

				}
				}
				recog.base.set_state(388);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(392);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__26 {
				{
				{
				/*InvokeRule semanticAction*/
				recog.base.set_state(389);
				recog.semanticAction()?;

				}
				}
				recog.base.set_state(394);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nonLiteralKind ----------------
pub type NonLiteralKindContextAll<'input> = NonLiteralKindContext<'input>;


pub type NonLiteralKindContext<'input> = BaseParserRuleContext<'input,NonLiteralKindContextExt<'input>>;

#[derive(Clone)]
pub struct NonLiteralKindContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NonLiteralKindContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NonLiteralKindContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nonLiteralKind(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_nonLiteralKind(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NonLiteralKindContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_nonLiteralKind(self);
	}
}

impl<'input> CustomRuleContext<'input> for NonLiteralKindContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nonLiteralKind }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nonLiteralKind }
}
antlr_rust::tid!{NonLiteralKindContextExt<'a>}

impl<'input> NonLiteralKindContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NonLiteralKindContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NonLiteralKindContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NonLiteralKindContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NonLiteralKindContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_IRI
/// Returns `None` if there is no child corresponding to token KW_IRI
fn KW_IRI(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_IRI, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_BNODE
/// Returns `None` if there is no child corresponding to token KW_BNODE
fn KW_BNODE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_BNODE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_NONLITERAL
/// Returns `None` if there is no child corresponding to token KW_NONLITERAL
fn KW_NONLITERAL(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_NONLITERAL, 0)
}

}

impl<'input> NonLiteralKindContextAttrs<'input> for NonLiteralKindContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nonLiteralKind(&mut self,)
	-> Result<Rc<NonLiteralKindContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NonLiteralKindContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_nonLiteralKind);
        let mut _localctx: Rc<NonLiteralKindContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(395);
			_la = recog.base.input.la(1);
			if { !(((((_la - 42)) & !0x3f) == 0 && ((1usize << (_la - 42)) & ((1usize << (KW_IRI - 42)) | (1usize << (KW_NONLITERAL - 42)) | (1usize << (KW_BNODE - 42)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- xsFacet ----------------
pub type XsFacetContextAll<'input> = XsFacetContext<'input>;


pub type XsFacetContext<'input> = BaseParserRuleContext<'input,XsFacetContextExt<'input>>;

#[derive(Clone)]
pub struct XsFacetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for XsFacetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for XsFacetContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_xsFacet(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_xsFacet(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for XsFacetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_xsFacet(self);
	}
}

impl<'input> CustomRuleContext<'input> for XsFacetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_xsFacet }
	//fn type_rule_index() -> usize where Self: Sized { RULE_xsFacet }
}
antlr_rust::tid!{XsFacetContextExt<'a>}

impl<'input> XsFacetContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<XsFacetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,XsFacetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait XsFacetContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<XsFacetContextExt<'input>>{

fn stringFacet(&self) -> Option<Rc<StringFacetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn numericFacet(&self) -> Option<Rc<NumericFacetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> XsFacetContextAttrs<'input> for XsFacetContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn xsFacet(&mut self,)
	-> Result<Rc<XsFacetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = XsFacetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_xsFacet);
        let mut _localctx: Rc<XsFacetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(399);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_LENGTH | KW_MINLENGTH | KW_MAXLENGTH | REGEXP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule stringFacet*/
					recog.base.set_state(397);
					recog.stringFacet()?;

					}
				}

			 KW_MININCLUSIVE | KW_MINEXCLUSIVE | KW_MAXINCLUSIVE | KW_MAXEXCLUSIVE |
			 KW_TOTALDIGITS | KW_FRACTIONDIGITS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numericFacet*/
					recog.base.set_state(398);
					recog.numericFacet()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stringFacet ----------------
pub type StringFacetContextAll<'input> = StringFacetContext<'input>;


pub type StringFacetContext<'input> = BaseParserRuleContext<'input,StringFacetContextExt<'input>>;

#[derive(Clone)]
pub struct StringFacetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for StringFacetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StringFacetContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stringFacet(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_stringFacet(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StringFacetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_stringFacet(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringFacetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringFacet }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringFacet }
}
antlr_rust::tid!{StringFacetContextExt<'a>}

impl<'input> StringFacetContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringFacetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringFacetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringFacetContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<StringFacetContextExt<'input>>{

fn stringLength(&self) -> Option<Rc<StringLengthContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token REGEXP
/// Returns `None` if there is no child corresponding to token REGEXP
fn REGEXP(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(REGEXP, 0)
}
/// Retrieves first TerminalNode corresponding to token REGEXP_FLAGS
/// Returns `None` if there is no child corresponding to token REGEXP_FLAGS
fn REGEXP_FLAGS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(REGEXP_FLAGS, 0)
}

}

impl<'input> StringFacetContextAttrs<'input> for StringFacetContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringFacet(&mut self,)
	-> Result<Rc<StringFacetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringFacetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_stringFacet);
        let mut _localctx: Rc<StringFacetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(408);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_LENGTH | KW_MINLENGTH | KW_MAXLENGTH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule stringLength*/
					recog.base.set_state(401);
					recog.stringLength()?;

					recog.base.set_state(402);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}

			 REGEXP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(404);
					recog.base.match_token(REGEXP,&mut recog.err_handler)?;

					recog.base.set_state(406);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==REGEXP_FLAGS {
						{
						recog.base.set_state(405);
						recog.base.match_token(REGEXP_FLAGS,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stringLength ----------------
pub type StringLengthContextAll<'input> = StringLengthContext<'input>;


pub type StringLengthContext<'input> = BaseParserRuleContext<'input,StringLengthContextExt<'input>>;

#[derive(Clone)]
pub struct StringLengthContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for StringLengthContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StringLengthContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stringLength(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_stringLength(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StringLengthContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_stringLength(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringLengthContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringLength }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringLength }
}
antlr_rust::tid!{StringLengthContextExt<'a>}

impl<'input> StringLengthContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringLengthContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringLengthContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringLengthContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<StringLengthContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_LENGTH
/// Returns `None` if there is no child corresponding to token KW_LENGTH
fn KW_LENGTH(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_LENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_MINLENGTH
/// Returns `None` if there is no child corresponding to token KW_MINLENGTH
fn KW_MINLENGTH(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_MINLENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_MAXLENGTH
/// Returns `None` if there is no child corresponding to token KW_MAXLENGTH
fn KW_MAXLENGTH(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_MAXLENGTH, 0)
}

}

impl<'input> StringLengthContextAttrs<'input> for StringLengthContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringLength(&mut self,)
	-> Result<Rc<StringLengthContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringLengthContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_stringLength);
        let mut _localctx: Rc<StringLengthContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(410);
			_la = recog.base.input.la(1);
			if { !(((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (KW_LENGTH - 51)) | (1usize << (KW_MINLENGTH - 51)) | (1usize << (KW_MAXLENGTH - 51)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- numericFacet ----------------
pub type NumericFacetContextAll<'input> = NumericFacetContext<'input>;


pub type NumericFacetContext<'input> = BaseParserRuleContext<'input,NumericFacetContextExt<'input>>;

#[derive(Clone)]
pub struct NumericFacetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NumericFacetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NumericFacetContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numericFacet(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_numericFacet(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NumericFacetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_numericFacet(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericFacetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericFacet }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericFacet }
}
antlr_rust::tid!{NumericFacetContextExt<'a>}

impl<'input> NumericFacetContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumericFacetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericFacetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumericFacetContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NumericFacetContextExt<'input>>{

fn numericRange(&self) -> Option<Rc<NumericRangeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rawNumeric(&self) -> Option<Rc<RawNumericContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn numericLength(&self) -> Option<Rc<NumericLengthContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}

}

impl<'input> NumericFacetContextAttrs<'input> for NumericFacetContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numericFacet(&mut self,)
	-> Result<Rc<NumericFacetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericFacetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_numericFacet);
        let mut _localctx: Rc<NumericFacetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(418);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_MININCLUSIVE | KW_MINEXCLUSIVE | KW_MAXINCLUSIVE | KW_MAXEXCLUSIVE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule numericRange*/
					recog.base.set_state(412);
					recog.numericRange()?;

					/*InvokeRule rawNumeric*/
					recog.base.set_state(413);
					recog.rawNumeric()?;

					}
				}

			 KW_TOTALDIGITS | KW_FRACTIONDIGITS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numericLength*/
					recog.base.set_state(415);
					recog.numericLength()?;

					recog.base.set_state(416);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- numericRange ----------------
pub type NumericRangeContextAll<'input> = NumericRangeContext<'input>;


pub type NumericRangeContext<'input> = BaseParserRuleContext<'input,NumericRangeContextExt<'input>>;

#[derive(Clone)]
pub struct NumericRangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NumericRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NumericRangeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numericRange(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_numericRange(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NumericRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_numericRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericRange }
}
antlr_rust::tid!{NumericRangeContextExt<'a>}

impl<'input> NumericRangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumericRangeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericRangeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumericRangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NumericRangeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_MININCLUSIVE
/// Returns `None` if there is no child corresponding to token KW_MININCLUSIVE
fn KW_MININCLUSIVE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_MININCLUSIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_MINEXCLUSIVE
/// Returns `None` if there is no child corresponding to token KW_MINEXCLUSIVE
fn KW_MINEXCLUSIVE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_MINEXCLUSIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_MAXINCLUSIVE
/// Returns `None` if there is no child corresponding to token KW_MAXINCLUSIVE
fn KW_MAXINCLUSIVE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_MAXINCLUSIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_MAXEXCLUSIVE
/// Returns `None` if there is no child corresponding to token KW_MAXEXCLUSIVE
fn KW_MAXEXCLUSIVE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_MAXEXCLUSIVE, 0)
}

}

impl<'input> NumericRangeContextAttrs<'input> for NumericRangeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numericRange(&mut self,)
	-> Result<Rc<NumericRangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericRangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_numericRange);
        let mut _localctx: Rc<NumericRangeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(420);
			_la = recog.base.input.la(1);
			if { !(((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (KW_MININCLUSIVE - 47)) | (1usize << (KW_MINEXCLUSIVE - 47)) | (1usize << (KW_MAXINCLUSIVE - 47)) | (1usize << (KW_MAXEXCLUSIVE - 47)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- numericLength ----------------
pub type NumericLengthContextAll<'input> = NumericLengthContext<'input>;


pub type NumericLengthContext<'input> = BaseParserRuleContext<'input,NumericLengthContextExt<'input>>;

#[derive(Clone)]
pub struct NumericLengthContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NumericLengthContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NumericLengthContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numericLength(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_numericLength(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NumericLengthContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_numericLength(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericLengthContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericLength }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericLength }
}
antlr_rust::tid!{NumericLengthContextExt<'a>}

impl<'input> NumericLengthContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumericLengthContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericLengthContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumericLengthContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NumericLengthContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_TOTALDIGITS
/// Returns `None` if there is no child corresponding to token KW_TOTALDIGITS
fn KW_TOTALDIGITS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_TOTALDIGITS, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_FRACTIONDIGITS
/// Returns `None` if there is no child corresponding to token KW_FRACTIONDIGITS
fn KW_FRACTIONDIGITS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_FRACTIONDIGITS, 0)
}

}

impl<'input> NumericLengthContextAttrs<'input> for NumericLengthContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numericLength(&mut self,)
	-> Result<Rc<NumericLengthContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericLengthContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_numericLength);
        let mut _localctx: Rc<NumericLengthContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(422);
			_la = recog.base.input.la(1);
			if { !(_la==KW_TOTALDIGITS || _la==KW_FRACTIONDIGITS) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rawNumeric ----------------
pub type RawNumericContextAll<'input> = RawNumericContext<'input>;


pub type RawNumericContext<'input> = BaseParserRuleContext<'input,RawNumericContextExt<'input>>;

#[derive(Clone)]
pub struct RawNumericContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for RawNumericContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RawNumericContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rawNumeric(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_rawNumeric(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RawNumericContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_rawNumeric(self);
	}
}

impl<'input> CustomRuleContext<'input> for RawNumericContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rawNumeric }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rawNumeric }
}
antlr_rust::tid!{RawNumericContextExt<'a>}

impl<'input> RawNumericContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RawNumericContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RawNumericContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RawNumericContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<RawNumericContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL
/// Returns `None` if there is no child corresponding to token DECIMAL
fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE
/// Returns `None` if there is no child corresponding to token DOUBLE
fn DOUBLE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE, 0)
}

}

impl<'input> RawNumericContextAttrs<'input> for RawNumericContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rawNumeric(&mut self,)
	-> Result<Rc<RawNumericContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RawNumericContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_rawNumeric);
        let mut _localctx: Rc<RawNumericContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(424);
			_la = recog.base.input.la(1);
			if { !(((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (INTEGER - 71)) | (1usize << (DECIMAL - 71)) | (1usize << (DOUBLE - 71)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeDefinition ----------------
pub type ShapeDefinitionContextAll<'input> = ShapeDefinitionContext<'input>;


pub type ShapeDefinitionContext<'input> = BaseParserRuleContext<'input,ShapeDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeDefinition }
}
antlr_rust::tid!{ShapeDefinitionContextExt<'a>}

impl<'input> ShapeDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeDefinitionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeDefinitionContextExt<'input>>{

fn inlineShapeDefinition(&self) -> Option<Rc<InlineShapeDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn semanticAction_all(&self) ->  Vec<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn semanticAction(&self, i: usize) -> Option<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ShapeDefinitionContextAttrs<'input> for ShapeDefinitionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeDefinition(&mut self,)
	-> Result<Rc<ShapeDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_shapeDefinition);
        let mut _localctx: Rc<ShapeDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inlineShapeDefinition*/
			recog.base.set_state(426);
			recog.inlineShapeDefinition()?;

			recog.base.set_state(430);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(427);
				recog.annotation()?;

				}
				}
				recog.base.set_state(432);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(436);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__26 {
				{
				{
				/*InvokeRule semanticAction*/
				recog.base.set_state(433);
				recog.semanticAction()?;

				}
				}
				recog.base.set_state(438);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineShapeDefinition ----------------
pub type InlineShapeDefinitionContextAll<'input> = InlineShapeDefinitionContext<'input>;


pub type InlineShapeDefinitionContext<'input> = BaseParserRuleContext<'input,InlineShapeDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct InlineShapeDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for InlineShapeDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for InlineShapeDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inlineShapeDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_inlineShapeDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for InlineShapeDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_inlineShapeDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineShapeDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineShapeDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineShapeDefinition }
}
antlr_rust::tid!{InlineShapeDefinitionContextExt<'a>}

impl<'input> InlineShapeDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineShapeDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineShapeDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineShapeDefinitionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<InlineShapeDefinitionContextExt<'input>>{

fn qualifier_all(&self) ->  Vec<Rc<QualifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn qualifier(&self, i: usize) -> Option<Rc<QualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn tripleExpression(&self) -> Option<Rc<TripleExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InlineShapeDefinitionContextAttrs<'input> for InlineShapeDefinitionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineShapeDefinition(&mut self,)
	-> Result<Rc<InlineShapeDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineShapeDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_inlineShapeDefinition);
        let mut _localctx: Rc<InlineShapeDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(442);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 21)) & !0x3f) == 0 && ((1usize << (_la - 21)) & ((1usize << (T__20 - 21)) | (1usize << (T__24 - 21)) | (1usize << (KW_EXTENDS - 21)) | (1usize << (KW_RESTRICTS - 21)) | (1usize << (KW_CLOSED - 21)) | (1usize << (KW_EXTRA - 21)))) != 0) {
				{
				{
				/*InvokeRule qualifier*/
				recog.base.set_state(439);
				recog.qualifier()?;

				}
				}
				recog.base.set_state(444);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(445);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			recog.base.set_state(447);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__1) | (1usize << T__2) | (1usize << T__10) | (1usize << T__21) | (1usize << T__24))) != 0) || ((((_la - 57)) & !0x3f) == 0 && ((1usize << (_la - 57)) & ((1usize << (KW_TRUE - 57)) | (1usize << (KW_FALSE - 57)) | (1usize << (RDF_TYPE - 57)) | (1usize << (IRIREF - 57)) | (1usize << (PNAME_NS - 57)) | (1usize << (PNAME_LN - 57)) | (1usize << (BLANK_NODE_LABEL - 57)) | (1usize << (INTEGER - 57)) | (1usize << (DECIMAL - 57)) | (1usize << (DOUBLE - 57)) | (1usize << (STRING_LITERAL1 - 57)) | (1usize << (STRING_LITERAL2 - 57)) | (1usize << (STRING_LITERAL_LONG1 - 57)) | (1usize << (STRING_LITERAL_LONG2 - 57)))) != 0) {
				{
				/*InvokeRule tripleExpression*/
				recog.base.set_state(446);
				recog.tripleExpression()?;

				}
			}

			recog.base.set_state(449);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qualifier ----------------
pub type QualifierContextAll<'input> = QualifierContext<'input>;


pub type QualifierContext<'input> = BaseParserRuleContext<'input,QualifierContextExt<'input>>;

#[derive(Clone)]
pub struct QualifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for QualifierContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for QualifierContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifier(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_qualifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for QualifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_qualifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifier }
}
antlr_rust::tid!{QualifierContextExt<'a>}

impl<'input> QualifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifierContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<QualifierContextExt<'input>>{

fn extension(&self) -> Option<Rc<ExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn restriction(&self) -> Option<Rc<RestrictionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn extraPropertySet(&self) -> Option<Rc<ExtraPropertySetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KW_CLOSED
/// Returns `None` if there is no child corresponding to token KW_CLOSED
fn KW_CLOSED(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_CLOSED, 0)
}

}

impl<'input> QualifierContextAttrs<'input> for QualifierContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifier(&mut self,)
	-> Result<Rc<QualifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_qualifier);
        let mut _localctx: Rc<QualifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(455);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__24 | KW_EXTENDS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule extension*/
					recog.base.set_state(451);
					recog.extension()?;

					}
				}

			 T__20 | KW_RESTRICTS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule restriction*/
					recog.base.set_state(452);
					recog.restriction()?;

					}
				}

			 KW_EXTRA 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule extraPropertySet*/
					recog.base.set_state(453);
					recog.extraPropertySet()?;

					}
				}

			 KW_CLOSED 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(454);
					recog.base.match_token(KW_CLOSED,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- extraPropertySet ----------------
pub type ExtraPropertySetContextAll<'input> = ExtraPropertySetContext<'input>;


pub type ExtraPropertySetContext<'input> = BaseParserRuleContext<'input,ExtraPropertySetContextExt<'input>>;

#[derive(Clone)]
pub struct ExtraPropertySetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ExtraPropertySetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ExtraPropertySetContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_extraPropertySet(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_extraPropertySet(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ExtraPropertySetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_extraPropertySet(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExtraPropertySetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_extraPropertySet }
	//fn type_rule_index() -> usize where Self: Sized { RULE_extraPropertySet }
}
antlr_rust::tid!{ExtraPropertySetContextExt<'a>}

impl<'input> ExtraPropertySetContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExtraPropertySetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExtraPropertySetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExtraPropertySetContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ExtraPropertySetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_EXTRA
/// Returns `None` if there is no child corresponding to token KW_EXTRA
fn KW_EXTRA(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_EXTRA, 0)
}
fn predicate_all(&self) ->  Vec<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn predicate(&self, i: usize) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExtraPropertySetContextAttrs<'input> for ExtraPropertySetContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn extraPropertySet(&mut self,)
	-> Result<Rc<ExtraPropertySetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExtraPropertySetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_extraPropertySet);
        let mut _localctx: Rc<ExtraPropertySetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(457);
			recog.base.match_token(KW_EXTRA,&mut recog.err_handler)?;

			recog.base.set_state(459); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule predicate*/
				recog.base.set_state(458);
				recog.predicate()?;

				}
				}
				recog.base.set_state(461); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & ((1usize << (RDF_TYPE - 61)) | (1usize << (IRIREF - 61)) | (1usize << (PNAME_NS - 61)) | (1usize << (PNAME_LN - 61)))) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tripleExpression ----------------
pub type TripleExpressionContextAll<'input> = TripleExpressionContext<'input>;


pub type TripleExpressionContext<'input> = BaseParserRuleContext<'input,TripleExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct TripleExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for TripleExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for TripleExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tripleExpression(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_tripleExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for TripleExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_tripleExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for TripleExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tripleExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tripleExpression }
}
antlr_rust::tid!{TripleExpressionContextExt<'a>}

impl<'input> TripleExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TripleExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TripleExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TripleExpressionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<TripleExpressionContextExt<'input>>{

fn oneOfTripleExpr(&self) -> Option<Rc<OneOfTripleExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TripleExpressionContextAttrs<'input> for TripleExpressionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tripleExpression(&mut self,)
	-> Result<Rc<TripleExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TripleExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_tripleExpression);
        let mut _localctx: Rc<TripleExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule oneOfTripleExpr*/
			recog.base.set_state(463);
			recog.oneOfTripleExpr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- oneOfTripleExpr ----------------
pub type OneOfTripleExprContextAll<'input> = OneOfTripleExprContext<'input>;


pub type OneOfTripleExprContext<'input> = BaseParserRuleContext<'input,OneOfTripleExprContextExt<'input>>;

#[derive(Clone)]
pub struct OneOfTripleExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for OneOfTripleExprContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for OneOfTripleExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_oneOfTripleExpr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_oneOfTripleExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for OneOfTripleExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_oneOfTripleExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for OneOfTripleExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oneOfTripleExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oneOfTripleExpr }
}
antlr_rust::tid!{OneOfTripleExprContextExt<'a>}

impl<'input> OneOfTripleExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OneOfTripleExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OneOfTripleExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OneOfTripleExprContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<OneOfTripleExprContextExt<'input>>{

fn groupTripleExpr(&self) -> Option<Rc<GroupTripleExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn multiElementOneOf(&self) -> Option<Rc<MultiElementOneOfContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OneOfTripleExprContextAttrs<'input> for OneOfTripleExprContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn oneOfTripleExpr(&mut self,)
	-> Result<Rc<OneOfTripleExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OneOfTripleExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_oneOfTripleExpr);
        let mut _localctx: Rc<OneOfTripleExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(467);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule groupTripleExpr*/
					recog.base.set_state(465);
					recog.groupTripleExpr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule multiElementOneOf*/
					recog.base.set_state(466);
					recog.multiElementOneOf()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiElementOneOf ----------------
pub type MultiElementOneOfContextAll<'input> = MultiElementOneOfContext<'input>;


pub type MultiElementOneOfContext<'input> = BaseParserRuleContext<'input,MultiElementOneOfContextExt<'input>>;

#[derive(Clone)]
pub struct MultiElementOneOfContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for MultiElementOneOfContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for MultiElementOneOfContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multiElementOneOf(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_multiElementOneOf(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for MultiElementOneOfContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_multiElementOneOf(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiElementOneOfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiElementOneOf }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiElementOneOf }
}
antlr_rust::tid!{MultiElementOneOfContextExt<'a>}

impl<'input> MultiElementOneOfContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiElementOneOfContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiElementOneOfContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiElementOneOfContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<MultiElementOneOfContextExt<'input>>{

fn groupTripleExpr_all(&self) ->  Vec<Rc<GroupTripleExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn groupTripleExpr(&self, i: usize) -> Option<Rc<GroupTripleExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MultiElementOneOfContextAttrs<'input> for MultiElementOneOfContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiElementOneOf(&mut self,)
	-> Result<Rc<MultiElementOneOfContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiElementOneOfContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_multiElementOneOf);
        let mut _localctx: Rc<MultiElementOneOfContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule groupTripleExpr*/
			recog.base.set_state(469);
			recog.groupTripleExpr()?;

			recog.base.set_state(472); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(470);
				recog.base.match_token(T__8,&mut recog.err_handler)?;

				/*InvokeRule groupTripleExpr*/
				recog.base.set_state(471);
				recog.groupTripleExpr()?;

				}
				}
				recog.base.set_state(474); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__8) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- groupTripleExpr ----------------
pub type GroupTripleExprContextAll<'input> = GroupTripleExprContext<'input>;


pub type GroupTripleExprContext<'input> = BaseParserRuleContext<'input,GroupTripleExprContextExt<'input>>;

#[derive(Clone)]
pub struct GroupTripleExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for GroupTripleExprContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for GroupTripleExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_groupTripleExpr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_groupTripleExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for GroupTripleExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_groupTripleExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for GroupTripleExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_groupTripleExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_groupTripleExpr }
}
antlr_rust::tid!{GroupTripleExprContextExt<'a>}

impl<'input> GroupTripleExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GroupTripleExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GroupTripleExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GroupTripleExprContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<GroupTripleExprContextExt<'input>>{

fn singleElementGroup(&self) -> Option<Rc<SingleElementGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn multiElementGroup(&self) -> Option<Rc<MultiElementGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GroupTripleExprContextAttrs<'input> for GroupTripleExprContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn groupTripleExpr(&mut self,)
	-> Result<Rc<GroupTripleExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GroupTripleExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_groupTripleExpr);
        let mut _localctx: Rc<GroupTripleExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(478);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(50,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule singleElementGroup*/
					recog.base.set_state(476);
					recog.singleElementGroup()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule multiElementGroup*/
					recog.base.set_state(477);
					recog.multiElementGroup()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- singleElementGroup ----------------
pub type SingleElementGroupContextAll<'input> = SingleElementGroupContext<'input>;


pub type SingleElementGroupContext<'input> = BaseParserRuleContext<'input,SingleElementGroupContextExt<'input>>;

#[derive(Clone)]
pub struct SingleElementGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for SingleElementGroupContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for SingleElementGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_singleElementGroup(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_singleElementGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for SingleElementGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_singleElementGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for SingleElementGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_singleElementGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_singleElementGroup }
}
antlr_rust::tid!{SingleElementGroupContextExt<'a>}

impl<'input> SingleElementGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SingleElementGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SingleElementGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SingleElementGroupContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<SingleElementGroupContextExt<'input>>{

fn unaryTripleExpr(&self) -> Option<Rc<UnaryTripleExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SingleElementGroupContextAttrs<'input> for SingleElementGroupContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn singleElementGroup(&mut self,)
	-> Result<Rc<SingleElementGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SingleElementGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_singleElementGroup);
        let mut _localctx: Rc<SingleElementGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unaryTripleExpr*/
			recog.base.set_state(480);
			recog.unaryTripleExpr()?;

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__9 {
				{
				recog.base.set_state(481);
				recog.base.match_token(T__9,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiElementGroup ----------------
pub type MultiElementGroupContextAll<'input> = MultiElementGroupContext<'input>;


pub type MultiElementGroupContext<'input> = BaseParserRuleContext<'input,MultiElementGroupContextExt<'input>>;

#[derive(Clone)]
pub struct MultiElementGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for MultiElementGroupContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for MultiElementGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multiElementGroup(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_multiElementGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for MultiElementGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_multiElementGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiElementGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiElementGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiElementGroup }
}
antlr_rust::tid!{MultiElementGroupContextExt<'a>}

impl<'input> MultiElementGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiElementGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiElementGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiElementGroupContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<MultiElementGroupContextExt<'input>>{

fn unaryTripleExpr_all(&self) ->  Vec<Rc<UnaryTripleExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unaryTripleExpr(&self, i: usize) -> Option<Rc<UnaryTripleExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MultiElementGroupContextAttrs<'input> for MultiElementGroupContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiElementGroup(&mut self,)
	-> Result<Rc<MultiElementGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiElementGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_multiElementGroup);
        let mut _localctx: Rc<MultiElementGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unaryTripleExpr*/
			recog.base.set_state(484);
			recog.unaryTripleExpr()?;

			recog.base.set_state(487); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(485);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule unaryTripleExpr*/
					recog.base.set_state(486);
					recog.unaryTripleExpr()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(489); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(52,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			recog.base.set_state(492);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__9 {
				{
				recog.base.set_state(491);
				recog.base.match_token(T__9,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryTripleExpr ----------------
pub type UnaryTripleExprContextAll<'input> = UnaryTripleExprContext<'input>;


pub type UnaryTripleExprContext<'input> = BaseParserRuleContext<'input,UnaryTripleExprContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryTripleExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for UnaryTripleExprContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for UnaryTripleExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unaryTripleExpr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_unaryTripleExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for UnaryTripleExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_unaryTripleExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryTripleExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryTripleExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryTripleExpr }
}
antlr_rust::tid!{UnaryTripleExprContextExt<'a>}

impl<'input> UnaryTripleExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryTripleExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryTripleExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryTripleExprContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<UnaryTripleExprContextExt<'input>>{

fn tripleConstraint(&self) -> Option<Rc<TripleConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bracketedTripleExpr(&self) -> Option<Rc<BracketedTripleExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tripleExprLabel(&self) -> Option<Rc<TripleExprLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn include(&self) -> Option<Rc<IncludeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnaryTripleExprContextAttrs<'input> for UnaryTripleExprContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryTripleExpr(&mut self,)
	-> Result<Rc<UnaryTripleExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryTripleExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_unaryTripleExpr);
        let mut _localctx: Rc<UnaryTripleExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(504);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(496);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__10 {
						{
						recog.base.set_state(494);
						recog.base.match_token(T__10,&mut recog.err_handler)?;

						/*InvokeRule tripleExprLabel*/
						recog.base.set_state(495);
						recog.tripleExprLabel()?;

						}
					}

					recog.base.set_state(500);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__1 | T__21 | RDF_TYPE | IRIREF | PNAME_NS | PNAME_LN 
						=> {
							{
							/*InvokeRule tripleConstraint*/
							recog.base.set_state(498);
							recog.tripleConstraint()?;

							}
						}

					 T__2 
						=> {
							{
							/*InvokeRule bracketedTripleExpr*/
							recog.base.set_state(499);
							recog.bracketedTripleExpr()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule include*/
					recog.base.set_state(502);
					recog.include()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expr*/
					recog.base.set_state(503);
					recog.expr_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bracketedTripleExpr ----------------
pub type BracketedTripleExprContextAll<'input> = BracketedTripleExprContext<'input>;


pub type BracketedTripleExprContext<'input> = BaseParserRuleContext<'input,BracketedTripleExprContextExt<'input>>;

#[derive(Clone)]
pub struct BracketedTripleExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for BracketedTripleExprContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BracketedTripleExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bracketedTripleExpr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_bracketedTripleExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BracketedTripleExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_bracketedTripleExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for BracketedTripleExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bracketedTripleExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bracketedTripleExpr }
}
antlr_rust::tid!{BracketedTripleExprContextExt<'a>}

impl<'input> BracketedTripleExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BracketedTripleExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BracketedTripleExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BracketedTripleExprContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<BracketedTripleExprContextExt<'input>>{

fn tripleExpression(&self) -> Option<Rc<TripleExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cardinality(&self) -> Option<Rc<CardinalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn semanticAction_all(&self) ->  Vec<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn semanticAction(&self, i: usize) -> Option<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BracketedTripleExprContextAttrs<'input> for BracketedTripleExprContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bracketedTripleExpr(&mut self,)
	-> Result<Rc<BracketedTripleExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BracketedTripleExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_bracketedTripleExpr);
        let mut _localctx: Rc<BracketedTripleExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(506);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			/*InvokeRule tripleExpression*/
			recog.base.set_state(507);
			recog.tripleExpression()?;

			recog.base.set_state(508);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			recog.base.set_state(510);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__11) | (1usize << T__12))) != 0) || _la==UNBOUNDED {
				{
				/*InvokeRule cardinality*/
				recog.base.set_state(509);
				recog.cardinality()?;

				}
			}

			recog.base.set_state(515);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(512);
				recog.annotation()?;

				}
				}
				recog.base.set_state(517);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(521);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__26 {
				{
				{
				/*InvokeRule semanticAction*/
				recog.base.set_state(518);
				recog.semanticAction()?;

				}
				}
				recog.base.set_state(523);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tripleConstraint ----------------
pub type TripleConstraintContextAll<'input> = TripleConstraintContext<'input>;


pub type TripleConstraintContext<'input> = BaseParserRuleContext<'input,TripleConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct TripleConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for TripleConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for TripleConstraintContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tripleConstraint(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_tripleConstraint(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for TripleConstraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_tripleConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for TripleConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tripleConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tripleConstraint }
}
antlr_rust::tid!{TripleConstraintContextExt<'a>}

impl<'input> TripleConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TripleConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TripleConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TripleConstraintContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<TripleConstraintContextExt<'input>>{

fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inlineShapeExpression(&self) -> Option<Rc<InlineShapeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn senseFlags(&self) -> Option<Rc<SenseFlagsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cardinality(&self) -> Option<Rc<CardinalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn semanticAction_all(&self) ->  Vec<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn semanticAction(&self, i: usize) -> Option<Rc<SemanticActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TripleConstraintContextAttrs<'input> for TripleConstraintContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tripleConstraint(&mut self,)
	-> Result<Rc<TripleConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TripleConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_tripleConstraint);
        let mut _localctx: Rc<TripleConstraintContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(525);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 || _la==T__21 {
				{
				/*InvokeRule senseFlags*/
				recog.base.set_state(524);
				recog.senseFlags()?;

				}
			}

			/*InvokeRule predicate*/
			recog.base.set_state(527);
			recog.predicate()?;

			/*InvokeRule inlineShapeExpression*/
			recog.base.set_state(528);
			recog.inlineShapeExpression()?;

			recog.base.set_state(530);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__11) | (1usize << T__12))) != 0) || _la==UNBOUNDED {
				{
				/*InvokeRule cardinality*/
				recog.base.set_state(529);
				recog.cardinality()?;

				}
			}

			recog.base.set_state(535);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(532);
				recog.annotation()?;

				}
				}
				recog.base.set_state(537);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(541);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__26 {
				{
				{
				/*InvokeRule semanticAction*/
				recog.base.set_state(538);
				recog.semanticAction()?;

				}
				}
				recog.base.set_state(543);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cardinality ----------------
#[derive(Debug)]
pub enum CardinalityContextAll<'input>{
	StarCardinalityContext(StarCardinalityContext<'input>),
	RepeatCardinalityContext(RepeatCardinalityContext<'input>),
	PlusCardinalityContext(PlusCardinalityContext<'input>),
	OptionalCardinalityContext(OptionalCardinalityContext<'input>),
Error(CardinalityContext<'input>)
}
antlr_rust::tid!{CardinalityContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for CardinalityContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for CardinalityContextAll<'input>{}

impl<'input> Deref for CardinalityContextAll<'input>{
	type Target = dyn CardinalityContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use CardinalityContextAll::*;
		match self{
			StarCardinalityContext(inner) => inner,
			RepeatCardinalityContext(inner) => inner,
			PlusCardinalityContext(inner) => inner,
			OptionalCardinalityContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for CardinalityContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for CardinalityContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type CardinalityContext<'input> = BaseParserRuleContext<'input,CardinalityContextExt<'input>>;

#[derive(Clone)]
pub struct CardinalityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for CardinalityContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for CardinalityContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for CardinalityContext<'input>{
}

impl<'input> CustomRuleContext<'input> for CardinalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cardinality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cardinality }
}
antlr_rust::tid!{CardinalityContextExt<'a>}

impl<'input> CardinalityContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CardinalityContextAll<'input>> {
		Rc::new(
		CardinalityContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CardinalityContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait CardinalityContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<CardinalityContextExt<'input>>{


}

impl<'input> CardinalityContextAttrs<'input> for CardinalityContext<'input>{}

pub type StarCardinalityContext<'input> = BaseParserRuleContext<'input,StarCardinalityContextExt<'input>>;

pub trait StarCardinalityContextAttrs<'input>: ShExDocParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UNBOUNDED
	/// Returns `None` if there is no child corresponding to token UNBOUNDED
	fn UNBOUNDED(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(UNBOUNDED, 0)
	}
}

impl<'input> StarCardinalityContextAttrs<'input> for StarCardinalityContext<'input>{}

pub struct StarCardinalityContextExt<'input>{
	base:CardinalityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StarCardinalityContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for StarCardinalityContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StarCardinalityContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_starCardinality(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_starCardinality(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StarCardinalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_starCardinality(self);
	}
}

impl<'input> CustomRuleContext<'input> for StarCardinalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cardinality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cardinality }
}

impl<'input> Borrow<CardinalityContextExt<'input>> for StarCardinalityContext<'input>{
	fn borrow(&self) -> &CardinalityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CardinalityContextExt<'input>> for StarCardinalityContext<'input>{
	fn borrow_mut(&mut self) -> &mut CardinalityContextExt<'input> { &mut self.base }
}

impl<'input> CardinalityContextAttrs<'input> for StarCardinalityContext<'input> {}

impl<'input> StarCardinalityContextExt<'input>{
	fn new(ctx: &dyn CardinalityContextAttrs<'input>) -> Rc<CardinalityContextAll<'input>>  {
		Rc::new(
			CardinalityContextAll::StarCardinalityContext(
				BaseParserRuleContext::copy_from(ctx,StarCardinalityContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RepeatCardinalityContext<'input> = BaseParserRuleContext<'input,RepeatCardinalityContextExt<'input>>;

pub trait RepeatCardinalityContextAttrs<'input>: ShExDocParserContext<'input>{
	fn repeatRange(&self) -> Option<Rc<RepeatRangeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RepeatCardinalityContextAttrs<'input> for RepeatCardinalityContext<'input>{}

pub struct RepeatCardinalityContextExt<'input>{
	base:CardinalityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RepeatCardinalityContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for RepeatCardinalityContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RepeatCardinalityContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_repeatCardinality(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_repeatCardinality(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RepeatCardinalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_repeatCardinality(self);
	}
}

impl<'input> CustomRuleContext<'input> for RepeatCardinalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cardinality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cardinality }
}

impl<'input> Borrow<CardinalityContextExt<'input>> for RepeatCardinalityContext<'input>{
	fn borrow(&self) -> &CardinalityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CardinalityContextExt<'input>> for RepeatCardinalityContext<'input>{
	fn borrow_mut(&mut self) -> &mut CardinalityContextExt<'input> { &mut self.base }
}

impl<'input> CardinalityContextAttrs<'input> for RepeatCardinalityContext<'input> {}

impl<'input> RepeatCardinalityContextExt<'input>{
	fn new(ctx: &dyn CardinalityContextAttrs<'input>) -> Rc<CardinalityContextAll<'input>>  {
		Rc::new(
			CardinalityContextAll::RepeatCardinalityContext(
				BaseParserRuleContext::copy_from(ctx,RepeatCardinalityContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PlusCardinalityContext<'input> = BaseParserRuleContext<'input,PlusCardinalityContextExt<'input>>;

pub trait PlusCardinalityContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> PlusCardinalityContextAttrs<'input> for PlusCardinalityContext<'input>{}

pub struct PlusCardinalityContextExt<'input>{
	base:CardinalityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PlusCardinalityContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for PlusCardinalityContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for PlusCardinalityContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_plusCardinality(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_plusCardinality(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for PlusCardinalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_plusCardinality(self);
	}
}

impl<'input> CustomRuleContext<'input> for PlusCardinalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cardinality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cardinality }
}

impl<'input> Borrow<CardinalityContextExt<'input>> for PlusCardinalityContext<'input>{
	fn borrow(&self) -> &CardinalityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CardinalityContextExt<'input>> for PlusCardinalityContext<'input>{
	fn borrow_mut(&mut self) -> &mut CardinalityContextExt<'input> { &mut self.base }
}

impl<'input> CardinalityContextAttrs<'input> for PlusCardinalityContext<'input> {}

impl<'input> PlusCardinalityContextExt<'input>{
	fn new(ctx: &dyn CardinalityContextAttrs<'input>) -> Rc<CardinalityContextAll<'input>>  {
		Rc::new(
			CardinalityContextAll::PlusCardinalityContext(
				BaseParserRuleContext::copy_from(ctx,PlusCardinalityContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type OptionalCardinalityContext<'input> = BaseParserRuleContext<'input,OptionalCardinalityContextExt<'input>>;

pub trait OptionalCardinalityContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> OptionalCardinalityContextAttrs<'input> for OptionalCardinalityContext<'input>{}

pub struct OptionalCardinalityContextExt<'input>{
	base:CardinalityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OptionalCardinalityContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for OptionalCardinalityContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for OptionalCardinalityContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_optionalCardinality(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_optionalCardinality(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for OptionalCardinalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_optionalCardinality(self);
	}
}

impl<'input> CustomRuleContext<'input> for OptionalCardinalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cardinality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cardinality }
}

impl<'input> Borrow<CardinalityContextExt<'input>> for OptionalCardinalityContext<'input>{
	fn borrow(&self) -> &CardinalityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CardinalityContextExt<'input>> for OptionalCardinalityContext<'input>{
	fn borrow_mut(&mut self) -> &mut CardinalityContextExt<'input> { &mut self.base }
}

impl<'input> CardinalityContextAttrs<'input> for OptionalCardinalityContext<'input> {}

impl<'input> OptionalCardinalityContextExt<'input>{
	fn new(ctx: &dyn CardinalityContextAttrs<'input>) -> Rc<CardinalityContextAll<'input>>  {
		Rc::new(
			CardinalityContextAll::OptionalCardinalityContext(
				BaseParserRuleContext::copy_from(ctx,OptionalCardinalityContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cardinality(&mut self,)
	-> Result<Rc<CardinalityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CardinalityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_cardinality);
        let mut _localctx: Rc<CardinalityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(548);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 UNBOUNDED 
				=> {
					let tmp = StarCardinalityContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(544);
					recog.base.match_token(UNBOUNDED,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					let tmp = PlusCardinalityContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(545);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					}
				}

			 T__12 
				=> {
					let tmp = OptionalCardinalityContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(546);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					}
				}

			 T__6 
				=> {
					let tmp = RepeatCardinalityContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					/*InvokeRule repeatRange*/
					recog.base.set_state(547);
					recog.repeatRange()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- repeatRange ----------------
#[derive(Debug)]
pub enum RepeatRangeContextAll<'input>{
	ExactRangeContext(ExactRangeContext<'input>),
	MinMaxRangeContext(MinMaxRangeContext<'input>),
Error(RepeatRangeContext<'input>)
}
antlr_rust::tid!{RepeatRangeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for RepeatRangeContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for RepeatRangeContextAll<'input>{}

impl<'input> Deref for RepeatRangeContextAll<'input>{
	type Target = dyn RepeatRangeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use RepeatRangeContextAll::*;
		match self{
			ExactRangeContext(inner) => inner,
			MinMaxRangeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RepeatRangeContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RepeatRangeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type RepeatRangeContext<'input> = BaseParserRuleContext<'input,RepeatRangeContextExt<'input>>;

#[derive(Clone)]
pub struct RepeatRangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for RepeatRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RepeatRangeContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RepeatRangeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for RepeatRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_repeatRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_repeatRange }
}
antlr_rust::tid!{RepeatRangeContextExt<'a>}

impl<'input> RepeatRangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RepeatRangeContextAll<'input>> {
		Rc::new(
		RepeatRangeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RepeatRangeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait RepeatRangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<RepeatRangeContextExt<'input>>{


}

impl<'input> RepeatRangeContextAttrs<'input> for RepeatRangeContext<'input>{}

pub type ExactRangeContext<'input> = BaseParserRuleContext<'input,ExactRangeContextExt<'input>>;

pub trait ExactRangeContextAttrs<'input>: ShExDocParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INTEGER
	/// Returns `None` if there is no child corresponding to token INTEGER
	fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(INTEGER, 0)
	}
}

impl<'input> ExactRangeContextAttrs<'input> for ExactRangeContext<'input>{}

pub struct ExactRangeContextExt<'input>{
	base:RepeatRangeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExactRangeContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for ExactRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ExactRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_exactRange(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_exactRange(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ExactRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_exactRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExactRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_repeatRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_repeatRange }
}

impl<'input> Borrow<RepeatRangeContextExt<'input>> for ExactRangeContext<'input>{
	fn borrow(&self) -> &RepeatRangeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RepeatRangeContextExt<'input>> for ExactRangeContext<'input>{
	fn borrow_mut(&mut self) -> &mut RepeatRangeContextExt<'input> { &mut self.base }
}

impl<'input> RepeatRangeContextAttrs<'input> for ExactRangeContext<'input> {}

impl<'input> ExactRangeContextExt<'input>{
	fn new(ctx: &dyn RepeatRangeContextAttrs<'input>) -> Rc<RepeatRangeContextAll<'input>>  {
		Rc::new(
			RepeatRangeContextAll::ExactRangeContext(
				BaseParserRuleContext::copy_from(ctx,ExactRangeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MinMaxRangeContext<'input> = BaseParserRuleContext<'input,MinMaxRangeContextExt<'input>>;

pub trait MinMaxRangeContextAttrs<'input>: ShExDocParserContext<'input>{
	fn min_range(&self) -> Option<Rc<Min_rangeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn max_range(&self) -> Option<Rc<Max_rangeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MinMaxRangeContextAttrs<'input> for MinMaxRangeContext<'input>{}

pub struct MinMaxRangeContextExt<'input>{
	base:RepeatRangeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MinMaxRangeContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for MinMaxRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for MinMaxRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_minMaxRange(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_minMaxRange(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for MinMaxRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_minMaxRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for MinMaxRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_repeatRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_repeatRange }
}

impl<'input> Borrow<RepeatRangeContextExt<'input>> for MinMaxRangeContext<'input>{
	fn borrow(&self) -> &RepeatRangeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RepeatRangeContextExt<'input>> for MinMaxRangeContext<'input>{
	fn borrow_mut(&mut self) -> &mut RepeatRangeContextExt<'input> { &mut self.base }
}

impl<'input> RepeatRangeContextAttrs<'input> for MinMaxRangeContext<'input> {}

impl<'input> MinMaxRangeContextExt<'input>{
	fn new(ctx: &dyn RepeatRangeContextAttrs<'input>) -> Rc<RepeatRangeContextAll<'input>>  {
		Rc::new(
			RepeatRangeContextAll::MinMaxRangeContext(
				BaseParserRuleContext::copy_from(ctx,MinMaxRangeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn repeatRange(&mut self,)
	-> Result<Rc<RepeatRangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RepeatRangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_repeatRange);
        let mut _localctx: Rc<RepeatRangeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(561);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
				1 =>{
					let tmp = ExactRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(550);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(551);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(552);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = MinMaxRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(553);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					/*InvokeRule min_range*/
					recog.base.set_state(554);
					recog.min_range()?;

					recog.base.set_state(555);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					recog.base.set_state(557);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==INTEGER || _la==UNBOUNDED {
						{
						/*InvokeRule max_range*/
						recog.base.set_state(556);
						recog.max_range()?;

						}
					}

					recog.base.set_state(559);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- min_range ----------------
pub type Min_rangeContextAll<'input> = Min_rangeContext<'input>;


pub type Min_rangeContext<'input> = BaseParserRuleContext<'input,Min_rangeContextExt<'input>>;

#[derive(Clone)]
pub struct Min_rangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for Min_rangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for Min_rangeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_min_range(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_min_range(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for Min_rangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_min_range(self);
	}
}

impl<'input> CustomRuleContext<'input> for Min_rangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_min_range }
	//fn type_rule_index() -> usize where Self: Sized { RULE_min_range }
}
antlr_rust::tid!{Min_rangeContextExt<'a>}

impl<'input> Min_rangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Min_rangeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Min_rangeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Min_rangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<Min_rangeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}

}

impl<'input> Min_rangeContextAttrs<'input> for Min_rangeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn min_range(&mut self,)
	-> Result<Rc<Min_rangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Min_rangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_min_range);
        let mut _localctx: Rc<Min_rangeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(563);
			recog.base.match_token(INTEGER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- max_range ----------------
pub type Max_rangeContextAll<'input> = Max_rangeContext<'input>;


pub type Max_rangeContext<'input> = BaseParserRuleContext<'input,Max_rangeContextExt<'input>>;

#[derive(Clone)]
pub struct Max_rangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for Max_rangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for Max_rangeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_max_range(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_max_range(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for Max_rangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_max_range(self);
	}
}

impl<'input> CustomRuleContext<'input> for Max_rangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_max_range }
	//fn type_rule_index() -> usize where Self: Sized { RULE_max_range }
}
antlr_rust::tid!{Max_rangeContextExt<'a>}

impl<'input> Max_rangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Max_rangeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Max_rangeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Max_rangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<Max_rangeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token UNBOUNDED
/// Returns `None` if there is no child corresponding to token UNBOUNDED
fn UNBOUNDED(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(UNBOUNDED, 0)
}

}

impl<'input> Max_rangeContextAttrs<'input> for Max_rangeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn max_range(&mut self,)
	-> Result<Rc<Max_rangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Max_rangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_max_range);
        let mut _localctx: Rc<Max_rangeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(565);
			_la = recog.base.input.la(1);
			if { !(_la==INTEGER || _la==UNBOUNDED) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_expr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

fn basicExpr(&self) -> Option<Rc<BasicExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn binOp(&self) -> Option<Rc<BinOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 106, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 106;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule basicExpr*/
			recog.base.set_state(568);
			recog.basicExpr()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(576);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(67,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
					_localctx = tmp;
					recog.base.set_state(570);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					/*InvokeRule binOp*/
					recog.base.set_state(571);
					recog.binOp()?;

					/*InvokeRule expr*/
					recog.base.set_state(572);
					recog.expr_rec(3)?;

					}
					} 
				}
				recog.base.set_state(578);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(67,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- binOp ----------------
#[derive(Debug)]
pub enum BinOpContextAll<'input>{
	DivContext(DivContext<'input>),
	AddContext(AddContext<'input>),
	MinusContext(MinusContext<'input>),
	MultContext(MultContext<'input>),
	EqualsContext(EqualsContext<'input>),
	NotEqualsContext(NotEqualsContext<'input>),
	LtContext(LtContext<'input>),
	LeContext(LeContext<'input>),
	GtContext(GtContext<'input>),
	GeContext(GeContext<'input>),
Error(BinOpContext<'input>)
}
antlr_rust::tid!{BinOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for BinOpContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for BinOpContextAll<'input>{}

impl<'input> Deref for BinOpContextAll<'input>{
	type Target = dyn BinOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use BinOpContextAll::*;
		match self{
			DivContext(inner) => inner,
			AddContext(inner) => inner,
			MinusContext(inner) => inner,
			MultContext(inner) => inner,
			EqualsContext(inner) => inner,
			NotEqualsContext(inner) => inner,
			LtContext(inner) => inner,
			LeContext(inner) => inner,
			GtContext(inner) => inner,
			GeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BinOpContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BinOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type BinOpContext<'input> = BaseParserRuleContext<'input,BinOpContextExt<'input>>;

#[derive(Clone)]
pub struct BinOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for BinOpContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BinOpContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BinOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for BinOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}
antlr_rust::tid!{BinOpContextExt<'a>}

impl<'input> BinOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BinOpContextAll<'input>> {
		Rc::new(
		BinOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait BinOpContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<BinOpContextExt<'input>>{


}

impl<'input> BinOpContextAttrs<'input> for BinOpContext<'input>{}

pub type DivContext<'input> = BaseParserRuleContext<'input,DivContextExt<'input>>;

pub trait DivContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> DivContextAttrs<'input> for DivContext<'input>{}

pub struct DivContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DivContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for DivContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for DivContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_div(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_div(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for DivContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_div(self);
	}
}

impl<'input> CustomRuleContext<'input> for DivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for DivContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for DivContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for DivContext<'input> {}

impl<'input> DivContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::DivContext(
				BaseParserRuleContext::copy_from(ctx,DivContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AddContext<'input> = BaseParserRuleContext<'input,AddContextExt<'input>>;

pub trait AddContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> AddContextAttrs<'input> for AddContext<'input>{}

pub struct AddContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AddContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for AddContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for AddContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_add(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_add(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for AddContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_add(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for AddContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for AddContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for AddContext<'input> {}

impl<'input> AddContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::AddContext(
				BaseParserRuleContext::copy_from(ctx,AddContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MinusContext<'input> = BaseParserRuleContext<'input,MinusContextExt<'input>>;

pub trait MinusContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> MinusContextAttrs<'input> for MinusContext<'input>{}

pub struct MinusContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MinusContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for MinusContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for MinusContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_minus(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_minus(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for MinusContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_minus(self);
	}
}

impl<'input> CustomRuleContext<'input> for MinusContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for MinusContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for MinusContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for MinusContext<'input> {}

impl<'input> MinusContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::MinusContext(
				BaseParserRuleContext::copy_from(ctx,MinusContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultContext<'input> = BaseParserRuleContext<'input,MultContextExt<'input>>;

pub trait MultContextAttrs<'input>: ShExDocParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UNBOUNDED
	/// Returns `None` if there is no child corresponding to token UNBOUNDED
	fn UNBOUNDED(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(UNBOUNDED, 0)
	}
}

impl<'input> MultContextAttrs<'input> for MultContext<'input>{}

pub struct MultContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MultContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for MultContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for MultContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mult(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_mult(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for MultContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_mult(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for MultContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for MultContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for MultContext<'input> {}

impl<'input> MultContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::MultContext(
				BaseParserRuleContext::copy_from(ctx,MultContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqualsContext<'input> = BaseParserRuleContext<'input,EqualsContextExt<'input>>;

pub trait EqualsContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> EqualsContextAttrs<'input> for EqualsContext<'input>{}

pub struct EqualsContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqualsContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for EqualsContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for EqualsContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_equals(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_equals(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for EqualsContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_equals(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for EqualsContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for EqualsContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for EqualsContext<'input> {}

impl<'input> EqualsContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::EqualsContext(
				BaseParserRuleContext::copy_from(ctx,EqualsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NotEqualsContext<'input> = BaseParserRuleContext<'input,NotEqualsContextExt<'input>>;

pub trait NotEqualsContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> NotEqualsContextAttrs<'input> for NotEqualsContext<'input>{}

pub struct NotEqualsContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NotEqualsContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for NotEqualsContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NotEqualsContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_notEquals(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_notEquals(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NotEqualsContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_notEquals(self);
	}
}

impl<'input> CustomRuleContext<'input> for NotEqualsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for NotEqualsContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for NotEqualsContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for NotEqualsContext<'input> {}

impl<'input> NotEqualsContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::NotEqualsContext(
				BaseParserRuleContext::copy_from(ctx,NotEqualsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LtContext<'input> = BaseParserRuleContext<'input,LtContextExt<'input>>;

pub trait LtContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> LtContextAttrs<'input> for LtContext<'input>{}

pub struct LtContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LtContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for LtContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LtContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_lt(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_lt(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_lt(self);
	}
}

impl<'input> CustomRuleContext<'input> for LtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for LtContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for LtContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for LtContext<'input> {}

impl<'input> LtContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::LtContext(
				BaseParserRuleContext::copy_from(ctx,LtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LeContext<'input> = BaseParserRuleContext<'input,LeContextExt<'input>>;

pub trait LeContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> LeContextAttrs<'input> for LeContext<'input>{}

pub struct LeContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LeContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for LeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LeContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_le(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_le(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_le(self);
	}
}

impl<'input> CustomRuleContext<'input> for LeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for LeContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for LeContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for LeContext<'input> {}

impl<'input> LeContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::LeContext(
				BaseParserRuleContext::copy_from(ctx,LeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GtContext<'input> = BaseParserRuleContext<'input,GtContextExt<'input>>;

pub trait GtContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> GtContextAttrs<'input> for GtContext<'input>{}

pub struct GtContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GtContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for GtContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for GtContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_gt(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_gt(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for GtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_gt(self);
	}
}

impl<'input> CustomRuleContext<'input> for GtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for GtContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for GtContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for GtContext<'input> {}

impl<'input> GtContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::GtContext(
				BaseParserRuleContext::copy_from(ctx,GtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GeContext<'input> = BaseParserRuleContext<'input,GeContextExt<'input>>;

pub trait GeContextAttrs<'input>: ShExDocParserContext<'input>{
}

impl<'input> GeContextAttrs<'input> for GeContext<'input>{}

pub struct GeContextExt<'input>{
	base:BinOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GeContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for GeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for GeContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ge(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_ge(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for GeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_ge(self);
	}
}

impl<'input> CustomRuleContext<'input> for GeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binOp }
}

impl<'input> Borrow<BinOpContextExt<'input>> for GeContext<'input>{
	fn borrow(&self) -> &BinOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BinOpContextExt<'input>> for GeContext<'input>{
	fn borrow_mut(&mut self) -> &mut BinOpContextExt<'input> { &mut self.base }
}

impl<'input> BinOpContextAttrs<'input> for GeContext<'input> {}

impl<'input> GeContextExt<'input>{
	fn new(ctx: &dyn BinOpContextAttrs<'input>) -> Rc<BinOpContextAll<'input>>  {
		Rc::new(
			BinOpContextAll::GeContext(
				BaseParserRuleContext::copy_from(ctx,GeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binOp(&mut self,)
	-> Result<Rc<BinOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BinOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_binOp);
        let mut _localctx: Rc<BinOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(589);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 
				=> {
					let tmp = EqualsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(579);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

			 T__14 
				=> {
					let tmp = NotEqualsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(580);
					recog.base.match_token(T__14,&mut recog.err_handler)?;

					}
				}

			 T__15 
				=> {
					let tmp = GtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(581);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					}
				}

			 T__16 
				=> {
					let tmp = LtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(582);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__17 
				=> {
					let tmp = GeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(583);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					}
				}

			 T__18 
				=> {
					let tmp = LeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(584);
					recog.base.match_token(T__18,&mut recog.err_handler)?;

					}
				}

			 UNBOUNDED 
				=> {
					let tmp = MultContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(585);
					recog.base.match_token(UNBOUNDED,&mut recog.err_handler)?;

					}
				}

			 T__19 
				=> {
					let tmp = DivContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(586);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					let tmp = AddContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					recog.base.set_state(587);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					}
				}

			 T__20 
				=> {
					let tmp = MinusContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					recog.base.set_state(588);
					recog.base.match_token(T__20,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- basicExpr ----------------
pub type BasicExprContextAll<'input> = BasicExprContext<'input>;


pub type BasicExprContext<'input> = BaseParserRuleContext<'input,BasicExprContextExt<'input>>;

#[derive(Clone)]
pub struct BasicExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for BasicExprContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BasicExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_basicExpr(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_basicExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BasicExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_basicExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for BasicExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_basicExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_basicExpr }
}
antlr_rust::tid!{BasicExprContextExt<'a>}

impl<'input> BasicExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BasicExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BasicExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BasicExprContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<BasicExprContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blankNode(&self) -> Option<Rc<BlankNodeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BasicExprContextAttrs<'input> for BasicExprContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn basicExpr(&mut self,)
	-> Result<Rc<BasicExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BasicExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_basicExpr);
        let mut _localctx: Rc<BasicExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(594);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_TRUE | KW_FALSE | INTEGER | DECIMAL | DOUBLE | STRING_LITERAL1 | STRING_LITERAL2 |
			 STRING_LITERAL_LONG1 | STRING_LITERAL_LONG2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal*/
					recog.base.set_state(591);
					recog.literal()?;

					}
				}

			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule iri*/
					recog.base.set_state(592);
					recog.iri()?;

					}
				}

			 BLANK_NODE_LABEL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule blankNode*/
					recog.base.set_state(593);
					recog.blankNode()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- senseFlags ----------------
pub type SenseFlagsContextAll<'input> = SenseFlagsContext<'input>;


pub type SenseFlagsContext<'input> = BaseParserRuleContext<'input,SenseFlagsContextExt<'input>>;

#[derive(Clone)]
pub struct SenseFlagsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for SenseFlagsContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for SenseFlagsContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_senseFlags(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_senseFlags(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for SenseFlagsContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_senseFlags(self);
	}
}

impl<'input> CustomRuleContext<'input> for SenseFlagsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_senseFlags }
	//fn type_rule_index() -> usize where Self: Sized { RULE_senseFlags }
}
antlr_rust::tid!{SenseFlagsContextExt<'a>}

impl<'input> SenseFlagsContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SenseFlagsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SenseFlagsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SenseFlagsContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<SenseFlagsContextExt<'input>>{


}

impl<'input> SenseFlagsContextAttrs<'input> for SenseFlagsContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn senseFlags(&mut self,)
	-> Result<Rc<SenseFlagsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SenseFlagsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_senseFlags);
        let mut _localctx: Rc<SenseFlagsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(604);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__1 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(596);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					recog.base.set_state(598);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__21 {
						{
						recog.base.set_state(597);
						recog.base.match_token(T__21,&mut recog.err_handler)?;

						}
					}

					}
				}

			 T__21 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(600);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					recog.base.set_state(602);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__1 {
						{
						recog.base.set_state(601);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- valueSet ----------------
pub type ValueSetContextAll<'input> = ValueSetContext<'input>;


pub type ValueSetContext<'input> = BaseParserRuleContext<'input,ValueSetContextExt<'input>>;

#[derive(Clone)]
pub struct ValueSetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ValueSetContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ValueSetContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueSet(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_valueSet(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ValueSetContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_valueSet(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueSetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueSet }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueSet }
}
antlr_rust::tid!{ValueSetContextExt<'a>}

impl<'input> ValueSetContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueSetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueSetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueSetContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ValueSetContextExt<'input>>{

fn valueSetValue_all(&self) ->  Vec<Rc<ValueSetValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueSetValue(&self, i: usize) -> Option<Rc<ValueSetValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ValueSetContextAttrs<'input> for ValueSetContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueSet(&mut self,)
	-> Result<Rc<ValueSetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueSetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_valueSet);
        let mut _localctx: Rc<ValueSetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(606);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(610);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__4 || _la==T__5 || ((((_la - 57)) & !0x3f) == 0 && ((1usize << (_la - 57)) & ((1usize << (KW_TRUE - 57)) | (1usize << (KW_FALSE - 57)) | (1usize << (IRIREF - 57)) | (1usize << (PNAME_NS - 57)) | (1usize << (PNAME_LN - 57)) | (1usize << (LANGTAG - 57)) | (1usize << (INTEGER - 57)) | (1usize << (DECIMAL - 57)) | (1usize << (DOUBLE - 57)) | (1usize << (STRING_LITERAL1 - 57)) | (1usize << (STRING_LITERAL2 - 57)) | (1usize << (STRING_LITERAL_LONG1 - 57)) | (1usize << (STRING_LITERAL_LONG2 - 57)))) != 0) {
				{
				{
				/*InvokeRule valueSetValue*/
				recog.base.set_state(607);
				recog.valueSetValue()?;

				}
				}
				recog.base.set_state(612);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(613);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- valueSetValue ----------------
pub type ValueSetValueContextAll<'input> = ValueSetValueContext<'input>;


pub type ValueSetValueContext<'input> = BaseParserRuleContext<'input,ValueSetValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueSetValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ValueSetValueContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ValueSetValueContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueSetValue(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_valueSetValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ValueSetValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_valueSetValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueSetValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueSetValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueSetValue }
}
antlr_rust::tid!{ValueSetValueContextExt<'a>}

impl<'input> ValueSetValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueSetValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueSetValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueSetValueContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ValueSetValueContextExt<'input>>{

fn iriRange(&self) -> Option<Rc<IriRangeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literalRange(&self) -> Option<Rc<LiteralRangeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn languageRange(&self) -> Option<Rc<LanguageRangeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iriExclusion_all(&self) ->  Vec<Rc<IriExclusionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn iriExclusion(&self, i: usize) -> Option<Rc<IriExclusionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn literalExclusion_all(&self) ->  Vec<Rc<LiteralExclusionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn literalExclusion(&self, i: usize) -> Option<Rc<LiteralExclusionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn languageExclusion_all(&self) ->  Vec<Rc<LanguageExclusionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn languageExclusion(&self, i: usize) -> Option<Rc<LanguageExclusionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ValueSetValueContextAttrs<'input> for ValueSetValueContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueSetValue(&mut self,)
	-> Result<Rc<ValueSetValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueSetValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_valueSetValue);
        let mut _localctx: Rc<ValueSetValueContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(636);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule iriRange*/
					recog.base.set_state(615);
					recog.iriRange()?;

					}
				}

			 KW_TRUE | KW_FALSE | INTEGER | DECIMAL | DOUBLE | STRING_LITERAL1 | STRING_LITERAL2 |
			 STRING_LITERAL_LONG1 | STRING_LITERAL_LONG2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule literalRange*/
					recog.base.set_state(616);
					recog.literalRange()?;

					}
				}

			 T__5 | LANGTAG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule languageRange*/
					recog.base.set_state(617);
					recog.languageRange()?;

					}
				}

			 T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(618);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					recog.base.set_state(634);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(77,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(620); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								/*InvokeRule iriExclusion*/
								recog.base.set_state(619);
								recog.iriExclusion()?;

								}
								}
								recog.base.set_state(622); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !(_la==T__20) {break}
							}
							}
						}
					,
						2 =>{
							{
							recog.base.set_state(625); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								/*InvokeRule literalExclusion*/
								recog.base.set_state(624);
								recog.literalExclusion()?;

								}
								}
								recog.base.set_state(627); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !(_la==T__20) {break}
							}
							}
						}
					,
						3 =>{
							{
							recog.base.set_state(630); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								/*InvokeRule languageExclusion*/
								recog.base.set_state(629);
								recog.languageExclusion()?;

								}
								}
								recog.base.set_state(632); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !(_la==T__20) {break}
							}
							}
						}

						_ => {}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iriRange ----------------
pub type IriRangeContextAll<'input> = IriRangeContext<'input>;


pub type IriRangeContext<'input> = BaseParserRuleContext<'input,IriRangeContextExt<'input>>;

#[derive(Clone)]
pub struct IriRangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for IriRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for IriRangeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_iriRange(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_iriRange(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for IriRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_iriRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for IriRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iriRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iriRange }
}
antlr_rust::tid!{IriRangeContextExt<'a>}

impl<'input> IriRangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IriRangeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IriRangeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IriRangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<IriRangeContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STEM_MARK
/// Returns `None` if there is no child corresponding to token STEM_MARK
fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STEM_MARK, 0)
}
fn iriExclusion_all(&self) ->  Vec<Rc<IriExclusionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn iriExclusion(&self, i: usize) -> Option<Rc<IriExclusionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IriRangeContextAttrs<'input> for IriRangeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iriRange(&mut self,)
	-> Result<Rc<IriRangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IriRangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_iriRange);
        let mut _localctx: Rc<IriRangeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule iri*/
			recog.base.set_state(638);
			recog.iri()?;

			recog.base.set_state(646);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STEM_MARK {
				{
				recog.base.set_state(639);
				recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

				recog.base.set_state(643);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__20 {
					{
					{
					/*InvokeRule iriExclusion*/
					recog.base.set_state(640);
					recog.iriExclusion()?;

					}
					}
					recog.base.set_state(645);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iriExclusion ----------------
pub type IriExclusionContextAll<'input> = IriExclusionContext<'input>;


pub type IriExclusionContext<'input> = BaseParserRuleContext<'input,IriExclusionContextExt<'input>>;

#[derive(Clone)]
pub struct IriExclusionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for IriExclusionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for IriExclusionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_iriExclusion(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_iriExclusion(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for IriExclusionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_iriExclusion(self);
	}
}

impl<'input> CustomRuleContext<'input> for IriExclusionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iriExclusion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iriExclusion }
}
antlr_rust::tid!{IriExclusionContextExt<'a>}

impl<'input> IriExclusionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IriExclusionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IriExclusionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IriExclusionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<IriExclusionContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STEM_MARK
/// Returns `None` if there is no child corresponding to token STEM_MARK
fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STEM_MARK, 0)
}

}

impl<'input> IriExclusionContextAttrs<'input> for IriExclusionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iriExclusion(&mut self,)
	-> Result<Rc<IriExclusionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IriExclusionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_iriExclusion);
        let mut _localctx: Rc<IriExclusionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(648);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule iri*/
			recog.base.set_state(649);
			recog.iri()?;

			recog.base.set_state(651);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STEM_MARK {
				{
				recog.base.set_state(650);
				recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literalRange ----------------
pub type LiteralRangeContextAll<'input> = LiteralRangeContext<'input>;


pub type LiteralRangeContext<'input> = BaseParserRuleContext<'input,LiteralRangeContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralRangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for LiteralRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LiteralRangeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literalRange(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_literalRange(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LiteralRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_literalRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literalRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literalRange }
}
antlr_rust::tid!{LiteralRangeContextExt<'a>}

impl<'input> LiteralRangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralRangeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralRangeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralRangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<LiteralRangeContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STEM_MARK
/// Returns `None` if there is no child corresponding to token STEM_MARK
fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STEM_MARK, 0)
}
fn literalExclusion_all(&self) ->  Vec<Rc<LiteralExclusionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn literalExclusion(&self, i: usize) -> Option<Rc<LiteralExclusionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LiteralRangeContextAttrs<'input> for LiteralRangeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literalRange(&mut self,)
	-> Result<Rc<LiteralRangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralRangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_literalRange);
        let mut _localctx: Rc<LiteralRangeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule literal*/
			recog.base.set_state(653);
			recog.literal()?;

			recog.base.set_state(661);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STEM_MARK {
				{
				recog.base.set_state(654);
				recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

				recog.base.set_state(658);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__20 {
					{
					{
					/*InvokeRule literalExclusion*/
					recog.base.set_state(655);
					recog.literalExclusion()?;

					}
					}
					recog.base.set_state(660);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literalExclusion ----------------
pub type LiteralExclusionContextAll<'input> = LiteralExclusionContext<'input>;


pub type LiteralExclusionContext<'input> = BaseParserRuleContext<'input,LiteralExclusionContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralExclusionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for LiteralExclusionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LiteralExclusionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literalExclusion(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_literalExclusion(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LiteralExclusionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_literalExclusion(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralExclusionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literalExclusion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literalExclusion }
}
antlr_rust::tid!{LiteralExclusionContextExt<'a>}

impl<'input> LiteralExclusionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralExclusionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralExclusionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralExclusionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<LiteralExclusionContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STEM_MARK
/// Returns `None` if there is no child corresponding to token STEM_MARK
fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STEM_MARK, 0)
}

}

impl<'input> LiteralExclusionContextAttrs<'input> for LiteralExclusionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literalExclusion(&mut self,)
	-> Result<Rc<LiteralExclusionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralExclusionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_literalExclusion);
        let mut _localctx: Rc<LiteralExclusionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(663);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule literal*/
			recog.base.set_state(664);
			recog.literal()?;

			recog.base.set_state(666);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STEM_MARK {
				{
				recog.base.set_state(665);
				recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- languageRange ----------------
#[derive(Debug)]
pub enum LanguageRangeContextAll<'input>{
	LanguageRangeFullContext(LanguageRangeFullContext<'input>),
	LanguageRangeAtContext(LanguageRangeAtContext<'input>),
Error(LanguageRangeContext<'input>)
}
antlr_rust::tid!{LanguageRangeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LanguageRangeContextAll<'input>{}

impl<'input> ShExDocParserContext<'input> for LanguageRangeContextAll<'input>{}

impl<'input> Deref for LanguageRangeContextAll<'input>{
	type Target = dyn LanguageRangeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LanguageRangeContextAll::*;
		match self{
			LanguageRangeFullContext(inner) => inner,
			LanguageRangeAtContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LanguageRangeContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LanguageRangeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn ShExDocListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LanguageRangeContext<'input> = BaseParserRuleContext<'input,LanguageRangeContextExt<'input>>;

#[derive(Clone)]
pub struct LanguageRangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for LanguageRangeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LanguageRangeContext<'input>{
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LanguageRangeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LanguageRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_languageRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_languageRange }
}
antlr_rust::tid!{LanguageRangeContextExt<'a>}

impl<'input> LanguageRangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LanguageRangeContextAll<'input>> {
		Rc::new(
		LanguageRangeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LanguageRangeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LanguageRangeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<LanguageRangeContextExt<'input>>{


}

impl<'input> LanguageRangeContextAttrs<'input> for LanguageRangeContext<'input>{}

pub type LanguageRangeFullContext<'input> = BaseParserRuleContext<'input,LanguageRangeFullContextExt<'input>>;

pub trait LanguageRangeFullContextAttrs<'input>: ShExDocParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LANGTAG
	/// Returns `None` if there is no child corresponding to token LANGTAG
	fn LANGTAG(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(LANGTAG, 0)
	}
	/// Retrieves first TerminalNode corresponding to token STEM_MARK
	/// Returns `None` if there is no child corresponding to token STEM_MARK
	fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(STEM_MARK, 0)
	}
	fn languageExclusion_all(&self) ->  Vec<Rc<LanguageExclusionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn languageExclusion(&self, i: usize) -> Option<Rc<LanguageExclusionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LanguageRangeFullContextAttrs<'input> for LanguageRangeFullContext<'input>{}

pub struct LanguageRangeFullContextExt<'input>{
	base:LanguageRangeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LanguageRangeFullContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for LanguageRangeFullContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LanguageRangeFullContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_languageRangeFull(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_languageRangeFull(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LanguageRangeFullContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_languageRangeFull(self);
	}
}

impl<'input> CustomRuleContext<'input> for LanguageRangeFullContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_languageRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_languageRange }
}

impl<'input> Borrow<LanguageRangeContextExt<'input>> for LanguageRangeFullContext<'input>{
	fn borrow(&self) -> &LanguageRangeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LanguageRangeContextExt<'input>> for LanguageRangeFullContext<'input>{
	fn borrow_mut(&mut self) -> &mut LanguageRangeContextExt<'input> { &mut self.base }
}

impl<'input> LanguageRangeContextAttrs<'input> for LanguageRangeFullContext<'input> {}

impl<'input> LanguageRangeFullContextExt<'input>{
	fn new(ctx: &dyn LanguageRangeContextAttrs<'input>) -> Rc<LanguageRangeContextAll<'input>>  {
		Rc::new(
			LanguageRangeContextAll::LanguageRangeFullContext(
				BaseParserRuleContext::copy_from(ctx,LanguageRangeFullContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LanguageRangeAtContext<'input> = BaseParserRuleContext<'input,LanguageRangeAtContextExt<'input>>;

pub trait LanguageRangeAtContextAttrs<'input>: ShExDocParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STEM_MARK
	/// Returns `None` if there is no child corresponding to token STEM_MARK
	fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
		self.get_token(STEM_MARK, 0)
	}
	fn languageExclusion_all(&self) ->  Vec<Rc<LanguageExclusionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn languageExclusion(&self, i: usize) -> Option<Rc<LanguageExclusionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LanguageRangeAtContextAttrs<'input> for LanguageRangeAtContext<'input>{}

pub struct LanguageRangeAtContextExt<'input>{
	base:LanguageRangeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LanguageRangeAtContextExt<'a>}

impl<'input> ShExDocParserContext<'input> for LanguageRangeAtContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LanguageRangeAtContext<'input>{
	fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_languageRangeAt(self);
	}
	fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
		listener.exit_languageRangeAt(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LanguageRangeAtContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_languageRangeAt(self);
	}
}

impl<'input> CustomRuleContext<'input> for LanguageRangeAtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_languageRange }
	//fn type_rule_index() -> usize where Self: Sized { RULE_languageRange }
}

impl<'input> Borrow<LanguageRangeContextExt<'input>> for LanguageRangeAtContext<'input>{
	fn borrow(&self) -> &LanguageRangeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LanguageRangeContextExt<'input>> for LanguageRangeAtContext<'input>{
	fn borrow_mut(&mut self) -> &mut LanguageRangeContextExt<'input> { &mut self.base }
}

impl<'input> LanguageRangeContextAttrs<'input> for LanguageRangeAtContext<'input> {}

impl<'input> LanguageRangeAtContextExt<'input>{
	fn new(ctx: &dyn LanguageRangeContextAttrs<'input>) -> Rc<LanguageRangeContextAll<'input>>  {
		Rc::new(
			LanguageRangeContextAll::LanguageRangeAtContext(
				BaseParserRuleContext::copy_from(ctx,LanguageRangeAtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn languageRange(&mut self,)
	-> Result<Rc<LanguageRangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LanguageRangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_languageRange);
        let mut _localctx: Rc<LanguageRangeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(686);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LANGTAG 
				=> {
					let tmp = LanguageRangeFullContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(668);
					recog.base.match_token(LANGTAG,&mut recog.err_handler)?;

					recog.base.set_state(676);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==STEM_MARK {
						{
						recog.base.set_state(669);
						recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

						recog.base.set_state(673);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==T__20 {
							{
							{
							/*InvokeRule languageExclusion*/
							recog.base.set_state(670);
							recog.languageExclusion()?;

							}
							}
							recog.base.set_state(675);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					}
				}

			 T__5 
				=> {
					let tmp = LanguageRangeAtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(678);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					recog.base.set_state(679);
					recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

					recog.base.set_state(683);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__20 {
						{
						{
						/*InvokeRule languageExclusion*/
						recog.base.set_state(680);
						recog.languageExclusion()?;

						}
						}
						recog.base.set_state(685);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- languageExclusion ----------------
pub type LanguageExclusionContextAll<'input> = LanguageExclusionContext<'input>;


pub type LanguageExclusionContext<'input> = BaseParserRuleContext<'input,LanguageExclusionContextExt<'input>>;

#[derive(Clone)]
pub struct LanguageExclusionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for LanguageExclusionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LanguageExclusionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_languageExclusion(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_languageExclusion(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LanguageExclusionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_languageExclusion(self);
	}
}

impl<'input> CustomRuleContext<'input> for LanguageExclusionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_languageExclusion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_languageExclusion }
}
antlr_rust::tid!{LanguageExclusionContextExt<'a>}

impl<'input> LanguageExclusionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LanguageExclusionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LanguageExclusionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LanguageExclusionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<LanguageExclusionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LANGTAG
/// Returns `None` if there is no child corresponding to token LANGTAG
fn LANGTAG(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(LANGTAG, 0)
}
/// Retrieves first TerminalNode corresponding to token STEM_MARK
/// Returns `None` if there is no child corresponding to token STEM_MARK
fn STEM_MARK(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STEM_MARK, 0)
}

}

impl<'input> LanguageExclusionContextAttrs<'input> for LanguageExclusionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn languageExclusion(&mut self,)
	-> Result<Rc<LanguageExclusionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LanguageExclusionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_languageExclusion);
        let mut _localctx: Rc<LanguageExclusionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(688);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(689);
			recog.base.match_token(LANGTAG,&mut recog.err_handler)?;

			recog.base.set_state(691);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STEM_MARK {
				{
				recog.base.set_state(690);
				recog.base.match_token(STEM_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- include ----------------
pub type IncludeContextAll<'input> = IncludeContext<'input>;


pub type IncludeContext<'input> = BaseParserRuleContext<'input,IncludeContextExt<'input>>;

#[derive(Clone)]
pub struct IncludeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for IncludeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for IncludeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_include(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_include(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for IncludeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_include(self);
	}
}

impl<'input> CustomRuleContext<'input> for IncludeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_include }
	//fn type_rule_index() -> usize where Self: Sized { RULE_include }
}
antlr_rust::tid!{IncludeContextExt<'a>}

impl<'input> IncludeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IncludeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IncludeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IncludeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<IncludeContextExt<'input>>{

fn tripleExprLabel(&self) -> Option<Rc<TripleExprLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IncludeContextAttrs<'input> for IncludeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn include(&mut self,)
	-> Result<Rc<IncludeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IncludeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_include);
        let mut _localctx: Rc<IncludeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(693);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			/*InvokeRule tripleExprLabel*/
			recog.base.set_state(694);
			recog.tripleExprLabel()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotation ----------------
pub type AnnotationContextAll<'input> = AnnotationContext<'input>;


pub type AnnotationContext<'input> = BaseParserRuleContext<'input,AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for AnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotation(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for AnnotationContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_annotation(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}
antlr_rust::tid!{AnnotationContextExt<'a>}

impl<'input> AnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>{

fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotation(&mut self,)
	-> Result<Rc<AnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(696);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			/*InvokeRule predicate*/
			recog.base.set_state(697);
			recog.predicate()?;

			recog.base.set_state(700);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					{
					/*InvokeRule iri*/
					recog.base.set_state(698);
					recog.iri()?;

					}
				}

			 KW_TRUE | KW_FALSE | INTEGER | DECIMAL | DOUBLE | STRING_LITERAL1 | STRING_LITERAL2 |
			 STRING_LITERAL_LONG1 | STRING_LITERAL_LONG2 
				=> {
					{
					/*InvokeRule literal*/
					recog.base.set_state(699);
					recog.literal()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- semanticAction ----------------
pub type SemanticActionContextAll<'input> = SemanticActionContext<'input>;


pub type SemanticActionContext<'input> = BaseParserRuleContext<'input,SemanticActionContextExt<'input>>;

#[derive(Clone)]
pub struct SemanticActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for SemanticActionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for SemanticActionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_semanticAction(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_semanticAction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for SemanticActionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_semanticAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for SemanticActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_semanticAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_semanticAction }
}
antlr_rust::tid!{SemanticActionContextExt<'a>}

impl<'input> SemanticActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SemanticActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SemanticActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SemanticActionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<SemanticActionContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CODE
/// Returns `None` if there is no child corresponding to token CODE
fn CODE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(CODE, 0)
}

}

impl<'input> SemanticActionContextAttrs<'input> for SemanticActionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn semanticAction(&mut self,)
	-> Result<Rc<SemanticActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SemanticActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_semanticAction);
        let mut _localctx: Rc<SemanticActionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(702);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			/*InvokeRule iri*/
			recog.base.set_state(703);
			recog.iri()?;

			recog.base.set_state(704);
			_la = recog.base.input.la(1);
			if { !(_la==T__26 || _la==CODE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

fn rdfLiteral(&self) -> Option<Rc<RdfLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn numericLiteral(&self) -> Option<Rc<NumericLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn booleanLiteral(&self) -> Option<Rc<BooleanLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(709);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING_LITERAL1 | STRING_LITERAL2 | STRING_LITERAL_LONG1 | STRING_LITERAL_LONG2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule rdfLiteral*/
					recog.base.set_state(706);
					recog.rdfLiteral()?;

					}
				}

			 INTEGER | DECIMAL | DOUBLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numericLiteral*/
					recog.base.set_state(707);
					recog.numericLiteral()?;

					}
				}

			 KW_TRUE | KW_FALSE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule booleanLiteral*/
					recog.base.set_state(708);
					recog.booleanLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- predicate ----------------
pub type PredicateContextAll<'input> = PredicateContext<'input>;


pub type PredicateContext<'input> = BaseParserRuleContext<'input,PredicateContextExt<'input>>;

#[derive(Clone)]
pub struct PredicateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for PredicateContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for PredicateContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predicate(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_predicate(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for PredicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate }
}
antlr_rust::tid!{PredicateContextExt<'a>}

impl<'input> PredicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredicateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredicateContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<PredicateContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn rdfType(&self) -> Option<Rc<RdfTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PredicateContextAttrs<'input> for PredicateContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate(&mut self,)
	-> Result<Rc<PredicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_predicate);
        let mut _localctx: Rc<PredicateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(713);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule iri*/
					recog.base.set_state(711);
					recog.iri()?;

					}
				}

			 RDF_TYPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule rdfType*/
					recog.base.set_state(712);
					recog.rdfType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rdfType ----------------
pub type RdfTypeContextAll<'input> = RdfTypeContext<'input>;


pub type RdfTypeContext<'input> = BaseParserRuleContext<'input,RdfTypeContextExt<'input>>;

#[derive(Clone)]
pub struct RdfTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for RdfTypeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RdfTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rdfType(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_rdfType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RdfTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_rdfType(self);
	}
}

impl<'input> CustomRuleContext<'input> for RdfTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rdfType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rdfType }
}
antlr_rust::tid!{RdfTypeContextExt<'a>}

impl<'input> RdfTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RdfTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RdfTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RdfTypeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<RdfTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RDF_TYPE
/// Returns `None` if there is no child corresponding to token RDF_TYPE
fn RDF_TYPE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(RDF_TYPE, 0)
}

}

impl<'input> RdfTypeContextAttrs<'input> for RdfTypeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rdfType(&mut self,)
	-> Result<Rc<RdfTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RdfTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_rdfType);
        let mut _localctx: Rc<RdfTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(715);
			recog.base.match_token(RDF_TYPE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- datatype ----------------
pub type DatatypeContextAll<'input> = DatatypeContext<'input>;


pub type DatatypeContext<'input> = BaseParserRuleContext<'input,DatatypeContextExt<'input>>;

#[derive(Clone)]
pub struct DatatypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for DatatypeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for DatatypeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_datatype(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_datatype(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for DatatypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_datatype(self);
	}
}

impl<'input> CustomRuleContext<'input> for DatatypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_datatype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_datatype }
}
antlr_rust::tid!{DatatypeContextExt<'a>}

impl<'input> DatatypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DatatypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DatatypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DatatypeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<DatatypeContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DatatypeContextAttrs<'input> for DatatypeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn datatype(&mut self,)
	-> Result<Rc<DatatypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DatatypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_datatype);
        let mut _localctx: Rc<DatatypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule iri*/
			recog.base.set_state(717);
			recog.iri()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeExprLabel ----------------
pub type ShapeExprLabelContextAll<'input> = ShapeExprLabelContext<'input>;


pub type ShapeExprLabelContext<'input> = BaseParserRuleContext<'input,ShapeExprLabelContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeExprLabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ShapeExprLabelContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ShapeExprLabelContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_shapeExprLabel(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_shapeExprLabel(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ShapeExprLabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_shapeExprLabel(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeExprLabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeExprLabel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeExprLabel }
}
antlr_rust::tid!{ShapeExprLabelContextExt<'a>}

impl<'input> ShapeExprLabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeExprLabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeExprLabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeExprLabelContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ShapeExprLabelContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blankNode(&self) -> Option<Rc<BlankNodeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShapeExprLabelContextAttrs<'input> for ShapeExprLabelContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeExprLabel(&mut self,)
	-> Result<Rc<ShapeExprLabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeExprLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_shapeExprLabel);
        let mut _localctx: Rc<ShapeExprLabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(721);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule iri*/
					recog.base.set_state(719);
					recog.iri()?;

					}
				}

			 BLANK_NODE_LABEL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule blankNode*/
					recog.base.set_state(720);
					recog.blankNode()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tripleExprLabel ----------------
pub type TripleExprLabelContextAll<'input> = TripleExprLabelContext<'input>;


pub type TripleExprLabelContext<'input> = BaseParserRuleContext<'input,TripleExprLabelContextExt<'input>>;

#[derive(Clone)]
pub struct TripleExprLabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for TripleExprLabelContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for TripleExprLabelContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tripleExprLabel(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_tripleExprLabel(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for TripleExprLabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_tripleExprLabel(self);
	}
}

impl<'input> CustomRuleContext<'input> for TripleExprLabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tripleExprLabel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tripleExprLabel }
}
antlr_rust::tid!{TripleExprLabelContextExt<'a>}

impl<'input> TripleExprLabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TripleExprLabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TripleExprLabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TripleExprLabelContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<TripleExprLabelContextExt<'input>>{

fn iri(&self) -> Option<Rc<IriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blankNode(&self) -> Option<Rc<BlankNodeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TripleExprLabelContextAttrs<'input> for TripleExprLabelContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tripleExprLabel(&mut self,)
	-> Result<Rc<TripleExprLabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TripleExprLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_tripleExprLabel);
        let mut _localctx: Rc<TripleExprLabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(725);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IRIREF | PNAME_NS | PNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule iri*/
					recog.base.set_state(723);
					recog.iri()?;

					}
				}

			 BLANK_NODE_LABEL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule blankNode*/
					recog.base.set_state(724);
					recog.blankNode()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- numericLiteral ----------------
pub type NumericLiteralContextAll<'input> = NumericLiteralContext<'input>;


pub type NumericLiteralContext<'input> = BaseParserRuleContext<'input,NumericLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NumericLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for NumericLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for NumericLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numericLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_numericLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for NumericLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_numericLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericLiteral }
}
antlr_rust::tid!{NumericLiteralContextExt<'a>}

impl<'input> NumericLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumericLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumericLiteralContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<NumericLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL
/// Returns `None` if there is no child corresponding to token DECIMAL
fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE
/// Returns `None` if there is no child corresponding to token DOUBLE
fn DOUBLE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE, 0)
}

}

impl<'input> NumericLiteralContextAttrs<'input> for NumericLiteralContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numericLiteral(&mut self,)
	-> Result<Rc<NumericLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_numericLiteral);
        let mut _localctx: Rc<NumericLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(727);
			_la = recog.base.input.la(1);
			if { !(((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (INTEGER - 71)) | (1usize << (DECIMAL - 71)) | (1usize << (DOUBLE - 71)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rdfLiteral ----------------
pub type RdfLiteralContextAll<'input> = RdfLiteralContext<'input>;


pub type RdfLiteralContext<'input> = BaseParserRuleContext<'input,RdfLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct RdfLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for RdfLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RdfLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rdfLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_rdfLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RdfLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_rdfLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for RdfLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rdfLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rdfLiteral }
}
antlr_rust::tid!{RdfLiteralContextExt<'a>}

impl<'input> RdfLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RdfLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RdfLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RdfLiteralContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<RdfLiteralContextExt<'input>>{

fn string(&self) -> Option<Rc<StringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LANGTAG
/// Returns `None` if there is no child corresponding to token LANGTAG
fn LANGTAG(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(LANGTAG, 0)
}
fn datatype(&self) -> Option<Rc<DatatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RdfLiteralContextAttrs<'input> for RdfLiteralContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rdfLiteral(&mut self,)
	-> Result<Rc<RdfLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RdfLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_rdfLiteral);
        let mut _localctx: Rc<RdfLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule string*/
			recog.base.set_state(729);
			recog.string()?;

			recog.base.set_state(733);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(95,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(730);
					recog.base.match_token(LANGTAG,&mut recog.err_handler)?;

					}
				}

				x if x == 2=>{
					{
					recog.base.set_state(731);
					recog.base.match_token(T__27,&mut recog.err_handler)?;

					/*InvokeRule datatype*/
					recog.base.set_state(732);
					recog.datatype()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- booleanLiteral ----------------
pub type BooleanLiteralContextAll<'input> = BooleanLiteralContext<'input>;


pub type BooleanLiteralContext<'input> = BaseParserRuleContext<'input,BooleanLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for BooleanLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BooleanLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_booleanLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_booleanLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BooleanLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_booleanLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanLiteral }
}
antlr_rust::tid!{BooleanLiteralContextExt<'a>}

impl<'input> BooleanLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanLiteralContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<BooleanLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_TRUE
/// Returns `None` if there is no child corresponding to token KW_TRUE
fn KW_TRUE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_FALSE
/// Returns `None` if there is no child corresponding to token KW_FALSE
fn KW_FALSE(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_FALSE, 0)
}

}

impl<'input> BooleanLiteralContextAttrs<'input> for BooleanLiteralContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn booleanLiteral(&mut self,)
	-> Result<Rc<BooleanLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_booleanLiteral);
        let mut _localctx: Rc<BooleanLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(735);
			_la = recog.base.input.la(1);
			if { !(_la==KW_TRUE || _la==KW_FALSE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- string ----------------
pub type StringContextAll<'input> = StringContext<'input>;


pub type StringContext<'input> = BaseParserRuleContext<'input,StringContextExt<'input>>;

#[derive(Clone)]
pub struct StringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for StringContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for StringContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_string(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_string(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for StringContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_string(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_string }
	//fn type_rule_index() -> usize where Self: Sized { RULE_string }
}
antlr_rust::tid!{StringContextExt<'a>}

impl<'input> StringContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<StringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING_LITERAL_LONG1
/// Returns `None` if there is no child corresponding to token STRING_LITERAL_LONG1
fn STRING_LITERAL_LONG1(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL_LONG1, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL_LONG2
/// Returns `None` if there is no child corresponding to token STRING_LITERAL_LONG2
fn STRING_LITERAL_LONG2(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL_LONG2, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL1
/// Returns `None` if there is no child corresponding to token STRING_LITERAL1
fn STRING_LITERAL1(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL1, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL2
/// Returns `None` if there is no child corresponding to token STRING_LITERAL2
fn STRING_LITERAL2(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL2, 0)
}

}

impl<'input> StringContextAttrs<'input> for StringContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn string(&mut self,)
	-> Result<Rc<StringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(737);
			_la = recog.base.input.la(1);
			if { !(((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & ((1usize << (STRING_LITERAL1 - 76)) | (1usize << (STRING_LITERAL2 - 76)) | (1usize << (STRING_LITERAL_LONG1 - 76)) | (1usize << (STRING_LITERAL_LONG2 - 76)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iri ----------------
pub type IriContextAll<'input> = IriContext<'input>;


pub type IriContext<'input> = BaseParserRuleContext<'input,IriContextExt<'input>>;

#[derive(Clone)]
pub struct IriContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for IriContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for IriContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_iri(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_iri(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for IriContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_iri(self);
	}
}

impl<'input> CustomRuleContext<'input> for IriContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iri }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iri }
}
antlr_rust::tid!{IriContextExt<'a>}

impl<'input> IriContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IriContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IriContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IriContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<IriContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IRIREF
/// Returns `None` if there is no child corresponding to token IRIREF
fn IRIREF(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(IRIREF, 0)
}
fn prefixedName(&self) -> Option<Rc<PrefixedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IriContextAttrs<'input> for IriContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iri(&mut self,)
	-> Result<Rc<IriContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IriContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_iri);
        let mut _localctx: Rc<IriContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(741);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IRIREF 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(739);
					recog.base.match_token(IRIREF,&mut recog.err_handler)?;

					}
				}

			 PNAME_NS | PNAME_LN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule prefixedName*/
					recog.base.set_state(740);
					recog.prefixedName()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- prefixedName ----------------
pub type PrefixedNameContextAll<'input> = PrefixedNameContext<'input>;


pub type PrefixedNameContext<'input> = BaseParserRuleContext<'input,PrefixedNameContextExt<'input>>;

#[derive(Clone)]
pub struct PrefixedNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for PrefixedNameContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for PrefixedNameContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prefixedName(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_prefixedName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for PrefixedNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_prefixedName(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrefixedNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prefixedName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prefixedName }
}
antlr_rust::tid!{PrefixedNameContextExt<'a>}

impl<'input> PrefixedNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrefixedNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrefixedNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrefixedNameContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<PrefixedNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PNAME_LN
/// Returns `None` if there is no child corresponding to token PNAME_LN
fn PNAME_LN(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(PNAME_LN, 0)
}
/// Retrieves first TerminalNode corresponding to token PNAME_NS
/// Returns `None` if there is no child corresponding to token PNAME_NS
fn PNAME_NS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(PNAME_NS, 0)
}

}

impl<'input> PrefixedNameContextAttrs<'input> for PrefixedNameContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prefixedName(&mut self,)
	-> Result<Rc<PrefixedNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrefixedNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_prefixedName);
        let mut _localctx: Rc<PrefixedNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(743);
			_la = recog.base.input.la(1);
			if { !(_la==PNAME_NS || _la==PNAME_LN) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- blankNode ----------------
pub type BlankNodeContextAll<'input> = BlankNodeContext<'input>;


pub type BlankNodeContext<'input> = BaseParserRuleContext<'input,BlankNodeContextExt<'input>>;

#[derive(Clone)]
pub struct BlankNodeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for BlankNodeContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for BlankNodeContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_blankNode(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_blankNode(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for BlankNodeContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_blankNode(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlankNodeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blankNode }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blankNode }
}
antlr_rust::tid!{BlankNodeContextExt<'a>}

impl<'input> BlankNodeContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlankNodeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlankNodeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlankNodeContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<BlankNodeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BLANK_NODE_LABEL
/// Returns `None` if there is no child corresponding to token BLANK_NODE_LABEL
fn BLANK_NODE_LABEL(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(BLANK_NODE_LABEL, 0)
}

}

impl<'input> BlankNodeContextAttrs<'input> for BlankNodeContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blankNode(&mut self,)
	-> Result<Rc<BlankNodeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlankNodeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_blankNode);
        let mut _localctx: Rc<BlankNodeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(745);
			recog.base.match_token(BLANK_NODE_LABEL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- extension ----------------
pub type ExtensionContextAll<'input> = ExtensionContext<'input>;


pub type ExtensionContext<'input> = BaseParserRuleContext<'input,ExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct ExtensionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for ExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for ExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_extension(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_extension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for ExtensionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_extension(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_extension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_extension }
}
antlr_rust::tid!{ExtensionContextExt<'a>}

impl<'input> ExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExtensionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExtensionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<ExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_EXTENDS
/// Returns `None` if there is no child corresponding to token KW_EXTENDS
fn KW_EXTENDS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_EXTENDS, 0)
}
fn shapeRef_all(&self) ->  Vec<Rc<ShapeRefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shapeRef(&self, i: usize) -> Option<Rc<ShapeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExtensionContextAttrs<'input> for ExtensionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn extension(&mut self,)
	-> Result<Rc<ExtensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_extension);
        let mut _localctx: Rc<ExtensionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(759);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_EXTENDS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(747);
					recog.base.match_token(KW_EXTENDS,&mut recog.err_handler)?;

					recog.base.set_state(749); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule shapeRef*/
						recog.base.set_state(748);
						recog.shapeRef()?;

						}
						}
						recog.base.set_state(751); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==T__5 || _la==ATPNAME_NS || _la==ATPNAME_LN) {break}
					}
					}
				}

			 T__24 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(753);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					recog.base.set_state(755); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule shapeRef*/
						recog.base.set_state(754);
						recog.shapeRef()?;

						}
						}
						recog.base.set_state(757); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==T__5 || _la==ATPNAME_NS || _la==ATPNAME_LN) {break}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- restriction ----------------
pub type RestrictionContextAll<'input> = RestrictionContext<'input>;


pub type RestrictionContext<'input> = BaseParserRuleContext<'input,RestrictionContextExt<'input>>;

#[derive(Clone)]
pub struct RestrictionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ShExDocParserContext<'input> for RestrictionContext<'input>{}

impl<'input,'a> Listenable<dyn ShExDocListener<'input> + 'a> for RestrictionContext<'input>{
		fn enter(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_restriction(self);
		}
		fn exit(&self,listener: &mut (dyn ShExDocListener<'input> + 'a)) {
			listener.exit_restriction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ShExDocVisitor<'input> + 'a> for RestrictionContext<'input>{
	fn accept(&self,visitor: &mut (dyn ShExDocVisitor<'input> + 'a)) {
		visitor.visit_restriction(self);
	}
}

impl<'input> CustomRuleContext<'input> for RestrictionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ShExDocParserContextType;
	fn get_rule_index(&self) -> usize { RULE_restriction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_restriction }
}
antlr_rust::tid!{RestrictionContextExt<'a>}

impl<'input> RestrictionContextExt<'input>{
	fn new(parent: Option<Rc<dyn ShExDocParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RestrictionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RestrictionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RestrictionContextAttrs<'input>: ShExDocParserContext<'input> + BorrowMut<RestrictionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_RESTRICTS
/// Returns `None` if there is no child corresponding to token KW_RESTRICTS
fn KW_RESTRICTS(&self) -> Option<Rc<TerminalNode<'input,ShExDocParserContextType>>> where Self:Sized{
	self.get_token(KW_RESTRICTS, 0)
}
fn shapeRef_all(&self) ->  Vec<Rc<ShapeRefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shapeRef(&self, i: usize) -> Option<Rc<ShapeRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RestrictionContextAttrs<'input> for RestrictionContext<'input>{}

impl<'input, I, H> ShExDocParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn restriction(&mut self,)
	-> Result<Rc<RestrictionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RestrictionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_restriction);
        let mut _localctx: Rc<RestrictionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(773);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 KW_RESTRICTS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(761);
					recog.base.match_token(KW_RESTRICTS,&mut recog.err_handler)?;

					recog.base.set_state(763); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule shapeRef*/
						recog.base.set_state(762);
						recog.shapeRef()?;

						}
						}
						recog.base.set_state(765); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==T__5 || _la==ATPNAME_NS || _la==ATPNAME_LN) {break}
					}
					}
				}

			 T__20 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(767);
					recog.base.match_token(T__20,&mut recog.err_handler)?;

					recog.base.set_state(769); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule shapeRef*/
						recog.base.set_state(768);
						recog.shapeRef()?;

						}
						}
						recog.base.set_state(771); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==T__5 || _la==ATPNAME_NS || _la==ATPNAME_LN) {break}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x51\u{30a}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x03\x02\x07\x02\u{aa}\x0a\x02\
	\x0c\x02\x0e\x02\u{ad}\x0b\x02\x03\x02\x03\x02\x05\x02\u{b1}\x0a\x02\x03\
	\x02\x07\x02\u{b4}\x0a\x02\x0c\x02\x0e\x02\u{b7}\x0b\x02\x05\x02\u{b9}\x0a\
	\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x05\x03\u{c0}\x0a\x03\x03\x04\
	\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\
	\x03\x07\x03\x07\x05\x07\u{ce}\x0a\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\
	\x09\x06\x09\u{d5}\x0a\x09\x0d\x09\x0e\x09\u{d6}\x03\x0a\x03\x0a\x05\x0a\
	\u{db}\x0a\x0a\x03\x0b\x05\x0b\u{de}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x05\
	\x0b\u{e3}\x0a\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\
	\x07\x0e\u{ec}\x0a\x0e\x0c\x0e\x0e\x0e\u{ef}\x0b\x0e\x03\x0f\x03\x0f\x03\
	\x0f\x07\x0f\u{f4}\x0a\x0f\x0c\x0f\x0e\x0f\u{f7}\x0b\x0f\x03\x10\x03\x10\
	\x03\x10\x07\x10\u{fc}\x0a\x10\x0c\x10\x0e\x10\u{ff}\x0b\x10\x03\x11\x03\
	\x11\x03\x11\x07\x11\u{104}\x0a\x11\x0c\x11\x0e\x11\u{107}\x0b\x11\x03\x12\
	\x05\x12\u{10a}\x0a\x12\x03\x12\x03\x12\x03\x13\x05\x13\u{10f}\x0a\x13\x03\
	\x13\x03\x13\x03\x14\x03\x14\x03\x15\x03\x15\x05\x15\u{117}\x0a\x15\x03\
	\x15\x03\x15\x03\x15\x05\x15\u{11c}\x0a\x15\x03\x15\x03\x15\x03\x15\x03\
	\x15\x03\x15\x05\x15\u{123}\x0a\x15\x03\x16\x03\x16\x05\x16\u{127}\x0a\x16\
	\x03\x16\x03\x16\x03\x16\x05\x16\u{12c}\x0a\x16\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x16\x05\x16\u{133}\x0a\x16\x03\x17\x03\x17\x05\x17\u{137}\x0a\
	\x17\x03\x18\x03\x18\x05\x18\u{13b}\x0a\x18\x03\x19\x03\x19\x03\x19\x03\
	\x19\x05\x19\u{141}\x0a\x19\x03\x1a\x03\x1a\x07\x1a\u{145}\x0a\x1a\x0c\x1a\
	\x0e\x1a\u{148}\x0b\x1a\x03\x1a\x03\x1a\x07\x1a\u{14c}\x0a\x1a\x0c\x1a\x0e\
	\x1a\u{14f}\x0b\x1a\x03\x1a\x03\x1a\x07\x1a\u{153}\x0a\x1a\x0c\x1a\x0e\x1a\
	\u{156}\x0b\x1a\x03\x1a\x03\x1a\x07\x1a\u{15a}\x0a\x1a\x0c\x1a\x0e\x1a\u{15d}\
	\x0b\x1a\x03\x1a\x06\x1a\u{160}\x0a\x1a\x0d\x1a\x0e\x1a\u{161}\x05\x1a\u{164}\
	\x0a\x1a\x03\x1b\x03\x1b\x07\x1b\u{168}\x0a\x1b\x0c\x1b\x0e\x1b\u{16b}\x0b\
	\x1b\x03\x1b\x07\x1b\u{16e}\x0a\x1b\x0c\x1b\x0e\x1b\u{171}\x0b\x1b\x03\x1c\
	\x03\x1c\x07\x1c\u{175}\x0a\x1c\x0c\x1c\x0e\x1c\u{178}\x0b\x1c\x03\x1c\x06\
	\x1c\u{17b}\x0a\x1c\x0d\x1c\x0e\x1c\u{17c}\x05\x1c\u{17f}\x0a\x1c\x03\x1d\
	\x03\x1d\x07\x1d\u{183}\x0a\x1d\x0c\x1d\x0e\x1d\u{186}\x0b\x1d\x03\x1d\x07\
	\x1d\u{189}\x0a\x1d\x0c\x1d\x0e\x1d\u{18c}\x0b\x1d\x03\x1e\x03\x1e\x03\x1f\
	\x03\x1f\x05\x1f\u{192}\x0a\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x05\x20\u{199}\x0a\x20\x05\x20\u{19b}\x0a\x20\x03\x21\x03\x21\x03\x22\x03\
	\x22\x03\x22\x03\x22\x03\x22\x03\x22\x05\x22\u{1a5}\x0a\x22\x03\x23\x03\
	\x23\x03\x24\x03\x24\x03\x25\x03\x25\x03\x26\x03\x26\x07\x26\u{1af}\x0a\
	\x26\x0c\x26\x0e\x26\u{1b2}\x0b\x26\x03\x26\x07\x26\u{1b5}\x0a\x26\x0c\x26\
	\x0e\x26\u{1b8}\x0b\x26\x03\x27\x07\x27\u{1bb}\x0a\x27\x0c\x27\x0e\x27\u{1be}\
	\x0b\x27\x03\x27\x03\x27\x05\x27\u{1c2}\x0a\x27\x03\x27\x03\x27\x03\x28\
	\x03\x28\x03\x28\x03\x28\x05\x28\u{1ca}\x0a\x28\x03\x29\x03\x29\x06\x29\
	\u{1ce}\x0a\x29\x0d\x29\x0e\x29\u{1cf}\x03\x2a\x03\x2a\x03\x2b\x03\x2b\x05\
	\x2b\u{1d6}\x0a\x2b\x03\x2c\x03\x2c\x03\x2c\x06\x2c\u{1db}\x0a\x2c\x0d\x2c\
	\x0e\x2c\u{1dc}\x03\x2d\x03\x2d\x05\x2d\u{1e1}\x0a\x2d\x03\x2e\x03\x2e\x05\
	\x2e\u{1e5}\x0a\x2e\x03\x2f\x03\x2f\x03\x2f\x06\x2f\u{1ea}\x0a\x2f\x0d\x2f\
	\x0e\x2f\u{1eb}\x03\x2f\x05\x2f\u{1ef}\x0a\x2f\x03\x30\x03\x30\x05\x30\u{1f3}\
	\x0a\x30\x03\x30\x03\x30\x05\x30\u{1f7}\x0a\x30\x03\x30\x03\x30\x05\x30\
	\u{1fb}\x0a\x30\x03\x31\x03\x31\x03\x31\x03\x31\x05\x31\u{201}\x0a\x31\x03\
	\x31\x07\x31\u{204}\x0a\x31\x0c\x31\x0e\x31\u{207}\x0b\x31\x03\x31\x07\x31\
	\u{20a}\x0a\x31\x0c\x31\x0e\x31\u{20d}\x0b\x31\x03\x32\x05\x32\u{210}\x0a\
	\x32\x03\x32\x03\x32\x03\x32\x05\x32\u{215}\x0a\x32\x03\x32\x07\x32\u{218}\
	\x0a\x32\x0c\x32\x0e\x32\u{21b}\x0b\x32\x03\x32\x07\x32\u{21e}\x0a\x32\x0c\
	\x32\x0e\x32\u{221}\x0b\x32\x03\x33\x03\x33\x03\x33\x03\x33\x05\x33\u{227}\
	\x0a\x33\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x05\x34\
	\u{230}\x0a\x34\x03\x34\x03\x34\x05\x34\u{234}\x0a\x34\x03\x35\x03\x35\x03\
	\x36\x03\x36\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x07\
	\x37\u{241}\x0a\x37\x0c\x37\x0e\x37\u{244}\x0b\x37\x03\x38\x03\x38\x03\x38\
	\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x05\x38\u{250}\
	\x0a\x38\x03\x39\x03\x39\x03\x39\x05\x39\u{255}\x0a\x39\x03\x3a\x03\x3a\
	\x05\x3a\u{259}\x0a\x3a\x03\x3a\x03\x3a\x05\x3a\u{25d}\x0a\x3a\x05\x3a\u{25f}\
	\x0a\x3a\x03\x3b\x03\x3b\x07\x3b\u{263}\x0a\x3b\x0c\x3b\x0e\x3b\u{266}\x0b\
	\x3b\x03\x3b\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x06\x3c\u{26f}\
	\x0a\x3c\x0d\x3c\x0e\x3c\u{270}\x03\x3c\x06\x3c\u{274}\x0a\x3c\x0d\x3c\x0e\
	\x3c\u{275}\x03\x3c\x06\x3c\u{279}\x0a\x3c\x0d\x3c\x0e\x3c\u{27a}\x05\x3c\
	\u{27d}\x0a\x3c\x05\x3c\u{27f}\x0a\x3c\x03\x3d\x03\x3d\x03\x3d\x07\x3d\u{284}\
	\x0a\x3d\x0c\x3d\x0e\x3d\u{287}\x0b\x3d\x05\x3d\u{289}\x0a\x3d\x03\x3e\x03\
	\x3e\x03\x3e\x05\x3e\u{28e}\x0a\x3e\x03\x3f\x03\x3f\x03\x3f\x07\x3f\u{293}\
	\x0a\x3f\x0c\x3f\x0e\x3f\u{296}\x0b\x3f\x05\x3f\u{298}\x0a\x3f\x03\x40\x03\
	\x40\x03\x40\x05\x40\u{29d}\x0a\x40\x03\x41\x03\x41\x03\x41\x07\x41\u{2a2}\
	\x0a\x41\x0c\x41\x0e\x41\u{2a5}\x0b\x41\x05\x41\u{2a7}\x0a\x41\x03\x41\x03\
	\x41\x03\x41\x07\x41\u{2ac}\x0a\x41\x0c\x41\x0e\x41\u{2af}\x0b\x41\x05\x41\
	\u{2b1}\x0a\x41\x03\x42\x03\x42\x03\x42\x05\x42\u{2b6}\x0a\x42\x03\x43\x03\
	\x43\x03\x43\x03\x44\x03\x44\x03\x44\x03\x44\x05\x44\u{2bf}\x0a\x44\x03\
	\x45\x03\x45\x03\x45\x03\x45\x03\x46\x03\x46\x03\x46\x05\x46\u{2c8}\x0a\
	\x46\x03\x47\x03\x47\x05\x47\u{2cc}\x0a\x47\x03\x48\x03\x48\x03\x49\x03\
	\x49\x03\x4a\x03\x4a\x05\x4a\u{2d4}\x0a\x4a\x03\x4b\x03\x4b\x05\x4b\u{2d8}\
	\x0a\x4b\x03\x4c\x03\x4c\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x05\x4d\u{2e0}\
	\x0a\x4d\x03\x4e\x03\x4e\x03\x4f\x03\x4f\x03\x50\x03\x50\x05\x50\u{2e8}\
	\x0a\x50\x03\x51\x03\x51\x03\x52\x03\x52\x03\x53\x03\x53\x06\x53\u{2f0}\
	\x0a\x53\x0d\x53\x0e\x53\u{2f1}\x03\x53\x03\x53\x06\x53\u{2f6}\x0a\x53\x0d\
	\x53\x0e\x53\u{2f7}\x05\x53\u{2fa}\x0a\x53\x03\x54\x03\x54\x06\x54\u{2fe}\
	\x0a\x54\x0d\x54\x0e\x54\u{2ff}\x03\x54\x03\x54\x06\x54\u{304}\x0a\x54\x0d\
	\x54\x0e\x54\u{305}\x05\x54\u{308}\x0a\x54\x03\x54\x02\x03\x6c\x55\x02\x04\
	\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\
	\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\
	\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\
	\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\
	\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\
	\x02\x0d\x04\x02\x04\x04\x3a\x3a\x03\x02\x2c\x2e\x03\x02\x35\x37\x03\x02\
	\x31\x34\x03\x02\x38\x39\x03\x02\x49\x4b\x04\x02\x49\x49\x4d\x4d\x04\x02\
	\x1d\x1d\x3e\x3e\x03\x02\x3b\x3c\x03\x02\x4e\x51\x03\x02\x41\x42\x02\u{33b}\
	\x02\u{ab}\x03\x02\x02\x02\x04\u{bf}\x03\x02\x02\x02\x06\u{c1}\x03\x02\x02\
	\x02\x08\u{c4}\x03\x02\x02\x02\x0a\u{c8}\x03\x02\x02\x02\x0c\u{cd}\x03\x02\
	\x02\x02\x0e\u{cf}\x03\x02\x02\x02\x10\u{d4}\x03\x02\x02\x02\x12\u{da}\x03\
	\x02\x02\x02\x14\u{dd}\x03\x02\x02\x02\x16\u{e4}\x03\x02\x02\x02\x18\u{e6}\
	\x03\x02\x02\x02\x1a\u{e8}\x03\x02\x02\x02\x1c\u{f0}\x03\x02\x02\x02\x1e\
	\u{f8}\x03\x02\x02\x02\x20\u{100}\x03\x02\x02\x02\x22\u{109}\x03\x02\x02\
	\x02\x24\u{10e}\x03\x02\x02\x02\x26\u{112}\x03\x02\x02\x02\x28\u{122}\x03\
	\x02\x02\x02\x2a\u{132}\x03\x02\x02\x02\x2c\u{136}\x03\x02\x02\x02\x2e\u{13a}\
	\x03\x02\x02\x02\x30\u{140}\x03\x02\x02\x02\x32\u{163}\x03\x02\x02\x02\x34\
	\u{165}\x03\x02\x02\x02\x36\u{17e}\x03\x02\x02\x02\x38\u{180}\x03\x02\x02\
	\x02\x3a\u{18d}\x03\x02\x02\x02\x3c\u{191}\x03\x02\x02\x02\x3e\u{19a}\x03\
	\x02\x02\x02\x40\u{19c}\x03\x02\x02\x02\x42\u{1a4}\x03\x02\x02\x02\x44\u{1a6}\
	\x03\x02\x02\x02\x46\u{1a8}\x03\x02\x02\x02\x48\u{1aa}\x03\x02\x02\x02\x4a\
	\u{1ac}\x03\x02\x02\x02\x4c\u{1bc}\x03\x02\x02\x02\x4e\u{1c9}\x03\x02\x02\
	\x02\x50\u{1cb}\x03\x02\x02\x02\x52\u{1d1}\x03\x02\x02\x02\x54\u{1d5}\x03\
	\x02\x02\x02\x56\u{1d7}\x03\x02\x02\x02\x58\u{1e0}\x03\x02\x02\x02\x5a\u{1e2}\
	\x03\x02\x02\x02\x5c\u{1e6}\x03\x02\x02\x02\x5e\u{1fa}\x03\x02\x02\x02\x60\
	\u{1fc}\x03\x02\x02\x02\x62\u{20f}\x03\x02\x02\x02\x64\u{226}\x03\x02\x02\
	\x02\x66\u{233}\x03\x02\x02\x02\x68\u{235}\x03\x02\x02\x02\x6a\u{237}\x03\
	\x02\x02\x02\x6c\u{239}\x03\x02\x02\x02\x6e\u{24f}\x03\x02\x02\x02\x70\u{254}\
	\x03\x02\x02\x02\x72\u{25e}\x03\x02\x02\x02\x74\u{260}\x03\x02\x02\x02\x76\
	\u{27e}\x03\x02\x02\x02\x78\u{280}\x03\x02\x02\x02\x7a\u{28a}\x03\x02\x02\
	\x02\x7c\u{28f}\x03\x02\x02\x02\x7e\u{299}\x03\x02\x02\x02\u{80}\u{2b0}\
	\x03\x02\x02\x02\u{82}\u{2b2}\x03\x02\x02\x02\u{84}\u{2b7}\x03\x02\x02\x02\
	\u{86}\u{2ba}\x03\x02\x02\x02\u{88}\u{2c0}\x03\x02\x02\x02\u{8a}\u{2c7}\
	\x03\x02\x02\x02\u{8c}\u{2cb}\x03\x02\x02\x02\u{8e}\u{2cd}\x03\x02\x02\x02\
	\u{90}\u{2cf}\x03\x02\x02\x02\u{92}\u{2d3}\x03\x02\x02\x02\u{94}\u{2d7}\
	\x03\x02\x02\x02\u{96}\u{2d9}\x03\x02\x02\x02\u{98}\u{2db}\x03\x02\x02\x02\
	\u{9a}\u{2e1}\x03\x02\x02\x02\u{9c}\u{2e3}\x03\x02\x02\x02\u{9e}\u{2e7}\
	\x03\x02\x02\x02\u{a0}\u{2e9}\x03\x02\x02\x02\u{a2}\u{2eb}\x03\x02\x02\x02\
	\u{a4}\u{2f9}\x03\x02\x02\x02\u{a6}\u{307}\x03\x02\x02\x02\u{a8}\u{aa}\x05\
	\x04\x03\x02\u{a9}\u{a8}\x03\x02\x02\x02\u{aa}\u{ad}\x03\x02\x02\x02\u{ab}\
	\u{a9}\x03\x02\x02\x02\u{ab}\u{ac}\x03\x02\x02\x02\u{ac}\u{b8}\x03\x02\x02\
	\x02\u{ad}\u{ab}\x03\x02\x02\x02\u{ae}\u{b1}\x05\x0c\x07\x02\u{af}\u{b1}\
	\x05\x10\x09\x02\u{b0}\u{ae}\x03\x02\x02\x02\u{b0}\u{af}\x03\x02\x02\x02\
	\u{b1}\u{b5}\x03\x02\x02\x02\u{b2}\u{b4}\x05\x12\x0a\x02\u{b3}\u{b2}\x03\
	\x02\x02\x02\u{b4}\u{b7}\x03\x02\x02\x02\u{b5}\u{b3}\x03\x02\x02\x02\u{b5}\
	\u{b6}\x03\x02\x02\x02\u{b6}\u{b9}\x03\x02\x02\x02\u{b7}\u{b5}\x03\x02\x02\
	\x02\u{b8}\u{b0}\x03\x02\x02\x02\u{b8}\u{b9}\x03\x02\x02\x02\u{b9}\u{ba}\
	\x03\x02\x02\x02\u{ba}\u{bb}\x07\x02\x02\x03\u{bb}\x03\x03\x02\x02\x02\u{bc}\
	\u{c0}\x05\x06\x04\x02\u{bd}\u{c0}\x05\x08\x05\x02\u{be}\u{c0}\x05\x0a\x06\
	\x02\u{bf}\u{bc}\x03\x02\x02\x02\u{bf}\u{bd}\x03\x02\x02\x02\u{bf}\u{be}\
	\x03\x02\x02\x02\u{c0}\x05\x03\x02\x02\x02\u{c1}\u{c2}\x07\x21\x02\x02\u{c2}\
	\u{c3}\x07\x40\x02\x02\u{c3}\x07\x03\x02\x02\x02\u{c4}\u{c5}\x07\x26\x02\
	\x02\u{c5}\u{c6}\x07\x41\x02\x02\u{c6}\u{c7}\x07\x40\x02\x02\u{c7}\x09\x03\
	\x02\x02\x02\u{c8}\u{c9}\x07\x23\x02\x02\u{c9}\u{ca}\x05\u{9e}\x50\x02\u{ca}\
	\x0b\x03\x02\x02\x02\u{cb}\u{ce}\x05\x0e\x08\x02\u{cc}\u{ce}\x05\x14\x0b\
	\x02\u{cd}\u{cb}\x03\x02\x02\x02\u{cd}\u{cc}\x03\x02\x02\x02\u{ce}\x0d\x03\
	\x02\x02\x02\u{cf}\u{d0}\x07\x27\x02\x02\u{d0}\u{d1}\x07\x03\x02\x02\u{d1}\
	\u{d2}\x05\x16\x0c\x02\u{d2}\x0f\x03\x02\x02\x02\u{d3}\u{d5}\x05\u{88}\x45\
	\x02\u{d4}\u{d3}\x03\x02\x02\x02\u{d5}\u{d6}\x03\x02\x02\x02\u{d6}\u{d4}\
	\x03\x02\x02\x02\u{d6}\u{d7}\x03\x02\x02\x02\u{d7}\x11\x03\x02\x02\x02\u{d8}\
	\u{db}\x05\x04\x03\x02\u{d9}\u{db}\x05\x0c\x07\x02\u{da}\u{d8}\x03\x02\x02\
	\x02\u{da}\u{d9}\x03\x02\x02\x02\u{db}\x13\x03\x02\x02\x02\u{dc}\u{de}\x07\
	\x1f\x02\x02\u{dd}\u{dc}\x03\x02\x02\x02\u{dd}\u{de}\x03\x02\x02\x02\u{de}\
	\u{df}\x03\x02\x02\x02\u{df}\u{e2}\x05\u{92}\x4a\x02\u{e0}\u{e3}\x05\x16\
	\x0c\x02\u{e1}\u{e3}\x07\x25\x02\x02\u{e2}\u{e0}\x03\x02\x02\x02\u{e2}\u{e1}\
	\x03\x02\x02\x02\u{e3}\x15\x03\x02\x02\x02\u{e4}\u{e5}\x05\x1a\x0e\x02\u{e5}\
	\x17\x03\x02\x02\x02\u{e6}\u{e7}\x05\x1c\x0f\x02\u{e7}\x19\x03\x02\x02\x02\
	\u{e8}\u{ed}\x05\x1e\x10\x02\u{e9}\u{ea}\x07\x30\x02\x02\u{ea}\u{ec}\x05\
	\x1e\x10\x02\u{eb}\u{e9}\x03\x02\x02\x02\u{ec}\u{ef}\x03\x02\x02\x02\u{ed}\
	\u{eb}\x03\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\x02\u{ee}\x1b\x03\x02\x02\
	\x02\u{ef}\u{ed}\x03\x02\x02\x02\u{f0}\u{f5}\x05\x20\x11\x02\u{f1}\u{f2}\
	\x07\x30\x02\x02\u{f2}\u{f4}\x05\x20\x11\x02\u{f3}\u{f1}\x03\x02\x02\x02\
	\u{f4}\u{f7}\x03\x02\x02\x02\u{f5}\u{f3}\x03\x02\x02\x02\u{f5}\u{f6}\x03\
	\x02\x02\x02\u{f6}\x1d\x03\x02\x02\x02\u{f7}\u{f5}\x03\x02\x02\x02\u{f8}\
	\u{fd}\x05\x22\x12\x02\u{f9}\u{fa}\x07\x2f\x02\x02\u{fa}\u{fc}\x05\x22\x12\
	\x02\u{fb}\u{f9}\x03\x02\x02\x02\u{fc}\u{ff}\x03\x02\x02\x02\u{fd}\u{fb}\
	\x03\x02\x02\x02\u{fd}\u{fe}\x03\x02\x02\x02\u{fe}\x1f\x03\x02\x02\x02\u{ff}\
	\u{fd}\x03\x02\x02\x02\u{100}\u{105}\x05\x24\x13\x02\u{101}\u{102}\x07\x2f\
	\x02\x02\u{102}\u{104}\x05\x24\x13\x02\u{103}\u{101}\x03\x02\x02\x02\u{104}\
	\u{107}\x03\x02\x02\x02\u{105}\u{103}\x03\x02\x02\x02\u{105}\u{106}\x03\
	\x02\x02\x02\u{106}\x21\x03\x02\x02\x02\u{107}\u{105}\x03\x02\x02\x02\u{108}\
	\u{10a}\x05\x26\x14\x02\u{109}\u{108}\x03\x02\x02\x02\u{109}\u{10a}\x03\
	\x02\x02\x02\u{10a}\u{10b}\x03\x02\x02\x02\u{10b}\u{10c}\x05\x28\x15\x02\
	\u{10c}\x23\x03\x02\x02\x02\u{10d}\u{10f}\x05\x26\x14\x02\u{10e}\u{10d}\
	\x03\x02\x02\x02\u{10e}\u{10f}\x03\x02\x02\x02\u{10f}\u{110}\x03\x02\x02\
	\x02\u{110}\u{111}\x05\x2a\x16\x02\u{111}\x25\x03\x02\x02\x02\u{112}\u{113}\
	\x09\x02\x02\x02\u{113}\x27\x03\x02\x02\x02\u{114}\u{116}\x05\x38\x1d\x02\
	\u{115}\u{117}\x05\x2c\x17\x02\u{116}\u{115}\x03\x02\x02\x02\u{116}\u{117}\
	\x03\x02\x02\x02\u{117}\u{123}\x03\x02\x02\x02\u{118}\u{123}\x05\x34\x1b\
	\x02\u{119}\u{11b}\x05\x2c\x17\x02\u{11a}\u{11c}\x05\x38\x1d\x02\u{11b}\
	\u{11a}\x03\x02\x02\x02\u{11b}\u{11c}\x03\x02\x02\x02\u{11c}\u{123}\x03\
	\x02\x02\x02\u{11d}\u{11e}\x07\x05\x02\x02\u{11e}\u{11f}\x05\x16\x0c\x02\
	\u{11f}\u{120}\x07\x06\x02\x02\u{120}\u{123}\x03\x02\x02\x02\u{121}\u{123}\
	\x07\x07\x02\x02\u{122}\u{114}\x03\x02\x02\x02\u{122}\u{118}\x03\x02\x02\
	\x02\u{122}\u{119}\x03\x02\x02\x02\u{122}\u{11d}\x03\x02\x02\x02\u{122}\
	\u{121}\x03\x02\x02\x02\u{123}\x29\x03\x02\x02\x02\u{124}\u{126}\x05\x36\
	\x1c\x02\u{125}\u{127}\x05\x2e\x18\x02\u{126}\u{125}\x03\x02\x02\x02\u{126}\
	\u{127}\x03\x02\x02\x02\u{127}\u{133}\x03\x02\x02\x02\u{128}\u{133}\x05\
	\x32\x1a\x02\u{129}\u{12b}\x05\x2e\x18\x02\u{12a}\u{12c}\x05\x36\x1c\x02\
	\u{12b}\u{12a}\x03\x02\x02\x02\u{12b}\u{12c}\x03\x02\x02\x02\u{12c}\u{133}\
	\x03\x02\x02\x02\u{12d}\u{12e}\x07\x05\x02\x02\u{12e}\u{12f}\x05\x16\x0c\
	\x02\u{12f}\u{130}\x07\x06\x02\x02\u{130}\u{133}\x03\x02\x02\x02\u{131}\
	\u{133}\x07\x07\x02\x02\u{132}\u{124}\x03\x02\x02\x02\u{132}\u{128}\x03\
	\x02\x02\x02\u{132}\u{129}\x03\x02\x02\x02\u{132}\u{12d}\x03\x02\x02\x02\
	\u{132}\u{131}\x03\x02\x02\x02\u{133}\x2b\x03\x02\x02\x02\u{134}\u{137}\
	\x05\x4a\x26\x02\u{135}\u{137}\x05\x30\x19\x02\u{136}\u{134}\x03\x02\x02\
	\x02\u{136}\u{135}\x03\x02\x02\x02\u{137}\x2d\x03\x02\x02\x02\u{138}\u{13b}\
	\x05\x4c\x27\x02\u{139}\u{13b}\x05\x30\x19\x02\u{13a}\u{138}\x03\x02\x02\
	\x02\u{13a}\u{139}\x03\x02\x02\x02\u{13b}\x2f\x03\x02\x02\x02\u{13c}\u{141}\
	\x07\x44\x02\x02\u{13d}\u{141}\x07\x43\x02\x02\u{13e}\u{13f}\x07\x08\x02\
	\x02\u{13f}\u{141}\x05\u{92}\x4a\x02\u{140}\u{13c}\x03\x02\x02\x02\u{140}\
	\u{13d}\x03\x02\x02\x02\u{140}\u{13e}\x03\x02\x02\x02\u{141}\x31\x03\x02\
	\x02\x02\u{142}\u{146}\x07\x2b\x02\x02\u{143}\u{145}\x05\x3c\x1f\x02\u{144}\
	\u{143}\x03\x02\x02\x02\u{145}\u{148}\x03\x02\x02\x02\u{146}\u{144}\x03\
	\x02\x02\x02\u{146}\u{147}\x03\x02\x02\x02\u{147}\u{164}\x03\x02\x02\x02\
	\u{148}\u{146}\x03\x02\x02\x02\u{149}\u{14d}\x05\x3a\x1e\x02\u{14a}\u{14c}\
	\x05\x3e\x20\x02\u{14b}\u{14a}\x03\x02\x02\x02\u{14c}\u{14f}\x03\x02\x02\
	\x02\u{14d}\u{14b}\x03\x02\x02\x02\u{14d}\u{14e}\x03\x02\x02\x02\u{14e}\
	\u{164}\x03\x02\x02\x02\u{14f}\u{14d}\x03\x02\x02\x02\u{150}\u{154}\x05\
	\u{90}\x49\x02\u{151}\u{153}\x05\x3c\x1f\x02\u{152}\u{151}\x03\x02\x02\x02\
	\u{153}\u{156}\x03\x02\x02\x02\u{154}\u{152}\x03\x02\x02\x02\u{154}\u{155}\
	\x03\x02\x02\x02\u{155}\u{164}\x03\x02\x02\x02\u{156}\u{154}\x03\x02\x02\
	\x02\u{157}\u{15b}\x05\x74\x3b\x02\u{158}\u{15a}\x05\x3c\x1f\x02\u{159}\
	\u{158}\x03\x02\x02\x02\u{15a}\u{15d}\x03\x02\x02\x02\u{15b}\u{159}\x03\
	\x02\x02\x02\u{15b}\u{15c}\x03\x02\x02\x02\u{15c}\u{164}\x03\x02\x02\x02\
	\u{15d}\u{15b}\x03\x02\x02\x02\u{15e}\u{160}\x05\x42\x22\x02\u{15f}\u{15e}\
	\x03\x02\x02\x02\u{160}\u{161}\x03\x02\x02\x02\u{161}\u{15f}\x03\x02\x02\
	\x02\u{161}\u{162}\x03\x02\x02\x02\u{162}\u{164}\x03\x02\x02\x02\u{163}\
	\u{142}\x03\x02\x02\x02\u{163}\u{149}\x03\x02\x02\x02\u{163}\u{150}\x03\
	\x02\x02\x02\u{163}\u{157}\x03\x02\x02\x02\u{163}\u{15f}\x03\x02\x02\x02\
	\u{164}\x33\x03\x02\x02\x02\u{165}\u{169}\x05\x32\x1a\x02\u{166}\u{168}\
	\x05\u{86}\x44\x02\u{167}\u{166}\x03\x02\x02\x02\u{168}\u{16b}\x03\x02\x02\
	\x02\u{169}\u{167}\x03\x02\x02\x02\u{169}\u{16a}\x03\x02\x02\x02\u{16a}\
	\u{16f}\x03\x02\x02\x02\u{16b}\u{169}\x03\x02\x02\x02\u{16c}\u{16e}\x05\
	\u{88}\x45\x02\u{16d}\u{16c}\x03\x02\x02\x02\u{16e}\u{171}\x03\x02\x02\x02\
	\u{16f}\u{16d}\x03\x02\x02\x02\u{16f}\u{170}\x03\x02\x02\x02\u{170}\x35\
	\x03\x02\x02\x02\u{171}\u{16f}\x03\x02\x02\x02\u{172}\u{176}\x05\x3a\x1e\
	\x02\u{173}\u{175}\x05\x3e\x20\x02\u{174}\u{173}\x03\x02\x02\x02\u{175}\
	\u{178}\x03\x02\x02\x02\u{176}\u{174}\x03\x02\x02\x02\u{176}\u{177}\x03\
	\x02\x02\x02\u{177}\u{17f}\x03\x02\x02\x02\u{178}\u{176}\x03\x02\x02\x02\
	\u{179}\u{17b}\x05\x3e\x20\x02\u{17a}\u{179}\x03\x02\x02\x02\u{17b}\u{17c}\
	\x03\x02\x02\x02\u{17c}\u{17a}\x03\x02\x02\x02\u{17c}\u{17d}\x03\x02\x02\
	\x02\u{17d}\u{17f}\x03\x02\x02\x02\u{17e}\u{172}\x03\x02\x02\x02\u{17e}\
	\u{17a}\x03\x02\x02\x02\u{17f}\x37\x03\x02\x02\x02\u{180}\u{184}\x05\x36\
	\x1c\x02\u{181}\u{183}\x05\u{86}\x44\x02\u{182}\u{181}\x03\x02\x02\x02\u{183}\
	\u{186}\x03\x02\x02\x02\u{184}\u{182}\x03\x02\x02\x02\u{184}\u{185}\x03\
	\x02\x02\x02\u{185}\u{18a}\x03\x02\x02\x02\u{186}\u{184}\x03\x02\x02\x02\
	\u{187}\u{189}\x05\u{88}\x45\x02\u{188}\u{187}\x03\x02\x02\x02\u{189}\u{18c}\
	\x03\x02\x02\x02\u{18a}\u{188}\x03\x02\x02\x02\u{18a}\u{18b}\x03\x02\x02\
	\x02\u{18b}\x39\x03\x02\x02\x02\u{18c}\u{18a}\x03\x02\x02\x02\u{18d}\u{18e}\
	\x09\x03\x02\x02\u{18e}\x3b\x03\x02\x02\x02\u{18f}\u{192}\x05\x3e\x20\x02\
	\u{190}\u{192}\x05\x42\x22\x02\u{191}\u{18f}\x03\x02\x02\x02\u{191}\u{190}\
	\x03\x02\x02\x02\u{192}\x3d\x03\x02\x02\x02\u{193}\u{194}\x05\x40\x21\x02\
	\u{194}\u{195}\x07\x49\x02\x02\u{195}\u{19b}\x03\x02\x02\x02\u{196}\u{198}\
	\x07\x45\x02\x02\u{197}\u{199}\x07\x46\x02\x02\u{198}\u{197}\x03\x02\x02\
	\x02\u{198}\u{199}\x03\x02\x02\x02\u{199}\u{19b}\x03\x02\x02\x02\u{19a}\
	\u{193}\x03\x02\x02\x02\u{19a}\u{196}\x03\x02\x02\x02\u{19b}\x3f\x03\x02\
	\x02\x02\u{19c}\u{19d}\x09\x04\x02\x02\u{19d}\x41\x03\x02\x02\x02\u{19e}\
	\u{19f}\x05\x44\x23\x02\u{19f}\u{1a0}\x05\x48\x25\x02\u{1a0}\u{1a5}\x03\
	\x02\x02\x02\u{1a1}\u{1a2}\x05\x46\x24\x02\u{1a2}\u{1a3}\x07\x49\x02\x02\
	\u{1a3}\u{1a5}\x03\x02\x02\x02\u{1a4}\u{19e}\x03\x02\x02\x02\u{1a4}\u{1a1}\
	\x03\x02\x02\x02\u{1a5}\x43\x03\x02\x02\x02\u{1a6}\u{1a7}\x09\x05\x02\x02\
	\u{1a7}\x45\x03\x02\x02\x02\u{1a8}\u{1a9}\x09\x06\x02\x02\u{1a9}\x47\x03\
	\x02\x02\x02\u{1aa}\u{1ab}\x09\x07\x02\x02\u{1ab}\x49\x03\x02\x02\x02\u{1ac}\
	\u{1b0}\x05\x4c\x27\x02\u{1ad}\u{1af}\x05\u{86}\x44\x02\u{1ae}\u{1ad}\x03\
	\x02\x02\x02\u{1af}\u{1b2}\x03\x02\x02\x02\u{1b0}\u{1ae}\x03\x02\x02\x02\
	\u{1b0}\u{1b1}\x03\x02\x02\x02\u{1b1}\u{1b6}\x03\x02\x02\x02\u{1b2}\u{1b0}\
	\x03\x02\x02\x02\u{1b3}\u{1b5}\x05\u{88}\x45\x02\u{1b4}\u{1b3}\x03\x02\x02\
	\x02\u{1b5}\u{1b8}\x03\x02\x02\x02\u{1b6}\u{1b4}\x03\x02\x02\x02\u{1b6}\
	\u{1b7}\x03\x02\x02\x02\u{1b7}\x4b\x03\x02\x02\x02\u{1b8}\u{1b6}\x03\x02\
	\x02\x02\u{1b9}\u{1bb}\x05\x4e\x28\x02\u{1ba}\u{1b9}\x03\x02\x02\x02\u{1bb}\
	\u{1be}\x03\x02\x02\x02\u{1bc}\u{1ba}\x03\x02\x02\x02\u{1bc}\u{1bd}\x03\
	\x02\x02\x02\u{1bd}\u{1bf}\x03\x02\x02\x02\u{1be}\u{1bc}\x03\x02\x02\x02\
	\u{1bf}\u{1c1}\x07\x09\x02\x02\u{1c0}\u{1c2}\x05\x52\x2a\x02\u{1c1}\u{1c0}\
	\x03\x02\x02\x02\u{1c1}\u{1c2}\x03\x02\x02\x02\u{1c2}\u{1c3}\x03\x02\x02\
	\x02\u{1c3}\u{1c4}\x07\x0a\x02\x02\u{1c4}\x4d\x03\x02\x02\x02\u{1c5}\u{1ca}\
	\x05\u{a4}\x53\x02\u{1c6}\u{1ca}\x05\u{a6}\x54\x02\u{1c7}\u{1ca}\x05\x50\
	\x29\x02\u{1c8}\u{1ca}\x07\x29\x02\x02\u{1c9}\u{1c5}\x03\x02\x02\x02\u{1c9}\
	\u{1c6}\x03\x02\x02\x02\u{1c9}\u{1c7}\x03\x02\x02\x02\u{1c9}\u{1c8}\x03\
	\x02\x02\x02\u{1ca}\x4f\x03\x02\x02\x02\u{1cb}\u{1cd}\x07\x2a\x02\x02\u{1cc}\
	\u{1ce}\x05\u{8c}\x47\x02\u{1cd}\u{1cc}\x03\x02\x02\x02\u{1ce}\u{1cf}\x03\
	\x02\x02\x02\u{1cf}\u{1cd}\x03\x02\x02\x02\u{1cf}\u{1d0}\x03\x02\x02\x02\
	\u{1d0}\x51\x03\x02\x02\x02\u{1d1}\u{1d2}\x05\x54\x2b\x02\u{1d2}\x53\x03\
	\x02\x02\x02\u{1d3}\u{1d6}\x05\x58\x2d\x02\u{1d4}\u{1d6}\x05\x56\x2c\x02\
	\u{1d5}\u{1d3}\x03\x02\x02\x02\u{1d5}\u{1d4}\x03\x02\x02\x02\u{1d6}\x55\
	\x03\x02\x02\x02\u{1d7}\u{1da}\x05\x58\x2d\x02\u{1d8}\u{1d9}\x07\x0b\x02\
	\x02\u{1d9}\u{1db}\x05\x58\x2d\x02\u{1da}\u{1d8}\x03\x02\x02\x02\u{1db}\
	\u{1dc}\x03\x02\x02\x02\u{1dc}\u{1da}\x03\x02\x02\x02\u{1dc}\u{1dd}\x03\
	\x02\x02\x02\u{1dd}\x57\x03\x02\x02\x02\u{1de}\u{1e1}\x05\x5a\x2e\x02\u{1df}\
	\u{1e1}\x05\x5c\x2f\x02\u{1e0}\u{1de}\x03\x02\x02\x02\u{1e0}\u{1df}\x03\
	\x02\x02\x02\u{1e1}\x59\x03\x02\x02\x02\u{1e2}\u{1e4}\x05\x5e\x30\x02\u{1e3}\
	\u{1e5}\x07\x0c\x02\x02\u{1e4}\u{1e3}\x03\x02\x02\x02\u{1e4}\u{1e5}\x03\
	\x02\x02\x02\u{1e5}\x5b\x03\x02\x02\x02\u{1e6}\u{1e9}\x05\x5e\x30\x02\u{1e7}\
	\u{1e8}\x07\x0c\x02\x02\u{1e8}\u{1ea}\x05\x5e\x30\x02\u{1e9}\u{1e7}\x03\
	\x02\x02\x02\u{1ea}\u{1eb}\x03\x02\x02\x02\u{1eb}\u{1e9}\x03\x02\x02\x02\
	\u{1eb}\u{1ec}\x03\x02\x02\x02\u{1ec}\u{1ee}\x03\x02\x02\x02\u{1ed}\u{1ef}\
	\x07\x0c\x02\x02\u{1ee}\u{1ed}\x03\x02\x02\x02\u{1ee}\u{1ef}\x03\x02\x02\
	\x02\u{1ef}\x5d\x03\x02\x02\x02\u{1f0}\u{1f1}\x07\x0d\x02\x02\u{1f1}\u{1f3}\
	\x05\u{94}\x4b\x02\u{1f2}\u{1f0}\x03\x02\x02\x02\u{1f2}\u{1f3}\x03\x02\x02\
	\x02\u{1f3}\u{1f6}\x03\x02\x02\x02\u{1f4}\u{1f7}\x05\x62\x32\x02\u{1f5}\
	\u{1f7}\x05\x60\x31\x02\u{1f6}\u{1f4}\x03\x02\x02\x02\u{1f6}\u{1f5}\x03\
	\x02\x02\x02\u{1f7}\u{1fb}\x03\x02\x02\x02\u{1f8}\u{1fb}\x05\u{84}\x43\x02\
	\u{1f9}\u{1fb}\x05\x6c\x37\x02\u{1fa}\u{1f2}\x03\x02\x02\x02\u{1fa}\u{1f8}\
	\x03\x02\x02\x02\u{1fa}\u{1f9}\x03\x02\x02\x02\u{1fb}\x5f\x03\x02\x02\x02\
	\u{1fc}\u{1fd}\x07\x05\x02\x02\u{1fd}\u{1fe}\x05\x52\x2a\x02\u{1fe}\u{200}\
	\x07\x06\x02\x02\u{1ff}\u{201}\x05\x64\x33\x02\u{200}\u{1ff}\x03\x02\x02\
	\x02\u{200}\u{201}\x03\x02\x02\x02\u{201}\u{205}\x03\x02\x02\x02\u{202}\
	\u{204}\x05\u{86}\x44\x02\u{203}\u{202}\x03\x02\x02\x02\u{204}\u{207}\x03\
	\x02\x02\x02\u{205}\u{203}\x03\x02\x02\x02\u{205}\u{206}\x03\x02\x02\x02\
	\u{206}\u{20b}\x03\x02\x02\x02\u{207}\u{205}\x03\x02\x02\x02\u{208}\u{20a}\
	\x05\u{88}\x45\x02\u{209}\u{208}\x03\x02\x02\x02\u{20a}\u{20d}\x03\x02\x02\
	\x02\u{20b}\u{209}\x03\x02\x02\x02\u{20b}\u{20c}\x03\x02\x02\x02\u{20c}\
	\x61\x03\x02\x02\x02\u{20d}\u{20b}\x03\x02\x02\x02\u{20e}\u{210}\x05\x72\
	\x3a\x02\u{20f}\u{20e}\x03\x02\x02\x02\u{20f}\u{210}\x03\x02\x02\x02\u{210}\
	\u{211}\x03\x02\x02\x02\u{211}\u{212}\x05\u{8c}\x47\x02\u{212}\u{214}\x05\
	\x18\x0d\x02\u{213}\u{215}\x05\x64\x33\x02\u{214}\u{213}\x03\x02\x02\x02\
	\u{214}\u{215}\x03\x02\x02\x02\u{215}\u{219}\x03\x02\x02\x02\u{216}\u{218}\
	\x05\u{86}\x44\x02\u{217}\u{216}\x03\x02\x02\x02\u{218}\u{21b}\x03\x02\x02\
	\x02\u{219}\u{217}\x03\x02\x02\x02\u{219}\u{21a}\x03\x02\x02\x02\u{21a}\
	\u{21f}\x03\x02\x02\x02\u{21b}\u{219}\x03\x02\x02\x02\u{21c}\u{21e}\x05\
	\u{88}\x45\x02\u{21d}\u{21c}\x03\x02\x02\x02\u{21e}\u{221}\x03\x02\x02\x02\
	\u{21f}\u{21d}\x03\x02\x02\x02\u{21f}\u{220}\x03\x02\x02\x02\u{220}\x63\
	\x03\x02\x02\x02\u{221}\u{21f}\x03\x02\x02\x02\u{222}\u{227}\x07\x4d\x02\
	\x02\u{223}\u{227}\x07\x0e\x02\x02\u{224}\u{227}\x07\x0f\x02\x02\u{225}\
	\u{227}\x05\x66\x34\x02\u{226}\u{222}\x03\x02\x02\x02\u{226}\u{223}\x03\
	\x02\x02\x02\u{226}\u{224}\x03\x02\x02\x02\u{226}\u{225}\x03\x02\x02\x02\
	\u{227}\x65\x03\x02\x02\x02\u{228}\u{229}\x07\x09\x02\x02\u{229}\u{22a}\
	\x07\x49\x02\x02\u{22a}\u{234}\x07\x0a\x02\x02\u{22b}\u{22c}\x07\x09\x02\
	\x02\u{22c}\u{22d}\x05\x68\x35\x02\u{22d}\u{22f}\x07\x10\x02\x02\u{22e}\
	\u{230}\x05\x6a\x36\x02\u{22f}\u{22e}\x03\x02\x02\x02\u{22f}\u{230}\x03\
	\x02\x02\x02\u{230}\u{231}\x03\x02\x02\x02\u{231}\u{232}\x07\x0a\x02\x02\
	\u{232}\u{234}\x03\x02\x02\x02\u{233}\u{228}\x03\x02\x02\x02\u{233}\u{22b}\
	\x03\x02\x02\x02\u{234}\x67\x03\x02\x02\x02\u{235}\u{236}\x07\x49\x02\x02\
	\u{236}\x69\x03\x02\x02\x02\u{237}\u{238}\x09\x08\x02\x02\u{238}\x6b\x03\
	\x02\x02\x02\u{239}\u{23a}\x08\x37\x01\x02\u{23a}\u{23b}\x05\x70\x39\x02\
	\u{23b}\u{242}\x03\x02\x02\x02\u{23c}\u{23d}\x0c\x04\x02\x02\u{23d}\u{23e}\
	\x05\x6e\x38\x02\u{23e}\u{23f}\x05\x6c\x37\x05\u{23f}\u{241}\x03\x02\x02\
	\x02\u{240}\u{23c}\x03\x02\x02\x02\u{241}\u{244}\x03\x02\x02\x02\u{242}\
	\u{240}\x03\x02\x02\x02\u{242}\u{243}\x03\x02\x02\x02\u{243}\x6d\x03\x02\
	\x02\x02\u{244}\u{242}\x03\x02\x02\x02\u{245}\u{250}\x07\x03\x02\x02\u{246}\
	\u{250}\x07\x11\x02\x02\u{247}\u{250}\x07\x12\x02\x02\u{248}\u{250}\x07\
	\x13\x02\x02\u{249}\u{250}\x07\x14\x02\x02\u{24a}\u{250}\x07\x15\x02\x02\
	\u{24b}\u{250}\x07\x4d\x02\x02\u{24c}\u{250}\x07\x16\x02\x02\u{24d}\u{250}\
	\x07\x0e\x02\x02\u{24e}\u{250}\x07\x17\x02\x02\u{24f}\u{245}\x03\x02\x02\
	\x02\u{24f}\u{246}\x03\x02\x02\x02\u{24f}\u{247}\x03\x02\x02\x02\u{24f}\
	\u{248}\x03\x02\x02\x02\u{24f}\u{249}\x03\x02\x02\x02\u{24f}\u{24a}\x03\
	\x02\x02\x02\u{24f}\u{24b}\x03\x02\x02\x02\u{24f}\u{24c}\x03\x02\x02\x02\
	\u{24f}\u{24d}\x03\x02\x02\x02\u{24f}\u{24e}\x03\x02\x02\x02\u{250}\x6f\
	\x03\x02\x02\x02\u{251}\u{255}\x05\u{8a}\x46\x02\u{252}\u{255}\x05\u{9e}\
	\x50\x02\u{253}\u{255}\x05\u{a2}\x52\x02\u{254}\u{251}\x03\x02\x02\x02\u{254}\
	\u{252}\x03\x02\x02\x02\u{254}\u{253}\x03\x02\x02\x02\u{255}\x71\x03\x02\
	\x02\x02\u{256}\u{258}\x07\x04\x02\x02\u{257}\u{259}\x07\x18\x02\x02\u{258}\
	\u{257}\x03\x02\x02\x02\u{258}\u{259}\x03\x02\x02\x02\u{259}\u{25f}\x03\
	\x02\x02\x02\u{25a}\u{25c}\x07\x18\x02\x02\u{25b}\u{25d}\x07\x04\x02\x02\
	\u{25c}\u{25b}\x03\x02\x02\x02\u{25c}\u{25d}\x03\x02\x02\x02\u{25d}\u{25f}\
	\x03\x02\x02\x02\u{25e}\u{256}\x03\x02\x02\x02\u{25e}\u{25a}\x03\x02\x02\
	\x02\u{25f}\x73\x03\x02\x02\x02\u{260}\u{264}\x07\x19\x02\x02\u{261}\u{263}\
	\x05\x76\x3c\x02\u{262}\u{261}\x03\x02\x02\x02\u{263}\u{266}\x03\x02\x02\
	\x02\u{264}\u{262}\x03\x02\x02\x02\u{264}\u{265}\x03\x02\x02\x02\u{265}\
	\u{267}\x03\x02\x02\x02\u{266}\u{264}\x03\x02\x02\x02\u{267}\u{268}\x07\
	\x1a\x02\x02\u{268}\x75\x03\x02\x02\x02\u{269}\u{27f}\x05\x78\x3d\x02\u{26a}\
	\u{27f}\x05\x7c\x3f\x02\u{26b}\u{27f}\x05\u{80}\x41\x02\u{26c}\u{27c}\x07\
	\x07\x02\x02\u{26d}\u{26f}\x05\x7a\x3e\x02\u{26e}\u{26d}\x03\x02\x02\x02\
	\u{26f}\u{270}\x03\x02\x02\x02\u{270}\u{26e}\x03\x02\x02\x02\u{270}\u{271}\
	\x03\x02\x02\x02\u{271}\u{27d}\x03\x02\x02\x02\u{272}\u{274}\x05\x7e\x40\
	\x02\u{273}\u{272}\x03\x02\x02\x02\u{274}\u{275}\x03\x02\x02\x02\u{275}\
	\u{273}\x03\x02\x02\x02\u{275}\u{276}\x03\x02\x02\x02\u{276}\u{27d}\x03\
	\x02\x02\x02\u{277}\u{279}\x05\u{82}\x42\x02\u{278}\u{277}\x03\x02\x02\x02\
	\u{279}\u{27a}\x03\x02\x02\x02\u{27a}\u{278}\x03\x02\x02\x02\u{27a}\u{27b}\
	\x03\x02\x02\x02\u{27b}\u{27d}\x03\x02\x02\x02\u{27c}\u{26e}\x03\x02\x02\
	\x02\u{27c}\u{273}\x03\x02\x02\x02\u{27c}\u{278}\x03\x02\x02\x02\u{27d}\
	\u{27f}\x03\x02\x02\x02\u{27e}\u{269}\x03\x02\x02\x02\u{27e}\u{26a}\x03\
	\x02\x02\x02\u{27e}\u{26b}\x03\x02\x02\x02\u{27e}\u{26c}\x03\x02\x02\x02\
	\u{27f}\x77\x03\x02\x02\x02\u{280}\u{288}\x05\u{9e}\x50\x02\u{281}\u{285}\
	\x07\x4c\x02\x02\u{282}\u{284}\x05\x7a\x3e\x02\u{283}\u{282}\x03\x02\x02\
	\x02\u{284}\u{287}\x03\x02\x02\x02\u{285}\u{283}\x03\x02\x02\x02\u{285}\
	\u{286}\x03\x02\x02\x02\u{286}\u{289}\x03\x02\x02\x02\u{287}\u{285}\x03\
	\x02\x02\x02\u{288}\u{281}\x03\x02\x02\x02\u{288}\u{289}\x03\x02\x02\x02\
	\u{289}\x79\x03\x02\x02\x02\u{28a}\u{28b}\x07\x17\x02\x02\u{28b}\u{28d}\
	\x05\u{9e}\x50\x02\u{28c}\u{28e}\x07\x4c\x02\x02\u{28d}\u{28c}\x03\x02\x02\
	\x02\u{28d}\u{28e}\x03\x02\x02\x02\u{28e}\x7b\x03\x02\x02\x02\u{28f}\u{297}\
	\x05\u{8a}\x46\x02\u{290}\u{294}\x07\x4c\x02\x02\u{291}\u{293}\x05\x7e\x40\
	\x02\u{292}\u{291}\x03\x02\x02\x02\u{293}\u{296}\x03\x02\x02\x02\u{294}\
	\u{292}\x03\x02\x02\x02\u{294}\u{295}\x03\x02\x02\x02\u{295}\u{298}\x03\
	\x02\x02\x02\u{296}\u{294}\x03\x02\x02\x02\u{297}\u{290}\x03\x02\x02\x02\
	\u{297}\u{298}\x03\x02\x02\x02\u{298}\x7d\x03\x02\x02\x02\u{299}\u{29a}\
	\x07\x17\x02\x02\u{29a}\u{29c}\x05\u{8a}\x46\x02\u{29b}\u{29d}\x07\x4c\x02\
	\x02\u{29c}\u{29b}\x03\x02\x02\x02\u{29c}\u{29d}\x03\x02\x02\x02\u{29d}\
	\x7f\x03\x02\x02\x02\u{29e}\u{2a6}\x07\x48\x02\x02\u{29f}\u{2a3}\x07\x4c\
	\x02\x02\u{2a0}\u{2a2}\x05\u{82}\x42\x02\u{2a1}\u{2a0}\x03\x02\x02\x02\u{2a2}\
	\u{2a5}\x03\x02\x02\x02\u{2a3}\u{2a1}\x03\x02\x02\x02\u{2a3}\u{2a4}\x03\
	\x02\x02\x02\u{2a4}\u{2a7}\x03\x02\x02\x02\u{2a5}\u{2a3}\x03\x02\x02\x02\
	\u{2a6}\u{29f}\x03\x02\x02\x02\u{2a6}\u{2a7}\x03\x02\x02\x02\u{2a7}\u{2b1}\
	\x03\x02\x02\x02\u{2a8}\u{2a9}\x07\x08\x02\x02\u{2a9}\u{2ad}\x07\x4c\x02\
	\x02\u{2aa}\u{2ac}\x05\u{82}\x42\x02\u{2ab}\u{2aa}\x03\x02\x02\x02\u{2ac}\
	\u{2af}\x03\x02\x02\x02\u{2ad}\u{2ab}\x03\x02\x02\x02\u{2ad}\u{2ae}\x03\
	\x02\x02\x02\u{2ae}\u{2b1}\x03\x02\x02\x02\u{2af}\u{2ad}\x03\x02\x02\x02\
	\u{2b0}\u{29e}\x03\x02\x02\x02\u{2b0}\u{2a8}\x03\x02\x02\x02\u{2b1}\u{81}\
	\x03\x02\x02\x02\u{2b2}\u{2b3}\x07\x17\x02\x02\u{2b3}\u{2b5}\x07\x48\x02\
	\x02\u{2b4}\u{2b6}\x07\x4c\x02\x02\u{2b5}\u{2b4}\x03\x02\x02\x02\u{2b5}\
	\u{2b6}\x03\x02\x02\x02\u{2b6}\u{83}\x03\x02\x02\x02\u{2b7}\u{2b8}\x07\x1b\
	\x02\x02\u{2b8}\u{2b9}\x05\u{94}\x4b\x02\u{2b9}\u{85}\x03\x02\x02\x02\u{2ba}\
	\u{2bb}\x07\x1c\x02\x02\u{2bb}\u{2be}\x05\u{8c}\x47\x02\u{2bc}\u{2bf}\x05\
	\u{9e}\x50\x02\u{2bd}\u{2bf}\x05\u{8a}\x46\x02\u{2be}\u{2bc}\x03\x02\x02\
	\x02\u{2be}\u{2bd}\x03\x02\x02\x02\u{2bf}\u{87}\x03\x02\x02\x02\u{2c0}\u{2c1}\
	\x07\x1d\x02\x02\u{2c1}\u{2c2}\x05\u{9e}\x50\x02\u{2c2}\u{2c3}\x09\x09\x02\
	\x02\u{2c3}\u{89}\x03\x02\x02\x02\u{2c4}\u{2c8}\x05\u{98}\x4d\x02\u{2c5}\
	\u{2c8}\x05\u{96}\x4c\x02\u{2c6}\u{2c8}\x05\u{9a}\x4e\x02\u{2c7}\u{2c4}\
	\x03\x02\x02\x02\u{2c7}\u{2c5}\x03\x02\x02\x02\u{2c7}\u{2c6}\x03\x02\x02\
	\x02\u{2c8}\u{8b}\x03\x02\x02\x02\u{2c9}\u{2cc}\x05\u{9e}\x50\x02\u{2ca}\
	\u{2cc}\x05\u{8e}\x48\x02\u{2cb}\u{2c9}\x03\x02\x02\x02\u{2cb}\u{2ca}\x03\
	\x02\x02\x02\u{2cc}\u{8d}\x03\x02\x02\x02\u{2cd}\u{2ce}\x07\x3f\x02\x02\
	\u{2ce}\u{8f}\x03\x02\x02\x02\u{2cf}\u{2d0}\x05\u{9e}\x50\x02\u{2d0}\u{91}\
	\x03\x02\x02\x02\u{2d1}\u{2d4}\x05\u{9e}\x50\x02\u{2d2}\u{2d4}\x05\u{a2}\
	\x52\x02\u{2d3}\u{2d1}\x03\x02\x02\x02\u{2d3}\u{2d2}\x03\x02\x02\x02\u{2d4}\
	\u{93}\x03\x02\x02\x02\u{2d5}\u{2d8}\x05\u{9e}\x50\x02\u{2d6}\u{2d8}\x05\
	\u{a2}\x52\x02\u{2d7}\u{2d5}\x03\x02\x02\x02\u{2d7}\u{2d6}\x03\x02\x02\x02\
	\u{2d8}\u{95}\x03\x02\x02\x02\u{2d9}\u{2da}\x09\x07\x02\x02\u{2da}\u{97}\
	\x03\x02\x02\x02\u{2db}\u{2df}\x05\u{9c}\x4f\x02\u{2dc}\u{2e0}\x07\x48\x02\
	\x02\u{2dd}\u{2de}\x07\x1e\x02\x02\u{2de}\u{2e0}\x05\u{90}\x49\x02\u{2df}\
	\u{2dc}\x03\x02\x02\x02\u{2df}\u{2dd}\x03\x02\x02\x02\u{2df}\u{2e0}\x03\
	\x02\x02\x02\u{2e0}\u{99}\x03\x02\x02\x02\u{2e1}\u{2e2}\x09\x0a\x02\x02\
	\u{2e2}\u{9b}\x03\x02\x02\x02\u{2e3}\u{2e4}\x09\x0b\x02\x02\u{2e4}\u{9d}\
	\x03\x02\x02\x02\u{2e5}\u{2e8}\x07\x40\x02\x02\u{2e6}\u{2e8}\x05\u{a0}\x51\
	\x02\u{2e7}\u{2e5}\x03\x02\x02\x02\u{2e7}\u{2e6}\x03\x02\x02\x02\u{2e8}\
	\u{9f}\x03\x02\x02\x02\u{2e9}\u{2ea}\x09\x0c\x02\x02\u{2ea}\u{a1}\x03\x02\
	\x02\x02\u{2eb}\u{2ec}\x07\x47\x02\x02\u{2ec}\u{a3}\x03\x02\x02\x02\u{2ed}\
	\u{2ef}\x07\x22\x02\x02\u{2ee}\u{2f0}\x05\x30\x19\x02\u{2ef}\u{2ee}\x03\
	\x02\x02\x02\u{2f0}\u{2f1}\x03\x02\x02\x02\u{2f1}\u{2ef}\x03\x02\x02\x02\
	\u{2f1}\u{2f2}\x03\x02\x02\x02\u{2f2}\u{2fa}\x03\x02\x02\x02\u{2f3}\u{2f5}\
	\x07\x1b\x02\x02\u{2f4}\u{2f6}\x05\x30\x19\x02\u{2f5}\u{2f4}\x03\x02\x02\
	\x02\u{2f6}\u{2f7}\x03\x02\x02\x02\u{2f7}\u{2f5}\x03\x02\x02\x02\u{2f7}\
	\u{2f8}\x03\x02\x02\x02\u{2f8}\u{2fa}\x03\x02\x02\x02\u{2f9}\u{2ed}\x03\
	\x02\x02\x02\u{2f9}\u{2f3}\x03\x02\x02\x02\u{2fa}\u{a5}\x03\x02\x02\x02\
	\u{2fb}\u{2fd}\x07\x24\x02\x02\u{2fc}\u{2fe}\x05\x30\x19\x02\u{2fd}\u{2fc}\
	\x03\x02\x02\x02\u{2fe}\u{2ff}\x03\x02\x02\x02\u{2ff}\u{2fd}\x03\x02\x02\
	\x02\u{2ff}\u{300}\x03\x02\x02\x02\u{300}\u{308}\x03\x02\x02\x02\u{301}\
	\u{303}\x07\x17\x02\x02\u{302}\u{304}\x05\x30\x19\x02\u{303}\u{302}\x03\
	\x02\x02\x02\u{304}\u{305}\x03\x02\x02\x02\u{305}\u{303}\x03\x02\x02\x02\
	\u{305}\u{306}\x03\x02\x02\x02\u{306}\u{308}\x03\x02\x02\x02\u{307}\u{2fb}\
	\x03\x02\x02\x02\u{307}\u{301}\x03\x02\x02\x02\u{308}\u{a7}\x03\x02\x02\
	\x02\x69\u{ab}\u{b0}\u{b5}\u{b8}\u{bf}\u{cd}\u{d6}\u{da}\u{dd}\u{e2}\u{ed}\
	\u{f5}\u{fd}\u{105}\u{109}\u{10e}\u{116}\u{11b}\u{122}\u{126}\u{12b}\u{132}\
	\u{136}\u{13a}\u{140}\u{146}\u{14d}\u{154}\u{15b}\u{161}\u{163}\u{169}\u{16f}\
	\u{176}\u{17c}\u{17e}\u{184}\u{18a}\u{191}\u{198}\u{19a}\u{1a4}\u{1b0}\u{1b6}\
	\u{1bc}\u{1c1}\u{1c9}\u{1cf}\u{1d5}\u{1dc}\u{1e0}\u{1e4}\u{1eb}\u{1ee}\u{1f2}\
	\u{1f6}\u{1fa}\u{200}\u{205}\u{20b}\u{20f}\u{214}\u{219}\u{21f}\u{226}\u{22f}\
	\u{233}\u{242}\u{24f}\u{254}\u{258}\u{25c}\u{25e}\u{264}\u{270}\u{275}\u{27a}\
	\u{27c}\u{27e}\u{285}\u{288}\u{28d}\u{294}\u{297}\u{29c}\u{2a3}\u{2a6}\u{2ad}\
	\u{2b0}\u{2b5}\u{2be}\u{2c7}\u{2cb}\u{2d3}\u{2d7}\u{2df}\u{2e7}\u{2f1}\u{2f7}\
	\u{2f9}\u{2ff}\u{305}\u{307}";

