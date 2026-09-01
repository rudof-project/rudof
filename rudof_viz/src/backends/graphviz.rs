//! Renders a [`Diagram`] to [Graphviz](https://graphviz.org) DOT syntax, and (off `wasm`) to an
//! image by shelling out to the `dot` command.

use crate::model::{ClassSkin, Connector, ConnectorKind, Diagram, DiagramBox, Direction, LineType, Shape};
use crate::render::{DiagramRenderer, RenderError};
use crate::style::{LineStyle, StyleRule, StyleSheet};
use std::io::Write;

#[cfg(not(target_family = "wasm"))]
use {
    crate::render::{ExternalToolRenderer, ImageFormat},
    std::path::Path,
    std::process::Command,
};

/// Renders a [`Diagram`] to Graphviz DOT text (and, off `wasm`, to an image via `dot`).
#[derive(Debug, Clone)]
pub struct GraphVizBackend {
    #[cfg_attr(target_family = "wasm", allow(dead_code))]
    dot_path: std::path::PathBuf,
}

impl GraphVizBackend {
    /// Creates a backend that will invoke `dot_path` (e.g. `"dot"`, resolved via `PATH`, or an
    /// absolute path) when asked to render an image. Ignored for text-only rendering.
    pub fn new(dot_path: impl Into<std::path::PathBuf>) -> Self {
        GraphVizBackend {
            dot_path: dot_path.into(),
        }
    }
}

impl Default for GraphVizBackend {
    fn default() -> Self {
        GraphVizBackend::new("dot")
    }
}

impl DiagramRenderer for GraphVizBackend {
    fn render<W: Write>(&self, diagram: &Diagram, writer: &mut W) -> Result<(), RenderError> {
        writeln!(writer, "digraph diagram {{")?;
        match diagram.direction() {
            Some(Direction::LeftToRight) => writeln!(writer, "  rankdir=LR;")?,
            Some(Direction::TopToBottom) => writeln!(writer, "  rankdir=TB;")?,
            None => {},
        }
        match diagram.line_type() {
            LineType::Orthogonal => writeln!(writer, "  splines=ortho;")?,
            LineType::Polyline => writeln!(writer, "  splines=polyline;")?,
            LineType::Default => {},
        }
        writeln!(writer, "  node [fontname=\"Helvetica\", fontsize=10];")?;
        writeln!(writer, "  edge [fontname=\"Helvetica\", fontsize=9];")?;
        if let Some(skin) = diagram.class_skin() {
            writeln!(writer, "  edge [color=\"{}\"];", skin.arrow_color.name())?;
        }

        for b in diagram.boxes() {
            write_box(writer, b, diagram.style_sheet(), diagram.class_skin())?;
        }
        for c in diagram.connectors() {
            write_connector(writer, c)?;
        }

        writeln!(writer, "}}")?;
        Ok(())
    }
}

fn style_rule_for<'a>(b: &DiagramBox, sheet: &'a StyleSheet) -> Option<&'a StyleRule> {
    let stereotype = b.stereotype()?;
    sheet.rules().find(|r| r.name() == stereotype)
}

