use prefixmap::IriRef;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab, ShaclVocab};
use rudof_rdf::rdf_core::{term::Object, BuildRDF, Rdf};

// impl<RDF: Rdf> Target<RDF> {
//     pub fn write<B: BuildRDF>(&self, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         let node: B::Subject = rdf_node.clone().try_into().map_err(|_| unreachable!())?;
//         match self {
//             Target::Node(target_rdf_node) => {
//                 rdf.add_triple(node, ShaclVocab::sh_target_node().clone(), target_rdf_node.clone())
//             },
//             Target::Class(node_class) => {
//                 rdf.add_triple(node, ShaclVocab::sh_target_class().clone(), node_class.clone())
//             },
//             Target::SubjectsOf(iri_ref) => rdf.add_triple(
//                 node,
//                 ShaclVocab::sh_target_subjects_of().clone(),
//                 iri_ref.get_iri().unwrap().clone(),
//             ),
//             Target::ObjectsOf(iri_ref) => rdf.add_triple(
//                 node,
//                 ShaclVocab::sh_target_objects_of().clone(),
//                 iri_ref.get_iri().unwrap().clone(),
//             ),
//             Target::ImplicitClass(_class) => {
//                 // TODO: Review this code and in SHACL 1.2, add sh_shape_class ?
//                 rdf.add_triple(node, RdfVocab::rdf_type().clone(), RdfsVocab::rdfs_class().clone())
//             },
//             Target::WrongNode(_) => todo!(),
//             Target::WrongClass(_) => todo!(),
//             Target::WrongSubjectsOf(_) => todo!(),
//             Target::WrongObjectsOf(_) => todo!(),
//             Target::WrongImplicitClass(_) => todo!(),
//         }
//     }
// }
