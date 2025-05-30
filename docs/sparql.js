/*
Language: SPARQL
Requires: turtle.js
Author: Redmer KRONEMEIJER <redmer.kronemeijer@rdmr.eu>
Contributors: Mark ELLIS <mark.ellis@stardog.com>, Vladimir ALEXIEV <vladimir.alexiev@ontotext.com>
*/

var module = module ? module : {};     // shim for browser use

function hljsDefineSparql(hljs) {
    // export default function (hljs) {
    var ttl = hljs.getLanguage('turtle').exports;
    var KEYWORDS = {
        keyword: 'base|10 prefix|10 @base|10 @prefix|10 add all as|0 ask bind by|0 clear construct|10 copymove create data default define delete describe distinct drop exists filter from|0 graph|10 group having in|0 insert limit load minus named|10 not offset optional order reduced select|0 service silent to union using values where with|0',
        function: 'abs asc avg bound ceil coalesce concat containsstrbefore count dayhours desc encode_for_uri floor group_concat if|0 iri isblank isiri isliteral isnumeric isuri langdatatype langmatches lcase max md5 min|0 minutes month now rand regex replace round sameterm sample seconds separator sha1 sha256 sha384 sha512 str strafter strdt strends strlang strlen strstarts struuid substr sum then timezone tz ucase uribnode uuid year',
        literal: 'true|0 false|0',
        built_in: 'a|0'
    };

    var VARIABLE = {
        className: 'variable',
        begin: '[?$]' + hljs.IDENT_RE,
        relevance: 0,
    };

    var JSON_QUOTE_STRING = {
        begin: /"""\s*\{/,          // TODO why can't I write (?=\{)
        end: /"""/,
        subLanguage: 'json',
        excludeBegin: true,
        excludeEnd: true,
        relevance: 0,
    };

    var JSON_APOS_STRING = {
        begin: /'''\s*\{/,          // TODO why can't I write (?=\{)
        end: /'''/,
        subLanguage: 'json',
        excludeBegin: true,
        excludeEnd: true,
        relevance: 0,
    };

    return {
        name: "SPARQL",
        case_insensitive: true,
        keywords: KEYWORDS,
        aliases: ['rql', 'rq', 'ru'],
        contains: [
            ttl.LANGTAG,
            ttl.DATATYPE,
            ttl.IRI_LITERAL,
            ttl.BLANK_NODE,
            ttl.PNAME,
            VARIABLE,
            JSON_QUOTE_STRING, // order matters
            JSON_APOS_STRING,
            ttl.TRIPLE_QUOTE_STRING,
            ttl.TRIPLE_APOS_STRING,
            ttl.QUOTE_STRING_LITERAL,
            ttl.APOS_STRING_LITERAL,
            ttl.NUMBER,
            hljs.HASH_COMMENT_MODE,
        ]
    };
}

hljs.registerLanguage('sparql', hljsDefineSparql);
