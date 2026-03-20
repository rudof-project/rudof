use crate::{component::Component, message_map::MessageMap, severity::Severity, target::Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{term::Object, BuildRDF, Rdf};

// impl<RDF: Rdf> NodeShape<RDF> {
//     pub fn is_deactivated(&self) -> bool {
//         for component in &self.components {
//             // TODO - For NodeExpr, do not delete
//             // if let Component::Deactivated(NodeExpr::Literal(ConcreteLiteral::BooleanLiteral(true))) = component {
//             if let Component::Deactivated(true) = component {
//                 return true;
//             }
//         }
//         false
//     }
//
//     pub fn closed_component(&self) -> (bool, HashSet<IriS>) {
//         for component in &self.components {
//             if let Component::Closed {
//                 is_closed,
//                 ignored_properties,
//             } = component
//             {
//                 return (*is_closed, ignored_properties.clone());
//             }
//         }
//         (false, HashSet::new())
//     }
//
//     // TODO: this is a bit ugly
//     pub fn write<B: BuildRDF>(&self, rdf: &mut B) -> Result<(), B::Err> {
//         let id: B::Subject = self.id.clone().try_into().map_err(|_| unreachable!())?;
//         rdf.add_type(id.clone(), ShaclVocab::sh_node_shape().clone())?;
//
//         self.name.iter().try_for_each(|(lang, value)| {
//             let literal: B::Literal = match lang {
//                 Some(_) => todo!(),
//                 None => value.clone().into(),
//             };
//             rdf.add_triple(id.clone(), ShaclVocab::sh_name().clone(), literal)
//         })?;
//
//         self.description.iter().try_for_each(|(lang, value)| {
//             let literal: B::Literal = match lang {
//                 Some(_) => todo!(),
//                 None => value.clone().into(),
//             };
//             rdf.add_triple(id.clone(), ShaclVocab::sh_description().clone(), literal)
//         })?;
//
//         self.components
//             .iter()
//             .try_for_each(|component| component.write(&self.id, rdf))?;
//
//         self.targets.iter().try_for_each(|target| target.write(&self.id, rdf))?;
//
//         self.property_shapes.iter().try_for_each(|property_shape| {
//             rdf.add_triple(id.clone(), ShaclVocab::sh_property().clone(), property_shape.clone())
//         })?;
//
//         if let Some(group) = &self.group {
//             rdf.add_triple(id.clone(), ShaclVocab::sh_group().clone(), group.clone())?;
//         }
//
//         if let Some(severity) = &self.severity {
//             let pred = match severity {
//                 Severity::Trace => ShaclVocab::sh_trace(),
//                 Severity::Debug => ShaclVocab::sh_debug(),
//                 Severity::Violation => ShaclVocab::sh_violation(),
//                 Severity::Info => ShaclVocab::sh_info(),
//                 Severity::Warning => ShaclVocab::sh_warning(),
//                 Severity::Generic(iri) => iri.get_iri().unwrap(),
//             };
//
//             rdf.add_triple(id.clone(), ShaclVocab::sh_severity().clone(), pred.clone())?;
//         }
//
//         Ok(())
//     }
// }
