//! Renders a [`Diagram`] to [PlantUML](https://plantuml.com) syntax, and (off `wasm`) to an
//! image by shelling out to `java -jar plantuml.jar`.

use crate::model::{ClassSkin, Connector, ConnectorKind, Diagram, DiagramBox, Direction, LineType, Shape};
use crate::render::{DiagramRenderer, RenderError};
use crate::style::{ArrowStyle, LineStyle, StyleRule};
use std::io::Write;

#[cfg(not(target_family = "wasm"))]
use {
    crate::render::{ExternalToolRenderer, ImageFormat},
    std::path::Path,
    std::process::Command,
};

/// Renders a [`Diagram`] to PlantUML text (and, off `wasm`, to an image via `plantuml.jar`).
#[derive(Debug, Clone)]
pub struct PlantUmlBackend {
    #[cfg_attr(target_family = "wasm", allow(dead_code))]
    plantuml_path: std::path::PathBuf,
}

impl PlantUmlBackend {
    /// Creates a backend that will look for a `plantuml.jar` at `plantuml_path` when asked to
    /// render an image. Ignored for text-only rendering.
    pub fn new(plantuml_path: impl Into<std::path::PathBuf>) -> Self {
        PlantUmlBackend {
            plantuml_path: plantuml_path.into(),
        }
    }
}

impl Default for PlantUmlBackend {
    /// A backend with no configured `plantuml.jar` path — usable for text-only [`DiagramRenderer::render`],
    /// but [`ExternalToolRenderer::render_image`] will fail until a real path is set via [`Self::new`].
    fn default() -> Self {
        PlantUmlBackend::new(std::path::PathBuf::new())
    }
}

impl DiagramRenderer for PlantUmlBackend {
    fn render<W: Write>(&self, diagram: &Diagram, writer: &mut W) -> Result<(), RenderError> {
        writeln!(writer, "@startuml")?;
        write_preamble(writer, diagram)?;
        write_style_sheet(writer, diagram)?;

        for b in diagram.boxes() {
            write_box(writer, b)?;
        }
        for c in diagram.connectors() {
            write_connector(writer, c)?;
        }

        writeln!(writer, "@enduml")?;
        Ok(())
    }
}

fn write_preamble<W: Write>(writer: &mut W, diagram: &Diagram) -> Result<(), RenderError> {
    if diagram.hide_empty_members() {
        writeln!(writer, "hide empty members")?;
    }
    if let Some(direction) = diagram.direction() {
        match direction {
            Direction::LeftToRight => writeln!(writer, "left to right direction")?,
            Direction::TopToBottom => writeln!(writer, "top to bottom direction")?,
        }
    }
    match diagram.line_type() {
        LineType::Orthogonal => writeln!(writer, "skinparam linetype ortho")?,
        LineType::Polyline => writeln!(writer, "skinparam linetype polyline")?,
        LineType::Default => {},
    }
    if diagram.hide_circles() {
        writeln!(writer, "hide circles")?;
    }
    if let Some(shadowing) = diagram.shadowing() {
        writeln!(writer, "skinparam shadowing {shadowing}")?;
    }
    if let Some(ClassSkin {
        border_color,
        background_color,
        arrow_color,
    }) = diagram.class_skin()
    {
        writeln!(writer, "skinparam class {{")?;
        writeln!(writer, " BorderColor {}", border_color.name())?;
        writeln!(writer, " BackgroundColor {}", background_color.name())?;
        writeln!(writer, " ArrowColor {}", arrow_color.name())?;
        writeln!(writer, "}}")?;
    }
    Ok(())
}

fn write_style_sheet<W: Write>(writer: &mut W, diagram: &Diagram) -> Result<(), RenderError> {
    if diagram.style_sheet().is_empty() {
        return Ok(());
    }
    writeln!(writer, "<style>")?;
    for rule in diagram.style_sheet().rules() {
        write_style_rule(writer, rule)?;
    }
    writeln!(writer, "</style>")?;
    writeln!(writer, "hide stereotype")?;
    Ok(())
}

fn write_style_rule<W: Write>(writer: &mut W, rule: &StyleRule) -> Result<(), RenderError> {
    writeln!(writer, ".{} {{", rule.name())?;
    if let Some(c) = rule.background_color() {
        writeln!(writer, "BackGroundColor {}", c.name())?;
    }
    if let Some(t) = rule.line_thickness() {
        writeln!(writer, "LineThickness {t}")?;
    }
    if let Some(c) = rule.line_color() {
        writeln!(writer, "LineColor {}", c.name())?;
    }
    if let Some(r) = rule.round_corner() {
        writeln!(writer, "RoundCorner {r}")?;
    }
    writeln!(writer, "}}")?;
    Ok(())
}

