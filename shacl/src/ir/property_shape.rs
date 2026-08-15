use crate::ast::{ASTPropertyShape, ASTSchema};
use crate::ir::component::IRComponent;
use crate::ir::dg::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::schema::IRSchema;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::ir::{OrderValue, ReifierInfo};
use crate::types::{ClosedInfo, MessageMap, Severity, Target};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, NumericLiteral};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{BuildRDF, SHACLPath};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct IRPropertyShape {
    id: Object,
    path: SHACLPath,
    components: Vec<IRComponent>,
    targets: Vec<Target>,
    property_shapes: Vec<ShapeLabelIdx>,
    closed_info: ClosedInfo,
    deactivated: bool,
    message: Option<MessageMap>,
    severity: Option<Severity>,

    name: Option<MessageMap>,
    description: Option<MessageMap>,
    order: Option<OrderValue>,
    group: Option<Object>,

    // SHACL 1.2: Reifier info is only present for property shapes
    reifier_info: Option<ReifierInfo>,
    // source_iri: Option<S::IRI>,
    // annotations: Vec<(S::IRI, S::Term)>,
}

impl IRPropertyShape {
    pub fn new(id: Object, path: SHACLPath, closed_info: ClosedInfo) -> Self {
        IRPropertyShape {
            id,
            path,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            closed_info,
            deactivated: false,
            message: None,
            severity: None,
            reifier_info: None,
            name: None,
            description: None,
            order: None,
            group: None,
        }
    }

    pub fn with_components(mut self, components: Vec<IRComponent>) -> Self {
        self.components = components;
        self
    }
    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
        self.targets = targets;
        self
    }
    pub fn with_property_shapes(mut self, property_shapes: Vec<ShapeLabelIdx>) -> Self {
        self.property_shapes = property_shapes;
        self
    }
    pub fn with_deactivated(mut self, deactivated: bool) -> Self {
        self.deactivated = deactivated;
        self
    }
    pub fn with_severity(mut self, severity: Option<Severity>) -> Self {
        self.severity = severity;
        self
    }
    pub fn with_reifier_info(mut self, reifier_info: Option<ReifierInfo>) -> Self {
        self.reifier_info = reifier_info;
        self
    }

    pub fn with_name(mut self, name: Option<MessageMap>) -> Self {
        self.name = name;
        self
    }
    pub fn with_description(mut self, description: Option<MessageMap>) -> Self {
        self.description = description;
        self
    }
    pub fn with_order(mut self, order: Option<OrderValue>) -> Self {
        self.order = order;
        self
    }

    pub fn with_group(mut self, group: Option<Object>) -> Self {
        self.group = group;
        self
    }

    pub fn with_message(mut self, message: Option<MessageMap>) -> Self {
        self.message = message;
        self
    }

    pub fn id(&self) -> &Object {
        &self.id
    }

    pub fn closed(&self) -> bool {
        self.closed_info.is_closed()
    }

    pub fn reifier_info(&self) -> Option<&ReifierInfo> {
        self.reifier_info.as_ref()
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        self.closed_info.allowed_properties().unwrap_or_default()
    }

    pub fn path(&self) -> &SHACLPath {
        &self.path
    }

    pub fn deactivated(&self) -> bool {
        self.deactivated
    }

    pub fn severity(&self) -> &Severity {
        match &self.severity {
            None => &Severity::Violation,
            Some(severity) => severity,
        }
    }

    pub fn components(&self) -> &Vec<IRComponent> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<Target> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<ShapeLabelIdx> {
        &self.property_shapes
    }

    pub fn message(&self) -> Option<&MessageMap> {
        self.message.as_ref()
    }

    pub fn name(&self) -> Option<&MessageMap> {
        self.name.as_ref()
    }

    pub fn description(&self) -> Option<&MessageMap> {
        self.description.as_ref()
    }

    pub fn order(&self) -> Option<&OrderValue> {
        self.order.as_ref()
    }
}

impl IRPropertyShape {
    pub fn compile(shape: &ASTPropertyShape, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let mut compiled_components = Vec::new();
        for component in shape.components() {
            let component = IRComponent::compile(component, ast, ir)?;
            compiled_components.push(component);
        }

        let mut compiled_prop_shapes = Vec::new();
        for prop_shape in shape.property_shapes() {
            let idx = ir.register_shape(prop_shape, None, ast)?;
            compiled_prop_shapes.push(idx);
        }

        let closed_info = shape.get_closed_info(ast)?;

        let reifier_info = ReifierInfo::get_reifier_info(shape, ast, ir)?;

        println!("Compiling property shape with order: {:?}", shape.order());

        let compiled_prop_shape = IRPropertyShape::new(shape.id().clone(), shape.path().to_owned(), closed_info)
            .with_components(compiled_components)
            .with_targets(shape.targets().to_owned())
            .with_property_shapes(compiled_prop_shapes)
            .with_deactivated(shape.is_deactivated())
            .with_severity(shape.severity().cloned())
            .with_reifier_info(reifier_info)
            .with_name(shape.name().cloned())
            .with_description(shape.description().cloned())
            .with_order(shape.order().cloned())
            .with_group(shape.group().cloned())
            .with_message(shape.message().cloned());

        Ok(compiled_prop_shape)
    }
}

