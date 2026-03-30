use crate::ast::{ASTNodeShape, ASTSchema};
use crate::ir::component::IRComponent;
use crate::ir::dependency_graph::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::schema::IRSchema;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::types::{ClosedInfo, MessageMap, Severity, Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct IRNodeShape {
    id: Object,
    components: Vec<IRComponent>,
    targets: Vec<Target>,
    property_shapes: Vec<ShapeLabelIdx>,
    closed_info: ClosedInfo,
    deactivated: bool,

    // message: MessageMap,
    severity: Option<Severity>,
    name: MessageMap,
    description: MessageMap,
    group: Option<Object>,
    // source_iri: S::Iri
}

impl IRNodeShape {
    pub fn new(id: Object) -> Self {
        IRNodeShape {
            id,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            closed_info: ClosedInfo::No,
            deactivated: false,
            severity: None,
            name: MessageMap::new(),
            description: MessageMap::new(),
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

    pub fn with_closed_info(mut self, closed_info: ClosedInfo) -> Self {
        self.closed_info = closed_info;
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

    pub fn with_name(mut self, name: MessageMap) -> Self {
        self.name = name;
        self
    }

    pub fn with_description(mut self, description: MessageMap) -> Self {
        self.description = description;
        self
    }

    pub fn with_group(mut self, group: Option<Object>) -> Self {
        self.group = group;
        self
    }

    pub fn id(&self) -> &Object {
        &self.id
    }

    pub fn deactivated(&self) -> bool {
        self.deactivated
    }

    pub fn severity(&self) -> Severity {
        match &self.severity {
            Some(severity) => severity.clone(),
            None => Severity::Violation,
        }
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        self.closed_info.allowed_properties().unwrap_or_default()
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

    pub fn closed(&self) -> bool {
        self.closed_info.is_closed()
    }

    // pub(crate) fn add_edges(
    //     &self,
    //     shape_idx: ShapeLabelIdx,
    //     dg: &mut DependencyGraph,
    //     posneg: PosNeg,
    //     schema: &IRSchema,
    //     visited: &mut HashSet<ShapeLabelIdx>
    // ) {
    //
    // }
}

impl IRNodeShape {
    /// Compiles an AST NodeShape to an internal representation NodeShape
    /// It embeds some components like deactivated as boolean attributes of the internal representation of the node shape
    pub fn compile(shape: &ASTNodeShape, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let mut compiled_components = Vec::new();
        for component in shape.components() {
            if let Some(compiled) = IRComponent::compile(component, ast, ir)? {
                compiled_components.push(compiled);
            }
        }

        let mut compiled_prop_shapes = Vec::new();
        for prop_shape in shape.property_shapes() {
            let idx = ir.register_shape(prop_shape, None, ast)?;
            compiled_prop_shapes.push(idx);
        }

        let closed_info = shape.get_closed_info(ast)?;

        let compiled_node_shape = IRNodeShape::new(shape.id().clone())
            .with_components(compiled_components)
            .with_targets(shape.targets().to_owned())
            .with_property_shapes(compiled_prop_shapes)
            .with_closed_info(closed_info)
            .with_deactivated(shape.is_deactivated())
            .with_severity(shape.severity().cloned())
            .with_name(shape.name().to_owned())
            .with_description(shape.description().to_owned())
            .with_group(shape.group().cloned());

        Ok(compiled_node_shape)
    }
}

impl IRNodeShape {
    // TODO - Maybe change error type
    pub fn register<RDF: BuildRDF>(
        &self,
        graph: &mut RDF,
        shapes_map: &HashMap<ShapeLabelIdx, IRShape>,
    ) -> Result<(), RDF::Err> {
        let id: RDF::Subject = self.id.clone().try_into().map_err(|_| unreachable!())?;
        graph.add_type(id.clone(), ShaclVocab::sh_node_shape().clone())?;

        self.name.iter().try_for_each(|(lang, value)| {
            let literal: RDF::Literal = match lang {
                None => value.clone().into(),
                Some(_) => todo!(),
            };

            graph.add_triple(id.clone(), ShaclVocab::sh_name().clone(), literal)
        })?;

        self.description.iter().try_for_each(|(lang, value)| {
            let literal: RDF::Literal = match lang {
                None => value.clone().into(),
                Some(_) => todo!(),
            };

            graph.add_triple(id.clone(), ShaclVocab::sh_description().clone(), literal)
        })?;

        self.components
            .iter()
            .try_for_each(|c| c.register(&self.id, graph, shapes_map))?;

        self.targets.iter().try_for_each(|t| t.register(&self.id, graph))?;

        self.property_shapes.iter().try_for_each(|idx| {
            // TODO - Throw error instead of unwrap
            let ps = shapes_map.get(idx).unwrap();

            graph.add_triple(id.clone(), ShaclVocab::sh_property().clone(), ps.id().clone())
        })?;

        if let Some(group) = &self.group {
            graph.add_triple(id.clone(), ShaclVocab::sh_group().clone(), group.clone())?;
        }

        if let Some(severity) = &self.severity {
            graph.add_triple::<_, _, IriS>(id.clone(), ShaclVocab::sh_severity().clone(), severity.clone().into())?;
        }

        Ok(())
    }
}

impl IRNodeShape {
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
