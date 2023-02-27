// ANTLR4 Equivalent of accompanying bnf, developed in
// http://www.w3.org/2005/01/yacker/uploads/ShEx3
// Updated to Jul 27 AM ShEx3
// Updated to Aug 23 AM ShEx3 (last change was EGP 20150820)
// Sept 21 AM disallow single internal unary (e.g. {(:p .{2}){3}}
//            Change (non-standard) "codeLabel" to "productionName"
// Oct 26 - change annotation predicate to include rdftype (how did this slip in to the production rules?
// Dec 30 - update to match http://www.w3.org/2005/01/yacker/uploads/ShEx2/bnf with last change "EGP 20151120"
// May 23, 2016 - Update to match http://www.w3.org/2005/01/yacker/uploads/ShEx2/bnf with last change "EGP20160520" AND ';' separator and '//' for annotations
// May 24, 2016 - EGP20150424
// Aug 11, 2016 - EGP20160708
// Sep 14, 2016 - Revised to match Eric's latest reshuffle
// Sep 24, 2016 - Switched to TT grammar (vs inner and outer shapes)
// Sep 26, 2016 - Refactored to match https://raw.githubusercontent.com/shexSpec/shex.js/7eb770fe2b5bab9edfe9558dc07bb6f6dcdf5d23/doc/bnf
// Oct 27, 2016 - Added comments to '*', '*' and '?' to facilitate parsing
// Oct 27, 2016 - Added qualifier rule to be reused by shapeDefinition and inlineShapeDefinition
// Oct 27, 2016 - Added negation rule
// Mar 03, 2017 - removed ^^-style facet arguments per shex#41
// Mar 03, 2017 - switch to ~/regexp/
// Apr 09, 2017 - removed WS fragment (unused)
// Apr 09, 2017 - revise REGEXP definition
// Apr 09, 2017 - factor out REGEXP_FLAGS so we don't have to parse them out
// Apr 09, 2017 - literalRange / languageRange additions
// Apr 09, 2017 - factor out shapeRef to match spec
// Apr 09, 2017 - update repeatRange to allow differentiation of {INTEGER} and {INTEGER,}
// Apr 09, 2017 - add STEM_MARK and UNBOUNDED tokens to eliminate lex token parsing
// Nov 09, 2017 - Applied the following renames according to ShEx BNF
// ~ s/someOfShape/someOfTripleExpr/ (EGP 20160930) (oneOf)
// ~ s/innerShape/innerTripleExpr/ (EGP 20160930)
// ~ s/groupShape/groupTripleExpr/ (EGP 20160930)
// ~ s/unaryShape/unaryTripleExpr/ (EGP 20160930)
// ~ s/encapsulatedShape/bracketedTripleExpr/ (EGP 20160930)
// Jul 26, 2018 - Replace extensions by includeSet, added extends/restricts
// Nov 14, 2020 - Replace shapeExprLabel in extends/restricts by shapeRef

grammar ShExDoc; 

shExDoc
 : directive* ((notStartAction | startActions) statement*)? EOF;  // leading CODE

directive
 : baseDecl
 | prefixDecl
 | importDecl
 ;

baseDecl
 : KW_BASE  IRIREF
 ;

prefixDecl
 : KW_PREFIX PNAME_NS IRIREF
 ;

importDecl
 : KW_IMPORT iri
 ;

notStartAction
 : start
 | shapeExprDecl
 ;

start
 : KW_START '=' shapeExpression
 ;

startActions
 : semanticAction+
 ;

statement
 : directive
 | notStartAction
 ;

shapeExprDecl
 : KW_ABSTRACT? shapeExprLabel (shapeExpression | KW_EXTERNAL)
 ;

shapeExpression
 : shapeOr
 ;

inlineShapeExpression
 : inlineShapeOr
 ;

shapeOr
 : shapeAnd (KW_OR shapeAnd)*
 ;

