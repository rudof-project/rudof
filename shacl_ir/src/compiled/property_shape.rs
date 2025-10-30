use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component_ir::ComponentIR;
use super::severity::CompiledSeverity;
use super::target::CompiledTarget;
use crate::closed_info::ClosedInfo;
use crate::compiled::Deps;
use crate::reifier_info;
use crate::reifier_info::ReifierInfo;
use crate::schema_ir::SchemaIR;
use crate::shape_label_idx::ShapeLabelIdx;
use iri_s::IriS;
use shacl_ast::Schema;
use shacl_ast::property_shape::PropertyShape;
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
    property_shapes: Vec<ShapeLabelIdx>,
    closed_info: ClosedInfo,
    deactivated: bool,
    // message: MessageMap,
    severity: Option<CompiledSeverity>,

    // SHACL 1.2: Reifier info is only present for property shapes
    reifier_info: Option<ReifierInfo>,
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
        property_shapes: Vec<ShapeLabelIdx>,
        closed_info: ClosedInfo,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
        reifier_info: Option<ReifierInfo>,
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
            reifier_info,
        }
    }

    pub fn id(&self) -> &RDFNode {
        &self.id
    }

    pub fn closed(&self) -> bool {
        self.closed_info.is_closed()
    }

    pub fn reifier_info(&self) -> Option<ReifierInfo> {
        self.reifier_info.clone()
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

    pub fn property_shapes(&self) -> &Vec<ShapeLabelIdx> {
        &self.property_shapes
    }
}

impl PropertyShapeIR {
    pub fn compile<S: Rdf>(
        shape: PropertyShape<S>,
        schema: &Schema<S>,
        schema_ir: &mut SchemaIR,
    ) -> Result<(Self, Deps), Box<CompiledShaclError>> {
        let id = shape.id().clone();
        let path = shape.path().to_owned();
        let deactivated = shape.is_deactivated().to_owned();
        let severity = CompiledSeverity::compile(shape.severity())?;

        let components = shape.components().iter().collect::<Vec<_>>();
        let mut compiled_components = Vec::new();
        let mut deps = Vec::new();
        for component in components {
            if let Some((component, new_deps)) =
                ComponentIR::compile(component.to_owned(), schema, schema_ir)?
            {
                compiled_components.push(component);
                deps.extend(new_deps);
            }
        }

        let mut targets = Vec::new();
        for target in shape.targets() {
            let ans = CompiledTarget::compile(target.to_owned())?;
            targets.push(ans);
        }

        let mut property_shapes = Vec::new();
        for property_shape in shape.property_shapes() {
            let (shape, new_deps) = compile_shape(property_shape, schema, schema_ir)?;
            property_shapes.push(shape);
            deps.extend(new_deps);
        }

        let closed_info = ClosedInfo::get_closed_info_property_shape(&shape, schema)
            .map_err(|e| Box::new(CompiledShaclError::ShaclError { source: e }))?;

        let reifier_info = if let Some((reifier_info, new_deps)) =
            reifier_info::ReifierInfo::get_reifier_info_property_shape(&shape, schema, schema_ir)?
        {
            deps.extend(new_deps);
            Some(reifier_info)
        } else {
            None
        };

        let compiled_property_shape = PropertyShapeIR::new(
            id,
            path,
            compiled_components,
            targets,
            property_shapes,
            closed_info,
            deactivated,
            severity,
            reifier_info,
        );

        Ok((compiled_property_shape, deps))
    }
}
