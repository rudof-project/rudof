mod errors;

pub use errors::RenderError;

use crate::model::Diagram;
use std::io::Write;

/// Image formats a backend may be able to render a [`Diagram`] to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Svg,
}

/// Renders a technology-agnostic [`Diagram`] to a specific textual format (e.g. PlantUML,
/// Mermaid, DOT). Implemented by backend types in [`crate::backends`].
pub trait DiagramRenderer {
    fn render<W: Write>(&self, diagram: &Diagram, writer: &mut W) -> Result<(), RenderError>;
}

/// Basename (inside the temp directory) that [`ExternalToolRenderer::render_image`]'s shared
/// plumbing writes the rendered diagram text to. A backend whose external tool derives the
/// output filename from the source file's stem (as PlantUML does) must return an
/// [`ExternalToolRenderer::output_file_name`] consistent with this stem.
pub const DIAGRAM_SOURCE_FILE_NAME: &str = "diagram.src";

/// A [`DiagramRenderer`] that can also produce a raster/vector image by shelling out to an
/// external tool (e.g. `java -jar plantuml.jar`, `mmdc`, `dot`).
///
/// The temp-file / process / copy-to-writer plumbing is shared here; each backend only needs to
/// describe how to build the external command and name its expected output file.
#[cfg(not(target_family = "wasm"))]
pub trait ExternalToolRenderer: DiagramRenderer {
    /// Builds the command that converts `source_file` (the rendered diagram text, named
    /// [`DIAGRAM_SOURCE_FILE_NAME`]) into `format`, writing its output into `out_dir`.
    fn build_command(
        &self,
        source_file: &std::path::Path,
        out_dir: &std::path::Path,
        format: ImageFormat,
    ) -> Result<std::process::Command, RenderError>;

    /// The filename (inside `out_dir`) the external tool is expected to produce for `format`.
    fn output_file_name(&self, format: ImageFormat) -> &'static str;

    /// Renders `diagram` to `format` by writing it to a temp file, invoking [`Self::build_command`],
    /// and copying the resulting file to `writer`.
    fn render_image<W: Write>(
        &self,
        diagram: &Diagram,
        format: ImageFormat,
        writer: &mut W,
    ) -> Result<(), RenderError> {
        let tempdir = tempfile::TempDir::new().map_err(|e| RenderError::TempFileError { error: e.to_string() })?;
        let out_dir = tempdir.path();
        let source_file = out_dir.join(DIAGRAM_SOURCE_FILE_NAME);

        {
            let mut file =
                std::fs::File::create(&source_file).map_err(|e| RenderError::TempFileError { error: e.to_string() })?;
            self.render(diagram, &mut file)?;
            file.flush()
                .map_err(|e| RenderError::TempFileError { error: e.to_string() })?;
        }

        let mut command = self.build_command(&source_file, out_dir, format)?;
        let output = command.output().map_err(|e| RenderError::CommandError {
            command: format!("{command:?}"),
            error: e.to_string(),
        })?;
        if !output.status.success() {
            return Err(RenderError::CommandError {
                command: format!("{command:?}"),
                error: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let out_file_path = out_dir.join(self.output_file_name(format));
        let mut out_file = std::fs::File::open(&out_file_path).map_err(|e| RenderError::CantOpenOutputFile {
            path: out_file_path.display().to_string(),
            error: e,
        })?;
        std::io::copy(&mut out_file, writer).map_err(|e| RenderError::CopyError {
            path: out_file_path.display().to_string(),
            error: e,
        })?;
        Ok(())
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::model::{BoxId, Diagram, DiagramBox, Shape};
    use std::process::Command;

    struct CopyingBackend;

    impl DiagramRenderer for CopyingBackend {
        fn render<W: Write>(&self, diagram: &Diagram, writer: &mut W) -> Result<(), RenderError> {
            for b in diagram.boxes() {
                writeln!(writer, "box {}", b.title())?;
            }
            Ok(())
        }
    }

    impl ExternalToolRenderer for CopyingBackend {
        fn build_command(
            &self,
            source_file: &std::path::Path,
            out_dir: &std::path::Path,
            format: ImageFormat,
        ) -> Result<Command, RenderError> {
            let mut cmd = Command::new("cp");
            cmd.arg(source_file).arg(out_dir.join(self.output_file_name(format)));
            Ok(cmd)
        }

        fn output_file_name(&self, format: ImageFormat) -> &'static str {
            match format {
                ImageFormat::Png => "out.png",
                ImageFormat::Svg => "out.svg",
            }
        }
    }

    #[test]
    fn render_image_writes_the_external_tools_output_to_the_writer() {
        let mut diagram = Diagram::new();
        diagram.add_box(DiagramBox::new(BoxId::new(0), Shape::Rectangle, "A"));

        let backend = CopyingBackend;
        let mut out = Vec::new();
        backend
            .render_image(&diagram, ImageFormat::Svg, &mut out)
            .expect("render_image should succeed");

        assert_eq!(String::from_utf8(out).unwrap(), "box A\n");
    }

    #[test]
    fn render_image_reports_command_failures() {
        struct FailingBackend;
        impl DiagramRenderer for FailingBackend {
            fn render<W: Write>(&self, _diagram: &Diagram, _writer: &mut W) -> Result<(), RenderError> {
                Ok(())
            }
        }
        impl ExternalToolRenderer for FailingBackend {
            fn build_command(
                &self,
                _source_file: &std::path::Path,
                _out_dir: &std::path::Path,
                _format: ImageFormat,
            ) -> Result<Command, RenderError> {
                Ok(Command::new("false"))
            }
            fn output_file_name(&self, _format: ImageFormat) -> &'static str {
                "out.png"
            }
        }

        let diagram = Diagram::new();
        let mut out = Vec::new();
        let result = FailingBackend.render_image(&diagram, ImageFormat::Png, &mut out);
        assert!(matches!(result, Err(RenderError::CommandError { .. })));
    }
}
