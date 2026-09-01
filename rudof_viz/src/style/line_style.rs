use serde::{Deserialize, Serialize};

/// The line thickness/style options shared by rudof's visualization backends.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
#[serde(rename_all = "snake_case")]
pub enum LineStyle {
    Bold,
    #[default]
    Normal,
    Dashed,
    Dotted,
}

#[cfg(test)]
mod tests {
    use super::LineStyle;

    #[test]
    fn default_is_normal() {
        assert_eq!(LineStyle::default(), LineStyle::Normal);
    }
}
