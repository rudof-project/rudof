use crate::ShapeLabelIdx;

/// One way to satisfy an extended parent shape expression, obtained by choosing one branch
/// of every `ShapeOr` reachable from it through references, `ShapeAnd`s and the `extends`
/// chains of the shapes encountered.
///
/// An extended parent such as
/// ```shex
/// <Tools> @<TBoss> OR @<TGeek> OR @<TLabor>
/// ```
/// resolves to one alternative per branch. Each alternative separates:
/// * **bucket shapes** — the `Shape`-typed expressions whose triple expressions become
///   partition buckets when this alternative is selected: the chosen branch's main shape
///   plus, recursively, the main shapes of everything it extends. Deduplicated, so a
///   "diamond" ancestor reached through several paths becomes a single bucket.
/// * **constraints** — the remaining conjuncts of the chosen branches (node constraints,
///   non-main shapes such as `EXTRA`-restrictions, negations), to be evaluated against the
///   node when this alternative is selected. They deliberately contribute no partition
///   buckets: a constraint describes triples without consuming them.
///
/// See `docs/src/internals/feasibility-model.md` §3 for the resolution rules and a worked example.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExtendAlternative {
    bucket_shapes: Vec<ShapeLabelIdx>,
    constraints: Vec<ShapeLabelIdx>,
}

impl ExtendAlternative {
    pub fn with_bucket(idx: ShapeLabelIdx) -> Self {
        ExtendAlternative {
            bucket_shapes: vec![idx],
            constraints: Vec::new(),
        }
    }

    pub fn with_constraint(idx: ShapeLabelIdx) -> Self {
        ExtendAlternative {
            bucket_shapes: Vec::new(),
            constraints: vec![idx],
        }
    }

    pub fn with_constraints(idxs: Vec<ShapeLabelIdx>) -> Self {
        ExtendAlternative {
            bucket_shapes: Vec::new(),
            constraints: idxs,
        }
    }

    /// The shapes whose triple expressions must each be satisfied by its part of a
    /// partition of the neighbourhood, when this alternative is selected.
    pub fn bucket_shapes(&self) -> &[ShapeLabelIdx] {
        &self.bucket_shapes
    }

    /// The other conjuncts to check against the node when this alternative is selected.
    pub fn constraints(&self) -> &[ShapeLabelIdx] {
        &self.constraints
    }

    /// Union of two alternatives, deduplicated, preserving order of first occurrence.
    /// Used to combine one chosen alternative per extended parent into a selection.
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for b in &other.bucket_shapes {
            push_unique(&mut result.bucket_shapes, *b);
        }
        for c in &other.constraints {
            push_unique(&mut result.constraints, *c);
        }
        result
    }
}

fn push_unique(v: &mut Vec<ShapeLabelIdx>, x: ShapeLabelIdx) {
    if !v.contains(&x) {
        v.push(x);
    }
}

/// All pairwise merges: the alternatives of a conjunction of two shape expressions whose
/// alternatives are `left` and `right`.
pub(crate) fn cross_merge(left: Vec<ExtendAlternative>, right: Vec<ExtendAlternative>) -> Vec<ExtendAlternative> {
    left.iter().flat_map(|a| right.iter().map(|b| a.merge(b))).collect()
}
