use crate::backends::graphviz::GraphVizBackend;
use crate::backends::plantuml::PlantUmlBackend;
use crate::model::Diagram;
use crate::render::{DiagramRenderer, RenderError};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

/// Selects which `rudof_viz` backend renders a [`Diagram`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VizEngine {
    /// PlantUML, via `java -jar plantuml.jar` (the default, preserving prior behavior).
    #[default]
    PlantUml,
    /// GraphViz, via the `dot` command.
    GraphViz,
}

impl Display for VizEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VizEngine::PlantUml => write!(f, "plantuml"),
            VizEngine::GraphViz => write!(f, "graphviz"),
        }
    }
}

/// Error returned when parsing an unknown [`VizEngine`] name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Unsupported visualization engine: '{engine}'. Valid engines are: 'plantuml', 'graphviz'")]
pub struct UnsupportedVizEngine {
    pub engine: String,
}

impl FromStr for VizEngine {
    type Err = UnsupportedVizEngine;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plantuml" => Ok(VizEngine::PlantUml),
            "graphviz" => Ok(VizEngine::GraphViz),
            other => Err(UnsupportedVizEngine {
                engine: other.to_string(),
            }),
        }
    }
}

/// Renders `diagram` to text using `engine`.
pub fn render_with_engine<W: Write>(diagram: &Diagram, engine: VizEngine, writer: &mut W) -> Result<(), RenderError> {
    match engine {
        VizEngine::PlantUml => PlantUmlBackend::default().render(diagram, writer),
        VizEngine::GraphViz => GraphVizBackend::default().render(diagram, writer),
    }
}

/// Renders `diagram` to an image using `engine`. `plantuml_path` is only consulted when
/// `engine` is [`VizEngine::PlantUml`] — GraphViz resolves its `dot` executable via `PATH`.
#[cfg(not(target_family = "wasm"))]
pub fn render_image_with_engine<W: Write>(
    diagram: &Diagram,
    format: crate::render::ImageFormat,
    engine: VizEngine,
    plantuml_path: &std::path::Path,
    writer: &mut W,
) -> Result<(), RenderError> {
    use crate::render::ExternalToolRenderer;

    match engine {
        VizEngine::PlantUml => PlantUmlBackend::new(plantuml_path).render_image(diagram, format, writer),
        VizEngine::GraphViz => GraphVizBackend::default().render_image(diagram, format, writer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_plantuml() {
        assert_eq!(VizEngine::default(), VizEngine::PlantUml);
    }

    #[test]
    fn round_trips_through_display_and_from_str() {
        assert_eq!("plantuml".parse::<VizEngine>().unwrap(), VizEngine::PlantUml);
        assert_eq!("PlantUML".parse::<VizEngine>().unwrap(), VizEngine::PlantUml);
        assert_eq!("graphviz".parse::<VizEngine>().unwrap(), VizEngine::GraphViz);
        assert_eq!(VizEngine::PlantUml.to_string(), "plantuml");
        assert_eq!(VizEngine::GraphViz.to_string(), "graphviz");
    }

    #[test]
    fn rejects_unknown_engine() {
        let err = "mermaid".parse::<VizEngine>().unwrap_err();
        assert_eq!(err.engine, "mermaid");
    }
}
