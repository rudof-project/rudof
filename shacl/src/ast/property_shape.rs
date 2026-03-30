use crate::ast::error::ASTError;
use crate::ast::reifier_info::ReifierInfo;
use crate::ast::{ASTComponent, ASTSchema};
use crate::types::{defined_properties_for, ClosedInfo, MessageMap, Severity, Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::term::literal::NumericLiteral;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::SHACLPath;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct ASTPropertyShape {
    id: Object,
    path: SHACLPath,
    components: Vec<ASTComponent>,
    targets: Vec<Target>,
    property_shapes: Vec<Object>,
    reifier_info: Option<ReifierInfo>,
    closed: bool,
    // ignored_properties: Vec<IriRef>,
    deactivated: bool,
    // message: MessageMap,
    severity: Option<Severity>,
    name: MessageMap,
    description: MessageMap,
    order: Option<NumericLiteral>,
    group: Option<Object>,
    // source_iri: Option<IriRef>,
    // annotations: Vec<(IriRef, RDFNode)>

    // TODO - For node expr
    // default_value: Option<NodeExpr<RDF>>, // ONLY WHEN PATH IS PREDICATE PATH
    // values: Option<NodeExpr<RDF>> // ONLY WHEN PATH IS PREDICATE PATH
}

impl ASTPropertyShape {
    pub fn new(id: Object, path: SHACLPath) -> Self {
        ASTPropertyShape {
            id,
            path,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            closed: false,
            deactivated: false,
            severity: None,
            name: MessageMap::new(),
            description: MessageMap::new(),
            order: None,
            group: None,
            reifier_info: None,
            // TODO - For NodeExpr, do not delete
            // default_value: None,
            // values: None,
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

    pub fn with_group(mut self, group: Option<Object>) -> Self {
        self.group = group;
        self
    }

    pub fn reifier_info(&self) -> Option<&ReifierInfo> {
        self.reifier_info.as_ref()
    }

    pub fn with_reifier_shape(mut self, reifier_info: Option<ReifierInfo>) -> Self {
        self.reifier_info = reifier_info;
        self
    }

    pub fn with_severity(mut self, severity: Option<Severity>) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
        self.targets = targets;
        self
    }

    pub fn with_property_shapes(mut self, property_shape: Vec<Object>) -> Self {
        self.property_shapes = property_shape;
        self
    }

    pub fn with_components(mut self, components: Vec<ASTComponent>) -> Self {
        self.components = components;
        self
    }

    pub fn with_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    // TODO - For NodeExpr, do not delete
    // pub fn with_values(mut self, values: Option<NodeExpr<RDF>>) -> Self {
    //     self.values = values;
    //     self
    // }
    //
    // pub fn with_default_value(mut self, default_value: Option<NodeExpr<RDF>>) -> Self {
    //     self.default_value = default_value;
    //     self
    // }

    // TODO - For NodeExpr, do not delete
    // pub fn values(&self) -> Option<&NodeExpr<RDF>> {
    //     self.values.as_ref()
    // }
    //
    // pub fn default_value(&self) -> Option<&NodeExpr<RDF>> {
    //     self.default_value.as_ref()
    // }

    pub fn id(&self) -> &Object {
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

    pub fn is_deactivated(&self) -> bool {
        self.deactivated
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

    pub fn order(&self) -> Option<&NumericLiteral> {
        self.order.as_ref()
    }

    pub fn group(&self) -> Option<&Object> {
        self.group.as_ref()
    }

    fn closed_component(&self) -> (bool, HashSet<IriS>) {
        for component in &self.components {
            if let ASTComponent::Closed {
                is_closed, ignored_properties
            } = component {
                return (*is_closed, ignored_properties.clone())
            }
        }
        (false, HashSet::new())
    }

    pub fn get_closed_info(&self, ast: &ASTSchema) -> Result<ClosedInfo, ASTError> {
        let (is_closed, ignored_properties) = self.closed_component();
        if is_closed {
            let defined_properties = defined_properties_for(self.property_shapes(), ast)?;
            Ok(ClosedInfo::Yes {
                ignored_properties,
                defined_properties
            })
        } else {
            Ok(ClosedInfo::No)
        }
    }
}

impl Display for ASTPropertyShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(severity) = self.severity() {
            write!(f, "{severity} ")?;
        }
        writeln!(f, "{{")?;
        writeln!(f, "\tPropertyShape")?;
        writeln!(f, "\tpath: {}", self.path)?;
        for target in self.targets.iter() {
            writeln!(f, "\t{target}")?
        }
        if self.closed {
            writeln!(f, "\tclosed: {}", self.closed)?
        }
        for property in self.property_shapes.iter() {
            writeln!(f, "\tProperty {property}")?
        }
        for reifier in self.reifier_info.iter() {
            writeln!(f, "\tReifierInfo {reifier}")?
        }
        for component in self.components.iter() {
            writeln!(f, "\t{component}")?
        }
        // TODO - For NodeExpr, do not delete
        // if let Some(v) = &self.default_value {
        //     writeln!(f, "\tdefault_value: {}", v)?;
        // }
        // if let Some(v) = &self.values {
        //     writeln!(f, "\tvalues: {}", v)?;
        // }
        write!(f, "}}")
    }
}
