use std::fmt::Display;

use crate::shacl_vocab::{
    sh_deactivated, sh_description, sh_group, sh_info, sh_name, sh_order, sh_path,
    sh_property_shape, sh_severity, sh_violation, sh_warning,
};
use crate::{component::Component, message_map::MessageMap, severity::Severity, target::Target};
use srdf::Rdf;
use srdf::{BuildRDF, RDFNode, SHACLPath, numeric_literal::NumericLiteral};

#[derive(Debug)]
pub struct PropertyShape<RDF: Rdf> {
    id: RDFNode,
    path: SHACLPath,
    components: Vec<Component>,
    targets: Vec<Target<RDF>>,
    property_shapes: Vec<RDFNode>,
    closed: bool,
    // ignored_properties: Vec<IriRef>,
    deactivated: bool,
    // message: MessageMap,
    severity: Option<Severity>,
    name: MessageMap,
    description: MessageMap,
    order: Option<NumericLiteral>,
    group: Option<RDFNode>,
    // source_iri: Option<IriRef>,
    // annotations: Vec<(IriRef, RDFNode)>,
}

impl<RDF: Rdf> PropertyShape<RDF> {
    pub fn new(id: RDFNode, path: SHACLPath) -> Self {
        PropertyShape {
            id,
            path,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            closed: false,
            // ignored_properties: Vec::new(),
            deactivated: false,
            // message: MessageMap::new(),
            severity: None,
            name: MessageMap::new(),
            description: MessageMap::new(),
            order: None,
            group: None,
            // source_iri: None,
            // annotations: Vec::new()
        }
    }

    pub fn with_name(mut self, name: MessageMap) -> Self {
        self.name = name;
        self
    }
    pub fn with_description(mut self, description: MessageMap) -> Self {
        self.description = description;
        self
    }

    pub fn with_order(mut self, order: Option<NumericLiteral>) -> Self {
        self.order = order;
        self
    }

    pub fn with_group(mut self, group: Option<RDFNode>) -> Self {
        self.group = group;
        self
    }

    pub fn with_targets(mut self, targets: Vec<Target<RDF>>) -> Self {
        self.targets = targets;
        self
    }

    pub fn with_property_shapes(mut self, property_shapes: Vec<RDFNode>) -> Self {
        self.property_shapes = property_shapes;
        self
    }

    pub fn with_components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn with_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    pub fn with_severity(mut self, severity: Option<Severity>) -> Self {
        self.severity = severity;
        self
    }

    pub fn id(&self) -> &RDFNode {
        &self.id
    }

    pub fn path(&self) -> &SHACLPath {
        &self.path
    }

    pub fn name(&self) -> &MessageMap {
        &self.name
    }

    pub fn description(&self) -> &MessageMap {
        &self.description
    }

    pub fn is_closed(&self) -> &bool {
        &self.closed
    }

    pub fn is_deactivated(&self) -> &bool {
        &self.deactivated
    }

    pub fn severity(&self) -> Option<Severity> {
        self.severity.to_owned()
    }

    pub fn components(&self) -> &Vec<Component> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<Target<RDF>> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<RDFNode> {
        &self.property_shapes
    }

    // pub fn get_value_nodes(
    //     &self,
    //     data_graph: &SRDFGraph,
    //     focus_node: &RDFNode,
    //     path: &SHACLPath,
    // ) -> HashSet<RDFNode> {
    //     match path {
    //         SHACLPath::Predicate { pred } => {
    //             let subject = match focus_node {
    //                 RDFNode::Iri(iri_s) => Subject::NamedNode(iri_s.as_named_node().to_owned()),
    //                 RDFNode::BlankNode(id) => Subject::BlankNode(BlankNode::new_unchecked(id)),
    //                 RDFNode::Literal(_) => todo!(),
    //             };
    //             if let Ok(objects) =
    //                 data_graph.objects_for_subject_predicate(&subject, pred.as_named_node())
    //             {
    //                 objects
    //                     .into_iter()
    //                     .map(|object| match object {
    //                         Term::NamedNode(node) => {
    //                             RDFNode::iri(IriS::new_unchecked(node.as_str()))
    //                         }
    //                         Term::BlankNode(node) => RDFNode::bnode(node.to_string()),
    //                         Term::Literal(literal) => RDFNode::literal(literal.into()),
    //                         #[cfg(feature = "rdf-star")]
    //                         Term::Triple(_) => unimplemented!(),
    //                     })
    //                     .collect::<HashSet<RDFNode>>()
    //             } else {
    //                 HashSet::new()
    //             }
    //         }
    //         SHACLPath::Alternative { .. } => todo!(),
    //         SHACLPath::Sequence { .. } => todo!(),
    //         SHACLPath::Inverse { .. } => todo!(),
    //         SHACLPath::ZeroOrMore { .. } => todo!(),
    //         SHACLPath::OneOrMore { .. } => todo!(),
    //         SHACLPath::ZeroOrOne { .. } => todo!(),
    //     }
    // }