fn write_box<W: Write>(
    writer: &mut W,
    b: &DiagramBox,
    style_sheet: &StyleSheet,
    class_skin: Option<ClassSkin>,
) -> Result<(), RenderError> {
    let id = escape_dot(&b.id().to_string());
    match b.shape() {
        Shape::Class => {
            let border_color = class_skin.map(|s| s.border_color.name()).unwrap_or("black");
            let background_color = class_skin.map(|s| s.background_color.name()).unwrap_or("white");
            let href_attr = b
                .href()
                .map(|h| format!(" HREF=\"{}\"", escape_html(h)))
                .unwrap_or_default();
            let (title, _) = strip_links(b.title());
            let mut label = format!(
                "<TABLE BORDER=\"1\" CELLBORDER=\"1\" CELLSPACING=\"0\" COLOR=\"{border_color}\" BGCOLOR=\"{background_color}\"{href_attr}>"
            );
            label.push_str(&format!("<TR><TD><B>{}</B></TD></TR>", escape_html(&title)));
            for line in b.compartments() {
                let (text, _) = strip_links(line);
                label.push_str(&format!("<TR><TD ALIGN=\"LEFT\">{}</TD></TR>", escape_html(&text)));
            }
            label.push_str("</TABLE>");
            writeln!(writer, "  \"{id}\" [shape=plaintext, label=<{label}>];")?;
        },
        Shape::Rectangle | Shape::Cloud => {
            let (text, text_href) = strip_links(b.title());
            // `shape=cloud` isn't a portable choice: it's absent from the classic Graphviz shape
            // catalog (as of at least 2.43, `dot` silently falls back to a plain box and prints a
            // warning), so `Shape::Cloud` uses `ellipse` instead — the closest native, warning-free
            // shape to PlantUML's literal cloud bubble, and still visually distinct from the flat
            // `box` used for `Shape::Rectangle`.
            let shape_keyword = if matches!(b.shape(), Shape::Cloud) {
                "ellipse"
            } else {
                "box"
            };
            let mut attrs = vec![
                format!("shape={shape_keyword}"),
                "style=\"filled\"".to_string(),
                format!("label=\"{}\"", escape_dot(&text)),
            ];
            if let Some(href) = b.href().or(text_href.as_deref()) {
                attrs.push(format!("URL=\"{}\"", escape_dot(href)));
            }
            if let Some(rule) = style_rule_for(b, style_sheet) {
                if let Some(bg) = rule.background_color() {
                    attrs.push(format!("fillcolor=\"{}\"", bg.name()));
                }
                if let Some(lc) = rule.line_color() {
                    attrs.push(format!("color=\"{}\"", lc.name()));
                }
                if let Some(t) = rule.line_thickness() {
                    attrs.push(format!("penwidth={}", (t as f64).max(1.0)));
                }
            }
            writeln!(writer, "  \"{id}\" [{}];", attrs.join(", "))?;
        },
    }
    Ok(())
}

fn write_connector<W: Write>(writer: &mut W, c: &Connector) -> Result<(), RenderError> {
    let mut attrs = Vec::new();
    match c.kind() {
        ConnectorKind::Generalization => {
            attrs.push("arrowhead=empty".to_string());
        },
        ConnectorKind::Association => {
            let (text, text_href) = strip_links(c.label().unwrap_or(""));
            if !text.is_empty() {
                attrs.push(format!("label=\"{}\"", escape_dot(&text)));
            }
            if let Some(decoration) = c.target_decoration() {
                let (dtext, _) = strip_links(decoration);
                let dtext = dtext.trim();
                if !dtext.is_empty() {
                    attrs.push(format!("headlabel=\"{}\"", escape_dot(dtext)));
                }
            }
            if let Some(style) = c.style() {
                attrs.push(format!("color=\"{}\"", style.line_color().name()));
                attrs.push(format!("fontcolor=\"{}\"", style.text_color().name()));
                if let Some(dot_style) = line_style_to_dot(style.line_thickness()) {
                    attrs.push(format!("style=\"{dot_style}\""));
                }
            }
            if let Some(href) = c.href().or(text_href.as_deref()) {
                attrs.push(format!("URL=\"{}\"", escape_dot(href)));
            }
        },
    }
    let attr_str = if attrs.is_empty() {
        String::new()
    } else {
        format!(" [{}]", attrs.join(", "))
    };
    let source = escape_dot(&c.source().to_string());
    let target = escape_dot(&c.target().to_string());
    writeln!(writer, "  \"{source}\" -> \"{target}\"{attr_str};")?;
    Ok(())
}

