# A feasibility model for EXTENDS, ShapeOr parents, and partition search in rudof

*Prior art: the same model implemented and proved for Apache Jena
(`fhircat/jena` branch `extends-validation-error-reporting`,
`jena-shex/docs/matching-search-optimization.md`), where it is accompanied by a soundness
proof, a differential test harness, and measurements; this document maps the model onto
rudof's crates and works through the two fixture schemas.*

Contents:
1. [The model in one page](#1-the-model-in-one-page)
2. [Worked example A: `person-OneOf.shex` → feasibility structure](#2-worked-example-a)
3. [Worked example B: `person-extends.shex` → selection alternatives](#3-worked-example-b)
4. [Selection-OneOf is not TripleExpr-OneOf](#4-selection-oneof-is-not-tripleexpr-oneof)
5. [Implementation plan mapped to rudof](#5-implementation-plan)
6. [Test plan](#6-test-plan)

---

## 1. The model in one page

**The search problem.** Partition-based ShEx asks: does *some* assignment of neighbourhood
triples to triple constraints (TCs) satisfy every triple expression in play? With `EXTENDS`,
"in play" means the shape's own expression **and** each ancestor's, over disjoint TC
alphabets — i.e. inheritance is semantically a single virtual `EachOf` over the union
alphabet. rudof's `partitions_iter` explores `k^n` bucket assignments guarded only by a
predicate-membership test; Jena explored the same space as a Cartesian product. Both are
blind to the reasons assignments fail.

**Partial bags.** Give every TC an interval `[lo, hi]`: `lo` = triples already committed to
it, `hi` = `lo` + triples that could still be assigned to it (they have it as a candidate).
A *completion* is any exact bag between `lo` and `hi`.

**The feasibility predicate `F`.** A compositional necessary condition: `F(lo,hi) = false`
guarantees *no* completion is accepted. Computed over the (SORBE-normalized) expression tree
in two modes — the tables below are the entire specification:

*Exact mode* `Fx(E)` — E must match exactly once; `zero(E)` ≝ every TC of E has `lo = 0`:

| E | Fx(E) |
|---|-------|
| `TC[m,n]` | `lo ≤ n ∧ hi ≥ m` |
| `EachOf(E₁…Eₖ)` | `⋀ᵢ Fx(Eᵢ)` |
| `OneOf(E₁…Eₖ)` | `⋁ᵢ ( Fx(Eᵢ) ∧ ⋀_{j≠i} zero(Eⱼ) )` |
| `E?` | `zero(E) ∨ Fx(E)` |
| `E*` | `zero(E) ∨ (Fi(E) ∧ once(E))` |
| `E+` | `Fi(E) ∧ once(E)` |

*Iterated mode* `Fi(E)` — E matches as a sum of `q ≥ 1` iterations. Only monotone
consequences survive summation: OneOf exclusivity is dropped (iterations mix branches),
upper bounds are dropped (`q` unbounded), EachOf co-occurrence is kept at occupancy level:

| E | Fi(E) |
|---|-------|
| `TC[m,n]` | `(m = 0 ∨ hi ≥ m) ∧ (n > 0 ∨ lo = 0)` |
| `EachOf(E₁…Eₖ)` | `⋀ᵢ Fi(Eᵢ)` |
| `OneOf(E₁…Eₖ)` | `⋀ᵢ (zero(Eᵢ) ∨ Fi(Eᵢ))` |
| `E?`, `E*` | `zero(E) ∨ Fi(E)` |
| `E+` | `Fi(E) ∧ once(E)` |

with `once(E)` (one non-empty iteration fits under `hi`): `TC[m,n] → m=0 ∨ hi≥m`;
`EachOf → ⋀`; `OneOf → ⋁`; min-0 cardinalities → true.

**The algorithm** (replaces blind enumeration; keeps the exact matcher as final verifier):

1. *Arc consistency*: delete candidate TC from a triple's candidate set when committing one
   such triple (`lo = 1` on that TC) already fails `F`. Iterate to fixpoint (`hi` shrinks).
   A triple with an emptied candidate set ⇒ shape fails (forced assignment: a value-matching
   triple on a mentioned predicate can never be left unmatched — approved shexTest
   `1val2IRIREFExtra1_fail-iri2`).
2. *Distribution search*: group triples into **classes** by identical candidate set;
   depth-first assign each class a count vector over its candidates (`C(n+k-1,k-1)` options,
   not `k^n`), re-checking `F` after each class.
3. *Expansion*: only bags surviving `F` are expanded to concrete triple→TC assignments (the
   witnesses that CLOSED/EXTRA bookkeeping, EXTENDS constraint conjuncts, and semantic
   actions consume), each verified by the exact matcher.

**Guarantees** (proved in the Jena treatise; the proofs carry over verbatim since they are
stated over bag languages, not over an implementation): `F` never refutes a satisfiable
state (soundness of pruning, by structural induction over the tables); every valid matching
is still enumerated (completeness); `F` deliberately ignores Parikh coupling
(`(a;b)+` needs #a=#b) and divisibility, which is why the exact verifier must stay.
Measured in Jena: a shared-predicate OneOf nonconformance went from 2^n exact checks
(16.7 M at n = 24) to **0**, with the 2135-test conformance suite unchanged.

**EXTENDS costs nothing extra**: the hierarchy is `Fx(root₁) ∧ … ∧ Fx(rootₖ)` — the EachOf
rule at a virtual root. **ShapeOr parents cost one more layer**: a *selection* choice point
above the virtual root (§3, §4).

## 2. Worked example A: `person-OneOf.shex` → feasibility structure {#2-worked-example-a}

[`examples/shex/person-OneOf.shex`](../../../examples/shex/person-OneOf.shex) keeps all disjunction inside one
triple expression; rudof validates it today via `rbe` derivatives. It grounds the tables on
a schema rudof already handles, and shows what `F` adds.

**TC inventory of `<Person>`'s closed triple expression** (13 TCs, tree below):

```
EachOf(
  t_id     ex:id IRI                       [1,1]
  t_rolls  ex:rolls @<RoleCode>            [1,∞)   (+)
  OneOf_A( A1: EachOf(t_fname foaf:name . , t_fmbox foaf:mbox IRI)
           A2: EachOf(t_sname schema:name ., t_smbox schema:mbox IRI) )
  OneOf_B( B1: EachOf(t_sun  ex:sunglassesBrand .,  t_tie    ex:tieMaterial .)
           B2: EachOf(t_slide ex:SlideRoolLength ., t_pocket ex:PocketProtectorMaterial .)
           B3: EachOf(t_glove ex:glovesSize .,      t_boot   ex:bootSize .) )
  t_badge  ex:badgeNumber xsd:integer      [1,1] )
```

**alice** ([`examples/shex/person-Engineer.ttl`](../../../examples/shex/person-Engineer.ttl)) has 10 triples.
Every predicate here belongs to exactly one TC, so candidate sets are singletons and there
are 9 classes (the two `ex:rolls` triples share the candidate set `{t_rolls}` and form one
class of size 2). Initial intervals: `hi(t_rolls) = 2`; `hi = 1` for the 7 present
predicates; `hi = 0` for `t_sname, t_smbox, t_sun, t_tie, t_glove, t_boot`.

*Arc consistency trace for `t_fname`* (commit one foaf:name triple, `lo = 1_{t_fname}`):
`Fx(EachOf)` → `Fx(OneOf_A)` → branch A1: `Fx(t_fname)`: `1 ≤ 1 ∧ 1 ≥ 1` ✓;
`Fx(t_fmbox)`: `hi = 1 ≥ 1` ✓; `zero(A2)` ✓ → supported. Every candidate survives; one
distribution exists; the exact matcher confirms it. Same answer as today, same cost.

*What `F` buys — the mutant:* add `ex:tieMaterial "silk"` to alice. The new triple's only
candidate is `t_tie`. AC test with `lo = 1_{t_tie}`: branch B1 requires `hi(t_sun) ≥ 1`,
but `hi(t_sun) = 0` ✗; branches B2/B3 require `zero(B1)`, but `lo(t_tie) = 1` ✗ →
`Fx(OneOf_B)` false → `t_tie` deleted → its class is empty → **refuted before any partition
or derivative is attempted**, with a precise culprit for error reporting: *"tieMaterial
requires co-occurring sunglassesBrand (branch B1), absent"*. An enumerating engine discovers
the same thing only by exhausting assignments.

The two shape-level conjuncts (`NOT EXTRA ex:rolls { ex:rolls [ex:Robot] }` and the
`OR` of the three tool-shape ANDs) stay at the shape-expression level, exactly as rudof
evaluates them today. Note the redundancy this formulation forces: the tool predicates
appear **twice** (inside `OneOf_B` and again in the AND'd OR), and the agreement between the
two choices is enforced only through the data. The EXTENDS formulation in example B states
the choice once — which is precisely why it is worth supporting.

## 3. Worked example B: `person-extends.shex` → selection alternatives {#3-worked-example-b}

[`examples/shex/person-extends.shex`](../../../examples/shex/person-extends.shex):
`<Person> EXTENDS @<Contact1> EXTENDS @<Tools> CLOSED { ex:badgeNumber xsd:integer }`.

**Step 1 — resolve each parent into *alternatives*** (DNF over *shape atoms*), by structural
recursion:

```
alts(Shape S)        = [ atoms(S) ]        where atoms(S) = {S} ∪ atoms of each shape S extends
alts(Ref i)          = alts(deref(i))
alts(AND(e₁…eₖ))     = cross-product of alts(eᵢ), unioning atom sets / constraint sets
alts(OR(e₁…eₖ))      = concatenation of alts(eᵢ)
alts(NodeConstraint) = [ {nc-constraint} ]
alts(NOT e)          = [ {negated-constraint e} ]
```

Atoms split into **bucket atoms** — `Shape`s whose triple expressions become partition
buckets — and **constraint atoms** — node constraints, negations, and the non-main
conjuncts, evaluated against the node once a partition is found (this is the
`get_main_shape_constraints` division, generalized per-alternative).

Applied to the fixture (`C1`/`C2` = the foaf/schema Contact branches; `X_ρ` = the
`EXTRA ex:rolls { ex:rolls [ρ] }` shapes):

* `alts(Contact1) = alts(Contact)` = **2** alternatives:
  `{C1, Item ; constraints: X_Human, ¬X_Robot}` and `{C2, Item ; …}`
  (Item enters via `C1/C2 EXTENDS @<Item>`).
* `alts(Tools)` = **3** alternatives:
  `{TBoss, Item ; X_Manager}`, `{TGeek, Item ; X_Engineer}`, `{TLabor, Item ; X_Laboror}`.

**Step 2 — selections.** A *selection* σ picks one alternative per parent: 2 × 3 = **6**
selections. Per σ, bucket atoms are **deduplicated** — `Item` arrives through both parents
(a diamond) and must appear **once**, or the two Item buckets would compete for the same
`ex:id`/`ex:rolls` triples and every partition would fail. (rudof's `merge_ancestor_exprs`
already solves this diamond for linear parents; the selection layer must apply the same
dedup per σ.)

**Step 3 — per-selection feasibility.** Each σ is the familiar virtual EachOf:
`Fx(Person-TE) ∧ Fx(bucket₁) ∧ …`. For alice:

| σ | buckets (beyond Person, Item) | fate |
|---|---|---|
| (C1, TBoss) | C1, TBoss | `Fx(t_sunglassesBrand)`: `hi = 0 < 1` → **refuted in O(‖E‖)** |
| (C1, TGeek) | C1, TGeek | feasible → partition search → **conformant** |
| (C1, TLabor) | C1, TLabor | `hi(t_glovesSize) = 0` → refuted |
| (C2, \*) | C2, … | `hi(t_schema:name) = 0` → refuted |

Five of six selections die on a single `F` evaluation without touching the partition
enumerator. The surviving σ's buckets: `Person{badge}`, `C1{foaf:name, foaf:mbox}`,
`Item{id, rolls×2 via +}`, `TGeek{SlideRoolLength, PocketProtectorMaterial}` — one
distribution, verified exactly. Then σ's constraint atoms run against the node:
`X_Human` ✓ (a rolls triple with value `ex:Human`; the Engineer triple is excused by
`EXTRA ex:rolls`), `X_Engineer` ✓, `¬X_Robot` ✓. Conformant, by the same evidence trail a
human would give.

**Why constraint atoms must not become buckets.** alice has *two* rolls triples but *three*
things to say about rolls (Item's `@<RoleCode>+`, `X_Human`'s `[ex:Human]`, `X_Engineer`'s
`[ex:Engineer]`). Under partition semantics each triple feeds exactly one bucket — if the
`X_ρ` shapes were buckets, the two triples could never satisfy three buckets and every
partition would fail on data the spec accepts. They are conjunct *constraints*: evaluated
over the relevant triples without consuming them. rudof today flat-maps every `Shape`
conjunct of a `ShapeAnd` into the same bucket list (`shape_expr.rs` `get_triple_exprs`,
ShapeAnd arm) — the redesign must route non-main conjuncts to the constraint side, as
`get_main_shape_constraints` already does for the shape's own level. (This is also where
Jena had its one-word `putIfAbsent`/`computeIfAbsent` bug that failed all 17 of its
extends/RESTRICTS tests — evaluate constraints against the triples of the right
hierarchy level, and test that path explicitly.)

## 4. Selection-OneOf is not TripleExpr-OneOf {#4-selection-oneof-is-not-tripleexpr-oneof}

The selection layer *is* an exact-mode OneOf over shape atoms — with two deliberate
differences from the `Fx(OneOf)` table row:

1. **No `zero()` on unselected branches.** A TE-OneOf requires the unchosen branches'
   TCs to receive nothing. An unselected ShapeOr branch imposes no such condition — its TCs
   simply *contribute no candidates*. Discipline comes from elsewhere: a triple whose only
   would-be candidates live in unselected branches has an empty candidate set, and the
   forced-assignment rule (or `CLOSED`) fails the selection. Example: add
   `ex:glovesSize "XL"` to alice. Under σ = (C1, TGeek): `glovesSize` is outside σ's
   alphabet → `CLOSED <Person>` remainder → σ fails. Under σ = (C1, TLabor):
   `hi(t_bootSize) = 0` → `F` refutes. All six fail → nonconformant, correctly.
2. **Matchables, `CLOSED`, and `EXTRA` are selection-dependent.** The matchable-predicate
   set is the union of σ's bucket alphabets — *not* the union over all branches.
   rudof's `get_preds_extends` recurses through ShapeOr today and would over-approximate,
   silently weakening `CLOSED` (the glovesSize example above would wrongly conform).
   The closed-remainder check, the `EXTRA` excusal test, and the case-(a)/(b) triple
   filtering in `check_node_shape_extends` all must move inside the per-selection loop.

Everything else — `F`, classes, arc consistency, distribution search, exact verification —
is unchanged inside each selection. That is the whole point: **no new matching engine, one
new choice point.**

## 5. Implementation plan mapped to rudof {#5-implementation-plan}

Ordered so each step lands independently with tests.

1. **`Ref` parents** — *landed*: `SchemaIR::dereference` (cycle-guarded reference
   following) applied in `get_main_shape_constraints`; `ref-parent.shex` now validates OK.
   `shape_expr.rs::get_triple_exprs` already recursed through `Ref`.
2. **`ExtendAlternative` in `shex_ast::ir`** — *landed*:
   `shex_ast/src/ir/extend_alternative.rs` defines
   `ExtendAlternative { bucket_shapes: Vec<ShapeLabelIdx>, constraints: Vec<ShapeLabelIdx> }`
   (constraints are plain shape-expression indexes — node constraints, non-main conjuncts,
   negations — evaluable uniformly by the engine), and
   `SchemaIR::extend_alternatives(&self, idx) -> Vec<ExtendAlternative>` implements the §3
   recursion: path-guarded against cycles, buckets deduplicated by `ShapeLabelIdx` (the
   Item diamond), first bucket-yielding `ShapeAnd` conjunct plays the main role, remaining
   conjuncts attach to every alternative. `shex_ast/tests/extend_alternatives.rs` pins the
   §3 worked example: 2 alternatives for `<Contact1>`, 3 for `<Tools>`, and 6 for
   `<Person>` each with exactly 4 buckets (Item once) and 3 constraints.
3. **Selection loop in `engine.rs::check_node_shape_extends`** — *landed*:
   when any parent yields more than one alternative, `check_node_shape_extends_selections`
   iterates the cartesian product of per-parent alternatives (odometer), merging each
   selection σ with bucket dedup; `check_node_selection` builds σ-local buckets, candidate
   predicates and CLOSED check, applies the σ-local M^∈/M^∉ triple filtering, evaluates σ's
   constraint conjuncts through a direct evaluator (`check_node_constraint_expr` — the
   typing-based `check_node_ref` cannot be used because anonymous conjuncts under a
   previously-unreachable OR are never scheduled by the solver), then runs the existing
   partition/RBE machinery. Parents with single alternatives keep the historical code path
   byte-identical. Two supporting fixes were required: (a) `ShapeExpr::references` now
   recurses through `Ref` targets (cycle-guarded), so value-reference dependencies behind
   `@<B>` indirections and OR branches (eg `ex:rolls @<RoleCode>` inside `<Item>`) are
   scheduled into the typing that `prove`/`dep` precompute; (b) step 1's dereference.
   Verified: `or-parent.shex` conforms via either branch, `person-extends.shex` +
   `person-Engineer.ttl` conforms, and the §6 mutants behave as specified — except the
   stray-`glovesSize` CLOSED mutant, which exposed a **pre-existing** rudof bug — now
   *fixed*: `RdfData::outgoing_arcs_from_list` (sparql_service) discarded the primary
   backend's remainder predicates and returned an empty vector, so CLOSED with an
   undeclared predicate passed even on plain shapes without extends
   (`:C CLOSED { :p . ; :q . }` on `:x :p 1; :q 2; :z 9` validated OK). The remainder is
   now plumbed through from the primary backend (SPARQL endpoints still contribute none,
   as documented there), and all six §6 mutants behave as specified.
4. **Feasibility layer in `rbe`** — *first half landed*: `rbe/src/feasibility.rs`
   implements the §1 tables as `Rbe::feasible(lo, hi)` (exact mode `fx`, iterated mode
   `fi`, `once`, `zero`; `Repeat{m,n}` on groups handled by the sound weakening
   "min ≥ 1 ⇒ sum of ≥ 1 iteration bags"), with unit tests pinning the screw cases
   (Or-branch co-occurrence refutation, exact upper bounds, mandatory symbols, exclusivity
   dissolving under `Plus`, count coupling deliberately ignored).
   `RbeTable::feasible_neighs(values)` derives per-component candidate counts from
   `MatchCond` evaluation (an over-approximation of any partition share, hence sound) and
   both engine paths — the historical `check_node_shape_extends` and the σ-loop's
   `check_node_selection` — refute in front of `partitions_iter` with the new
   `ValidatorError::TripleExprRefuted`, replacing k^n partition enumeration by an O(‖E‖)
   test whenever a bucket is structurally unsatisfiable.
   *Second half also landed*: `shex_validation/src/class_partitions.rs` replaces
   `KPartitionIteratorMultiPredicate` at both engine call sites. Values are grouped into
   classes by eligible bucket set; a depth-first search assigns count compositions per
   class, and after every commitment each bucket is re-tested with `feasible_neighs`
   against its *candidate pool* (values of classes that can still reach it) — pools shrink
   as classes commit elsewhere, giving the cross-bucket propagation the per-bucket guard
   cannot see. Surviving distributions are expanded into concrete partitions (multiset
   permutations per class, odometer across classes) and verified by the derivative matcher
   as before. Its unit tests include the differential contract against the old enumerator:
   no invented partitions, identical valid-partition sets, never more enumeration than the
   Cartesian space, plus upfront refutation and empty-neighbourhood cases.
   (`k_partitions.rs` is retained as the differential oracle.)
5. **Differential testing** — *landed for the guard*: `rbe/tests/feasibility_differential.rs`
   exhaustively checks, over every sub-bag of a 6-value pool and four adversarial table
   patterns (shared-key Or, mandatory pair, coupled Plus, Star mixing with optional
   restricted symbol), that `feasible_neighs = false` implies the derivative matcher
   accepts nothing — refutation soundness against the exact semantics — plus a positive
   test that the guard actually fires on the blowup pattern. Extend the same harness to
   the class-based enumerator when it lands (valid-matching-*set* equality, as below):
   the original recipe was a test that runs
   the pruned and the unpruned enumerators over exhaustive small instances of adversarial
   patterns (`{ :p . {1,2} | :p . + ; :q . }`, the `nPlus1` pattern
   `{ :p .*; (:p .+ | :p .); :p . }`, `(:p .; :q .)+` count coupling, `{0,0}`-branches,
   two-expression conjunctions) and asserts the *sets of valid matchings* are equal.
   In Jena this harness caught, on its first run, a latent soundness bug in the exact
   interval verifier (empty intervals leaking through the OneOf sum) that 2135 conformance
   tests had never touched. rudof's derivative-based `rbe` is a different algorithm and may
   be clean — but the harness is cheap and the class of bug (non-canonical empty/failure
   states flowing through algebraic operations) is algorithm-independent.

## 6. Test plan {#6-test-plan}

* `examples/shex/direct-parent.shex` — regression: stays OK after every step.
* `examples/shex/ref-parent.shex` — OK after step 1.
* `examples/shex/or-parent.shex` — OK after steps 2–3 (`:x :p 1 ; :q 2 .` conforms via `:A1`,
  `:x :r 1 ; :q 2 .` via `:A2`) — *verified*.
* `examples/shex/person-extends.shex` + `examples/shex/person-Engineer.ttl` — the full example, OK
  after steps 2–3. Mutants for each failure mode: add `ex:rolls ex:Robot` (¬X_Robot fails);
  remove `ex:rolls ex:Human` (X_Human fails); add `ex:glovesSize "XL"` (CLOSED under every
  σ — pins selection-dependent matchables, §4.2); replace foaf:name with schema:name only
  (σ flips to C2); remove `ex:badgeNumber` (Person's own bucket fails).
* `examples/shex/person-OneOf.shex` — must keep validating identically; it is the semantic
  oracle for the EXTENDS form on shared data.
* Existing `examples/shex/extends*.shex` and the shexTest suite — unchanged behaviour.
