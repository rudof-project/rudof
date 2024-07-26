use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Name {
    repr: String,
    path: Option<PathBuf>,
    href: Option<String>,
}

impl Name {
    pub fn new<P: AsRef<Path>>(str: &str, href: Option<&str>, target_folder: P) -> Name {
        if let Some(local_name) = str.strip_prefix(':') {
            let local_name_html = format!("{local_name}.html");
            Name {
                repr: str.to_string(),
                path: Some(
                    target_folder
                        .as_ref()
                        .join(Path::new(local_name_html.as_str())),
                ),
                href: Some(local_name_html),
            }
        } else if let Some(href) = href {
            Name {
                repr: str.to_string(),
                href: Some(href.to_string()),
                path: None,
            }
        } else {
            Name {
                repr: str.to_string(),
                path: None,
                href: None,
            }
        }
    }

    pub fn name(&self) -> String {
        self.repr.to_string()
    }

    pub fn href(&self) -> Option<String> {
        self.href.as_ref().map(|str| str.to_string())
    }

    pub fn as_local_ref(&self) -> Option<(PathBuf, String)> {
        if let Some(href) = &self.href {
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

    pub fn as_local_href(&self) -> Option<String> {
        self.href.as_ref().map(|href| href.to_string())
    }
}