fn line_style_to_dot(style: LineStyle) -> Option<&'static str> {
    match style {
        LineStyle::Bold => Some("bold"),
        LineStyle::Normal => None,
        LineStyle::Dashed => Some("dashed"),
        LineStyle::Dotted => Some("dotted"),
    }
}

/// Parses out any PlantUML-style `[[url text]]` hyperlinks embedded in `s` (a convention some
/// `rudof_viz` callers still use for plain-`String` labels — see the module-level design note in
/// this crate's migration history), returning the plain display text with every such marker
/// replaced by its `text` portion, plus the first `url` found (Graphviz's HTML-like labels only
/// support links at the whole-cell granularity, so only one link per string can be honored).
fn strip_links(s: &str) -> (String, Option<String>) {
    let mut result = String::new();
    let mut first_href = None;
    let mut rest = s;
    while let Some(start) = rest.find("[[") {
        result.push_str(&rest[..start]);
        let after_open = &rest[start + 2..];
        match after_open.find("]]") {
            Some(end) => {
                let inner = &after_open[..end];
                let (url, label) = inner.split_once(' ').unwrap_or(("", inner));
                if first_href.is_none() && !url.is_empty() {
                    first_href = Some(url.to_string());
                }
                result.push_str(label);
                rest = &after_open[end + 2..];
            },
            None => {
                result.push_str("[[");
                rest = after_open;
            },
        }
    }
    result.push_str(rest);
    (result, first_href)
}

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(not(target_family = "wasm"))]
impl ExternalToolRenderer for GraphVizBackend {
    fn build_command(&self, source_file: &Path, out_dir: &Path, format: ImageFormat) -> Result<Command, RenderError> {
        let out_param = match format {
            ImageFormat::Png => "-Tpng",
            ImageFormat::Svg => "-Tsvg",
        };
        let mut command = Command::new(&self.dot_path);
        command
            .arg(out_param)
            .arg(source_file)
            .arg("-o")
            .arg(out_dir.join(self.output_file_name(format)));
        Ok(command)
    }

    fn output_file_name(&self, format: ImageFormat) -> &'static str {
        match format {
            ImageFormat::Png => "diagram.png",
            ImageFormat::Svg => "diagram.svg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BoxId, ClassSkin, Diagram, DiagramBox, Shape};
    use crate::style::{ArrowStyle, Color, StyleRule, StyleSheet};

    fn render_to_string(diagram: &Diagram) -> String {
        let backend = GraphVizBackend::default();
        let mut out = Vec::new();
        backend.render(diagram, &mut out).expect("render should succeed");
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn strips_a_single_link_and_captures_its_href() {
        let (text, href) = strip_links("[[http://example.org/x label]]");
        assert_eq!(text, "label");
        assert_eq!(href, Some("http://example.org/x".to_string()));
    }

    #[test]
    fn strips_multiple_links_keeping_only_the_first_href() {
        let (text, href) = strip_links("[ [[http://example.org/a A]] [[http://example.org/b B]] ]");
        assert_eq!(text, "[ A B ]");
        assert_eq!(href, Some("http://example.org/a".to_string()));
    }

    #[test]
    fn leaves_plain_text_untouched() {
        let (text, href) = strip_links("plain text");
        assert_eq!(text, "plain text");
        assert_eq!(href, None);
    }

    #[test]
    fn wraps_output_in_digraph() {
        let text = render_to_string(&Diagram::new());
        assert!(text.starts_with("digraph diagram {"));
        assert!(text.trim_end().ends_with('}'));
    }

    #[test]
    fn renders_a_rectangle_node_with_stripped_link_and_style_sheet_colors() {
        let mut sheet = StyleSheet::new();
        sheet.add_rule(
            StyleRule::new("uri")
                .with_background_color(Color::White)
                .with_line_color(Color::Blue)
                .with_line_thickness(1),
        );
        let mut diagram = Diagram::new().with_style_sheet(sheet);
        diagram.add_box(
            DiagramBox::new(BoxId::new(0), Shape::Rectangle, "ex:Alice")
                .with_href("http://example.org/Alice")
                .with_stereotype("uri"),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("shape=box"));
        assert!(text.contains("label=\"ex:Alice\""));
        assert!(text.contains("URL=\"http://example.org/Alice\""));
        assert!(text.contains("fillcolor=\"White\""));
        assert!(text.contains("color=\"Blue\""));
    }

    #[test]
    fn renders_a_cloud_node_as_an_ellipse_not_a_boxed_shape() {
        // Regression test: `shape=cloud` isn't in the portable Graphviz shape catalog, so this
        // must not silently fall back to a plain/rounded box (indistinguishable from
        // `Shape::Rectangle`) the way `dot` itself would.
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Cloud, " ").with_stereotype("non_asserted"));

        let text = render_to_string(&diagram);
        assert!(text.contains("shape=ellipse"));
        assert!(!text.contains("shape=box"));
        assert!(!text.contains("shape=cloud"));
    }

