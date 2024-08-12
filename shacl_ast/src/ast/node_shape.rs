use crate::{
    component::Component, message_map::MessageMap, severity::Severity, target::Target,
    SH_CLOSED_STR, SH_DEACTIVATED_STR, SH_DESCRIPTION_STR, SH_GROUP_STR, SH_INFO_STR, SH_NAME_STR,
    SH_NODE_SHAPE, SH_PROPERTY_STR, SH_SEVERITY_STR, SH_VIOLATION_STR, SH_WARNING_STR,
};
use iri_s::iri;
use oxrdf::{Literal as OxLiteral, Term as OxTerm};
use srdf::{RDFNode, SRDFBuilder};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct NodeShape {
    id: RDFNode,
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
    group: Option<RDFNode>,
    // source_iri: Option<IriRef>,
}

impl NodeShape {
    pub fn new(id: RDFNode) -> Self {
        NodeShape {
            id,
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
            group: None,
            // source_iri: None,
        }
    }

    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
        self.targets = targets;
        self
    }

    pub fn set_targets(&mut self, targets: Vec<Target>) {
        self.targets = targets;
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

    pub fn id(&self) -> RDFNode {
        self.id.clone()
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

    pub fn write<RDF>(&self, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        rdf.add_type(&self.id, RDF::iri_s2term(&SH_NODE_SHAPE))?;

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

        self.components
            .iter()
            .try_for_each(|component| component.write(&self.id, rdf))?;

        self.targets
            .iter()
            .try_for_each(|target| target.write(&self.id, rdf))?;

        self.property_shapes.iter().try_for_each(|property_shape| {
            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_PROPERTY_STR)),
                &RDF::object_as_term(property_shape),
            )
        })?;

        if self.deactivated {
            let term = OxTerm::Literal(OxLiteral::new_simple_literal("true"));

            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_DEACTIVATED_STR)),
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

        if self.closed {
            let term = OxTerm::Literal(OxLiteral::from(true));

            rdf.add_triple(
                &RDF::object_as_subject(&self.id).unwrap(),
                &RDF::iri_s2iri(&iri!(SH_CLOSED_STR)),
                &RDF::term_s2term(&term),
            )?;
        }

        Ok(())
    }
}

impl Display for NodeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        if self.closed {
            writeln!(f, "       closed: {}", self.closed)?
        }
        for target in self.targets.iter() {
            writeln!(f, "       {target}")?
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
