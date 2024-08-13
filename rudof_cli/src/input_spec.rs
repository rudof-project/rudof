use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub enum InputSpec {
    Path(PathBuf),
    Stdin,
    Url(String),
}

impl InputSpec {
    pub fn path<P: AsRef<Path>>(path: P) -> InputSpec {
        InputSpec::Path(PathBuf::from(path.as_ref()))
    }
}

impl FromStr for InputSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.starts_with("http://") => Ok(InputSpec::Url(s.to_string())),
            _ if s.starts_with("https://") => Ok(InputSpec::Url(s.to_string())),
            _ if s == "-" => Ok(InputSpec::Stdin),
            _ => {
                let pb: PathBuf = PathBuf::from_str(s)
                    .map_err(|e| format!("Error parsing {s} as a path: {e}"))?;
                Ok(InputSpec::Path(pb))
            }
        }
    }
}
