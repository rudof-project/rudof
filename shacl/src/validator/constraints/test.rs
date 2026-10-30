#[cfg(all(not(target_family = "wasm"), test))]
mod tests {
    use crate::ir::IRSchema;
    use crate::rdf::ShaclParser;
    use crate::validator::RecursionSemantics;
    use crate::validator::ShaclConfig;
    use crate::validator::ShaclValidationMode;
    use crate::validator::processor::{DataValidation, ShaclProcessor};
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_impl::ReaderMode;
    use sparql_service::RdfData;

    #[test]
    fn test_min_exclusive_native() {
        let graph = r#"
prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:MinInclusive a sh:NodeShape ;
  sh:targetClass :Node ;
  sh:property [
    sh:path :p ;
    sh:datatype xsd:double ;
    sh:minInclusive "0.0"^^xsd:double ;
    sh:minCount 1
 ] .

:ok1 a :Node; :p "0"^^xsd:double .
:ok2 a :Node; :p "10.5"^^xsd:double .
:ko1 a :Node; :p "-5.3"^^xsd:double .
:ko2 a :Node; :p "other" .
:ko3 a :Node; :p "other"^^xsd:double .
"#;

        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let mut validator: DataValidation = rdf.clone().into();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let schema_ir: IRSchema = schema.try_into().unwrap();
        let report = validator
            .validate(&schema_ir, &ShaclValidationMode::Native, &ShaclConfig::default())
            .unwrap();
        assert_eq!(report.results().len(), 5);
    }

    fn validate_with_config(graph: &str, config: &ShaclConfig) -> crate::validator::report::ValidationReport {
        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let mut validator: DataValidation = rdf.clone().into();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let schema_ir = IRSchema::compile_with_recursion(&schema, config.recursion_semantics()).unwrap();
        validator
            .validate(&schema_ir, &ShaclValidationMode::Native, config)
            .unwrap()
    }

    const MIN_COUNT_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
  sh:targetClass :Node ;
  sh:property [
    sh:path :p ;
    sh:minCount 1
 ] .

:ok a :Node; :p "v" .
:ko a :Node .
"#;

    #[test]
    fn default_config_keeps_errors_but_not_evidence() {
        let report = validate_with_config(MIN_COUNT_GRAPH, &ShaclConfig::default());
        assert!(!report.conforms());
        assert_eq!(report.results().len(), 1);
        assert!(report.evidences().is_empty());
    }

    #[test]
    fn no_errors_mode_still_reports_conforms_correctly() {
        let config = ShaclConfig::default().with_store_errors(false);
        let report = validate_with_config(MIN_COUNT_GRAPH, &config);
        assert!(!report.conforms());
        assert!(report.results().is_empty());
        assert!(report.evidences().is_empty());
    }

    #[test]
    fn evidence_mode_records_a_pass_for_the_conforming_node() {
        let config = ShaclConfig::default().with_store_evidences(true);
        let report = validate_with_config(MIN_COUNT_GRAPH, &config);
        assert_eq!(report.results().len(), 1);
        assert!(
            report
                .evidences()
                .iter()
                .any(|e| e.focus_node().to_string().contains("ok"))
        );
    }

    #[test]
    fn shape_level_evidence_is_recorded_even_when_no_sub_constraint_evidence_fires() {
        // `:e` has no `:knows` value at all, so the Node component's
        // per-value iteration never runs and produces no evidence of its
        // own — but `:e` still fully conforms to `:PersonShape` (the
        // property is optional, no sh:minCount), and that should be
        // visible as its own evidence, not silently absent.
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:property [ sh:path :knows ; sh:node :PersonShape ] .

:e a :Person .
"#;
        let config = ShaclConfig::default().with_store_evidences(true);
        let report = validate_with_config(graph, &config);
        assert!(report.conforms());
        assert!(
            report
                .evidences()
                .iter()
                .any(|e| e.focus_node().to_string().contains("e")
                    && e.constraint_component().to_string().contains("PersonShape")),
            "{:?}",
            report.evidences()
        );
    }

