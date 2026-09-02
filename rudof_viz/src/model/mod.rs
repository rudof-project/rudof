mod box_id;
#[allow(clippy::module_inception)]
mod diagram;

pub use box_id::BoxId;
pub use diagram::{ClassSkin, Connector, ConnectorKind, Diagram, DiagramBox, DiagramScope, Direction, LineType, Shape};
