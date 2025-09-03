use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component_ir::ComponentIR;
use super::severity::CompiledSeverity;
use super::shape::ShapeIR;
use super::target::CompiledTarget;
use crate::closed_info::ClosedInfo;
use iri_s::IriS;
use shacl_ast::property_shape::PropertyShape;
use shacl_ast::Schema;
use srdf::RDFNode;
use srdf::Rdf;
use srdf::SHACLPath;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PropertyShapeIR {
    id: RDFNode,
    path: SHACLPath,
    components: Vec<ComponentIR>,
    targets: Vec<CompiledTarget>,
    property_shapes: Vec<ShapeIR>,
    closed_info: ClosedInfo,
    // ignored_properties: Vec<S::IRI>,
    deactivated: bool,
    // message: MessageMap,
    severity: Option<CompiledSeverity>,
    // name: MessageMap,
    // description: MessageMap,
    // order: Option<NumericLiteral>,
    // group: Option<S::Term>,
    // source_iri: Option<S::IRI>,
    // annotations: Vec<(S::IRI, S::Term)>,
}

impl PropertyShapeIR {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: RDFNode,
        path: SHACLPath,
        components: Vec<ComponentIR>,
        targets: Vec<CompiledTarget>,
        property_shapes: Vec<ShapeIR>,
        closed_info: ClosedInfo,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
    ) -> Self {
        PropertyShapeIR {
            id,
            path,
            components,
            targets,
            property_shapes,
            closed_info,
            deactivated,
            severity,
        }
    }

    pub fn id(&self) -> &RDFNode {
        &self.id
    }

    pub fn closed(&self) -> bool {
        self.closed_info.is_closed()
    }

    pub fn allowed_properties(&self) -> HashSet<IriS> {
        self.closed_info
            .allowed_properties()
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    pub fn path(&self) -> &SHACLPath {
        &self.path
    }

    pub fn deactivated(&self) -> bool {
        self.deactivated
    }

    pub fn severity(&self) -> CompiledSeverity {
        match &self.severity {
            Some(severity) => severity.clone(),
            None => CompiledSeverity::Violation,
        }
    }

    pub fn components(&self) -> &Vec<ComponentIR> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<ShapeIR> {
        &self.property_shapes
    }
}

impl PropertyShapeIR {
    pub fn compile<S: Rdf>(
        shape: PropertyShape<S>,
        schema: &Schema<S>,
    ) -> Result<Self, CompiledShaclError> {
        let id = shape.id().clone();
        let path = shape.path().to_owned();
        let deactivated = shape.is_deactivated().to_owned();
        let severity = CompiledSeverity::compile(shape.severity())?;

        let components = shape.components().iter().collect::<Vec<_>>();
        let mut compiled_components = Vec::new();
        for component in components {
            if let Some(component) = ComponentIR::compile(component.to_owned(), schema)? {
                compiled_components.push(component);
            }
        }

        let mut targets = Vec::new();
        for target in shape.targets() {
            let ans = CompiledTarget::compile(target.to_owned())?;
            targets.push(ans);
        }

        let mut property_shapes = Vec::new();
        for property_shape in shape.property_shapes() {
            let shape = compile_shape(property_shape.to_owned(), schema)?;
            property_shapes.push(shape);
        }

        let closed_info = ClosedInfo::get_closed_info_property_shape(&shape, schema)?;

        let compiled_property_shape = PropertyShapeIR::new(
            id,
            path,
            compiled_components,
            targets,
            property_shapes,
            closed_info,
            deactivated,
            severity,
        );

        Ok(compiled_property_shape)
    }
}