    #[test]
    fn renders_a_class_box_as_an_html_table_with_compartments() {
        let mut diagram = Diagram::new().with_class_skin(ClassSkin {
            border_color: Color::Black,
            background_color: Color::LightBlue,
            arrow_color: Color::Black,
        });
        diagram.add_box(
            DiagramBox::new(BoxId::new(0), Shape::Class, ":Person")
                .with_compartments(vec!["[[http://schema.org/name schema:name]] : xsd:string".to_string()]),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("shape=plaintext"));
        assert!(text.contains("<TABLE"));
        assert!(text.contains("BGCOLOR=\"LightBlue\""));
        assert!(text.contains("<B>:Person</B>"));
        assert!(text.contains("schema:name : xsd:string"));
        assert!(!text.contains("[["));
    }

    #[test]
    fn renders_a_generalization_arrow() {
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Class, ":Employee"));
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Class, ":Person"));
        diagram.add_connector(crate::model::Connector::new(
            BoxId::new(0),
            BoxId::new(1),
            ConnectorKind::Generalization,
        ));

        let text = render_to_string(&diagram);
        assert!(text.contains("\"0\" -> \"1\" [arrowhead=empty];"));
    }

    #[test]
    fn renders_an_association_with_cardinality_and_arrow_style() {
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Class, ":Person"));
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Class, ":Company"));
        diagram.add_connector(
            crate::model::Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Association)
                .with_label(":worksFor")
                .with_target_decoration("*"),
        );
        diagram.add_connector(
            crate::model::Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Association)
                .with_label("subj")
                .with_style(ArrowStyle::new().with_line_color(Color::Blue)),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("label=\":worksFor\""));
        assert!(text.contains("headlabel=\"*\""));
        assert!(text.contains("label=\"subj\""));
        assert!(text.contains("color=\"Blue\""));
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn render_image_produces_a_valid_svg_via_the_real_dot_binary() {
        // `dot` isn't guaranteed to be installed everywhere this test suite runs (e.g. it isn't
        // on the plain `ubuntu-latest` GitHub Actions runner this crate's CI uses, mirroring
        // how the PlantUML backend has no equivalent "real java -jar" test either) — skip rather
        // than fail when it's simply not present, but still fail loudly on any other error.
        if Command::new("dot").arg("-V").output().is_err() {
            eprintln!("skipping: `dot` is not installed in this environment");
            return;
        }

        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Rectangle, "A"));
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Rectangle, "B"));
        diagram.add_connector(crate::model::Connector::new(
            BoxId::new(0),
            BoxId::new(1),
            ConnectorKind::Association,
        ));

        let backend = GraphVizBackend::default();
        let mut out = Vec::new();
        backend
            .render_image(&diagram, ImageFormat::Svg, &mut out)
            .expect("dot is installed (checked above), so rendering should succeed");

        let svg = String::from_utf8(out).unwrap();
        assert!(svg.contains("<svg"), "expected SVG output, got: {svg}");
    }
}