inlineShapeOr
 : inlineShapeAnd (KW_OR inlineShapeAnd)*
 ;

shapeAnd
 : shapeNot (KW_AND shapeNot)*
 ;

inlineShapeAnd
 : inlineShapeNot (KW_AND inlineShapeNot)*
 ;

shapeNot
 : negation? shapeAtom
 ;

inlineShapeNot
 : negation? inlineShapeAtom
 ;

negation
 : KW_NOT
 | '!'
 ;

shapeAtom
 : nonLitNodeConstraint shapeOrRef?	# shapeAtomNonLitNodeConstraint
 | litNodeConstraint # shapeAtomLitNodeConstraint
 | shapeOrRef nonLitNodeConstraint?	# shapeAtomShapeOrRef
 | '(' shapeExpression ')' # shapeAtomShapeExpression
 | '.' # shapeAtomAny			// no constraint
 ;

inlineShapeAtom
 : inlineNonLitNodeConstraint inlineShapeOrRef? # inlineShapeAtomNonLitNodeConstraint
 | inlineLitNodeConstraint # inlineShapeAtomLitNodeConstraint
 | inlineShapeOrRef inlineNonLitNodeConstraint? # inlineShapeAtomShapeOrRef
 | '(' shapeExpression ')' # inlineShapeAtomShapeExpression
 | '.' # inlineShapeAtomAny   // no constraint
 ;

shapeOrRef
 : shapeDefinition
 | shapeRef
 ;

inlineShapeOrRef
 : inlineShapeDefinition
 | shapeRef
 ;

shapeRef
 : ATPNAME_LN
 | ATPNAME_NS
 | '@' shapeExprLabel
 ;

inlineLitNodeConstraint
 : KW_LITERAL xsFacet* # nodeConstraintLiteral
 | nonLiteralKind stringFacet* # nodeConstraintNonLiteral
 | datatype xsFacet* # nodeConstraintDatatype
 | valueSet xsFacet* # nodeConstraintValueSet
 | numericFacet+ # nodeConstraintNumericFacet
 ;

litNodeConstraint
 : inlineLitNodeConstraint annotation* semanticAction*
 ;

inlineNonLitNodeConstraint
 : nonLiteralKind stringFacet* # litNodeConstraintLiteral
 | stringFacet+ # litNodeConstraintStringFacet
 ;

nonLitNodeConstraint
 : inlineNonLitNodeConstraint annotation* semanticAction*
 ;

nonLiteralKind
 : KW_IRI
 | KW_BNODE
 | KW_NONLITERAL
 ;

xsFacet
 : stringFacet
 | numericFacet
 ;

stringFacet
 : stringLength INTEGER
 | REGEXP REGEXP_FLAGS?
 ;

stringLength
 : KW_LENGTH
 | KW_MINLENGTH
 | KW_MAXLENGTH
 ;

numericFacet
 : numericRange rawNumeric
 | numericLength INTEGER
 ;

numericRange
 : KW_MININCLUSIVE
 | KW_MINEXCLUSIVE
 | KW_MAXINCLUSIVE
 | KW_MAXEXCLUSIVE
 ;

numericLength
 : KW_TOTALDIGITS
 | KW_FRACTIONDIGITS
 ;

// rawNumeric is like numericLiteral but returns a JSON integer or float
rawNumeric
 : INTEGER
 | DECIMAL
 | DOUBLE
 ;

shapeDefinition
 : inlineShapeDefinition annotation* semanticAction*
 ;

inlineShapeDefinition
 : qualifier* '{' tripleExpression? '}'
 ;

qualifier
 : extension
 | restriction
 | extraPropertySet
 | KW_CLOSED
 ;

extraPropertySet
 : KW_EXTRA predicate+
 ;

tripleExpression
 : oneOfTripleExpr
 ;

oneOfTripleExpr
 : groupTripleExpr
 | multiElementOneOf
 ;

multiElementOneOf
 : groupTripleExpr ( '|' groupTripleExpr)+
 ;

