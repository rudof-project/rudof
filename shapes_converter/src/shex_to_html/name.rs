use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
// use tracing::debug;

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Name {
    repr: String,
    path: Option<PathBuf>,
    href: Option<String>,
    relative_href: Option<String>,
    label: Option<String>,
}

impl Name {
    pub fn new<P: AsRef<Path>>(str: &str, href: Option<&str>, target_folder: P) -> Name {
        let local_name = str.replace(':', "_");
        let local_name_html = format!("{local_name}.html");
        Name {
            repr: str.to_string(),
            path: Some(
                target_folder
                    .as_ref()
                    .join(Path::new(local_name_html.as_str())),
            ),
            href: href.map(|s| s.to_string()),
            relative_href: Some(local_name_html),
            label: None,
        }
    }

    pub fn name(&self) -> String {
        self.repr.to_string()
    }

    pub fn href(&self) -> Option<String> {
        self.href.as_ref().map(|str| str.to_string())
    }

    pub fn get_path_localname(&self) -> Option<(PathBuf, String)> {
        if let Some(href) = &self.relative_href {
            self.path
                .as_ref()
                .map(|path| (path.to_owned(), href.to_string()))
        } else {
            None
        }
    }

    pub fn as_path(&self) -> Option<PathBuf> {
        self.path.as_ref().map(|path| path.to_owned())
    }

    pub fn as_href(&self) -> Option<String> {
        self.href.as_ref().map(|href| href.to_string())
    }

    pub fn as_relative_href(&self) -> Option<String> {
        self.relative_href.as_ref().map(|href| href.to_string())
    }

    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    pub fn add_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }
}
