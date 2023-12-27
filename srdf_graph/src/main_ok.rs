use iri_s::IriS;
use srdf::{rdf_parser, RDFParser, RDF, FocusRDF, satisfy, RDFNodeParse, SRDF, SRDFComparisons, property_value};
use srdf_graph::SRDFGraph;


fn main() {
    let add = IriS::new_unchecked("http://example.org/p");
    

    rdf_parser!{
        fn expr[RDF]()(RDF) -> RDF::Term
        where [
        ] { 
            property_value(&add)
        }
    }
    let s = r#"prefix : <http://example.org/>
    prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    
    :x :add (1 2).
    "#;

    let mut graph = SRDFGraph::from_str(s, None).unwrap();
    let x = IriS::new_unchecked("http://example.org/x");
    let value = expr().parse(&x, &mut graph);
    println!("SRDFGraph. Result of parser: {value:?}");

} 