    // TODO: this is a bit ugly
    pub fn write<B>(&self, rdf: &mut B) -> Result<(), B::Err>
    where
        B: BuildRDF,
    {
        let id: B::Subject = self.id.clone().try_into().map_err(|_| unreachable!())?;
        rdf.add_type(id.clone(), sh_property_shape().clone())?;

        self.name.iter().try_for_each(|(lang, value)| {
            let literal: B::Literal = match lang {
                Some(_) => todo!(),
                None => value.clone().into(),
            };
            rdf.add_triple(id.clone(), sh_name().clone(), literal)
        })?;

        self.description.iter().try_for_each(|(lang, value)| {
            let literal: B::Literal = match lang {
                Some(_) => todo!(),
                None => value.clone().into(),
            };
            rdf.add_triple(id.clone(), sh_description().clone(), literal)
        })?;

        if let Some(order) = self.order.clone() {
            let literal: B::Literal = match order {
                NumericLiteral::Decimal(_) => todo!(),
                NumericLiteral::Double(float) => float.into(),
                NumericLiteral::Integer(int) => {
                    let i: i128 = int.try_into().unwrap();
                    i.into()
                }
            };
            rdf.add_triple(id.clone(), sh_order().clone(), literal)?;
        }

        if let Some(group) = &self.group {
            rdf.add_triple(id.clone(), sh_group().clone(), group.clone())?;
        }

        if let SHACLPath::Predicate { pred } = &self.path {
            rdf.add_triple(id.clone(), sh_path().clone(), pred.clone())?;
        } else {
            unimplemented!()
        }

        self.components
            .iter()
            .try_for_each(|component| component.write(&self.id, rdf))?;

        self.targets
            .iter()
            .try_for_each(|target| target.write(&self.id, rdf))?;

        if self.deactivated {
            let literal: B::Literal = "true".to_string().into();

            rdf.add_triple(id.clone(), sh_deactivated().clone(), literal)?;
        }

        if let Some(severity) = &self.severity {
            let pred = match severity {
                Severity::Violation => sh_violation(),
                Severity::Info => sh_info(),
                Severity::Warning => sh_warning(),
                Severity::Generic(iri) => &iri.get_iri().unwrap(),
            };

            rdf.add_triple(id.clone(), sh_severity().clone(), pred.clone())?;
        }

        Ok(())
    }
}

impl<RDF: Rdf> Display for PropertyShape<RDF> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "       PropertyShape")?;
        writeln!(f, "       path: {}", self.path)?;
        for target in self.targets.iter() {
            writeln!(f, "       {target}")?
        }
        if self.closed {
            writeln!(f, "       closed: {}", self.closed)?
        }
        for property in self.property_shapes.iter() {
            writeln!(f, "       Property {property}")?
        }
        for component in self.components.iter() {
            writeln!(f, "       {component}")?
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<RDF: Rdf> Clone for PropertyShape<RDF> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            path: self.path.clone(),
            components: self.components.clone(),
            targets: self.targets.clone(),
            property_shapes: self.property_shapes.clone(),
            closed: self.closed,
            deactivated: self.deactivated,
            severity: self.severity.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            order: self.order.clone(),
            group: self.group.clone(),
        }
    }
}

impl<RDF: Rdf> PartialEq for PropertyShape<RDF> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.path == other.path
            && self.components == other.components
            && self.targets == other.targets
            && self.property_shapes == other.property_shapes
            && self.closed == other.closed
            && self.deactivated == other.deactivated
            && self.severity == other.severity
            && self.name == other.name
            && self.description == other.description
            && self.order == other.order
            && self.group == other.group
    }
}
