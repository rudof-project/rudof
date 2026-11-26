use crate::model::BoxId;
use crate::style::{ArrowStyle, Color, StyleSheet};
use serde::{Deserialize, Serialize};

/// Global default border/background/arrow colors for [`Shape::Class`] boxes, applied regardless
/// of stereotype (unlike [`crate::style::StyleSheet`], which styles boxes by matching stereotype).
///
/// Defaults to a neutral black-on-white look; a domain crate (e.g. `shapes_converter`'s
/// `ShEx2UmlConfig`) is expected to pick its own opinionated defaults on top of this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClassSkin {
    pub border_color: Color,
    pub background_color: Color,
    pub arrow_color: Color,
}

impl Default for ClassSkin {
    fn default() -> Self {
        ClassSkin {
            border_color: Color::Black,
            background_color: Color::White,
            arrow_color: Color::Black,
        }
    }
}

/// The outline shape a box is drawn with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Shape {
    /// A cloud, used by rudof for RDF triple terms that are not asserted.
    Cloud,
    /// A plain box, used for RDF nodes (IRIs, blank nodes, literals, asserted triples).
    #[default]
    Rectangle,
    /// A UML class box (title + attribute compartments), used for ShEx shapes.
    Class,
}

/// How a connector between two boxes should be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectorKind {
    /// A plain, labeled association between two boxes.
    #[default]
    Association,
    /// A generalization/inheritance arrow (e.g. ShEx `EXTENDS`).
    Generalization,
}

/// The layout direction of a diagram, when the backend supports one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    LeftToRight,
    #[default]
    TopToBottom,
}

/// The routing style used for connector lines, when the backend supports one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineType {
    Orthogonal,
    Polyline,
    #[default]
    Default,
}

/// How much of a diagram to render.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DiagramScope {
    /// Render every box and connector.
    #[default]
    All,
    /// Render only the given box (identified by its title) and its immediate neighbours.
    Neighs(String),
}

impl DiagramScope {
    pub fn all() -> DiagramScope {
        DiagramScope::All
    }

    pub fn neighs(title: &str) -> DiagramScope {
        DiagramScope::Neighs(title.to_string())
    }
}

/// A single node in a [`Diagram`].
#[derive(Debug, Clone, PartialEq)]
pub struct DiagramBox {
    id: BoxId,
    shape: Shape,
    title: String,
    href: Option<String>,
    /// Raw stereotype text inserted as PlantUML's `<<...>>`, e.g. `"uri"` (looked up in the
    /// diagram's [`StyleSheet`]) or a literal spot spec like `"(S,#FF7700)"`.
    stereotype: Option<String>,
    /// Pre-formatted attribute lines shown inside a [`Shape::Class`] box; ignored otherwise.
    compartments: Vec<String>,
}