    #[test]
    fn evidence_shapes_only_drops_component_level_evidence_but_keeps_shape_level() {
        let full_config = ShaclConfig::default().with_store_evidences(true);
        let full = validate_with_config(MIN_COUNT_GRAPH, &full_config);
        // Sanity check: without the filter, both granularities are present.
        assert!(full.evidences().iter().any(|e| e.is_shape_level()));
        assert!(full.evidences().iter().any(|e| !e.is_shape_level()));

        let shapes_only_config = ShaclConfig::default()
            .with_store_evidences(true)
            .with_evidence_shapes_only(true);
        let shapes_only = validate_with_config(MIN_COUNT_GRAPH, &shapes_only_config);
        assert!(!shapes_only.evidences().is_empty());
        assert!(
            shapes_only.evidences().iter().all(|e| e.is_shape_level()),
            "{:?}",
            shapes_only.evidences()
        );
        // Violations are untouched by this flag.
        assert_eq!(shapes_only.results().len(), full.results().len());
    }

    #[test]
    fn errors_and_evidences_mode_records_both() {
        let config = ShaclConfig::default()
            .with_store_errors(true)
            .with_store_evidences(true);
        let report = validate_with_config(MIN_COUNT_GRAPH, &config);
        assert_eq!(report.results().len(), 1);
        assert!(!report.evidences().is_empty());
    }

    const AND_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:HasAge a sh:NodeShape ; sh:property [ sh:path :age ; sh:datatype xsd:integer ] .
:HasName a sh:NodeShape ; sh:property [ sh:path :name ; sh:minCount 1 ] .
:S a sh:NodeShape ;
  sh:targetNode :ok , :ko ;
  sh:and ( :HasAge :HasName ) .

:ok :age "5"^^xsd:integer ; :name "n" .
:ko :age "5"^^xsd:integer .
"#;

    #[test]
    fn and_combinator_emits_evidence_when_all_branches_conform() {
        let config = ShaclConfig::default().with_store_evidences(true);
        let report = validate_with_config(AND_GRAPH, &config);
        assert!(!report.conforms());
        assert!(
            report
                .evidences()
                .iter()
                .any(|e| e.focus_node().to_string().contains("ok"))
        );
        assert!(
            !report
                .evidences()
                .iter()
                .any(|e| e.focus_node().to_string().contains("ko"))
        );
    }

    const CLOSED_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
  sh:targetNode :ok , :ko ;
  sh:closed true ;
  sh:property [ sh:path :p ] .

:ok :p "v" .
:ko :p "v" ; :extra "unexpected" .
"#;

    #[test]
    fn closed_emits_one_evidence_per_conforming_focus_node() {
        let config = ShaclConfig::default().with_store_evidences(true);
        let report = validate_with_config(CLOSED_GRAPH, &config);
        assert!(!report.conforms());
        assert!(
            report
                .evidences()
                .iter()
                .any(|e| e.focus_node().to_string().contains("ok"))
        );
        assert!(
            !report
                .evidences()
                .iter()
                .any(|e| e.focus_node().to_string().contains("ko"))
        );
    }

    /// A schema with a purely positive, mutually-recursive pair of shapes
    /// (`:A` depends on `:B`, `:B` depends on `:A`), and data forming a
    /// 2-cycle with no independent base case: `:n1 :next :n2 :next :n1`.
    const MUTUAL_CYCLE_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:A a sh:NodeShape ;
  sh:targetNode :n1 ;
  sh:property [ sh:path :next ; sh:node :B ] .

:B a sh:NodeShape ;
  sh:property [ sh:path :next ; sh:node :A ] .

:n1 :next :n2 .
:n2 :next :n1 .
"#;

