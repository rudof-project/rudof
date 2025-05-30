/*
Language: ShExC
Description: Shape Expressions Compact Syntax, a schema language for graphs, may be good for Turtle
Author: Eric Prud'hommeaux <eric+github@w3.org>
Category: misc
*/

/**
 * highlight.js ShExC syntax highlighting definition
 *
 * @see https://github.com/ericprud/highlightjs-shexc
 *
 * @package: highlightjs-shexc
 * @author:  Eric Prud'hommeaux <eric@w3.org>
 * @since:   2019-21-12
 *
 *
 * Maintenance notes:
 *
 * This module is largely transliterated from the ShExC grammar.
 * <http://shex.io/shex-semantics/index.html#shexc> For terminals, this acts as
 * a validator. If e.g. your URL doesn't show up with syntax highlighting, it's
 * probably malformed.
 *
 * Anything with begin: iris_RE gets relevance: 0 in order not to assume
 * anything with URLs is ShExC.
 *
 * TODO, in decreasing priority:
 *   annotations <http://shex.io/shex-semantics/index.html#prod-annotation>
 *   semacts <http://shex.io/shex-semantics/index.html#prod-semanticAction>
 *   strings <http://shex.io/shex-semantics/index.html#prod-string>
 *   comments in odd places.
 *
 * Limitations: There is no way (afaik) in highlight.js to assign classes based
 * on position. As a result, a TripleConstraint with a predicate and a datatype
 * will have the same class applied to both.
 */

