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
///   buckets: a constraint describes triples without consuming them.  Each constraint
///   carries its **split scope**: the bucket shapes whose partition parts it may see when
///   it is validated (its owning shape's main bucket and, recursively, the buckets of
///   everything that main shape extends).
///
/// See `docs/src/internals/feasibility-model.md` §3 for the resolution rules and a worked example.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExtendAlternative {
    bucket_shapes: Vec<ShapeLabelIdx>,
    constraints: Vec<ScopedConstraint>,
}

/// A constraint conjunct of an alternative together with its split scope.
#[derive(Debug, Clone, PartialEq)]
pub struct ScopedConstraint {
    expr: ShapeLabelIdx,
    scope: Vec<ShapeLabelIdx>,
}

impl ScopedConstraint {
    /// The constraint's shape expression.
    pub fn expr(&self) -> &ShapeLabelIdx {
        &self.expr
    }

    /// The bucket shapes whose partition parts the constraint may see: split-constraint
    /// validation runs the constraint's triple expressions against the union of the
    /// triples the partition allocated to these buckets.
    pub fn scope(&self) -> &[ShapeLabelIdx] {
        &self.scope
    }
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
            constraints: vec![ScopedConstraint {
                expr: idx,
                scope: Vec::new(),
            }],
        }
    }

    /// Attach `idxs` as constraints scoped to this alternative's current bucket shapes —
    /// used when resolving a `ShapeAnd`, where the non-main conjuncts may see exactly the
    /// splits of the main conjunct's buckets.
    pub fn with_scoped_constraints(mut self, idxs: &[ShapeLabelIdx]) -> Self {
        let scope = self.bucket_shapes.clone();
        for c in idxs {
            push_unique_constraint(
                &mut self.constraints,
                ScopedConstraint {
                    expr: *c,
                    scope: scope.clone(),
                },
            );
        }
        self
    }

    /// The shapes whose triple expressions must each be satisfied by its part of a
    /// partition of the neighbourhood, when this alternative is selected.
    pub fn bucket_shapes(&self) -> &[ShapeLabelIdx] {
        &self.bucket_shapes
    }

    /// The other conjuncts to check against the node when this alternative is selected.
    pub fn constraints(&self) -> &[ScopedConstraint] {
        &self.constraints
    }

    /// Union of two alternatives, deduplicated, preserving order of first occurrence.
    /// Used to combine one chosen alternative per extended parent into a selection.
    /// A constraint reaching the merge through several paths (a diamond) keeps one
    /// entry whose scope is the union of the paths' scopes.
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for b in &other.bucket_shapes {
            push_unique(&mut result.bucket_shapes, *b);
        }
        for c in &other.constraints {
            push_unique_constraint(&mut result.constraints, c.clone());
        }
        result
    }
}

fn push_unique_constraint(v: &mut Vec<ScopedConstraint>, c: ScopedConstraint) {
    if let Some(existing) = v.iter_mut().find(|e| e.expr == c.expr) {
        for s in c.scope {
            push_unique(&mut existing.scope, s);
        }
    } else {
        v.push(c);
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
