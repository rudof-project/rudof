use crate::reifier_info::ReifierInfo;
use crate::{component::Component, message_map::MessageMap, severity::Severity, target::Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{
    term::{literal::NumericLiteral, Object}, BuildRDF, Rdf,
    SHACLPath,
};

// impl<RDF: Rdf> PropertyShape<RDF> {

    // pub fn closed_component(&self) -> (bool, HashSet<IriS>) {
    //     for component in &self.components {
    //         if let Component::Closed {
    //             is_closed,
    //             ignored_properties,
    //         } = component
    //         {
    //             return (*is_closed, ignored_properties.clone());
    //         }
    //     }
    //     (false, HashSet::new())
    // }

    // // TODO: this is a bit ugly
    // pub fn write<B: BuildRDF>(&self, rdf: &mut B) -> Result<(), B::Err> {
    //     let id: B::Subject = self.id.clone().try_into().map_err(|_| unreachable!())?;
    //     rdf.add_type(id.clone(), ShaclVocab::sh_property_shape().clone())?;
    //
    //     self.name.iter().try_for_each(|(lang, value)| {
    //         let literal: B::Literal = match lang {
    //             Some(_) => todo!(),
    //             None => value.clone().into(),
    //         };
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_name().clone(), literal)
    //     })?;
    //
    //     self.description.iter().try_for_each(|(lang, value)| {
    //         let literal: B::Literal = match lang {
    //             Some(_) => todo!(),
    //             None => value.clone().into(),
    //         };
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_description().clone(), literal)
    //     })?;
    //
    //     if let Some(order) = self.order.clone() {
    //         let literal: B::Literal = match order {
    //             NumericLiteral::Decimal(_) => todo!(),
    //             NumericLiteral::Double(float) => float.into(),
    //             NumericLiteral::Float(float) => float.to_string().into(),
    //             #[allow(clippy::useless_conversion)]
    //             NumericLiteral::Integer(int) => {
    //                 let i: i128 = int.try_into().unwrap();
    //                 i.into()
    //             },
    //             NumericLiteral::Long(_) => todo!(),
    //             NumericLiteral::Byte(_) => todo!(),
    //             NumericLiteral::Short(_) => todo!(),
    //             NumericLiteral::NonNegativeInteger(_) => todo!(),
    //             NumericLiteral::UnsignedLong(_) => todo!(),
    //             NumericLiteral::UnsignedInt(_) => todo!(),
    //             NumericLiteral::UnsignedShort(_) => todo!(),
    //             NumericLiteral::UnsignedByte(_) => todo!(),
    //             NumericLiteral::PositiveInteger(_) => todo!(),
    //             NumericLiteral::NegativeInteger(_) => todo!(),
    //             NumericLiteral::NonPositiveInteger(_) => todo!(),
    //         };
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_order().clone(), literal)?;
    //     }
    //
    //     if let Some(group) = &self.group {
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_group().clone(), group.clone())?;
    //     }
    //
    //     if let SHACLPath::Predicate { pred } = &self.path {
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_path().clone(), pred.clone())?;
    //     } else {
    //         unimplemented!()
    //     }
    //
    //     self.components
    //         .iter()
    //         .try_for_each(|component| component.write(&self.id, rdf))?;
    //
    //     self.targets.iter().try_for_each(|target| target.write(&self.id, rdf))?;
    //
    //     if self.deactivated {
    //         let literal: B::Literal = "true".to_string().into();
    //
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_deactivated().clone(), literal)?;
    //     }
    //
    //     if let Some(severity) = &self.severity {
    //         let pred = match severity {
    //             Severity::Trace => ShaclVocab::sh_trace(),
    //             Severity::Debug => ShaclVocab::sh_debug(),
    //             Severity::Violation => ShaclVocab::sh_violation(),
    //             Severity::Info => ShaclVocab::sh_info(),
    //             Severity::Warning => ShaclVocab::sh_warning(),
    //             Severity::Generic(iri) => iri.get_iri().unwrap(),
    //         };
    //
    //         rdf.add_triple(id.clone(), ShaclVocab::sh_severity().clone(), pred.clone())?;
    //     }
    //
    //     Ok(())
    // }
// }