groupTripleExpr
 : singleElementGroup
 | multiElementGroup
 ;

/*
innerTripleExpr
 : multiElementGroup
 | multiElementOneOf
 ;
*/

singleElementGroup
 : unaryTripleExpr ';'?
 ;

multiElementGroup
 : unaryTripleExpr (';' unaryTripleExpr)+ ';'?
 ;

unaryTripleExpr
 : ('$' tripleExprLabel)? (tripleConstraint | bracketedTripleExpr )
 | include
 | expr
 ;

bracketedTripleExpr
 : '(' tripleExpression ')' cardinality? annotation* semanticAction*
 ;

tripleConstraint
 : senseFlags? predicate inlineShapeExpression cardinality? annotation* semanticAction* /* variableDecl? */
 ;

cardinality
 :  '*'         # starCardinality
 | '+'          # plusCardinality
 | '?'          # optionalCardinality
 | repeatRange  # repeatCardinality
 ;

repeatRange
 : '{' INTEGER '}'		            # exactRange
 | '{' min_range ',' max_range? '}'  # minMaxRange
 ;

min_range
 : INTEGER
 ;

max_range
 : INTEGER
 | '*'
 ;

/*
variableDecl
 : KW_AS varName
 ;
*/

/*
varName
 : VAR
 ;
*/

expr
 : expr binOp expr
 | basicExpr
 ;

binOp
 : '='  # equals
 | '!=' # notEquals
 | '>'  # gt
 | '<'  # lt
 | '>=' # ge
 | '<=' # le
 | '*'  # mult
 | '/'  # div
 | '+'  # add
 | '-'  # minus
 ;

basicExpr
 : /* varName
 | */ literal
 | iri
 | blankNode
 ;

senseFlags
 : '!' '^'?
 | '^' '!'?		// inverse not
 ;

valueSet
 : '[' valueSetValue* ']'
 ;

valueSetValue
 : iriRange
 | literalRange
 | languageRange
 | '.' (iriExclusion+ | literalExclusion+ | languageExclusion+)
 ;

iriRange
 : iri (STEM_MARK iriExclusion*)?
 ;

iriExclusion
 : '-' iri STEM_MARK?
 ;

literalRange
 : literal (STEM_MARK literalExclusion*)?
 ;

literalExclusion
 : '-' literal STEM_MARK?
 ;

languageRange
 : LANGTAG (STEM_MARK languageExclusion*)? # languageRangeFull
 | '@' STEM_MARK languageExclusion*        # languageRangeAt
 ;

languageExclusion
 : '-' LANGTAG STEM_MARK?
 ;
include
 : '&' tripleExprLabel
 ;

annotation
 : '//' predicate (iri | literal)
 ;

semanticAction
 : '%' iri (CODE | '%')
 ;

literal
 : rdfLiteral
 | numericLiteral
 | booleanLiteral
 ;

// BNF: predicate ::= iri | RDF_TYPE
predicate
 : iri
 | rdfType
 ;

rdfType
 : RDF_TYPE
 ;

datatype
 : iri
 ;

shapeExprLabel
 : iri
 | blankNode
 ;

tripleExprLabel
 : iri
 | blankNode
 ;

numericLiteral
 : INTEGER
 | DECIMAL
 | DOUBLE
 ;

rdfLiteral
 : string (LANGTAG | '^^' datatype)?
 ;

booleanLiteral
 : KW_TRUE
 | KW_FALSE
 ;

string
 : STRING_LITERAL_LONG1
 | STRING_LITERAL_LONG2
 | STRING_LITERAL1
 | STRING_LITERAL2
 ;

iri
 : IRIREF
 | prefixedName
 ;

prefixedName
 : PNAME_LN
 | PNAME_NS
 ;

blankNode
 : BLANK_NODE_LABEL
 ;

extension
 : KW_EXTENDS shapeRef +
 | '&' shapeRef +
 ;

