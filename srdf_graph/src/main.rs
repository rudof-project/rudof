use iri_s::IriS;
use srdf::{rdf_parser, RDFParser, RDF, FocusRDF, satisfy, RDFNodeParse, SRDF, SRDFComparisons, property_value, rdf_list};
use srdf_graph::SRDFGraph;


fn main() {

    rdf_parser!{
        fn expr[RDF]()(RDF) -> RDF::Term
        where [
        ] { 
            let add = IriS::new_unchecked("http://example.org/add");
            property_value(&add)
        }
    }


    rdf_parser!{
        fn my_ok_['a, RDF](x: &'a RDF::Term)(RDF) -> ()
        where [
        ] { 
            let name = format!("is_{x:?}");
            satisfy(|t| { t == *x }, name.as_str()) 
        }
    }
    let s = r#"prefix : <http://example.org/>
    prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    
    :x :p rdf:nil .
    "#;

    let mut graph = SRDFGraph::from_str(s, None).unwrap();
    let x = IriS::new_unchecked("http://example.org/x");
    let y = IriS::new_unchecked("http://example.org/y");
    
    let y_term = <SRDFGraph as SRDFComparisons>::iri_s2term(&y);
    let value = my_ok_(&y_term).parse(&x, &mut graph);
    println!("SRDFGraph. Result of parser: {value:?}");

}