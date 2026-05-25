use rudof_iri::IriS;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub enum ClosedInfo {
    Yes {
        /// Properties that have been declared as ignored
        ignored_properties: HashSet<IriS>,
        /// Properties that appear in the definition
        defined_properties: HashSet<IriS>,
    },

    #[default]
    No,
}

impl ClosedInfo {
    pub fn is_closed(&self) -> bool {
        matches!(self, ClosedInfo::Yes { .. })
    }

    pub fn ignored_properties(&self) -> Option<&HashSet<IriS>> {
        match self {
            ClosedInfo::Yes { ignored_properties, .. } => Some(ignored_properties),
            ClosedInfo::No => None,
        }
    }

    pub fn defined_properties(&self) -> Option<&HashSet<IriS>> {
        match self {
            ClosedInfo::Yes { defined_properties, .. } => Some(defined_properties),
            ClosedInfo::No => None,
        }
    }

    /// Allowed properties are the union of ignored properties and the properties that are defined in a shape
    pub fn allowed_properties(&self) -> Option<HashSet<IriS>> {
        match self {
            ClosedInfo::Yes {
                defined_properties,
                ignored_properties,
            } => {
                let result = defined_properties.union(ignored_properties).cloned().collect();
                Some(result)
            },
            ClosedInfo::No => None,
        }
    }
}