impl DiagramBox {
    pub fn new(id: BoxId, shape: Shape, title: impl Into<String>) -> Self {
        DiagramBox {
            id,
            shape,
            title: title.into(),
            href: None,
            stereotype: None,
            compartments: Vec::new(),
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_stereotype(mut self, stereotype: impl Into<String>) -> Self {
        self.stereotype = Some(stereotype.into());
        self
    }

    pub fn with_compartments(mut self, compartments: Vec<String>) -> Self {
        self.compartments = compartments;
        self
    }

    pub fn id(&self) -> BoxId {
        self.id
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }

    pub fn stereotype(&self) -> Option<&str> {
        self.stereotype.as_deref()
    }

    pub fn compartments(&self) -> &[String] {
        &self.compartments
    }
}

/// An edge between two boxes in a [`Diagram`].
#[derive(Debug, Clone, PartialEq)]
pub struct Connector {
    source: BoxId,
    target: BoxId,
    kind: ConnectorKind,
    label: Option<String>,
    /// A short decoration shown near the target end, e.g. a cardinality like `"0..*"`.
    target_decoration: Option<String>,
    style: Option<ArrowStyle>,
    href: Option<String>,
}

impl Connector {
    pub fn new(source: BoxId, target: BoxId, kind: ConnectorKind) -> Self {
        Connector {
            source,
            target,
            kind,
            label: None,
            target_decoration: None,
            style: None,
            href: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_target_decoration(mut self, decoration: impl Into<String>) -> Self {
        self.target_decoration = Some(decoration.into());
        self
    }

    pub fn with_style(mut self, style: ArrowStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn source(&self) -> BoxId {
        self.source
    }

    pub fn target(&self) -> BoxId {
        self.target
    }

    pub fn kind(&self) -> ConnectorKind {
        self.kind
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn target_decoration(&self) -> Option<&str> {
        self.target_decoration.as_deref()
    }

    pub fn style(&self) -> Option<&ArrowStyle> {
        self.style.as_ref()
    }

    pub fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }
}

/// A technology-agnostic diagram: boxes, connectors between them, a style sheet, and a handful
/// of opt-in presentation hints backends may or may not honor.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Diagram {
    boxes: Vec<DiagramBox>,
    connectors: Vec<Connector>,
    style_sheet: StyleSheet,
    direction: Option<Direction>,
    line_type: LineType,
    hide_empty_members: bool,
    hide_circles: bool,
    shadowing: Option<bool>,
    class_skin: Option<ClassSkin>,
}

impl Diagram {
    pub fn new() -> Self {
        Diagram::default()
    }

    pub fn add_box(&mut self, b: DiagramBox) {
        self.boxes.push(b);
    }

    pub fn add_connector(&mut self, c: Connector) {
        self.connectors.push(c);
    }

    pub fn with_style_sheet(mut self, style_sheet: StyleSheet) -> Self {
        self.style_sheet = style_sheet;
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn with_line_type(mut self, line_type: LineType) -> Self {
        self.line_type = line_type;
        self
    }

    pub fn with_hide_empty_members(mut self, hide: bool) -> Self {
        self.hide_empty_members = hide;
        self
    }

    pub fn with_hide_circles(mut self, hide: bool) -> Self {
        self.hide_circles = hide;
        self
    }

    pub fn with_shadowing(mut self, shadowing: bool) -> Self {
        self.shadowing = Some(shadowing);
        self
    }

    pub fn with_class_skin(mut self, skin: ClassSkin) -> Self {
        self.class_skin = Some(skin);
        self
    }

    pub fn boxes(&self) -> impl Iterator<Item = &DiagramBox> {
        self.boxes.iter()
    }

    pub fn connectors(&self) -> impl Iterator<Item = &Connector> {
        self.connectors.iter()
    }

    pub fn style_sheet(&self) -> &StyleSheet {
        &self.style_sheet
    }

    pub fn direction(&self) -> Option<Direction> {
        self.direction
    }

    pub fn line_type(&self) -> LineType {
        self.line_type
    }

    pub fn hide_empty_members(&self) -> bool {
        self.hide_empty_members
    }

    pub fn hide_circles(&self) -> bool {
        self.hide_circles
    }

    pub fn shadowing(&self) -> Option<bool> {
        self.shadowing
    }

    pub fn class_skin(&self) -> Option<ClassSkin> {
        self.class_skin
    }

    /// Restricts this diagram to the given [`DiagramScope`]: `All` returns a clone of `self`,
    /// `Neighs(title)` keeps only the box with that exact title, its direct neighbours (as
    /// connector source/target), and the connectors between them.
    ///
    /// Matching by title is a convenience for simple callers; a domain layer that already knows
    /// the target box's [`BoxId`] (e.g. because it resolved a name through its own lookup table)
    /// should prefer [`Self::scoped_by_id`], which cannot be confused by two boxes sharing a
    /// display title.
    pub fn scoped(&self, scope: &DiagramScope) -> Diagram {
        let DiagramScope::Neighs(title) = scope else {
            return self.clone();
        };
        match self.boxes.iter().find(|b| b.title() == title).map(|b| b.id()) {
            Some(target_id) => self.scoped_by_id(target_id),
            None => Diagram {
                boxes: Vec::new(),
                connectors: Vec::new(),
                ..self.clone_settings()
            },
        }
    }

    /// Restricts this diagram to the box identified by `target_id`, its direct neighbours (as
    /// connector source/target), and the connectors between them.
    pub fn scoped_by_id(&self, target_id: BoxId) -> Diagram {
        let neigh_connectors: Vec<Connector> = self
            .connectors
            .iter()
            .filter(|c| c.source() == target_id || c.target() == target_id)
            .cloned()
            .collect();
        let mut keep: std::collections::HashSet<BoxId> = std::collections::HashSet::new();
        keep.insert(target_id);
        for c in &neigh_connectors {
            keep.insert(c.source());
            keep.insert(c.target());
        }
        let boxes = self.boxes.iter().filter(|b| keep.contains(&b.id())).cloned().collect();
        Diagram {
            boxes,
            connectors: neigh_connectors,
            ..self.clone_settings()
        }
    }

    fn clone_settings(&self) -> Diagram {
        Diagram {
            boxes: Vec::new(),
            connectors: Vec::new(),
            style_sheet: self.style_sheet.clone(),
            direction: self.direction,
            line_type: self.line_type,
            hide_empty_members: self.hide_empty_members,
            hide_circles: self.hide_circles,
            shadowing: self.shadowing,
            class_skin: self.class_skin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_diagram() -> Diagram {
        let mut d = Diagram::new();
        d.add_box(DiagramBox::new(BoxId::new(0), Shape::Rectangle, "A"));
        d.add_box(DiagramBox::new(BoxId::new(1), Shape::Rectangle, "B"));
        d.add_box(DiagramBox::new(BoxId::new(2), Shape::Rectangle, "C"));
        d.add_connector(Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Association).with_label("knows"));
        d
    }

    #[test]
    fn new_diagram_is_empty() {
        let d = Diagram::new();
        assert_eq!(d.boxes().count(), 0);
        assert_eq!(d.connectors().count(), 0);
        assert!(d.style_sheet().is_empty());
    }

    #[test]
    fn scope_all_keeps_everything() {
        let d = sample_diagram();
        let scoped = d.scoped(&DiagramScope::all());
        assert_eq!(scoped.boxes().count(), 3);
        assert_eq!(scoped.connectors().count(), 1);
    }

    #[test]
    fn scope_neighs_keeps_only_target_and_neighbours() {
        let d = sample_diagram();
        let scoped = d.scoped(&DiagramScope::neighs("A"));
        let titles: std::collections::HashSet<_> = scoped.boxes().map(|b| b.title()).collect();
        assert_eq!(titles, std::collections::HashSet::from(["A", "B"]));
        assert_eq!(scoped.connectors().count(), 1);
    }

    #[test]
    fn scope_neighs_of_unknown_title_is_empty() {
        let d = sample_diagram();
        let scoped = d.scoped(&DiagramScope::neighs("does-not-exist"));
        assert_eq!(scoped.boxes().count(), 0);
        assert_eq!(scoped.connectors().count(), 0);
    }

    #[test]
    fn scoped_by_id_matches_scoped_by_title() {
        let d = sample_diagram();
        let by_id = d.scoped_by_id(BoxId::new(0));
        let by_title = d.scoped(&DiagramScope::neighs("A"));
        assert_eq!(by_id, by_title);
    }

    #[test]
    fn scoped_by_id_is_precise_even_with_duplicate_titles() {
        let mut d = Diagram::new();
        d.add_box(DiagramBox::new(BoxId::new(0), Shape::Rectangle, "Same"));
        d.add_box(DiagramBox::new(BoxId::new(1), Shape::Rectangle, "Same"));
        d.add_box(DiagramBox::new(BoxId::new(2), Shape::Rectangle, "Neighbour"));
        d.add_connector(Connector::new(BoxId::new(1), BoxId::new(2), ConnectorKind::Association));

        let scoped = d.scoped_by_id(BoxId::new(1));
        let ids: std::collections::HashSet<_> = scoped.boxes().map(|b| b.id()).collect();
        assert_eq!(ids, std::collections::HashSet::from([BoxId::new(1), BoxId::new(2)]));
    }

    #[test]
    fn box_builder_sets_optional_fields() {
        let b = DiagramBox::new(BoxId::new(0), Shape::Class, "Person")
            .with_href("http://example.org/Person")
            .with_stereotype("(S,#FF7700)")
            .with_compartments(vec![":name xsd:string".to_string()]);
        assert_eq!(b.title(), "Person");
        assert_eq!(b.href(), Some("http://example.org/Person"));
        assert_eq!(b.stereotype(), Some("(S,#FF7700)"));
        assert_eq!(b.compartments(), &[":name xsd:string".to_string()]);
    }

    #[test]
    fn preamble_hints_default_to_off() {
        let d = Diagram::new();
        assert_eq!(d.direction(), None);
        assert_eq!(d.line_type(), LineType::Default);
        assert!(!d.hide_empty_members());
        assert!(!d.hide_circles());
        assert_eq!(d.shadowing(), None);
        assert_eq!(d.class_skin(), None);
    }

    #[test]
    fn preamble_hints_can_be_set() {
        let d = Diagram::new()
            .with_direction(Direction::TopToBottom)
            .with_line_type(LineType::Polyline)
            .with_hide_empty_members(true)
            .with_hide_circles(true)
            .with_shadowing(true)
            .with_class_skin(ClassSkin {
                border_color: Color::Black,
                background_color: Color::LightBlue,
                arrow_color: Color::Black,
            });
        assert_eq!(d.direction(), Some(Direction::TopToBottom));
        assert_eq!(d.line_type(), LineType::Polyline);
        assert!(d.hide_empty_members());
        assert!(d.hide_circles());
        assert_eq!(d.shadowing(), Some(true));
        assert_eq!(
            d.class_skin(),
            Some(ClassSkin {
                border_color: Color::Black,
                background_color: Color::LightBlue,
                arrow_color: Color::Black
            })
        );
    }

    #[test]
    fn connector_builder_sets_optional_fields() {
        let c = Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Generalization)
            .with_label("worksFor")
            .with_target_decoration("0..*")
            .with_href("http://example.org/worksFor");
        assert_eq!(c.kind(), ConnectorKind::Generalization);
        assert_eq!(c.label(), Some("worksFor"));
        assert_eq!(c.target_decoration(), Some("0..*"));
        assert_eq!(c.href(), Some("http://example.org/worksFor"));
    }
}