fn write_box<W: Write>(writer: &mut W, b: &DiagramBox) -> Result<(), RenderError> {
    let keyword = match b.shape() {
        Shape::Rectangle => "rectangle",
        Shape::Cloud => "cloud",
        Shape::Class => "class",
    };
    let stereotype = b.stereotype().map(|s| format!(" <<{s}>>")).unwrap_or_default();
    match b.shape() {
        Shape::Class => {
            let href = b.href().map(|h| format!(" [[{h} {}]]", b.title())).unwrap_or_default();
            writeln!(writer, "{keyword} \"{}\" as {}{stereotype}{href} {{", b.title(), b.id())?;
            for line in b.compartments() {
                writeln!(writer, "{line}")?;
                writeln!(writer, "--")?;
            }
            writeln!(writer, "}}")?;
        },
        Shape::Rectangle | Shape::Cloud => {
            let display = match b.href() {
                Some(href) => format!("[[{href} {}]]", b.title()),
                None => b.title().to_string(),
            };
            writeln!(writer, "{keyword} \"{display}\"{stereotype} as {}", b.id())?;
        },
    }
    Ok(())
}

fn write_connector<W: Write>(writer: &mut W, c: &Connector) -> Result<(), RenderError> {
    match c.kind() {
        ConnectorKind::Generalization => {
            writeln!(writer, "{} -|> {}", c.source(), c.target())?;
        },
        ConnectorKind::Association => {
            let label = c.label().unwrap_or("");
            if let Some(decoration) = c.target_decoration() {
                writeln!(writer, "{} --> \"{decoration}\" {} : {label}", c.source(), c.target())?;
            } else if let Some(style) = c.style() {
                writeln!(
                    writer,
                    "{}-->{} {} : {label}",
                    c.source(),
                    c.target(),
                    arrow_style_plantuml(style)
                )?;
            } else {
                writeln!(writer, "{} --> {} : {label}", c.source(), c.target())?;
            }
        },
    }
    Ok(())
}

fn arrow_style_plantuml(style: &ArrowStyle) -> String {
    format!(
        "#line:{};{}text:{}",
        style.line_color().name().to_lowercase(),
        line_style_plantuml(style.line_thickness()),
        style.text_color().name().to_lowercase()
    )
}

fn line_style_plantuml(style: LineStyle) -> &'static str {
    match style {
        LineStyle::Bold => "line.bold;",
        LineStyle::Normal => "",
        LineStyle::Dashed => "line.dashed;",
        LineStyle::Dotted => "line.dotted;",
    }
}

#[cfg(not(target_family = "wasm"))]
impl ExternalToolRenderer for PlantUmlBackend {
    fn build_command(&self, source_file: &Path, out_dir: &Path, format: ImageFormat) -> Result<Command, RenderError> {
        if !self.plantuml_path.exists() {
            return Err(RenderError::ExternalResourceMissing {
                path: self.plantuml_path.display().to_string(),
                error: "file does not exist".to_string(),
                env_var: "PLANTUML".to_string(),
            });
        }
        check_java_installed()?;
        check_plantuml_jar(&self.plantuml_path)?;

        let out_param = match format {
            ImageFormat::Png => "-png",
            ImageFormat::Svg => "-svg",
        };
        let mut command = Command::new("java");
        command.args([
            "-jar",
            &self.plantuml_path.display().to_string(),
            "-o",
            &out_dir.display().to_string(),
            out_param,
            "--verbose",
            &source_file.display().to_string(),
        ]);
        Ok(command)
    }

