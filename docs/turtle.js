/*
Language: Turtle
Author: Redmer KRONEMEIJER <redmer.kronemeijer@rdmr.eu>
Contributors: Mark ELLIS <mark.ellis@stardog.com>, Vladimir ALEXIEV <vladimir.alexiev@ontotext.com>
*/

var module = module ? module : {}; // shim for browser use

function hljsDefineTurtle(hljs) {
    // export default function (hljs) {
    var KEYWORDS = {
        keyword: "BASE|2 PREFIX|5 @base|10 @prefix|10",
        literal: "true false",
        built_in: "a",
    };

    var IRI_LITERAL = {
        // https://www.w3.org/TR/turtle/#grammar-production-IRIREF
        className: "literal",
        relevance: 1, // XML tags look also like relative IRIs
        begin: /</,
        end: />/,
        // illegal: /[^\x00-\x20"{}|^`\\]/, // TODO: https://www.w3.org/TR/turtle/#grammar-production-UCHAR
    };

    // https://www.w3.org/TR/turtle/#terminals
    var PN_CHARS_BASE =
        "A-Za-z\u00C0-\u00D6\u00D8-\u00F6\u00F8-\u02FF\u0370-\u037D\u037F-\u1FFF\u200C-\u200D\u2070-\u218F\u2C00-\u2FEF\u3001-\uD7FF\uF900-\uFDCF\uFDF0-\uFFFD\u10000-\uEFFFF";
    var PN_CHARS_U = PN_CHARS_BASE + "_";
    var PN_CHARS = "-" + PN_CHARS_U + "0-9\u00B7\u0300-\u036F\u203F-\u2040";
    var BLANK_NODE_LABEL =
        "_:[" + PN_CHARS_U + "0-9]([" + PN_CHARS + ".]*[" + PN_CHARS + "])?";
    var PN_PREFIX =
        "[" + PN_CHARS_BASE + "]([" + PN_CHARS + ".]*[" + PN_CHARS + "])?";
    var PERCENT = "%[0-9A-Fa-f][0-9A-Fa-f]";
    var PN_LOCAL_ESC = "\\\\[_~.!$&'()*+,;=/?#@%-]";
    var PLX = PERCENT + "|" + PN_LOCAL_ESC;
    var PNAME_NS = "(" + PN_PREFIX + ")?:";
    var PN_LOCAL =
        "([" +
        PN_CHARS_U +
        ":0-9]|" +
        PLX +
        ")([" +
        PN_CHARS +
        ".:]|" +
        PLX +
        ")*([" +
        PN_CHARS +
        ":]|" +
        PLX +
        ")?";
    var PNAME_LN = PNAME_NS + PN_LOCAL;
    var PNAME_NS_or_LN = PNAME_NS + "(" + PN_LOCAL + ")?";

    var PNAME = {
        begin: PNAME_NS_or_LN,
        relevance: 0,
        className: "symbol",
    };

    var BLANK_NODE = {
        begin: BLANK_NODE_LABEL,
        relevance: 10,
        className: "template-variable",
    };

    var LANGTAG = {
        begin: /@[a-zA-Z]+([a-zA-Z0-9-]+)*/,
        className: "type",
        relevance: 0, // also catches objectivec keywords like: @protocol, @optional
    };

    var DATATYPE = {
        begin: "\\^\\^" + PNAME_LN,
        className: "type",
        relevance: 10,
    };

    var TRIPLE_APOS_STRING = {
        begin: /'''/,
        end: /'''/,
        className: "string",
        relevance: 0,
    };

    var TRIPLE_QUOTE_STRING = {
        begin: /"""/,
        end: /"""/,
        className: "string",
        relevance: 0,
    };

    var APOS_STRING_LITERAL = JSON.parse(JSON.stringify(hljs.APOS_STRING_MODE));
    APOS_STRING_LITERAL.relevance = 0;

    var QUOTE_STRING_LITERAL = JSON.parse(JSON.stringify(hljs.QUOTE_STRING_MODE));
    QUOTE_STRING_LITERAL.relevance = 0;

    return {
        name: "Turtle",
        case_insensitive: true, // however `true` and `@prefix` are oblig. cased thus
        keywords: KEYWORDS,
        aliases: ["turtle", "ttl", "n3"],
        contains: [
            LANGTAG,
            DATATYPE,
            IRI_LITERAL,
            BLANK_NODE,
            PNAME,
            TRIPLE_APOS_STRING,
            TRIPLE_QUOTE_STRING, // order matters
            APOS_STRING_LITERAL,
            QUOTE_STRING_LITERAL,
            hljs.C_NUMBER_MODE,
            hljs.HASH_COMMENT_MODE,
        ],
        exports: {
            LANGTAG: LANGTAG,
            DATATYPE: DATATYPE,
            IRI_LITERAL: IRI_LITERAL,
            BLANK_NODE: BLANK_NODE,
            PNAME: PNAME,
            TRIPLE_APOS_STRING: TRIPLE_APOS_STRING,
            TRIPLE_QUOTE_STRING: TRIPLE_QUOTE_STRING,
            APOS_STRING_LITERAL: APOS_STRING_LITERAL,
            QUOTE_STRING_LITERAL: QUOTE_STRING_LITERAL,
            NUMBER: hljs.C_NUMBER_MODE,
            KEYWORDS: KEYWORDS,
        },
    };
}
hljs.registerLanguage("turtle", hljsDefineTurtle);
