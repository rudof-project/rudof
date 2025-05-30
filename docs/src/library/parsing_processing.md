# Parsing RDF

SRDF contains a trait to parse RDF data using combinators. The library follows a parser combinator approach where instead of parsing characters in sequence,
we parse RDF nodes in a graph.

The design adapts the [combine](https://docs.rs/combine/latest/combine/) parser combinators library and tries to use a similar set of combinators for RDF parsing combinators.

Some of the combinators are:

- [and](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html#method.and): applies a parser followed by another one.
- [and_then](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html#method.and_then): applies a function to the result of a parser.
- ...

You can see more information and examples in [RDFNodeParse](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html) trait.

This library has been used to implement the [SHACL parser](https://github.com/rudof-project/rudof/blob/master/shacl_ast/src/converter/rdf_to_shacl/shacl_parser.rs) that reads RDF data as input and converts it to a [SHACL abstract syntax tree](https://github.com/rudof-project/rudof/blob/master/shacl_ast/src/ast/schema.rs).

One advantage of this approach is that `rudof` doesn't depend on any specific RDF format as it works directly with the RDF data model, which can in fact be obtained from RDf data in syntaxes like Turtle, RDF/XML, etc. or as a set of triples from an SPARQL endpoint.
