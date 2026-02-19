use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::PathBuf;

use anyhow::{Result, bail};

use crate::output::color::{ColorSupport, detect_color_support_cached};

/// Configures and returns an appropriate output writer and its color capability.
///
/// This function handles:
/// 1. Mapping `None` or `"-"` to standard output.
/// 2. Checking for file existence and preventing accidental overwrites.
/// 3. Detecting color support (enabled for TTYs, disabled for files).
/// 4. Wrapping the output in a [BufWriter] for performance.
pub fn get_writer(
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<(Box<dyn Write>, ColorSupport)> {

    match output {

        // Handle stdout case (default)
        None => {
            let writer = BufWriter::new(io::stdout());
            let color = detect_color_support_cached();
            Ok((Box::new(writer), color))
        }

        Some(path) => {

            // Standard Unix convention: "-" represents stdout
            if path.to_string_lossy() == "-" {
                let writer = BufWriter::new(io::stdout());
                let color = detect_color_support_cached();
                return Ok((Box::new(writer), color));
            }

            // Guard against accidental data loss
            if path.exists() && !force_overwrite {
                bail!(
                    "File '{}' already exists. Use --force-overwrite to overwrite.",
                    path.display()
                );
            }

            // Open file for writing (truncates if exists and force_overwrite is true)
            let file = File::create(path)?;
            let writer = BufWriter::new(file);

            // Files should generally not contain ANSI color codes by default
            Ok((Box::new(writer), ColorSupport::NoColor))
        }
    }
}