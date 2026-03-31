use crate::ast::{ASTPropertyShape, ASTSchema};
use crate::ir::ReifierInfo;
use crate::ir::component::IRComponent;
use crate::ir::dependency_graph::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::schema::IRSchema;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::types::{ClosedInfo, MessageMap, Severity, Target};
use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::NumericLiteral;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{BuildRDF, SHACLPath};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct IRPropertyShape {
    id: Object,
    path: SHACLPath,
    components: Vec<IRComponent>,
    targets: Vec<Target>,
    property_shapes: Vec<ShapeLabelIdx>,
    closed_info: ClosedInfo,
    deactivated: bool,
    // message: MessageMap
    severity: Option<Severity>,

    // SHACL 1.2: Reifier info is only present for property shapes
    reifier_info: Option<ReifierInfo>,
    name: MessageMap,
    description: MessageMap,
    order: Option<NumericLiteral>,
    group: Option<Object>,
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
            severity: None,
            reifier_info: None,
            name: MessageMap::new(),
            description: MessageMap::new(),
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

    pub fn severity(&self) -> Severity {
        match &self.severity {
            None => Severity::Violation,
            Some(severity) => severity.clone(),
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

        let compiled_prop_shape = IRPropertyShape::new(shape.id().clone(), shape.path().to_owned(), closed_info)
            .with_components(compiled_components)
            .with_targets(shape.targets().to_owned())
            .with_property_shapes(compiled_prop_shapes)
            .with_deactivated(shape.is_deactivated())
            .with_severity(shape.severity().cloned())
            .with_reifier_info(reifier_info)
            .with_name(shape.name().to_owned())
            .with_description(shape.description().to_owned())
            .with_order(shape.order().cloned())
            .with_group(shape.group().cloned());

        Ok(compiled_prop_shape)
    }
}

impl IRPropertyShape {
    pub fn register<RDF: BuildRDF>(
        &self,
        graph: &mut RDF,
        shapes_map: &HashMap<ShapeLabelIdx, IRShape>,
    ) -> Result<(), RDF::Err> {
        let id: RDF::Subject = self.id.clone().try_into().map_err(|_| unreachable!())?;
        graph.add_type(id.clone(), ShaclVocab::sh_property_shape())?;

        self.name.iter().try_for_each(|(lang, value)| {
            let lit: RDF::Literal = match lang {
                None => value.clone().into(),
                Some(_) => todo!(),
            };

            graph.add_triple(id.clone(), ShaclVocab::sh_name(), lit)
        })?;

        self.description.iter().try_for_each(|(lang, value)| {
            let lit: RDF::Literal = match lang {
                None => value.clone().into(),
                Some(_) => todo!(),
            };

            graph.add_triple(id.clone(), ShaclVocab::sh_description(), lit)
        })?;

        if let Some(order) = &self.order {
            let lit: RDF::Literal = match order {
                NumericLiteral::Integer(i) => (*i).into(),
                NumericLiteral::Byte(_) => todo!(),
                NumericLiteral::Short(_) => todo!(),
                NumericLiteral::NonNegativeInteger(_) => todo!(),
                NumericLiteral::UnsignedLong(_) => todo!(),
                NumericLiteral::UnsignedInt(_) => todo!(),
                NumericLiteral::UnsignedShort(_) => todo!(),
                NumericLiteral::UnsignedByte(_) => todo!(),
                NumericLiteral::PositiveInteger(_) => todo!(),
                NumericLiteral::NegativeInteger(_) => todo!(),
                NumericLiteral::NonPositiveInteger(_) => todo!(),
                NumericLiteral::Long(_) => todo!(),
                NumericLiteral::Decimal(_) => todo!(),
                NumericLiteral::Double(f) => (*f).into(),
                NumericLiteral::Float(f) => f.to_string().into(),
            };

            graph.add_triple(id.clone(), ShaclVocab::sh_order(), lit)?;
        }

        if let Some(group) = &self.group {
            graph.add_triple(id.clone(), ShaclVocab::sh_group(), group.clone())?;
        }

        if let SHACLPath::Predicate { pred } = &self.path {
            graph.add_triple(id.clone(), ShaclVocab::sh_path(), pred.clone())?;
        } else {
            unimplemented!()
        }

        self.components
            .iter()
            .try_for_each(|component| component.register(&self.id, graph, shapes_map))?;

        self.targets
            .iter()
            .try_for_each(|target| target.register(&self.id, graph))?;

        if self.deactivated {
            let lit: RDF::Literal = "true".to_string().into();

            graph.add_triple(id.clone(), ShaclVocab::sh_deactivated(), lit)?;
        }

        if let Some(severity) = &self.severity {
            graph.add_triple::<_, _, IriS>(id.clone(), ShaclVocab::sh_severity(), severity.clone().into())?;
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
