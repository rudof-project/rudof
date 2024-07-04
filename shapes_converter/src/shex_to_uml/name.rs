#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub struct Name {
    str: String,
    href: Option<String>,
}

impl Name {
    pub fn new(str: &str, href: Option<&str>) -> Name {
        Name {
            str: str.to_string(),
            href: if let Some(href) = href {
                Some(href.to_string())
            } else {
                None
            },
        }
    }

    pub fn name(&self) -> String {
        self.str.clone()
    }

    pub fn href(&self) -> Option<String> {
        self.href.clone()
    }
}
