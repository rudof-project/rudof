use iri_s::{iri, IriS};
use oxrdf::{BlankNode, Literal as OxLiteral, NamedNode, Subject, Term as OxTerm};
use srdf::{
    numeric_literal::NumericLiteral, RDFNode, SHACLPath, SRDFBuilder, SRDFGraph, SRDF,
    XSD_DECIMAL_STR,
};
use std::{collections::HashSet, fmt::Display};

use crate::{
    component::Component, message_map::MessageMap, severity::Severity, target::Target,
    SH_DEACTIVATED_STR, SH_DESCRIPTION_STR, SH_GROUP_STR, SH_INFO_STR, SH_NAME_STR, SH_ORDER_STR,
    SH_PATH_STR, SH_PROPERTY_SHAPE, SH_SEVERITY_STR, SH_VIOLATION_STR, SH_WARNING_STR,
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

    pub fn get_value_nodes(
        &self,
        data_graph: &SRDFGraph,
        focus_node: &RDFNode,
        path: &SHACLPath,
    ) -> HashSet<RDFNode> {
        match path {
            SHACLPath::Predicate { pred } => {
                let subject = match focus_node {
                    RDFNode::Iri(iri_s) => Subject::NamedNode(iri_s.as_named_node().to_owned()),
                    RDFNode::BlankNode(id) => Subject::BlankNode(BlankNode::new_unchecked(id)),
                    RDFNode::Literal(_) => todo!(),
                };
                if let Ok(objects) =
                    data_graph.objects_for_subject_predicate(&subject, pred.as_named_node())
                {
                    objects
                        .into_iter()
                        .map(|object| match object {
                            OxTerm::NamedNode(node) => {
                                RDFNode::iri(IriS::new_unchecked(node.as_str()))
                            }
                            OxTerm::BlankNode(node) => RDFNode::bnode(node.to_string()),
                            OxTerm::Literal(literal) => RDFNode::literal(literal.into()),
                            #[cfg(feature = "rdf-star")]
                            OxTerm::Triple(_) => unimplemented!(),
                        })
                        .collect::<HashSet<RDFNode>>()
                } else {
                    HashSet::new()
                }
            }
            _ => HashSet::new(),
        }
    }

    pub fn write<RDF>(&self, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        rdf.add_type(&self.id, RDF::iri_s2term(&SH_PROPERTY_SHAPE))?;

        self.name.to_term_iter().try_for_each(|term| {
            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_NAME_STR)),
                &RDF::term_s2term(&term),
            )
        })?;

        self.description.to_term_iter().try_for_each(|term| {
            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_DESCRIPTION_STR)),
                &RDF::term_s2term(&term),
            )
        })?;

        if let Some(order) = &self.order {
            let decimal_type = NamedNode::new(XSD_DECIMAL_STR).unwrap();

            let term = OxTerm::Literal(OxLiteral::new_typed_literal(
                order.to_string(),
                decimal_type,
            ));

            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_ORDER_STR)),
                &RDF::term_s2term(&term),
            )?;
        }

        if let Some(group) = &self.group {
            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_GROUP_STR)),
                &RDF::object_as_term(group),
            )?;
        }

        if let SHACLPath::Predicate { pred } = &self.path {
            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_PATH_STR)),
                &RDF::iri_s2term(pred),
            )?;
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
            let term = OxTerm::Literal(OxLiteral::new_simple_literal("true"));

            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_DEACTIVATED_STR)),
                &RDF::term_s2term(&term),
            )?;
        }

        if let Some(severity) = &self.severity {
            let pred = match severity {
                Severity::Violation => iri!(SH_VIOLATION_STR),
                Severity::Info => iri!(SH_INFO_STR),
                Severity::Warning => iri!(SH_WARNING_STR),
                Severity::Generic(iri) => iri.get_iri().unwrap(),
            };

            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_SEVERITY_STR)),
                &RDF::iri_s2term(&pred),
            )?;
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