function hljsDefineShExC(hljs, opts = {}) {
    /** terminals from <http://shex.io/shex-semantics/index.html#term-IRIREF>
     * <IRIREF>      ::=          "<" ([^#0000- <>\"{}|^`\\] | UCHAR)* ">"
     * <PNAME_NS>	   ::=   	PN_PREFIX? ":"
     * <PNAME_LN>	   ::=   	PNAME_NS PN_LOCAL
     * ... (see link for the rest)
     */
    const HEX_RE = '[0-9a-fA-F]'
    const UCHAR_RE = '\\\\(?:u' + HEX_RE + '{4}|U' + HEX_RE + '{8})'
    const IRIREF_RE = '<([^<>"{}|^`\\\\]|' + UCHAR_RE + ')*>'
    const PN_CHARS_BASE_RE = '[a-zA-Z]'
    const PN_CHARS_U_RE = [PN_CHARS_BASE_RE, '_'].join('|')
    const PN_CHARS_RE = [PN_CHARS_U_RE, '-', '[0-9]'].join('|')
    const PN_PREFIX_RE = PN_CHARS_BASE_RE + '((' + PN_CHARS_RE + '|\\.)*' + PN_CHARS_RE + ')?'
    const PNAME_NS_RE = '(' + PN_PREFIX_RE + ')?:'
    const PN_LOCAL_ESC_RE = '\\\\[_~.!$&\'()*+,;=/?#@%-]'
    const PERCENT_RE = '%' + HEX_RE + HEX_RE
    const PLX_RE = [PERCENT_RE, PN_LOCAL_ESC_RE].join('|')
    const PN_LOCAL_RE = '(' + [PN_CHARS_U_RE, ':', '[0-9]', PLX_RE].join('|') + ')'
        + '(' + '(' + [PN_CHARS_RE, '\\.', ':', PLX_RE].join('|') + ')' + ')*'
    const PNAME_LN_RE = PNAME_NS_RE + PN_LOCAL_RE

    /** IRI forms from <https://shexspec.github.io/spec/#prod-iri>
     * iri	   ::=   	IRIREF | prefixedName
     * prefixedName  ::=   	PNAME_LN | PNAME_NS
     * ... (see link for the rest)
     */
    const prefixedName_RE = PNAME_LN_RE + '|' + PNAME_NS_RE
    const iris_RE = '(' + [prefixedName_RE, IRIREF_RE].join('|') + ')'
    const PERCENT = { className: 'meta-keyword', begin: PERCENT_RE }
    const UCHAR = { className: 'meta-keyword', begin: UCHAR_RE }
    const PN_LOCAL_ESC = { className: 'meta-keyword', begin: PN_LOCAL_ESC_RE }
    const productions = {}
    productions.IRIREF = {
        className: 'symbol',
        begin: /</, end: />/, // can't use begin: IRIREF_RE because of contains.
        contains: [PERCENT, UCHAR]
    }
    productions.prefixedName = {
        begin: prefixedName_RE,
        returnBegin: true,
        contains: [
            {
                className: "type",
                begin: PNAME_NS_RE,
            },
            {
                className: "variable",
                begin: PN_LOCAL_RE,
                endsWithParent: true,
                contains: [PN_LOCAL_ESC], //  doesn't work
            },
        ]
    }

    /** Special regexp which consumes rest of string. */
    const EndOfDocument = /\B\b/

    /** directives from <https://shexspec.github.io/spec/#prod-directive>
     * baseDecl      ::=          "BASE" IRIREF
     * prefixDecl    ::=          "PREFIX" PNAME_NS IRIREF
     * importDecl    ::=          "IMPORT" IRIREF
     */
    productions.prefix = {
        beginKeywords: "prefix",
        // begin: "prefix",
        end: EndOfDocument,
        returnBegin: true,
        contains: [
            // { // not needed if using beginKeywords in parent
            //   className: "keyword",
            //   beginKeywords: 'prefix',
            // },
            {
                className: "type",
                begin: PNAME_NS_RE,
            },
            Object.assign({ endsParent: true }, productions.IRIREF),
        ]
    }
    productions.base = {
        beginKeywords: "base",
        end: EndOfDocument,
        returnBegin: true,
        contains: [
            Object.assign({ endsParent: true }, productions.IRIREF),
        ]
    }
    productions._import = { // Need a leading '_' because "import" is a js keyword.
        beginKeywords: "import",
        end: EndOfDocument,
        returnBegin: true,
        contains: [
            Object.assign({ endsParent: true }, productions.IRIREF),
        ]
    }

    /** shape expressions from <http://shex.io/shex-semantics/index.html#prod-shapeExpression> 
     */
    productions.shape = {
        begin: /{/, end: /}/,
        relevance: 0
        // Add .contains (below) after constructing its contents.
    }
    const shapeExprContentModel = [
        hljs.HASH_COMMENT_MODE,
        hljs.C_BLOCK_COMMENT_MODE,

        /** <http://shex.io/shex-semantics/index.html#prod-shapeExprLabel>
         * shapeExprDecl	::=   	shapeExprLabel (shapeExpression | "EXTERNAL")
         */
        {
            className: 'title',
            begin: iris_RE,
            contains: [PERCENT, UCHAR],
            relevance: 0
        },

        /** <http://shex.io/shex-semantics/index.html#prod-shapeRef>
         * shapeRef	   ::=   	   ATPNAME_LN | ATPNAME_NS | '@' shapeExprLabel
         */
        {
            className: 'name',
            begin: '@' + iris_RE,
            contains: [PERCENT, UCHAR],
            relevance: 10
        },

        /**
         * shapeDefinition	   ::=   	(extraPropertySet | "CLOSED")* '{' tripleExpression? '}'
         *                                  annotation* semanticActions
         */
        {
            beginKeywords: 'extra closed', end: /{/,
            returnEnd: true,
            contains: [productions.IRIREF, productions.prefixedName],
            relevance: 10
        },

        /** simplified form of <http://shex.io/shex-semantics/index.html#term-REGEXP>
        <REGEXP>	   ::=   	'/' ([^/\\\n\r]
                                                 | '\\' [nrt\\|.?*+(){}$-\[\]^/]
                                                 | UCHAR
                                                )+ '/' [smix]*
         */
        {
            className: 'regexp',
            begin: /\/([^\\/]|\\.)*\//,
            contains: [
                hljs.REGEXP_MODE,
            ],
        },

        productions.shape,
    ]
    const shapeExpression_keywords = 'and or not closed abstract extends restricts iri bnode literal nonliteran'
        + ' length minlength maxlength'
        + ' mininclusive minexclusive maxinclusive maxexclusive'
    productions.shapeExpression = {
        begin: iris_RE,
        end: EndOfDocument,
        returnBegin: true,
        keywords: shapeExpression_keywords,
        contains: shapeExprContentModel,
        relevance: 0
    }

    /** shape expressions from <http://shex.io/shex-semantics/index.html#prod-unaryTripleExpr> 
     */
    productions.tripleExpression = {
        begin: iris_RE,
        end: EndOfDocument,
        returnBegin: true,
        endsWithParent: true,
        keywords: shapeExpression_keywords,
        contains: [productions.IRIREF, productions.prefixedName].concat(shapeExprContentModel),
        relevance: 0
    }
    productions.tripleExprLabel = {
        className: 'name',
        begin: '$' + iris_RE,
        contains: [PERCENT, UCHAR],
        relevance: 10
    }
    productions.inclusion = {
        className: 'name',
        begin: '&' + iris_RE,
        contains: [PERCENT, UCHAR],
        relevance: 10
    }
    // The root language is called "shexDoc" <http://shex.io/shex-semantics/index.html#prod-shexDoc>
    productions.shexDoc = {
        case_insensitive: true,
        contains: [
            hljs.HASH_COMMENT_MODE,
            hljs.C_BLOCK_COMMENT_MODE,
            productions.prefix,
            productions.base,
            productions._import,
            productions.shapeExpression,
        ],
        relevance: 10
    }

    // Add last component in this cycle:
    //   shape ➜ tripleExpression ➜ shapeExpression ➜ shape
    productions.shape.contains = [productions.tripleExprLabel, productions.inclusion, productions.tripleExpression]

    const startingProduction = opts.startingProduction || 'shexDoc'
    if (!(startingProduction in productions))
        throw Error(`starting production ${startingProduction} not found in ${Object.keys(productions).join(', ')}}`)
    return productions[startingProduction]
}

hljs.registerLanguage('shexc', hljsDefineShExC);
