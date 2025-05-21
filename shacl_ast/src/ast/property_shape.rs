use std::fmt::Display;

use iri_s::iri;
use srdf::{numeric_literal::NumericLiteral, RDFNode, SHACLPath, SRDFBuilder};

use crate::{
    component::Component, message_map::MessageMap, severity::Severity, target::Target,
    SH_DEACTIVATED, SH_DESCRIPTION, SH_GROUP, SH_INFO_STR, SH_NAME, SH_ORDER, SH_PATH,
    SH_PROPERTY_SHAPE, SH_SEVERITY, SH_VIOLATION_STR, SH_WARNING_STR,
};

#[derive(Debug, Clone)]
pub struct PropertyShape {
    id: RDFNode,
    path: SHACLPath,
    components: Vec<Component>,
    targets: Vec<Target>,
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

impl PropertyShape {
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

    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
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

    pub fn targets(&self) -> &Vec<Target> {
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
    pub fn write<RDF>(&self, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        let id: RDF::Subject = self.id.try_into().map_err(|_| unreachable!())?;
        rdf.add_type(&id, &SH_PROPERTY_SHAPE.clone())?;

        self.name.iter().try_for_each(|(lang, value)| {
            let literal: RDF::Literal = match lang {
                Some(_) => todo!(),
                None => value.clone().into(),
            };
            rdf.add_triple(id.clone(), SH_NAME.clone(), literal)
        })?;

        self.description.iter().try_for_each(|(lang, value)| {
            let literal: RDF::Literal = match lang {
                Some(_) => todo!(),
                None => value.clone().into(),
            };
            rdf.add_triple(id.clone(), SH_DESCRIPTION.clone(), literal)
        })?;

        if let Some(order) = self.order.clone() {
            let literal: RDF::Literal = match order {
                NumericLiteral::Decimal(_) => todo!(),
                NumericLiteral::Double(float) => float.into(),
                NumericLiteral::Integer(int) => {
                    let i: i128 = int.try_into().unwrap();
                    i.into()
                }
            };
            rdf.add_triple(id.clone(), SH_ORDER.clone(), literal)?;
        }

        if let Some(group) = &self.group {
            rdf.add_triple(id.clone(), SH_GROUP.clone(), group.clone())?;
        }

        if let SHACLPath::Predicate { pred } = &self.path {
            rdf.add_triple(id.clone(), SH_PATH.clone(), pred.clone())?;
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
            let literal: RDF::Literal = "true".to_string().into();

            rdf.add_triple(id.clone(), SH_DEACTIVATED.clone(), literal)?;
        }

        if let Some(severity) = &self.severity {
            let pred = match severity {
                Severity::Violation => iri!(SH_VIOLATION_STR),
                Severity::Info => iri!(SH_INFO_STR),
                Severity::Warning => iri!(SH_WARNING_STR),
                Severity::Generic(iri) => iri.get_iri().unwrap(),
            };

            rdf.add_triple(id.clone(), SH_SEVERITY.clone(), pred.clone())?;
        }

        Ok(())
    }
}

impl Display for PropertyShape {
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