impl IRPropertyShape {
    // Register the property shape in the RDF graph
    // This is used for serializing the IR back to RDF
    pub fn register<RDF: BuildRDF>(
        &self,
        graph: &mut RDF,
        shapes_map: &HashMap<ShapeLabelIdx, IRShape>,
    ) -> Result<(), IRError> {
        let id: RDF::Subject = self.id.clone().try_into().unwrap_or_else(|_| unreachable!());
        graph
            .add_type(id.clone(), ShaclVocab::sh_property_shape())
            .map_err(|e| IRError::from_rdf_err::<RDF>("add type", e))?;

        if let Some(name) = &self.name {
            name.iter_literals().try_for_each(|lit| {
                graph
                    .add_triple::<_, _, RDF::Literal>(id.clone(), ShaclVocab::sh_name(), lit.into())
                    .map_err(IRError::add_triple::<RDF>)
            })?;
        }

        if let Some(description) = &self.description {
            description.iter_literals().try_for_each(|lit| {
                graph
                    .add_triple::<_, _, RDF::Literal>(id.clone(), ShaclVocab::sh_description(), lit.into())
                    .map_err(IRError::add_triple::<RDF>)
            })?;
        }

        if let Some(order) = &self.order {
            let lit: RDF::Literal = match order {
                OrderValue::Integer(i) => (*i).into(),
                OrderValue::Decimal(d) => {
                    let decimal_literal = ConcreteLiteral::NumericLiteral(NumericLiteral::Decimal(d.clone()));
                    let literal: RDF::Literal = decimal_literal.try_into().unwrap_or_else(|_| unreachable!());
                    literal
                },
            };

            graph
                .add_triple(id.clone(), ShaclVocab::sh_order(), lit)
                .map_err(IRError::add_triple::<RDF>)?;
        }

        if let Some(group) = &self.group {
            graph
                .add_triple(id.clone(), ShaclVocab::sh_group(), group.clone())
                .map_err(IRError::add_triple::<RDF>)?;
        }

        if let SHACLPath::Predicate { pred } = &self.path {
            graph
                .add_triple(id.clone(), ShaclVocab::sh_path(), pred.clone())
                .map_err(IRError::add_triple::<RDF>)?;
        } else {
            unimplemented!()
        }

        self.components
            .iter()
            .try_for_each(|component| component.register(&self.id, graph, shapes_map))?;

        self.targets
            .iter()
            .try_for_each(|target| target.register(&self.id, graph))
            .map_err(|e| IRError::from_rdf_err::<RDF>("add target to graph", e))?;

        if self.deactivated {
            let lit: RDF::Literal = "true".to_string().into();

            graph
                .add_triple(id.clone(), ShaclVocab::sh_deactivated(), lit)
                .map_err(IRError::add_triple::<RDF>)?;
        }

        if let Some(severity) = &self.severity {
            graph
                .add_triple::<_, _, IriS>(id.clone(), ShaclVocab::sh_severity(), severity.clone().into())
                .map_err(IRError::add_triple::<RDF>)?;
        }

        Ok(())
    }
}

impl IRPropertyShape {
    pub fn add_edges(
        &self,
        idx: ShapeLabelIdx,
        dg: &mut DependencyGraph,
        posneg: PosNeg,
        ir: &IRSchema,
        cache: &mut HashSet<ShapeLabelIdx>,
    ) {
        for component in &self.components {
            component.add_edges(idx, dg, posneg, ir, cache);
        }

        for prop_shape_idx in &self.property_shapes {
            if let Some(shape) = ir.get_shape_from_idx(prop_shape_idx) {
                dg.add_edge(idx, *prop_shape_idx, posneg);
                if !cache.contains(prop_shape_idx) {
                    cache.insert(*prop_shape_idx);
                    shape.add_edges(*prop_shape_idx, dg, posneg, ir, cache);
                }
            }
        }
    }
}

impl Display for IRPropertyShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PropertyShape {}", self.id())?;
        writeln!(f, " path: {}", self.path())?;
        if let Some(reifier_info) = self.reifier_info() {
            writeln!(
                f,
                " reifier info: reification required: {}, reifier shapes: [{}]",
                reifier_info.reification_required(),
                reifier_info
                    .reifier_shape()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        if let Some(message) = self.message() {
            writeln!(f, " message: {message}")?;
        }
        if let Some(name) = self.name() {
            writeln!(f, " name: {name}")?;
        }
        if let Some(description) = self.description() {
            writeln!(f, " description: {description}")?;
        }
        if let Some(order) = self.order() {
            writeln!(f, " order: {order}")?;
        }

        if self.deactivated() {
            writeln!(f, " Deactivated: {}", self.deactivated())?;
        }
        if self.severity() != &Severity::Violation {
            writeln!(f, " Severity: {}", self.severity())?;
        }
        if self.closed() {
            writeln!(f, " closed: {}", self.closed())?;
        }
        let mut components = self.components().iter().peekable();
        if components.peek().is_some() {
            writeln!(f, "Components:")?;
            for component in components {
                writeln!(f, " - {component}")?;
            }
        }
        let mut targets = self.targets().iter().peekable();
        if targets.peek().is_some() {
            writeln!(f, "Targets:")?;
            for target in targets {
                writeln!(f, " - {target}")?;
            }
        }
        let mut property_shapes = self.property_shapes().iter().peekable();
        if property_shapes.peek().is_some() {
            writeln!(
                f,
                " Property Shapes: [{}]",
                property_shapes.map(|ps| ps.to_string()).collect::<Vec<_>>().join(", ")
            )?;
        }
        Ok(())
    }
}