    #[test]
    fn none_mode_rejects_the_cyclic_schema_outright() {
        // Default: recursive shapes are opt-in, so a cyclic schema fails to
        // compile at all rather than being validated.
        let rdf = RdfData::from_str(MUTUAL_CYCLE_GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let result = IRSchema::compile_with_recursion(&schema, RecursionSemantics::None);
        assert!(result.is_err(), "expected a cyclic schema to be rejected under None");
    }

    #[test]
    fn cycle_error_shows_the_shapes_with_prefixed_names() {
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
   sh:property [ sh:path :p ; sh:qualifiedValueShape :S ; sh:qualifiedMinCount 1 ] .
"#;
        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let error = IRSchema::compile_with_recursion(&schema, RecursionSemantics::None)
            .expect_err("expected a cyclic schema to be rejected under None")
            .to_string();
        assert!(
            error.starts_with("Dependency graph has cycles: [:S, _:"),
            "unexpected message: {error}"
        );
        assert!(
            error.ends_with(']') && !error.contains("Iri {"),
            "unexpected message: {error}"
        );
    }

    #[test]
    fn in_message_shows_the_values_with_prefixed_names() {
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
   sh:targetNode :x ;
   sh:property [ sh:path :p ; sh:in ( :a "b" 3 ) ] .

:x :p :y .
"#;
        let report = validate_with_config(graph, &ShaclConfig::default());
        let message = report.results()[0].message().get(None).unwrap();
        assert_eq!(
            message,
            r#"In constraint not satisfied. Expected one of [:a, "b", "3"^^xsd:integer]"#
        );
    }

    #[test]
    fn lfp_mode_rejects_an_ungrounded_cycle() {
        // Least fixpoint: nothing in the cycle can be justified without
        // assuming itself, so it's assumed non-conformant.
        let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);
        let report = validate_with_config(MUTUAL_CYCLE_GRAPH, &config);
        assert!(!report.conforms());
    }

    #[test]
    fn gfp_mode_accepts_a_self_consistent_cycle() {
        // Greatest fixpoint: the assignment "everyone in the cycle conforms"
        // is self-consistent, so it's accepted.
        let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Gfp);
        let report = validate_with_config(MUTUAL_CYCLE_GRAPH, &config);
        assert!(report.conforms());
    }

    #[test]
    fn self_referencing_shape_with_a_terminating_chain_conforms_under_both_semantics() {
        // `:ListShape` references itself directly (a 1-cycle in the shapes
        // graph — this used to be rejected outright at compile time even
        // though no *data* cycle exists here: the chain n1 -> n2 -> n3
        // terminates because n3 has no `:next`, so `sh:minCount 0` makes the
        // node-check on an absent value node vacuous). Recursive-reference
        // cutting should never even trigger, so both semantics agree.
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:ListShape a sh:NodeShape ;
  sh:targetClass :ListNode ;
  sh:property [ sh:path :next ; sh:minCount 0 ; sh:node :ListShape ] .

:n1 a :ListNode ; :next :n2 .
:n2 a :ListNode ; :next :n3 .
:n3 a :ListNode .
"#;
        let lfp_config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);
        let lfp = validate_with_config(graph, &lfp_config);
        assert!(lfp.conforms(), "{:?}", lfp.results());

        let gfp_config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Gfp);
        let gfp = validate_with_config(graph, &gfp_config);
        assert!(gfp.conforms(), "{:?}", gfp.results());
    }

    /// A terminating chain and a separate pure cycle against the same
    /// self-recursive shape, both reachable in one `:targetClass` batch —
    /// this combination previously triggered a regression (found during
    /// development of this feature, before it ever shipped) where batching
    /// multiple independent `:targetClass` instances together made sibling
    /// targets falsely look like ancestors of each other, cutting the chain
    /// nodes as if they were cyclic. Every chain node must conform; every
    /// cycle node must not (LFP).
    ///
    /// Lengths are kept modest deliberately: this recursive-descent
    /// validator's call-stack depth scales with recursion depth regardless
    /// of whether it's cyclic — a pre-existing, orthogonal characteristic of
    /// the architecture (the same is true of e.g. deeply nested `sh:and`
    /// trees) — so this test is about correctness of cycle-cutting, not a
    /// claim about maximum supported chain length.
    #[test]
    fn chain_and_cycle_in_the_same_batch_terminate_correctly() {
        let mut graph = String::from(
            r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:ListShape a sh:NodeShape ;
  sh:targetClass :ListNode ;
  sh:property [ sh:path :next ; sh:minCount 0 ; sh:node :ListShape ] .

"#,
        );

        const CHAIN_LEN: usize = 15;
        for i in 0..CHAIN_LEN {
            graph.push_str(&format!(":chain{i} a :ListNode ; :next :chain{} .\n", i + 1));
        }
        graph.push_str(&format!(":chain{CHAIN_LEN} a :ListNode .\n"));

        const CYCLE_LEN: usize = 15;
        for i in 0..CYCLE_LEN {
            graph.push_str(&format!(
                ":cycle{i} a :ListNode ; :next :cycle{} .\n",
                (i + 1) % CYCLE_LEN
            ));
        }

        let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);
        let report = validate_with_config(&graph, &config);
        assert!(
            report
                .results()
                .iter()
                .all(|r| r.focus_node().to_string().contains("cycle")),
            "only cycle nodes should violate: {:?}",
            report.results()
        );
        assert_eq!(report.results().len(), CYCLE_LEN, "{:?}", report.results());
    }

    /// `:PersonShape` is recursive (via `:knowsShape`), and also carries a
    /// `sh:not :RobotShape` constraint. `:RobotShape` is not itself
    /// recursive and doesn't depend on anything that is, so the negation is
    /// stratified: it can be settled independently of the recursive part.
    const STRATIFIED_NEGATION_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:not :RobotShape ;
  sh:property [ sh:path :knows ; sh:node :PersonShape ] .

