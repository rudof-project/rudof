use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Name {
    str: String,
    href: Option<String>,

    // Local_ref keeps track of the local file where the shape will be written and the local name to refer to that file
    local_ref: Option<(PathBuf, String)>,
}

impl Name {
    pub fn new<P: AsRef<Path>>(str: &str, href: Option<&str>, target_folder: P) -> Name {
        if let Some(local_name) = str.strip_prefix(':') {
            let local_name_html = format!("{local_name}.html");
            Name {
                str: str.to_string(),
                href: href.map(|href| href.to_string()),
                local_ref: Some((
                    target_folder
                        .as_ref()
                        .join(Path::new(local_name_html.as_str())),
                    local_name_html,
                )),
            }
        } else {
            Name {
                str: str.to_string(),
                href: href.map(|href| href.to_string()),
                local_ref: None,
            }
        }
    }

    pub fn name(&self) -> String {
        self.str.clone()
    }

    pub fn href(&self) -> Option<String> {
        self.href.clone()
    }

    pub fn as_local_ref(&self) -> Option<(PathBuf, String)> {
        self.local_ref.clone()
    }

    pub fn as_path(&self) -> Option<PathBuf> {
        self.local_ref.as_ref().map(|(r, _)| r.clone())
    }

    pub fn as_local_href(&self) -> Option<String> {
        self.local_ref.as_ref().map(|(_, local)| local.clone())
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)?;
        if let Some(href) = &self.href {
            write!(f, " href={href}")?;
        }
        if let Some((path, local_ref)) = &self.local_ref {
            write!(f, " path={} local_ref={}", path.display(), local_ref)?;
        }
        Ok(())
    }
}
