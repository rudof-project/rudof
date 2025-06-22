use crate::shacl_vocab::{
    sh_closed, sh_deactivated, sh_description, sh_group, sh_info, sh_name, sh_node_shape,
    sh_property, sh_severity, sh_violation, sh_warning,
};
use crate::{component::Component, message_map::MessageMap, severity::Severity, target::Target};
use srdf::{BuildRDF, RDFNode};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn id(&self) -> &RDFNode {
        &self.id
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

    // TODO: this is a bit ugly
    pub fn write<RDF>(&self, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        let id: RDF::Subject = self.id.clone().try_into().map_err(|_| unreachable!())?;
        rdf.add_type(id.clone(), sh_node_shape().clone())?;

        self.name.iter().try_for_each(|(lang, value)| {
            let literal: RDF::Literal = match lang {
                Some(_) => todo!(),
                None => value.clone().into(),
            };
            rdf.add_triple(id.clone(), sh_name().clone(), literal)
        })?;

        self.description.iter().try_for_each(|(lang, value)| {
            let literal: RDF::Literal = match lang {
                Some(_) => todo!(),
                None => value.clone().into(),
            };
            rdf.add_triple(id.clone(), sh_description().clone(), literal)
        })?;

        self.components
            .iter()
            .try_for_each(|component| component.write(&self.id, rdf))?;

        self.targets
            .iter()
            .try_for_each(|target| target.write(&self.id, rdf))?;

        self.property_shapes.iter().try_for_each(|property_shape| {
            rdf.add_triple(id.clone(), sh_property().clone(), property_shape.clone())
        })?;

        if self.deactivated {
            let literal: RDF::Literal = "true".to_string().into();

            rdf.add_triple(id.clone(), sh_deactivated().clone(), literal)?;
        }

        if let Some(group) = &self.group {
            rdf.add_triple(id.clone(), sh_group().clone(), group.clone())?;
        }

        if let Some(severity) = &self.severity {
            let pred = match severity {
                Severity::Violation => sh_violation().clone(),
                Severity::Info => sh_info().clone(),
                Severity::Warning => sh_warning().clone(),
                Severity::Generic(iri) => iri.get_iri().unwrap(),
            };

            rdf.add_triple(id.clone(), sh_severity().clone(), pred.clone())?;
        }

        if self.closed {
            let literal: RDF::Literal = "true".to_string().into();

            rdf.add_triple(id.clone(), sh_closed().clone(), literal)?;
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