    fn output_file_name(&self, format: ImageFormat) -> &'static str {
        match format {
            ImageFormat::Png => "diagram.png",
            ImageFormat::Svg => "diagram.svg",
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn check_java_installed() -> Result<(), RenderError> {
    let output = Command::new("java")
        .arg("-version")
        .output()
        .map_err(|e| RenderError::ExternalToolUnavailable {
            tool: "java".to_string(),
            error: e.to_string(),
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(RenderError::ExternalToolUnavailable {
            tool: "java".to_string(),
            error: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[cfg(not(target_family = "wasm"))]
fn check_plantuml_jar(path: &Path) -> Result<(), RenderError> {
    let mut command = Command::new("java");
    command.args(["-jar", &path.display().to_string(), "-version"]);
    let output = command.output().map_err(|e| RenderError::ExternalToolUnavailable {
        tool: "java -jar plantuml.jar -version".to_string(),
        error: e.to_string(),
    })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(RenderError::ExternalResourceMissing {
            path: path.display().to_string(),
            error: String::from_utf8_lossy(&output.stderr).to_string(),
            env_var: "PLANTUML".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BoxId, ClassSkin, Connector, ConnectorKind, Diagram, DiagramBox, Shape};
    use crate::style::{ArrowStyle, Color, LineStyle, StyleRule, StyleSheet};

    fn render_to_string(diagram: &Diagram) -> String {
        let backend = PlantUmlBackend::new("plantuml.jar");
        let mut out = Vec::new();
        backend.render(diagram, &mut out).expect("render should succeed");
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn wraps_output_in_startuml_enduml() {
        let text = render_to_string(&Diagram::new());
        assert!(text.starts_with("@startuml\n"));
        assert!(text.trim_end().ends_with("@enduml"));
    }

    #[test]
    fn renders_a_plain_rectangle_node_with_stereotype_style_sheet() {
        let mut sheet = StyleSheet::new();
        sheet.add_rule(
            StyleRule::new("uri")
                .with_background_color(Color::White)
                .with_line_color(Color::Blue)
                .with_line_thickness(1)
                .with_round_corner(25),
        );
        let mut diagram = Diagram::new().with_style_sheet(sheet);
        diagram.add_box(
            DiagramBox::new(BoxId::new(0), Shape::Rectangle, "ex:Alice")
                .with_href("http://example.org/Alice")
                .with_stereotype("uri"),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("<style>"));
        assert!(text.contains(".uri {"));
        assert!(text.contains("BackGroundColor White"));
        assert!(text.contains("hide stereotype"));
        assert!(text.contains("rectangle \"[[http://example.org/Alice ex:Alice]]\" <<uri>> as 0"));
    }

    #[test]
    fn renders_a_plain_edge_with_label() {
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Rectangle, "A"));
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Rectangle, "B"));
        diagram.add_connector(
            Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Association).with_label("knows"),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("0 --> 1 : knows"));
    }

    #[test]
    fn renders_an_edge_with_an_arrow_style_and_no_target_decoration() {
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Cloud, " "));
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Rectangle, "A"));
        let style = ArrowStyle::new()
            .with_line_color(Color::Blue)
            .with_line_thickness(LineStyle::Dashed);
        diagram.add_connector(
            Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Association)
                .with_label("subj")
                .with_style(style),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("0-->1 #line:blue;line.dashed;text:black : subj"));
    }

    #[test]
    fn renders_a_class_box_with_compartments_and_a_link_with_cardinality() {
        let mut diagram = Diagram::new()
            .with_hide_empty_members(true)
            .with_hide_circles(true)
            .with_direction(crate::model::Direction::TopToBottom)
            .with_shadowing(true)
            .with_class_skin(ClassSkin {
                border_color: Color::Black,
                background_color: Color::LightBlue,
                arrow_color: Color::Black,
            });
        diagram.add_box(
            DiagramBox::new(BoxId::new(0), Shape::Class, ":Person")
                .with_stereotype("(S,#FF7700)")
                .with_compartments(vec![":name xsd:string".to_string()]),
        );
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Class, ":Company").with_stereotype("(S,#FF7700)"));
        diagram.add_connector(
            Connector::new(BoxId::new(0), BoxId::new(1), ConnectorKind::Association)
                .with_label(":worksFor")
                .with_target_decoration("*"),
        );

        let text = render_to_string(&diagram);
        assert!(text.contains("hide empty members"));
        assert!(text.contains("top to bottom direction"));
        assert!(text.contains("hide circles"));
        assert!(text.contains("skinparam shadowing true"));
        assert!(text.contains("BorderColor Black"));
        assert!(text.contains("BackgroundColor LightBlue"));
        assert!(text.contains(":name xsd:string"));
        assert!(text.contains("--"));
        assert!(text.contains("0 --> \"*\" 1 : :worksFor"));
    }

    #[test]
    fn renders_a_generalization_arrow() {
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Class, ":Employee"));
        diagram.add_box(DiagramBox::new(BoxId::new(1), Shape::Class, ":Person"));
        diagram.add_connector(Connector::new(
            BoxId::new(0),
            BoxId::new(1),
            ConnectorKind::Generalization,
        ));

        let text = render_to_string(&diagram);
        assert!(text.contains("0 -|> 1"));
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn build_command_reports_missing_jar_without_needing_java() {
        let backend = PlantUmlBackend::new("/no/such/plantuml.jar");
        let tmp = tempfile::TempDir::new().unwrap();
        let result = backend.build_command(&tmp.path().join("diagram.src"), tmp.path(), ImageFormat::Svg);
        assert!(matches!(result, Err(RenderError::ExternalResourceMissing { .. })));
    }
}
