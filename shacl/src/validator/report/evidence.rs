use rudof_rdf::rdf_core::SHACLPath;
use rudof_rdf::rdf_core::term::Object;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

/// Evidence that a `(focus node, constraint component)` pair conforms —
/// the positive counterpart of [`crate::validator::report::ValidationResult`].
///
/// Recorded at the same granularity as violations: one `Evidence` wherever a
/// constraint validator would otherwise have pushed one `ValidationResult`.
#[derive(Debug, Clone, Eq)]
pub struct Evidence {
    focus_node: Object,
    constraint_component: Object,

    path: Option<SHACLPath>,
    value: Option<Object>,
    source: Option<Object>,
}

impl Evidence {
    pub fn new(focus_node: Object, constraint_component: Object) -> Self {
        Self {
            focus_node,
            constraint_component,
            path: None,
            value: None,
            source: None,
        }
    }

    pub fn with_path(mut self, path: Option<SHACLPath>) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: Option<Object>) -> Self {
        self.value = value;
        self
    }

    pub fn with_source(mut self, source: Option<Object>) -> Self {
        self.source = source;
        self
    }

    pub fn focus_node(&self) -> &Object {
        &self.focus_node
    }

    pub fn constraint_component(&self) -> &Object {
        &self.constraint_component
    }

    pub fn path(&self) -> Option<&SHACLPath> {
        self.path.as_ref()
    }

    pub fn value(&self) -> Option<&Object> {
        self.value.as_ref()
    }

    pub fn source(&self) -> Option<&Object> {
        self.source.as_ref()
    }
}

impl Display for Evidence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Evidence(focus_node: {}, constraint_component: {}, path: {:?}, value: {:?}, source: {:?})",
            self.focus_node, self.constraint_component, self.path, self.value, self.source
        )
    }
}

impl PartialEq for Evidence {
    fn eq(&self, other: &Self) -> bool {
        self.focus_node == other.focus_node
            && self.constraint_component == other.constraint_component
            && self.path == other.path
            && self.value == other.value
            && self.source == other.source
    }
}

impl Hash for Evidence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.focus_node.hash(state);
        self.constraint_component.hash(state);
        self.path.hash(state);
        self.value.hash(state);
        self.source.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builders_set_optional_fields() {
        let focus = Object::iri(rudof_iri::IriS::new_unchecked("http://ex/n1"));
        let component = Object::iri(rudof_iri::IriS::new_unchecked("http://ex/comp"));
        let value = Object::iri(rudof_iri::IriS::new_unchecked("http://ex/v"));
        let source = Object::iri(rudof_iri::IriS::new_unchecked("http://ex/s"));

        let ev = Evidence::new(focus.clone(), component.clone())
            .with_value(Some(value.clone()))
            .with_source(Some(source.clone()));

        assert_eq!(ev.focus_node(), &focus);
        assert_eq!(ev.constraint_component(), &component);
        assert_eq!(ev.value(), Some(&value));
        assert_eq!(ev.source(), Some(&source));
        assert_eq!(ev.path(), None);
    }

    #[test]
    fn equality_ignores_nothing_but_matches_on_all_fields() {
        let focus = Object::iri(rudof_iri::IriS::new_unchecked("http://ex/n1"));
        let component = Object::iri(rudof_iri::IriS::new_unchecked("http://ex/comp"));
        let a = Evidence::new(focus.clone(), component.clone());
        let b = Evidence::new(focus, component);
        assert_eq!(a, b);
    }
}
