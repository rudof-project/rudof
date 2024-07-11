use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Name {
    str: String,
    href: Option<String>,
    path: PathBuf,
}

impl Name {
    pub fn new(str: &str, href: Option<&str>) -> Name {
        Name {
            str: str.to_string(),
            href: href.map(|href| href.to_string()),
            path: Path::new(str).to_path_buf(),
        }
    }

    pub fn name(&self) -> String {
        self.str.clone()
    }

    pub fn href(&self) -> Option<String> {
        self.href.clone()
    }

    pub fn as_path(&self) -> PathBuf {
        self.path.clone()
    }
}
