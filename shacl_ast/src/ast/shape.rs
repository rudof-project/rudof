use rudof_rdf::rdf_core::{BuildRDF, Rdf};
use std::fmt::Display;

use crate::{node_shape::NodeShape, property_shape::PropertyShape};

// impl<RDF: Rdf> Shape<RDF> {
//     pub fn write<B>(&self, rdf: &mut B) -> Result<(), B::Err>
//     where
//         B: BuildRDF,
//     {
//         match self {
//             Shape::NodeShape(ns) => {
//                 ns.write(rdf)?;
//             },
//             Shape::PropertyShape(ps) => {
//                 ps.write(rdf)?;
//             },
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use rudof_rdf::{rdf_core::term::Object, rdf_impl::InMemoryGraph};

    use crate::{node_shape::NodeShape, shape::Shape};

    #[test]
    fn test_clone() {
        let ns: NodeShape<InMemoryGraph> = NodeShape::new(Object::Iri(iri!("http://example.org/id")));
        let s1 = Shape::node_shape(ns);
        let s2 = s1.clone();
        assert_eq!(s1, s2)
    }
}
