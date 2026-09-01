use crate::style::Color;

/// A named style rule (e.g. for a PlantUML `<<stereotype>>`), with every attribute optional so
/// backends can fall back to their own defaults for whatever is left unset.
#[derive(Debug, PartialEq, Clone)]
pub struct StyleRule {
    name: String,
    background_color: Option<Color>,
    line_thickness: Option<u32>,
    line_color: Option<Color>,
    round_corner: Option<u32>,
}

impl StyleRule {
    pub fn new(name: &str) -> Self {
        StyleRule {
            name: name.to_string(),
            background_color: None,
            line_thickness: None,
            line_color: None,
            round_corner: None,
        }
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_line_thickness(mut self, thickness: u32) -> Self {
        self.line_thickness = Some(thickness);
        self
    }

    pub fn with_line_color(mut self, color: Color) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn with_round_corner(mut self, corner: u32) -> Self {
        self.round_corner = Some(corner);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    pub fn line_thickness(&self) -> Option<u32> {
        self.line_thickness
    }

    pub fn line_color(&self) -> Option<Color> {
        self.line_color
    }

    pub fn round_corner(&self) -> Option<u32> {
        self.round_corner
    }
}

/// A collection of named style rules, applied by a backend to boxes carrying a matching stereotype.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct StyleSheet {
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        StyleSheet { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: StyleRule) {
        self.rules.push(rule);
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn rules(&self) -> impl Iterator<Item = &StyleRule> {
        self.rules.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_style_sheet_has_no_rules() {
        let sheet = StyleSheet::new();
        assert!(sheet.is_empty());
        assert_eq!(sheet.rules().count(), 0);
    }

    #[test]
    fn added_rule_is_visible() {
        let mut sheet = StyleSheet::new();
        sheet.add_rule(
            StyleRule::new("uri")
                .with_background_color(Color::White)
                .with_line_color(Color::Blue)
                .with_line_thickness(1)
                .with_round_corner(25),
        );
        assert!(!sheet.is_empty());
        let rule = sheet.rules().next().unwrap();
        assert_eq!(rule.name(), "uri");
        assert_eq!(rule.background_color(), Some(Color::White));
        assert_eq!(rule.line_color(), Some(Color::Blue));
        assert_eq!(rule.line_thickness(), Some(1));
        assert_eq!(rule.round_corner(), Some(25));
    }
}
