use std::collections::HashSet;
use std::fmt::Display;

use iri_s::IriS;
use srdf::RDFNode;
use srdf::Rdf;
use srdf::SHACLPath;

use shacl_ast::property_shape::PropertyShape;
use shacl_ast::Schema;

use crate::closed_info::ClosedInfo;

use super::compile_shape;
use super::compiled_shacl_error::CompiledShaclError;
use super::component::CompiledComponent;
use super::severity::CompiledSeverity;
use super::shape::CompiledShape;
use super::target::CompiledTarget;

#[derive(Debug, Clone)]
pub struct CompiledPropertyShape {
    id: RDFNode,
    path: SHACLPath,
    components: Vec<CompiledComponent>,
    targets: Vec<CompiledTarget>,
    property_shapes: Vec<CompiledShape>,
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

impl CompiledPropertyShape {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: RDFNode,
        path: SHACLPath,
        components: Vec<CompiledComponent>,
        targets: Vec<CompiledTarget>,
        property_shapes: Vec<CompiledShape>,
        closed_info: ClosedInfo,
        deactivated: bool,
        severity: Option<CompiledSeverity>,
    ) -> Self {
        CompiledPropertyShape {
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

    pub fn is_deactivated(&self) -> &bool {
        &self.deactivated
    }

    pub fn severity(&self) -> &CompiledSeverity {
        match &self.severity {
            Some(severity) => severity,
            None => &CompiledSeverity::Violation,
        }
    }

    pub fn components(&self) -> &Vec<CompiledComponent> {
        &self.components
    }

    pub fn targets(&self) -> &Vec<CompiledTarget> {
        &self.targets
    }

    pub fn property_shapes(&self) -> &Vec<CompiledShape> {
        &self.property_shapes
    }
}

impl CompiledPropertyShape {
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
            if let Some(component) = CompiledComponent::compile(component.to_owned(), schema)? {
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

        let compiled_property_shape = CompiledPropertyShape::new(
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

impl Display for CompiledPropertyShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Property shape {}", self.id)?;
        writeln!(f, " Deactivated: {}", self.deactivated)?;
        writeln!(f, " Severity: {}", self.severity())?;
        writeln!(f, " Closed: {}", self.closed())?;
        writeln!(f, " Components:")?;
        for component in &self.components {
            writeln!(f, "  - {}", component)?;
        }
        writeln!(f, " Targets:")?;
        for target in &self.targets {
            writeln!(f, "  - {}", target)?;
        }
        writeln!(f, " Property Shapes:")?;
        for property_shape in &self.property_shapes {
            writeln!(f, "  - {}", property_shape)?;
        }
        Ok(())
    }
}
