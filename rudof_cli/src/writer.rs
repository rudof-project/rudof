use crate::ColorSupport;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter};
use std::path::Path;
use std::{io::Write, path::PathBuf};

use supports_color::Stream;

use anyhow::{Result, bail};
// use ColorSupport;

pub fn get_writer(
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<(Box<dyn Write>, ColorSupport)> {
    match output {
        None => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let color_support = match supports_color::on(Stream::Stdout) {
                Some(_) => ColorSupport::WithColor,
                _ => ColorSupport::NoColor,
            };
            Ok((Box::new(handle), color_support))
        }
        Some(path) => {
            let file = if Path::exists(path) {
                if force_overwrite {
                    OpenOptions::new().write(true).truncate(true).open(path)
                } else {
                    bail!(
                        "File {} already exists. If you want to overwrite it, use the `force-overwrite` option",
                        path.display()
                    );
                }
            } else {
                File::create(path)
            }?;
            let writer = BufWriter::new(file);
            Ok((Box::new(writer), ColorSupport::NoColor))
        }
    }
}
