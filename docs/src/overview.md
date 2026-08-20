# Overview

`rudof` is a tool to help describe and validate Knowledge Graphs. 
It supports Graph data like [RDF](https://www.w3.org/RDF/) and [RDF 1.2](https://www.w3.org/TR/rdf12-concepts/), 
 as well as Labeled Property Graphs as well as Query languages like [SPARQL](https://www.w3.org/TR/sparql11-query/) and [SPARQL 1.2](https://www.w3.org/TR/sparql12-query/).

It implements several technologies for describing and validating Knowledge Graph data like
 [ShEx](https://shex.io/), [SHACL](https://www.w3.org/TR/shacl/), [DCTAP](https://www.dublincore.org/specifications/dctap/) for RDF data as well as [PG-Schema](https://arxiv.org/abs/2211.10962) for Property Graphs.

The library is implemented in [Rust](https://www.rust-lang.org/) and it also provides [Python bindings](https://pyrudof.readthedocs.io/en/stable/), a [Rust API](https://docs.rs/rudof_lib/) and an interactive [Shell](https://rudof-project.github.io/rudof/cli_usage/shell.html).

`rudof` can be used to validate RDF data represented with different syntaxes like Turtle, NTriples, etc.
   as well as RDF data available through SPARQL endpoints like Wikidata.
It can also be used to convert between different RDF data validation technologies (like ShEx, SHACL, or DCTAP), and to generate UML like visualizations and HTML views.

More links about `rudof`:

- [Linux, Windows and macOS binaries](https://github.com/weso/shex-rs/releases/).
- [Source code in Github repository](https://github.com/rudof-project/rudof).
- The [List of issues](https://github.com/rudof-project/rudof/issues).
- The [Discussions](https://github.com/rudof-project/rudof/discussions) page.
- A set of [How-to guides](https://github.com/rudof-project/rudof/wiki/How%E2%80%90to-guides) is also published.
- The collection of [Frequently-Asked-Questions](https://github.com/rudof-project/rudof/wiki/FAQ).

## Publications and tutorials

- [rudof-MCP: A Model Context Protocol Server for Semantic Web Operations](https://labra.weso.es/publication/2026_mcp_rudof/) , Samuel Bustamante Larriet, Diego Martín Fernández, Álvaro García Fernández, Daniel Fernández Álvarez, Jose Emilio Labra Gayo, Extended Semantic Web Conference, ESWC26, Posters and Demos - 2026
- [PG-Schema with Property Constraints validator in Rudof](https://labra.weso.es/publication/2026_rudof_pgschema_pc/), Jose Emilio Labra Gayo, Dominik Tomaszuk, Diego Martín Fernández, Samuel Bustamante Larriet, Álvaro García Fernández, Daniel Fernández Álvarez, Knowledge Capture and Knowledge Representation Conference, KCAP25, Posters and Demos - 2025
- [Introduction to `rudof for Wikibase users](https://docs.google.com/presentation/d/1K7lDn3Kln3IYku_m0dUxz-_ERfMwzf8_vPQCETXZCGQ). [Wikibase stakeholders Group meeting](https://notepad.rhizome.org/wbsg-2026-07-02). Video recording of the presentation is available [here](https://video.rhizome.org/w/kJXNskK6xusKU1xaqvGfak).
- A [rudof: A Rust Library for handling RDF data models and Shapes](https://labra.weso.es/publication/2024_rudof_demo/) was presented at [International Semantic Web Conference](https://iswc2024.semanticweb.org/event/3715c6fc-e2d7-47eb-8c01-5fe4ac589a52/summary) (Baltimore, USA, October 2024) in the Demos and Posters session.

## Modules

`rudof` has been implemented using a modular structure and the different modules are available as Rust crates:

```mermaid
graph TD;
 user((Person)) --> rudof_cli;
 application(application);
 application -->|Rust| rudof_lib ;
 application --> |Python| pyrudof ;
 rudof_iri --> oxigraph;
 rudof_iri --> reqwest ;
 dctap --> calamine ;
 dctap --> csv ;
 sparql_service --> oxigraph ;
subgraph rudof
    rudof_lib[<a href='https://crates.io/crates/rudof_lib'>rudof_lib</a>];
    rudof_cli[<a href='https://crates.io/crates/rudof_cli'>rudof_cli</a>];
    pyrudof[<a href='https://pypi.org/project/pyrudof/'>pyrudof</a>];
    shex_ast[<a href='https://crates.io/crates/shex_ast'>shex_ast</a>];
    srdf[<a href='https://crates.io/crates/srdf'>srdf</a>];
    shex_validation[<a href='https://crates.io/crates/shex_validation'>shex_validation</a>];
    shacl_validation[<a href='https://crates.io/crates/shacl_validation'>shacl_validation</a>];
    shacl_ir[<a href='https://crates.io/crates/shacl_ir'>shacl_ir</a>];
    shacl_ast[<a href='https://crates.io/crates/shacl_ast'>shacl_ast</a>];
    rudof_iri[<a href='https://crates.io/crates/rudof_iri'>rudof_iri</a>];
    prefixmap[<a href='https://crates.io/crates/prefixmap'>prefixmap</a>];
    rbe[<a href='https://crates.io/crates/rbe'>rbe</a>];
    shapes_converter[<a href='https://crates.io/crates/shapes_converter'>shapes_converter</a>];
    shapes_comparator[<a href='https://crates.io/crates/shapes_comparator'>shapes_comparator</a>];
 dctap[<a href='https://crates.io/crates/dctap'>dctap</a>];
 sparql_service[<a href='https://crates.io/crates/sparql_service'>sparql_service</a>];

 pyrudof --> rudof_lib ;
 rudof_cli --> rudof_lib ;
 shex_ast --> srdf ;
 shex_validation-->shex_ast;
 srdf-->rudof_iri;
 shacl_ir --> shacl_ast;
 shacl_ast-->srdf;
 shacl_validation-->shacl_ir;
 shex_ast-->prefixmap;
 srdf-->prefixmap;
 shex_validation-->rbe;
 dctap-->prefixmap;
 dctap --> rudof_iri;

 shapes_comparator-->shex_ast;
 shapes_comparator-->shacl_ast;
 shapes_converter-->shacl_ast;
 shapes_converter-->shex_ast;
 shapes_converter-->dctap;
 prefixmap --> rudof_iri ;
 shex_validation --> shex_ast
 sparql_service --> rudof_iri ;
 rudof_lib --> shex_validation ;
 rudof_lib --> shacl_validation ;
 rudof_lib --> shapes_converter ;
 rudof_lib --> sparql_service ;
 rudof_lib --> shapes_comparator ;
end
subgraph external dependencies
 oxigraph[<a href='https://crates.io/crates/oxigraph'>oxigraph</a>] ;
 calamine[<a href='https://docs.rs/calamine/latest/calamine/'>calamine</a>] ;
 reqwest[<a href='https://docs.rs/reqwest/latest/reqwest/'>reqwest</a>] ;
 csv[<a href='https://docs.rs/csv/latest/csv/'>csv</a>]
end
```

- [ShEx Validation algorithm](https://docs.rs/shex_validation/).
- [ShEx Compact syntax parser](https://docs.rs/shex_compact), a ShEx Compact syntax parser that follows the [ShEx compact grammar](https://shex.io/shex-semantics/index.html#shexc).
- [ShEx AST](https://docs.rs/shex_ast), that represents the [ShEx Abstract syntax](https://shex.io/shex-semantics/index.html#shape-expressions-shexj) based on ShExJ (JSON-LD).
- [SRDF](https://docs.rs/srdf), a Simple RDF Interface in Rust.
- [PrefixMap](https://docs.rs/prefixmap): Turtle based prefixMap representation
- [Conversions between different RDF data modelling technologies](https://docs.rs/shapes_convert).
- [Comparator between shapes](https://docs.rs/shapes_comparator).
- [SHACL AST](https://docs.rs/shacl_ast), that represents the [SHACL core abstract syntax](https://www.w3.org/TR/shacl).
- [SHACL Validation algorithm](https://docs.rs/shacl_validation/).
- [RBE](https://docs.rs/rbe), Regular Bag Expressions.
- [ShEx testsuite](https://docs.rs/shex_testsuite/), the Code in charge of checking the [ShEx testsuite](https://shexspec.github.io/test-suite/).

## Related projects

An incomplete list of projects which are related to `rudof` is the following:

- [ShEx-s](https://www.weso.es/shex-s/), a Scala implementation of ShEx (This project started as a re-implementation of ShEx-s in Rust).
- [SHACL-s](https://www.weso.es/shacl-s/), a Scala implementation of SHACL.
- [ShEx.js](https://github.com/shexjs/shex.js), a Javascript implementation of ShEx.
- [Oxigraph](https://github.com/oxigraph/oxigraph), a SPARQL implementation in Rust that also contains RDF libraries.
- [Nemo](https://github.com/knowsys/nemo), an in-memory rule engine that also contains some `nom` parsers.

## Contributors

- [Jose Emilio Labra Gayo](https://labra.weso.es/)
- [Samuel Bustamante Larriet](https://github.com/samuel-bustamante)
- [Álvaro García Fernández](https://algarferx.dev/)
- [Ángel Iglesias Préstamo](http://angelip2303.github.io/)
- [Diego Martín Fernández](https://github.com/DiegoMfer)
- [Marc-Antoine Arnaud](https://luminvent.com/)
- [Jonas Smedegaard](http://dr.jones.dk/blog/)
- [Full list of contributors](https://github.com/rudof-project/rudof/graphs/contributors)

## Funding and sponsors

The project has been partially funded by some grants or institutions like:

- [WESO - WEb Semantics Oviedo](https://www.weso.es/) is the research group at the [University of Oviedo, Spain](https://www.uniovi.es/) where some of the contributors are participating and has driven the main features implemented by `rudof`.
- [USDA - United States Department of Agriculture](https://www.usda.gov/)
- [Spanish Research Agency](https://www.aei.gob.es/) through the project ANGLIRU - Applying kNowledge Graphs for research Data Interoperability and Reusability (CODE MCI-21-PID2020-117912RB-C21).
- [Database Center for Life Science, Japan](https://dbcls.rois.ac.jp/index-en.html) has provided funding for attending several Biohackathons and RDF Summit events where some of the ideas behind `rudof` materialized as well as the logo.

In case you want to help the project, please contact [Jose E. Labra Gayo](https://labra.weso.es/).

## Supporters and adopters

The following is a list of `rudof` adopters and supporters:

- [WESO (WEb Semantics Oviedo)](http://www.weso.es/). Most of the contributors are part of this research group at the [University of Oviedo](http://www.uniovi.es)
- [USDA - United States Department of Agriculture](https://www.usda.gov/) has been partially funding this project through a Non-Assistance Cooperative Agreement with [WESO](http://www.weso.es/) where `rudof` is used to develop Data Shapes based on the [National Agricultural Library Thesaurus Concept Space](https://lod.nal.usda.gov/en/).
- [Luminvent](https://luminvent.com/) is using `rudof` to validate RDF using Rust code.

If you are using `rudof` and want to be listed, please contact us or add a Pull Request.