restriction
 : KW_RESTRICTS shapeRef +
 | '-' shapeRef +
 ;


// Keywords
KW_ABSTRACT
 : A B S T R A C T
 ;

KW_AS
 : A S
 ;

KW_BASE
 : B A S E
 ;

KW_EXTENDS
 : E X T E N D S
 ;

KW_IMPORT
 : I M P O R T
 ;

KW_RESTRICTS
 : R E S T R I C T S
 ;

KW_EXTERNAL
 : E X T E R N A L
 ;

KW_PREFIX
 : P R E F I X
 ;

KW_START
 : S T A R T
 ;

KW_VIRTUAL
 : V I R T U A L
 ;


KW_CLOSED
 : C L O S E D
 ;

KW_EXTRA
 : E X T R A
 ;

KW_LITERAL
 : L I T E R A L
 ;

KW_IRI
 : I R I
 ;

KW_NONLITERAL
 : N O N L I T E R A L
 ;

KW_BNODE
 : B N O D E
 ;

KW_AND
 : A N D
 ;

KW_OR
 : O R
 ;

KW_MININCLUSIVE
 : M I N I N C L U S I V E
 ;

KW_MINEXCLUSIVE
 : M I N E X C L U S I V E
 ;

KW_MAXINCLUSIVE
 : M A X I N C L U S I V E
 ;

KW_MAXEXCLUSIVE
 : M A X E X C L U S I V E
 ;

KW_LENGTH
 : L E N G T H
 ;

KW_MINLENGTH
 : M I N L E N G T H
 ;

KW_MAXLENGTH
 : M A X L E N G T H
 ;

KW_TOTALDIGITS
 : T O T A L D I G I T S
 ;

KW_FRACTIONDIGITS
 : F R A C T I O N D I G I T S
 ;

KW_NOT
 : N O T
 ;

KW_TRUE
 : 'true'
 ;

KW_FALSE
 : 'false'
 ;

 // --------------------------
 // TERMINALS
 // --------------------------

// Skip white spaces in the shEx and comments.
SKIP_
 : (WHITE_SPACE | COMMENT) -> skip
 ;

fragment COMMENT
 : ('#' ~[\r\n]* | '/*' (~[*] | '*' ('\\/' | ~[/]))* '*/') 
 ;

// A white space is defined as '\t' or '\r' or '\n'.
fragment WHITE_SPACE
 : [ \t\r\n]+
 ;

CODE
 : '{' (~[%\\] | '\\' [%\\] | UCHAR)* '%' '}'
 ;

/*
VAR
 : /* VAR1
 | VAR2
 ;
*/

/*
VAR1
 : '$' VARNAME
 ;
*/

/*
VAR2
 : '?' VARNAME
 ;
*/

RDF_TYPE
 : 'a'
 ;

