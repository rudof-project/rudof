# Architecture

The following diagram presents the main modules (called crates in Rust) and their dependencies:

```mermaid
graph TD;
 rudof_iri --> oxrdf;
 rudof_iri --> reqwest ;
 dctap --> calamine ;
 dctap --> csv ;
 sparql_service --> oxigraph ;
 rudof_rdf --> oxigraph ;
subgraph rudof
    rudof_lib[<a href='https://crates.io/crates/rudof_lib'>rudof_lib</a>];
    rudof_cli[<a href='https://crates.io/crates/rudof_cli'>rudof_cli</a>];
    shex_ast[<a href='https://crates.io/crates/shex_ast'>shex_ast</a>];
    rudof_rdf[<a href='https://crates.io/crates/rudof_rdf'>rudof_rdf</a>];
    shex_validation[<a href='https://crates.io/crates/shex_validation'>shex_validation</a>];
    shacl[<a href='https://crates.io/crates/shacl'>shacl</a>];
    shapes_comparator[<a href='https://crates.io/crates/shapes_comparator'>shapes_comparator</a>];
    rudof_iri[<a href='https://crates.io/crates/rudof_iri'>rudof_iri</a>];
    prefixmap[<a href='https://crates.io/crates/prefixmap'>prefixmap</a>];
    rbe[<a href='https://crates.io/crates/rbe'>rbe</a>];
    shapes_converter[<a href='https://crates.io/crates/shapes_converter'>shapes_converter</a>];
 dctap[<a href='https://crates.io/crates/dctap'>dctap</a>];
 sparql_service[<a href='https://crates.io/crates/sparql_service'>sparql_service</a>];

 rudof_cli --> rudof_lib ;
 shex_ast --> rudof_rdf ;
 shex_ast --> rbe;
 shex_validation-->shex_ast;
 rudof_rdf-->rudof_iri;
 shacl-->rudof_rdf;
 shacl-->sparql_service;
 shex_validation-->prefixmap;
 shex_ast-->prefixmap;
 rudof_rdf-->prefixmap;
 shex_validation-->rbe;
 dctap-->prefixmap;
 dctap --> rudof_iri;
 shapes_comparator-->shacl;
 shapes_comparator-->shex_ast;
 shapes_comparator-->shex_validation;
 shapes_comparator-->sparql_service;
 shapes_converter-->shacl;
 shapes_converter-->shex_ast;
 shapes_converter-->shex_validation;
 shapes_converter-->dctap;
 prefixmap --> rudof_iri ;
 sparql_service --> rudof_iri ;
 sparql_service --> rudof_rdf ;
 rudof_lib --> shex_validation ;
 rudof_lib --> shacl ;
 rudof_lib --> shapes_comparator ;
 rudof_lib --> shapes_converter ;
end
subgraph external dependencies
 oxigraph[<a href='https://crates.io/crates/oxigraph'>oxigraph</a>] ;
 oxrdf[<a href='https://crates.io/crates/oxrdf'>oxrdf</a>] ;
 calamine[<a href='https://docs.rs/calamine/latest/calamine/'>calamine</a>] ;
 reqwest[<a href='https://docs.rs/reqwest/latest/reqwest/'>reqwest</a>] ;
 csv[<a href='https://docs.rs/csv/latest/csv/'>csv</a>]
end
```
