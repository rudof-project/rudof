//! Technology-agnostic visualization primitives for rudof.
//!
//! This crate defines a small diagram model (boxes, connectors, styles) and renderer traits
//! that a backend implements for a specific technology. Today the only backend is
//! [`backends::plantuml::PlantUmlBackend`]; future backends (Mermaid, GraphViz, Cytoscape, ...)
//! implement the same traits without any change to the code that builds the [`model::Diagram`].

pub mod backends;
pub mod model;
pub mod render;
pub mod style;

pub use model::{
    BoxId, ClassSkin, Connector, ConnectorKind, Diagram, DiagramBox, DiagramScope, Direction, LineType, Shape,
};
#[cfg(not(target_family = "wasm"))]
pub use render::ExternalToolRenderer;
pub use render::{DiagramRenderer, ImageFormat, RenderError};
pub use style::{ArrowStyle, BoxStyle, Color, LineStyle, StyleRule, StyleSheet};