IRIREF
 : '<' (~[\u0000-\u0020=<>"{}|^`\\] | UCHAR)* '>'
 ; /* #x00=NULL #01-#x1F=control codes #x20=space */

PNAME_NS
 : PN_PREFIX? ':'
 ;

PNAME_LN
 : PNAME_NS PN_LOCAL
 ;

ATPNAME_NS
 : '@' PN_PREFIX? ':'
 ;

ATPNAME_LN
 : '@' PNAME_NS PN_LOCAL
 ;

REGEXP
 : '/' (~[/\n\r\\] | '\\' [/nrt\\|.?*+(){}[\]$^-] | UCHAR)+ '/'
 ;

REGEXP_FLAGS
 : [smix]+
 ;

BLANK_NODE_LABEL
 : '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?
 ;

LANGTAG
 : '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*
 ;

INTEGER
 : [+-]? [0-9]+
 ;

DECIMAL
 : [+-]? [0-9]* '.' [0-9]+
 ;

DOUBLE
 : [+-]? ([0-9]+ '.' [0-9]* EXPONENT | '.'? [0-9]+ EXPONENT)
 ;

STEM_MARK
 : '~'
 ;

UNBOUNDED
 : '*'
 ;

fragment EXPONENT
 : [eE] [+-]? [0-9]+
 ;

STRING_LITERAL1
 : '\'' (~[\u0027\u005C\u000A\u000D] | ECHAR | UCHAR)* '\''
 ; /* #x27=' #x5C=\ #xA=new line #xD=carriage return */

STRING_LITERAL2
 : '"' (~[\u0022\u005C\u000A\u000D] | ECHAR | UCHAR)* '"'
 ;   /* #x22=" #x5C=\ #xA=new line #xD=carriage return */

STRING_LITERAL_LONG1
 : '\'\'\'' (('\'' | '\'\'')? (~['\\] | ECHAR | UCHAR))* '\'\'\''
 ;

STRING_LITERAL_LONG2
 : '"""' (('"' | '""')? (~["\\] | ECHAR | UCHAR))* '"""'
 ;

fragment UCHAR
 : '\\u' HEX HEX HEX HEX
 | '\\U' HEX HEX HEX HEX HEX HEX HEX HEX
 ;

fragment ECHAR
 : '\\' [tbnrf\\"']
 ;

fragment PN_CHARS_BASE
 : [A-Z]
 | [a-z]
 | [\u00C0-\u00D6]
 | [\u00D8-\u00F6]
 | [\u00F8-\u02FF]
 | [\u0370-\u037D]
 | [\u037F-\u1FFF]
 | [\u200C-\u200D]
 | [\u2070-\u218F]
 | [\u2C00-\u2FEF]
 | [\u3001-\uD7FF]
 | [\uF900-\uFDCF]
 | [\uFDF0-\uFFFD]
 | [\u{10000}-\u{EFFFD}]
 // | [\uD800-\uDB7F] [\uDC00-\uDFFF]
 ;

fragment PN_CHARS_U
 : PN_CHARS_BASE
 | '_'
 ;

fragment PN_CHARS
 : PN_CHARS_U
 | '-'
 | [0-9]
 | [\u00B7]
 | [\u0300-\u036F]
 | [\u203F-\u2040]
 ;

fragment PN_PREFIX
 : PN_CHARS_BASE ((PN_CHARS | '.')* PN_CHARS)?
 ;

fragment PN_LOCAL
 : (PN_CHARS_U | ':' | [0-9] | PLX) ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX))?
 ;

fragment PLX
 : PERCENT
 | PN_LOCAL_ESC
 ;

fragment PERCENT
 : '%' HEX HEX
 ;

fragment HEX
 : [0-9]
 | [A-F]
 | [a-f]
 ;

fragment PN_LOCAL_ESC
 : '\\' ('_' | '~' | '.' | '-' | '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%')
 ;

/*
VARNAME
 : ( PN_CHARS_U | DIGIT ) ( PN_CHARS_U | DIGIT | '\u00B7' | ('\u0300'..'\u036F') | ('\u203F'..'\u2040') )*
 ;
*/

/* fragment DIGIT: '0'..'9' ; */
fragment A:('a'|'A');
fragment B:('b'|'B');
fragment C:('c'|'C');
fragment D:('d'|'D');
fragment E:('e'|'E');
fragment F:('f'|'F');
fragment G:('g'|'G');
fragment H:('h'|'H');
fragment I:('i'|'I');
fragment J:('j'|'J');
fragment K:('k'|'K');
fragment L:('l'|'L');
fragment M:('m'|'M');
fragment N:('n'|'N');
fragment O:('o'|'O');
fragment P:('p'|'P');
fragment Q:('q'|'Q');
fragment R:('r'|'R');
fragment S:('s'|'S');
fragment T:('t'|'T');
fragment U:('u'|'U');
fragment V:('v'|'V');
fragment W:('w'|'W');
fragment X:('x'|'X');
fragment Y:('y'|'Y');
fragment Z:('z'|'Z');