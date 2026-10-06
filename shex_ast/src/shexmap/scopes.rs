//! The scope tree: a binding tree read structurally, for materialization by iteration scopes.
//!
//! A binding tree (see [`crate::shexmap::bindings`]) is a *scope*: an object of own bindings,
//! or an array whose first element is that object and whose other elements are *lists*, each
//! list holding one *iteration* (a scope) per match of a repeated constraint or group.
//! [`ScopeTree`] holds the scopes of a tree in an arena, each knowing its parent, and answers
//! the questions materialization asks: at what list a variable is bound, and which scopes
//! under a given one belong to a given list.
//!
//! A **list path** names a list by position: the list indices from the root, so `[]` is the
//! root scope itself, `[0]` the root's first list, `[0, 1]` the second list of an iteration
//! of that list.  Every iteration of a list shares the list's path; a variable bound in
//! those iterations is *bound at* that list path.

use std::collections::{BTreeMap, HashMap, HashSet};

use rudof_rdf::rdf_core::term::Object;

use crate::shexmap::bindings::{BindingTree, NODE_KEY};
use crate::shexmap::error::ShExMapError;

/// The list indices from the root to a list.
pub type ListPath = Vec<usize>;

/// An index into a [`ScopeTree`]'s arena.
pub type ScopeId = usize;

/// One scope of the tree.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Variables bound here (never `@node`).
    pub own: BTreeMap<String, Object>,
    /// The node this iteration matched, if recorded.
    pub node: Option<Object>,
    /// One list of iteration scopes per repeated expression.
    pub lists: Vec<Vec<ScopeId>>,
    pub parent: Option<ScopeId>,
    /// The list this scope is an iteration of.
    pub list_path: ListPath,
    /// (list index, iteration index, ...) from the root.
    pub path: Vec<usize>,
}

impl Scope {
    pub fn depth(&self) -> usize {
        self.list_path.len()
    }
}

fn is_prefix(a: &[usize], b: &[usize]) -> bool {
    b.len() >= a.len() && b[..a.len()] == a[..]
}

/// A parsed tree plus the two indexes materialization needs.
#[derive(Debug, Clone)]
pub struct ScopeTree {
    scopes: Vec<Scope>,
    /// Variable -> the list path it is bound at.
    pub bound_at: HashMap<String, ListPath>,
    /// How many bindings the tree holds.
    pub bindings: usize,
}

impl ScopeTree {
    pub fn new(tree: &BindingTree) -> Result<ScopeTree, ShExMapError> {
        let mut st = ScopeTree {
            scopes: Vec::new(),
            bound_at: HashMap::new(),
            bindings: 0,
        };
        st.add(tree, None, Vec::new(), Vec::new());
        for i in 0..st.scopes.len() {
            let list_path = st.scopes[i].list_path.clone();
            for v in st.scopes[i].own.keys() {
                st.bindings += 1;
                match st.bound_at.get(v) {
                    None => {
                        st.bound_at.insert(v.clone(), list_path.clone());
                    },
                    Some(at) if *at != list_path => {
                        return Err(ShExMapError::binding_tree(format!(
                            "variable {v} is bound at two lists, {at:?} and {list_path:?}"
                        )));
                    },
                    Some(_) => {},
                }
            }
        }
        Ok(st)
    }

    fn add(&mut self, tree: &BindingTree, parent: Option<ScopeId>, list_path: ListPath, path: Vec<usize>) -> ScopeId {
        let id = self.scopes.len();
        let own: BTreeMap<String, Object> = tree
            .own
            .iter()
            .filter(|(k, _)| !k.starts_with('@'))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        self.scopes.push(Scope {
            own,
            node: tree.own.get(NODE_KEY).cloned(),
            lists: Vec::new(),
            parent,
            list_path: list_path.clone(),
            path: path.clone(),
        });
        let mut lists = Vec::new();
        for (i, list) in tree.lists.iter().enumerate() {
            let mut lp = list_path.clone();
            lp.push(i);
            let mut ids = Vec::new();
            for (j, it) in list.iter().enumerate() {
                let mut p = path.clone();
                p.push(i);
                p.push(j);
                ids.push(self.add(it, Some(id), lp.clone(), p));
            }
            lists.push(ids);
        }
        self.scopes[id].lists = lists;
        id
    }

    pub fn root(&self) -> ScopeId {
        0
    }

    pub fn scope(&self, id: ScopeId) -> &Scope {
        &self.scopes[id]
    }

    pub fn variables(&self) -> HashSet<String> {
        self.bound_at.keys().cloned().collect()
    }

    /// `(value, scope)` for `var` from `id` or the nearest ancestor binding it.
    pub fn lookup(&self, id: ScopeId, var: &str) -> Option<(&Object, ScopeId)> {
        let mut cur = Some(id);
        while let Some(s) = cur {
            if let Some(v) = self.scopes[s].own.get(var) {
                return Some((v, s));
            }
            cur = self.scopes[s].parent;
        }
        None
    }

    /// The node of this scope or of the nearest ancestor that recorded one.
    pub fn nearest_node(&self, id: ScopeId) -> Option<&Object> {
        let mut cur = Some(id);
        while let Some(s) = cur {
            if let Some(n) = &self.scopes[s].node {
                return Some(n);
            }
            cur = self.scopes[s].parent;
        }
        None
    }

    /// The scopes below `id` that are iterations of `list_path`, in document order.
    pub fn descendants_at(&self, id: ScopeId, list_path: &[usize]) -> Vec<ScopeId> {
        let here = &self.scopes[id];
        if !is_prefix(&here.list_path, list_path) || list_path == here.list_path.as_slice() {
            return Vec::new();
        }
        let mut out = Vec::new();
        for list in &here.lists {
            for &it in list {
                let lp = &self.scopes[it].list_path;
                if lp.as_slice() == list_path {
                    out.push(it);
                } else if is_prefix(lp, list_path) {
                    out.extend(self.descendants_at(it, list_path));
                }
            }
        }
        out
    }

    pub fn iter(&self) -> impl Iterator<Item = (ScopeId, &Scope)> {
        self.scopes.iter().enumerate()
    }
}
