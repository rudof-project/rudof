#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub struct Name {
    str: String,
    href: Option<String>,
    local_href: Option<String>,
    label: Option<String>,
}

impl Name {
    pub fn new(str: &str) -> Name {
        Name {
            str: str.to_string(),
            href: None,
            local_href: None,
            label: None,
        }
    }

    pub fn name(&self) -> String {
        self.str.clone()
    }

    pub fn href(&self) -> Option<String> {
        self.href.clone()
    }

    pub fn local_href(&self) -> Option<String> {
        self.local_href.clone()
    }

    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    pub fn add_href(&mut self, href: &str) {
        self.href = Some(href.to_string());
    }

    pub fn add_local_href(&mut self, local_href: &str) {
        self.local_href = Some(local_href.to_string());
    }

    pub fn add_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }
}