:RobotShape a sh:NodeShape ;
  sh:property [ sh:path :isRobot ; sh:hasValue true ] .

:human a :Person ; :knows :human .
:robot a :Person ; :isRobot true ; :knows :robot .
"#;

    #[test]
    fn stratified_negation_compiles_under_both_recursion_semantics() {
        for semantics in [RecursionSemantics::Lfp, RecursionSemantics::Gfp] {
            let rdf =
                RdfData::from_str(STRATIFIED_NEGATION_GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let schema = ShaclParser::new(rdf).parse().unwrap();
            let result = IRSchema::compile_with_recursion(&schema, semantics);
            assert!(
                result.is_ok(),
                "expected a stratified cycle to compile under {semantics:?}"
            );
        }
    }

    #[test]
    fn stratified_negation_rejects_the_negated_node_independently_of_the_recursion_semantics() {
        // The `sh:not` check doesn't depend on the cycle at all, so it gives
        // the same answer for `:robot` under both LFP and GFP: the
        // negation fails regardless of how the *unrelated* recursive part
        // (which `:robot` also happens to sit in) is resolved.
        for semantics in [RecursionSemantics::Lfp, RecursionSemantics::Gfp] {
            let config = ShaclConfig::default().with_recursion_semantics(semantics);
            let report = validate_with_config(STRATIFIED_NEGATION_GRAPH, &config);
            assert!(
                report
                    .results()
                    .iter()
                    .any(|r| r.focus_node().to_string().contains("robot")),
                "expected :robot to violate sh:not under {semantics:?}: {:?}",
                report.results()
            );
        }
    }

    #[test]
    fn stratified_negation_agrees_with_recursion_semantics_for_the_cycle_itself() {
        // `:human`'s conformance hinges purely on the self-cycle (it isn't a
        // robot, so `sh:not` never fires for it) — LFP/GFP disagree
        // on it exactly as they would without the negation present at all.
        let lfp_config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);
        let lfp = validate_with_config(STRATIFIED_NEGATION_GRAPH, &lfp_config);
        assert!(
            lfp.results()
                .iter()
                .any(|r| r.focus_node().to_string().contains("human")),
            "expected :human to be rejected under lfp: {:?}",
            lfp.results()
        );

        let gfp_config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Gfp);
        let gfp = validate_with_config(STRATIFIED_NEGATION_GRAPH, &gfp_config);
        assert!(
            gfp.results()
                .iter()
                .all(|r| !r.focus_node().to_string().contains("human")),
            "expected :human to conform under gfp: {:?}",
            gfp.results()
        );
    }

    #[test]
    fn recursion_kinds_reports_stratified_for_the_stratified_negation_schema() {
        use crate::ir::dg::ShapeRecursionKind;

        let rdf = RdfData::from_str(STRATIFIED_NEGATION_GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let schema_ir = IRSchema::compile_with_recursion(&schema, RecursionSemantics::Gfp).unwrap();

        let kinds = schema_ir.recursion_kinds();
        let person_kind = kinds
            .iter()
            .find(|(label, _)| label.to_string().contains("PersonShape"))
            .map(|(_, kind)| *kind)
            .expect("PersonShape should appear in the dependency graph");
        assert_eq!(person_kind, ShapeRecursionKind::Stratified);

        let robot_kind = kinds
            .iter()
            .find(|(label, _)| label.to_string().contains("RobotShape"))
            .map(|(_, kind)| *kind);
        assert_ne!(robot_kind, Some(ShapeRecursionKind::Stratified));
    }

    /// `:PersonShape` negates `:OtherRecursiveShape`, which is itself
    /// recursive (via `:friendShape`) and entirely unrelated to
    /// `:PersonShape`'s own cycle. There is no order in which both
    /// fixpoints can be soundly resolved, so this is rejected outright —
    /// under every recursion semantics, including `gfp`.
    const NON_STRATIFIED_NEGATION_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:not :OtherRecursiveShape ;
  sh:property [ sh:path :knows ; sh:node :PersonShape ] .

:OtherRecursiveShape a sh:NodeShape ;
  sh:property [ sh:path :friend ; sh:node :OtherRecursiveShape ] .

:a a :Person ; :knows :a .
"#;

    #[test]
    fn negation_of_an_unrelated_recursive_shape_is_rejected_under_every_semantics() {
        for semantics in [RecursionSemantics::Lfp, RecursionSemantics::Gfp] {
            let rdf = RdfData::from_str(
                NON_STRATIFIED_NEGATION_GRAPH,
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
            )
            .unwrap();
            let schema = ShaclParser::new(rdf).parse().unwrap();
            let result = IRSchema::compile_with_recursion(&schema, semantics);
            assert!(
                result.is_err(),
                "expected negation of an unrelated recursive shape to be rejected under {semantics:?}"
            );
        }
    }

    /// A SPARQL-based constraint with `{?varName}` and `{$varName}` blocks in `sh:message`.
    /// The query selects `$this`, `?value` and `?age`. It does not select `?missing`.
    const SPARQL_MESSAGE_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix : <http://example.org/>

:PersonShape a sh:NodeShape ;
  sh:targetClass :Person ;
  sh:sparql [
    sh:message "{$this} has name {?value} and age {?age}. Unknown: {?missing}." ,
               "{$this} tiene nombre {?value}"@es ;
    sh:select """
      SELECT $this ?value ?age WHERE {
        $this <http://example.org/name> ?value .
        $this <http://example.org/age> ?age .
        FILTER (?age < 18)
      }
    """
  ] .

:alice a :Person ; :name "Alice" ; :age 30 .
:bob a :Person ; :name "Bob"@en ; :age 12 .
"#;

    fn sparql_message_report(mode: ShaclValidationMode) -> crate::validator::report::ValidationReport {
        let rdf = RdfData::from_str(SPARQL_MESSAGE_GRAPH, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let mut validator: DataValidation = rdf.clone().into();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let schema_ir: IRSchema = schema.try_into().unwrap();
        validator.validate(&schema_ir, &mode, &ShaclConfig::default()).unwrap()
    }

    #[test]
    fn sparql_constraint_message_interpolates_result_variables() {
        use rudof_rdf::rdf_core::term::literal::Lang;

        for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
            let report = sparql_message_report(mode);
            assert_eq!(report.results().len(), 1, "mode {mode:?}");

            let message = report.results()[0].message();
            assert_eq!(
                message.get(None).map(String::as_str),
                Some("http://example.org/bob has name Bob and age 12. Unknown: {?missing}."),
                "mode {mode:?}"
            );
            assert_eq!(
                message.get(Some(&Lang::new("es").unwrap())).map(String::as_str),
                Some("http://example.org/bob tiene nombre Bob"),
                "mode {mode:?}"
            );
        }
    }

    /// Issue #897: `sh:sourceShape` of a failed `sh:node` is the shape that
    /// declares the `sh:node` (here, the property shape of `:S`), as the
    /// SHACL spec requires. The results that made the value node fail `:T`
    /// are attached as `sh:detail`.
    const NODE_DETAIL_GRAPH: &str = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
  sh:targetNode :a ;
  sh:property [ sh:path :p ; sh:node :T ] .

:T a sh:NodeShape ;
  sh:property [ sh:path :p ; sh:hasValue :c ] .

:a :p :b .
:b :p :a .
"#;

    #[test]
    fn node_violation_carries_the_inner_results_as_details() {
        use rudof_rdf::rdf_core::term::Object;

        let report = validate_with_config(NODE_DETAIL_GRAPH, &ShaclConfig::default());
        assert_eq!(report.results().len(), 1, "{:?}", report.results());

        let result = &report.results()[0];
        assert_eq!(result.focus_node().to_string(), "http://example.org/a");
        assert!(
            result
                .constraint_component()
                .to_string()
                .contains("NodeConstraintComponent")
        );
        assert!(matches!(result.source(), Some(Object::BlankNode(_))), "{result}");
        let message = result.message().get(None).unwrap();
        assert!(message.contains("http://example.org/T"), "{message}");
        assert!(!message.contains('\n'), "{message}");

        let [detail] = result.details() else {
            panic!("expected exactly one detail: {result}");
        };
        assert_eq!(detail.focus_node().to_string(), "http://example.org/b");
        assert!(
            detail
                .constraint_component()
                .to_string()
                .contains("HasValueConstraintComponent")
        );
        assert!(matches!(detail.source(), Some(Object::BlankNode(_))), "{detail}");
        assert_ne!(detail.source(), result.source());
    }

    #[test]
    fn details_round_trip_through_rdf() {
        use crate::validator::report::ValidationReport;
        use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShaclVocab};
        use rudof_rdf::rdf_core::{BuildRDF, NeighsRDF, Rdf};
        use rudof_rdf::rdf_impl::OxigraphInMemory;

        let report = validate_with_config(NODE_DETAIL_GRAPH, &ShaclConfig::default());
        let mut graph = OxigraphInMemory::empty();
        report.to_rdf(&mut graph).unwrap();

        let rdf_type: <OxigraphInMemory as Rdf>::IRI = RdfVocab::rdf_type().into();
        let report_type: <OxigraphInMemory as Rdf>::Term = ShaclVocab::sh_validation_report().into();
        let report_node = graph
            .subjects_for(&rdf_type, &report_type)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        let parsed = ValidationReport::parse(&mut graph, report_node).unwrap();

        let [result] = parsed.results().as_slice() else {
            panic!("expected one result: {:?}", parsed.results());
        };
        assert_eq!(result.details().len(), 1);
        assert_eq!(result.details(), report.results()[0].details());
        assert_eq!(parsed.clone().without_details().results()[0].details().len(), 0);
    }

    #[test]
    fn or_violation_lists_why_every_alternative_failed() {
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
  sh:targetNode :a ;
  sh:or ( [ sh:property [ sh:path :p ; sh:minCount 1 ] ]
          [ sh:property [ sh:path :q ; sh:minCount 1 ] ] ) .

:a :r 1 .
"#;
        let report = validate_with_config(graph, &ShaclConfig::default());
        let [result] = report.results().as_slice() else {
            panic!("expected one result: {:?}", report.results());
        };
        assert!(
            result
                .constraint_component()
                .to_string()
                .contains("OrConstraintComponent")
        );
        assert_eq!(result.details().len(), 2, "{result}");
        assert!(result.details().iter().all(|d| {
            d.constraint_component()
                .to_string()
                .contains("MinCountConstraintComponent")
        }));
    }

    #[test]
    fn inverse_of_an_alternative_path() {
        // Issue #920
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix : <http://example.org/>

:OptionShape a sh:NodeShape ;
  sh:targetClass :Option ;
  sh:property [
    sh:path [ sh:inversePath [ sh:alternativePath ( :considers :chooses ) ] ] ;
    sh:minCount 1
  ] .

:option1 a :Option .
:iteration1 :considers :option1 .
:option2 a :Option .
"#;
        let report = validate_with_config(graph, &ShaclConfig::default());
        let [result] = report.results().as_slice() else {
            panic!("expected one result: {:?}", report.results());
        };
        assert_eq!(result.focus_node().to_string(), "http://example.org/option2");
    }
}
