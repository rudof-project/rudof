use crate::ast::error::ASTError;
use crate::ast::{ASTComponent, ASTSchema};
use crate::types::{defined_properties_for, ClosedInfo, MessageMap, Severity, Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ASTNodeShape {
    id: Object,
    components: Vec<ASTComponent>,
    targets: Vec<Target>,
    property_shapes: Vec<Object>,
    // closed: bool,
    // ignored_properties: Vec<IriRef>,
    // message: MessageMap,
    severity: Option<Severity>,
    name: MessageMap,
    description: MessageMap,
    group: Option<Object>,
    // source_iri: Option<IriRef>,
}

impl ASTNodeShape {
    pub fn new(id: Object) -> Self {
        ASTNodeShape {
            id,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            severity: None,
            name: MessageMap::new(),
            description: MessageMap::new(),
            group: None,
        }
    }

    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
        self.targets = targets;
        self
    }

    pub fn with_severity(mut self, severity: Option<Severity>) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_property_shapes(mut self, property_shapes: Vec<Object>) -> Self {
        self.property_shapes = property_shapes;
        self
    }

    pub fn with_components(mut self, components: Vec<ASTComponent>) -> Self {
        self.components = components;
        self
    }

    pub fn id(&self) -> &Object {
        &self.id
    }

    pub fn severity(&self) -> Option<&Severity> {
        self.severity.as_ref()
    }

    pub fn components(&self) -> &Vec<ASTComponent> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<Target> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<Object> {
        &self.property_shapes
    }

    pub fn get_closed_info(&self, ast: &ASTSchema) -> Result<ClosedInfo, ASTError> {
        let (is_closed, ignored_properties) = self.closed_component();
        if is_closed {
            let defined_properties = defined_properties_for(self.property_shapes(), ast)?;
            Ok(ClosedInfo::Yes {
                ignored_properties,
                defined_properties,
            })
        } else {
            Ok(ClosedInfo::No)
        }
    }
}

impl Display for ASTNodeShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(severity) = self.severity() {
            write!(f, "{severity} ")?;
        }
        writeln!(f, "{{")?;
        for target in self.targets.iter() {
            writeln!(f, "\t{target}")?
        }
        for prop in self.property_shapes().iter() {
            writeln!(f, "\t{prop}")?
        }
        for component in self.components().iter() {
            writeln!(f, "\t{component}")?
        }
        writeln!(f, "}}")
    }
}
