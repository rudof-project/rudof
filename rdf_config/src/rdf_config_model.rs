use crate::RdfConfigError;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::{fs, io::Read};
use tracing::info;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Clone, Debug)]
pub struct RdfConfigModel {
    yaml: Yaml,
}

impl RdfConfigModel {
    pub fn new(yaml: Yaml) -> Self {
        RdfConfigModel { yaml }
    }

    pub fn serialize<W: Write>(
        &self,
        rdf_config_format: &RdfConfigFormat,
        writer: &mut W,
    ) -> Result<(), RdfConfigError> {
        match rdf_config_format {
            RdfConfigFormat::Yaml => {
                let fmt_writer = &mut IoWriterAsFmtWriter(writer);
                let mut emitter = yaml_rust2::YamlEmitter::new(fmt_writer);
                emitter
                    .dump(&self.yaml)
                    .map_err(|e| RdfConfigError::WritingRdfConfig { error: e.to_string() })?;
            },
            RdfConfigFormat::Internal => {
                write!(writer, "{self}").map_err(|e| RdfConfigError::WritingRdfConfig { error: e.to_string() })?;
            },
        }
        Ok(())
    }

    pub fn from_reader<R: std::io::Read>(reader: R, source_name: String) -> Result<RdfConfigModel, RdfConfigError> {
        let mut reader = std::io::BufReader::new(reader);
        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .map_err(|_| RdfConfigError::ErrorReadingFile {
                source_name: source_name.clone(),
            })?;
        let yamls = YamlLoader::load_from_str(buf.as_str()).map_err(|e| RdfConfigError::ErrorParsingYaml {
            error: e.to_string(),
            source_name: source_name.clone(),
        })?;
        let yaml = match yamls.len() {
            0 => {
                return Err(RdfConfigError::ErrorParsingYamlEmpty {
                    source_name: source_name.clone(),
                });
            },
            1 => yamls.into_iter().next().unwrap(),
            _ => {
                info!("Multiple config documents found, using the first one");
                yamls.into_iter().next().unwrap()
            },
        };
        Ok(RdfConfigModel::new(yaml))
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<RdfConfigModel, RdfConfigError> {
        Self::from_reader(
            fs::File::open(&path).map_err(|_| RdfConfigError::ErrorReadingFile {
                source_name: path.as_ref().display().to_string(),
            })?,
            path.as_ref().display().to_string(),
        )
    }

    pub fn yaml(&self) -> &Yaml {
        &self.yaml
    }
}

/// Supported rdf-config format
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RdfConfigFormat {
    Yaml,
    Internal,
}

struct IoWriterAsFmtWriter<T: Write>(T);

impl<T: Write> std::fmt::Write for IoWriterAsFmtWriter<T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}

impl Display for RdfConfigModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.yaml)
    }
}
