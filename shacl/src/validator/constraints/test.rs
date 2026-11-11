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

    #[test]
    fn evidences_are_kept_and_counted_only_for_a_report_with_evidences() {
        // Each of the 10 conforming targets has a sh:minCount evidence and a shape evidence
        let mut graph = format!("{PREFIXES}:S sh:property [ sh:path :p ; sh:minCount 1 ] .\n");
        for i in 0..10 {
            graph.push_str(&format!(":S sh:targetNode :n{i} .\n:n{i} :p 1 .\n"));
        }
        let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema_ir: IRSchema = ShaclParser::new(rdf.clone()).parse().unwrap().try_into().unwrap();
        let config = ShaclConfig::default().with_max_results(Some(5));
        for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
            let mut validator: DataValidation = rdf.clone().into();
            let report = validator.validate(&schema_ir, &mode, &config).unwrap();
            assert!(report.conforms() && report.evidences().is_empty());
            let result = validator.validate(&schema_ir, &mode, &config.clone().with_store_evidences(true));
            assert!(
                matches!(result, Err(crate::error::ValidationError::TooManyResults(_, 5))),
                "{mode:?}: {result:?}"
            );
        }
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
            error.starts_with("Dependency graph has cycles: [") && error.contains(":S") && error.contains("_:"),
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

    #[test]
    fn typed_literals_keep_their_lexical_form() {
        let graph = r#"
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix : <http://example.org/>

:S a sh:NodeShape ;
  sh:targetClass :Node ;
  sh:property [ sh:path :p ; sh:hasValue "01"^^xsd:long ] ;
  sh:property [ sh:path :q ; sh:minInclusive "3"^^xsd:long ] .

:ok a :Node ; :p "01"^^xsd:long ; :q 5 .
:ko a :Node ; :p "1"^^xsd:long ; :q "02"^^xsd:long .
"#;
        let report = validate_with_config(graph, &ShaclConfig::default());
        assert_eq!(report.results().len(), 2, "{:?}", report.results());
        assert!(
            report
                .results()
                .iter()
                .all(|r| r.focus_node().to_string() == "http://example.org/ko")
        );
        let values: Vec<String> = report
            .results()
            .iter()
            .filter_map(|r| r.value())
            .map(|v| v.to_string())
            .collect();
        assert_eq!(values, ["\"02\"^^xsd:long"]);
    }

    const DEPTH: usize = 10_000;

    /// Runs `f` on a 256 KiB thread, far less stack than `DEPTH` levels of recursion take.
    fn on_small_stack(f: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(f)
            .unwrap()
            .join()
            .unwrap();
    }

    const PREFIXES: &str = "prefix sh: <http://www.w3.org/ns/shacl#>\nprefix : <http://example.org/>\n";

    #[test]
    fn deeply_nested_property_shapes_in_a_cycle() {
        on_small_stack(|| {
            let mut graph = format!("{PREFIXES}:S sh:targetNode :a ; sh:property _:p0 .\n:a :p :a .\n");
            for i in 0..DEPTH {
                graph.push_str(&format!("_:p{i} sh:path :p ; sh:property _:p{} .\n", i + 1));
            }
            graph.push_str(&format!("_:p{DEPTH} sh:path :p ; sh:node :S .\n"));
            let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);

            let report = validate_with_config(&graph, &config);
            assert_eq!(report.results().len(), 1, "{:?}", report.results());
        });
    }

    #[test]
    fn deeply_nested_not() {
        on_small_stack(|| {
            let mut graph = format!("{PREFIXES}:S sh:targetNode :a ; sh:not _:n0 .\n");
            for i in 0..DEPTH {
                graph.push_str(&format!("_:n{i} sh:not _:n{} .\n", i + 1));
            }
            // An odd number of sh:not, so :S is sh:not [ sh:class :C ]
            graph.push_str(&format!("_:n{DEPTH} sh:class :C .\n"));

            assert!(validate_with_config(&graph, &ShaclConfig::default()).conforms());
        });
    }

    #[test]
    fn long_cycle_of_node_shapes_is_rejected() {
        on_small_stack(|| {
            let mut graph = PREFIXES.to_string();
            for i in 0..DEPTH {
                graph.push_str(&format!(":S{i} sh:node :S{} .\n", i + 1));
            }
            graph.push_str(&format!(":S{DEPTH} sh:node :S0 .\n"));
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let schema = ShaclParser::new(rdf).parse().unwrap();

            assert!(IRSchema::compile_with_recursion(&schema, RecursionSemantics::None).is_err());
        });
    }

    #[test]
    fn long_chain_of_node_shapes_keeps_one_level_of_details() {
        on_small_stack(|| {
            let mut graph = format!("{PREFIXES}:S0 sh:targetNode :a .\n");
            for i in 0..DEPTH {
                graph.push_str(&format!(":S{i} sh:node :S{} .\n", i + 1));
            }
            graph.push_str(&format!(":S{DEPTH} sh:class :C .\n"));

            let report = validate_with_config(&graph, &ShaclConfig::default());
            let [result] = report.results().as_slice() else {
                panic!("expected one result: {:?}", report.results());
            };
            let [detail] = result.details() else {
                panic!("expected exactly one detail: {result}");
            };
            assert!(detail.details().is_empty(), "{detail}");
        });
    }

    #[test]
    fn recursive_shape_over_a_long_data_chain() {
        use rudof_rdf::rdf_core::BuildRDF;
        use rudof_rdf::rdf_impl::OxigraphInMemory;

        on_small_stack(|| {
            let mut graph = format!(
                "{PREFIXES}:S sh:targetNode :n0 ;
                   sh:property [ sh:path :next ; sh:node :S ] ;
                   sh:property [ sh:path :label ; sh:minCount 1 ] .\n"
            );
            for i in 0..DEPTH {
                graph.push_str(&format!(":n{i} :label {i} ; :next :n{} .\n", i + 1));
            }
            let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);

            // The last node has no label, so every node before it fails too
            let report = validate_with_config(&graph, &config);
            let [result] = report.results().as_slice() else {
                panic!("expected one result: {:?}", report.results());
            };
            assert_eq!(result.focus_node().to_string(), "http://example.org/n0");
            let [detail] = result.details() else {
                panic!("expected exactly one detail: {result}");
            };
            assert!(detail.details().is_empty(), "{detail}");
            report.to_rdf(&mut OxigraphInMemory::empty()).unwrap();

            graph.push_str(&format!(":n{DEPTH} :label {DEPTH} .\n"));
            let report = validate_with_config(&graph, &config);
            assert!(report.conforms(), "{:?}", report.results());
        });
    }

    #[test]
    fn recursive_property_shape_over_a_data_cycle_with_fan_out() {
        on_small_stack(|| {
            let graph = format!(
                "{PREFIXES}:S sh:targetNode :a ; sh:property _:p .
                 _:p sh:path :next ; sh:property _:p .
                 :a :next :a, :b .
                 :b :next :a, :b .\n"
            );

            // Under LFP each cut reference is a violation: _:p reaches :a again twice and :b once
            let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);
            let report = validate_with_config(&graph, &config);
            let mut focus_nodes: Vec<_> = report.results().iter().map(|r| r.focus_node().to_string()).collect();
            focus_nodes.sort();
            assert_eq!(
                focus_nodes,
                ["http://example.org/a", "http://example.org/a", "http://example.org/b"]
            );

            let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Gfp);
            let report = validate_with_config(&graph, &config);
            assert!(report.conforms(), "{:?}", report.results());
        });
    }

    #[test]
    fn recursive_reifier_shape_over_several_reifiers() {
        on_small_stack(|| {
            // Each :x triple has the reifiers :r1 and :r2, which have :x triples of their own
            let graph = format!(
                "{PREFIXES}:S sh:targetNode :a ; sh:property _:q .
                 _:q sh:path :x ; sh:reifierShape _:q .
                 :a :x :c ~ :r1 ~ :r2 .
                 :r1 :x :c ~ :r1 ~ :r2 .
                 :r2 :x :c ~ :r1 ~ :r2 .\n"
            );

            // _:q is its own reifier shape, so it is recursive
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let schema = ShaclParser::new(rdf).parse().unwrap();
            assert!(IRSchema::compile_with_recursion(&schema, RecursionSemantics::None).is_err());
            let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Lfp);
            assert!(!validate_with_config(&graph, &config).conforms());
            let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Gfp);
            let report = validate_with_config(&graph, &config);
            assert!(report.conforms(), "{:?}", report.results());
        });
    }

    #[test]
    fn error_inside_not_and_xone_or_qualified_value_shape_is_an_error() {
        // :T conforms on :a with 9 evidences, so it fails with an error under max_results 5,
        // which the shapes around it stay under. Evidences count only when they are stored
        let t = ":T sh:property [ sh:path :p ; sh:nodeKind sh:IRI ] .\n:U sh:nodeKind sh:IRI .\n\
                 :a :p :b1, :b2, :b3, :b4, :b5, :b6, :b7, :b8, :b9 ; :r :a .\n";
        let config = ShaclConfig::default()
            .with_max_results(Some(5))
            .with_store_evidences(true);
        for component in [
            "sh:not :T",
            "sh:and ( :T )",
            "sh:xone ( :T )",
            "sh:property [ sh:path :r ; sh:qualifiedValueShape :T ; sh:qualifiedMinCount 1 ]",
            // :T is validated only as the sibling of :U
            "sh:property [ sh:path :r ; sh:qualifiedValueShape :U ; sh:qualifiedMinCount 1 ; sh:qualifiedValueShapesDisjoint true ], \
             [ sh:path :s ; sh:qualifiedValueShape :T ; sh:qualifiedMinCount 0 ]",
        ] {
            let graph = format!("{PREFIXES}:S sh:targetNode :a ; {component} .\n{t}");
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let mut validator: DataValidation = rdf.clone().into();
            let schema_ir: IRSchema = ShaclParser::new(rdf).parse().unwrap().try_into().unwrap();
            let result = validator.validate(&schema_ir, &ShaclValidationMode::Native, &config);
            assert!(
                matches!(result, Err(crate::error::ValidationError::TooManyResults(..))),
                "{component}: {result:?}"
            );
        }
    }

    #[test]
    fn literal_focus_node_of_a_reifier_shape_has_no_reifiers() {
        // "Bob" is the subject of no triple, so the shape under sh:not conforms and sh:not fails
        let graph = format!(
            "{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path :p ; sh:not [ sh:path :q ; sh:reifierShape :R ] ] .\n\
             :R sh:class :C .\n:a :p \"Bob\" .\n"
        );
        let report = try_validate(&graph).unwrap();
        assert_eq!(report.results().len(), 1, "{:?}", report.results());
    }

    #[test]
    fn long_chain_of_owl_imports() {
        on_small_stack(|| {
            let mut graph = format!(
                "{PREFIXES}prefix owl: <http://www.w3.org/2002/07/owl#>\n\
                 :S sh:targetNode :a ; sh:sparql [ sh:select \"SELECT $this WHERE {{ }}\" ; sh:prefixes :o0 ] .\n"
            );
            for i in 0..DEPTH {
                graph.push_str(&format!(":o{i} owl:imports :o{} .\n", i + 1));
            }
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

            assert!(ShaclParser::new(rdf).parse().is_ok());
        });
    }

    #[test]
    fn cyclic_detail_in_a_report_is_read_one_level_deep() {
        on_small_stack(|| {
            let graph = format!(
                "{PREFIXES}:r sh:result _:x .\n\
                 _:x sh:focusNode :a ; sh:resultSeverity sh:Violation ;\n\
                 sh:sourceConstraintComponent sh:NodeConstraintComponent ; sh:detail _:x .\n"
            );
            let mut rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let report_node = rudof_rdf::rdf_core::term::Object::iri(rudof_iri::iri!("http://example.org/r"));
            let report = crate::validator::report::ValidationReport::parse(&mut rdf, report_node.into()).unwrap();

            let [result] = report.results().as_slice() else {
                panic!("expected one result: {:?}", report.results());
            };
            let [detail] = result.details() else {
                panic!("expected one detail: {:?}", result.details());
            };
            assert!(detail.details().is_empty());
        });
    }

    const SHNEX: &str = "prefix shnex: <http://www.w3.org/ns/shacl-node-expr#>\n";

    /// Parses, compiles and validates `graph`, returning the first error as a string.
    fn try_validate(graph: &str) -> Result<crate::validator::report::ValidationReport, String> {
        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let mut validator: DataValidation = rdf.clone().into();
        let schema = ShaclParser::new(rdf).parse().map_err(|e| e.to_string())?;
        let schema_ir =
            IRSchema::compile_with_recursion(&schema, RecursionSemantics::default()).map_err(|e| e.to_string())?;
        validator
            .validate(&schema_ir, &ShaclValidationMode::Native, &ShaclConfig::default())
            .map_err(|e| e.to_string())
    }

    #[test]
    fn cyclic_deep_and_shared_node_expressions_are_errors() {
        on_small_stack(|| {
            let shared: String = (0..40)
                .map(|i| format!("_:e{i} shnex:concat ( _:e{j} _:e{j} ) .\n", j = i + 1))
                .collect();
            let deep: String = (0..DEPTH)
                .map(|i| format!("_:e{i} shnex:exists _:e{} .\n", i + 1))
                .collect();
            for (expr, error) in [
                ("_:e0 shnex:distinct _:e0 .\n".to_string(), "cyclic node expression"),
                (
                    "_:e0 shnex:count _:e1 .\n_:e1 shnex:if _:e0 ; shnex:then 1 .\n".to_string(),
                    "cyclic node expression",
                ),
                (deep, "cyclic node expression"),
                (shared, "steps plus"),
            ] {
                let graph = format!("{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:deactivated _:e0 .\n{expr}");
                let result = try_validate(&graph).map(|_| ());
                assert!(matches!(&result, Err(e) if e.contains(error)), "{expr}: {result:?}");
            }
        });
    }

    #[test]
    fn shapes_share_one_budget_for_their_node_expressions() {
        // _:e0 nests 10 shared shnex:concat, 2047 expressions, and 2000 shapes parse it again each
        let mut graph = format!("{PREFIXES}{SHNEX}_:e10 shnex:pathValues :p .\n");
        for i in 0..10 {
            graph.push_str(&format!("_:e{i} shnex:concat ( _:e{j} _:e{j} ) .\n", j = i + 1));
        }
        for i in 0..2000 {
            graph.push_str(&format!(":S{i} sh:targetNode :a ; sh:deactivated _:e0 .\n"));
        }

        let result = try_validate(&graph).map(|_| ());
        assert!(matches!(&result, Err(e) if e.contains("steps plus")), "{result:?}");
    }

    #[test]
    fn node_expression_at_the_depth_limit_is_validated_on_a_small_stack() {
        on_small_stack(|| {
            let mut graph =
                format!("{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:class :C ; sh:deactivated _:e0 .\n:a :p :b .\n");
            for i in 0..61 {
                graph.push_str(&format!("_:e{i} shnex:exists _:e{} .\n", i + 1));
            }
            graph.push_str("_:e61 shnex:pathValues [ sh:inversePath [ sh:inversePath :p ] ] .\n");

            assert!(try_validate(&graph).unwrap().conforms());
        });
    }

    #[test]
    fn path_at_the_depth_limit_is_validated_on_a_small_stack() {
        on_small_stack(|| {
            let mut graph = format!("{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path _:p0 ] .\n");
            for i in 0..63 {
                graph.push_str(&format!("_:p{i} sh:zeroOrOnePath _:p{} .\n", i + 1));
            }
            graph.push_str("_:p63 sh:zeroOrOnePath :p .\n");

            assert!(try_validate(&graph).unwrap().conforms());
        });
    }

    #[test]
    fn chain_of_shapes_with_paths_and_node_expressions_at_the_depth_limit() {
        // Each shape of the chain checks the stack a little deeper, so some check lands just above the
        // red zone, and the 64-deep path and node expression of that shape must then fit in it
        let n = 100;
        let mut graph = format!("{PREFIXES}{SHNEX}:S0 sh:targetNode :a .\n:S{n} sh:class :C .\n");
        for i in 0..n {
            let next = i + 1;
            graph.push_str(&format!(
                ":S{i} sh:node :S{next} ; sh:property [ sh:path _:p{i}x0 ] .\n"
            ));
            for j in 0..63 {
                let k = j + 1;
                graph.push_str(&format!("_:p{i}x{j} sh:zeroOrOnePath _:p{i}x{k} .\n"));
            }
            graph.push_str(&format!("_:p{i}x63 sh:zeroOrOnePath :p .\n"));
            graph.push_str(&format!(
                ":S{i} sh:deactivated _:e{i}x0 .\n_:e{i}x64 shnex:var \"focusNode\" .\n"
            ));
            for j in 0..64 {
                let k = j + 1;
                graph.push_str(&format!(
                    "_:e{i}x{j} shnex:pathValues :p ; shnex:focusNode _:e{i}x{k} .\n"
                ));
            }
        }

        on_small_stack(move || {
            // Only the last shape fails, so every shape of the chain was active
            let report = try_validate(&graph).unwrap();
            assert_eq!(report.results().len(), 1, "{:?}", report.results());
        });
    }

    #[test]
    fn shape_deactivated_by_its_own_nodes_is_an_error() {
        for shapes in [
            ":S sh:targetNode :a ; sh:class :C ; sh:deactivated [ shnex:nodesMatching :S ] .",
            ":S sh:targetNode :a ; sh:node :T .\n:T sh:deactivated [ shnex:exists [ shnex:nodesMatching :S ] ] .",
        ] {
            let graph = format!("{PREFIXES}{SHNEX}{shapes}\n:a :p :b .\n");
            let result = try_validate(&graph).map(|_| ());
            assert!(
                matches!(&result, Err(e) if e.contains("negative cycles")),
                "{shapes}: {result:?}"
            );
        }
    }

    #[test]
    fn node_expression_cycles_are_rejected_by_the_dependency_graph() {
        for shapes in [
            // One shape-taking node expression per kind
            ":S sh:targetNode :a ; sh:expression [ shnex:nodesMatching :S ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:filterShape :S ; shnex:nodes ( :a ) ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:findFirst :S ; shnex:nodes ( :a ) ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:matchAll :S ; shnex:nodes ( :a ) ] .",
            // Nested inside another expression
            ":S sh:targetNode :a ; sh:expression [ shnex:count [ shnex:nodesMatching :S ] ] .",
            // Through a second shape, and through a node expression based target
            ":S sh:targetNode :a ; sh:expression [ shnex:nodesMatching :T ] .\n:T sh:node :S .",
            ":S sh:targetNode [ shnex:nodesMatching :S ] ; sh:class :C .",
            ":S sh:targetWhere :T ; sh:class :C .\n:T sh:node :S .",
            // The expression of a sh:nodeByExpression
            ":S sh:targetNode :a ; sh:nodeByExpression [ shnex:findFirst :S ; shnex:nodes ( :a ) ] .",
        ] {
            let graph = format!("{PREFIXES}{SHNEX}{shapes}\n:a :p :b .\n");
            let result = try_validate(&graph).map(|_| ());
            assert!(
                matches!(&result, Err(e) if e.contains("negative cycles")),
                "{shapes}: {result:?}"
            );
        }
    }

    #[test]
    fn chain_of_nodes_matching_is_computed_once_per_shape() {
        // Recomputing the next shape for every node took over 30 s for 6 shapes over 20 nodes
        let mut graph = format!("{PREFIXES}{SHNEX}:T0 sh:targetNode :n0 .\n:T6 sh:class :C .\n");
        for i in 0..6 {
            graph.push_str(&format!(":T{i} sh:deactivated [ shnex:nodesMatching :T{} ] .\n", i + 1));
        }
        for i in 0..20 {
            graph.push_str(&format!(":n{i} :p :n{} .\n", i + 1));
        }

        assert!(try_validate(&graph).is_ok());
    }

    #[test]
    fn deactivation_that_does_not_read_the_focus_node_is_evaluated_once_per_shape() {
        // Evaluating it for each of 10,000 focus nodes listed all the nodes each time, which took 28 s
        let nodes: String = (0..10_000).map(|i| format!(":a{i} :q 1 .\n")).collect();
        for (t, results) in [("sh:class :C", 20_000), ("sh:nodeKind sh:IRI", 0)] {
            let graph = format!(
                "{PREFIXES}{SHNEX}:R sh:targetSubjectsOf :q ; sh:node :S .\n\
                 :S sh:targetSubjectsOf :q ; sh:class :C ; sh:deactivated [ shnex:exists [ shnex:nodesMatching :T ] ] .\n\
                 :T {t} .\n{nodes}"
            );
            assert_eq!(try_validate(&graph).unwrap().results().len(), results, "{t}");
        }

        // One that reads the focus node is still evaluated for each of them
        let graph = format!(
            "{PREFIXES}{SHNEX}:S sh:targetSubjectsOf :q ; sh:class :C ; sh:deactivated [ shnex:exists [ shnex:pathValues :r ] ] .\n\
             :a0 :r 1 .\n{nodes}"
        );
        assert_eq!(try_validate(&graph).unwrap().results().len(), 9_999);
    }

    #[test]
    fn node_expression_fanned_out_by_the_data_is_an_error() {
        // Every shnex:flatMap level doubles the nodes, so 40 levels would never finish
        let mut graph = format!(
            "{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:deactivated _:f40 .\n\
             :a :p :a, :b .\n:b :p :a, :b .\n_:f0 shnex:pathValues :p .\n"
        );
        for i in 1..=40 {
            graph.push_str(&format!(
                "_:f{i} shnex:flatMap _:f{} ; shnex:nodes [ shnex:pathValues :p ] .\n",
                i - 1
            ));
        }

        let result = try_validate(&graph).map(|_| ());
        assert!(matches!(&result, Err(e) if e.contains("steps")), "{result:?}");
    }

    #[test]
    fn nodes_matching_keeps_the_step_count_of_the_outer_expression() {
        // Each _:f18 takes over half the steps, and validating :T used to reset the count between them
        let mut graph = format!(
            "{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:deactivated [ shnex:concat ( _:f18 _:m _:f18 ) ] .\n\
             _:m shnex:nodesMatching :T .\n:T sh:deactivated false .\n\
             :a :p :a, :b .\n:b :p :a, :b .\n_:f0 shnex:pathValues :p .\n"
        );
        for i in 1..=18 {
            graph.push_str(&format!(
                "_:f{i} shnex:flatMap _:f{} ; shnex:nodes [ shnex:pathValues :p ] .\n",
                i - 1
            ));
        }

        let result = try_validate(&graph).map(|_| ());
        assert!(matches!(&result, Err(e) if e.contains("steps")), "{result:?}");
    }

    #[test]
    fn a_triple_term_expression_evaluates_to_itself() {
        for (expr, conforms) in [
            ("shnex:exists <<( :a :p :b )>>", true),
            // The triple term is removed from itself by term equality, leaving no node
            (
                "shnex:exists [ shnex:remove <<( :a :p :b )>> ; shnex:nodes <<( :a :p :b )>> ]",
                false,
            ),
            // A different triple term is kept
            (
                "shnex:exists [ shnex:remove <<( :a :p :c )>> ; shnex:nodes <<( :a :p :b )>> ]",
                true,
            ),
        ] {
            let graph = format!("{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:expression [ {expr} ] .\n");
            assert_eq!(try_validate(&graph).unwrap().conforms(), conforms, "{expr}");
        }
    }

    #[test]
    fn conforms_to_shape_validates_its_node_against_its_shape() {
        let data = ":T a sh:NodeShape ; sh:class :C .\n:a a :C ; :p :a, :b .\n";
        for (args, conforms) in [
            ("[ shnex:var \"value\" ] :T", true),
            (":b :T", false),
            // The empty list is not ( true ), as either argument without a value gives no output node
            ("[] :T", false),
            ("[ shnex:var \"value\" ] []", false),
        ] {
            let graph = format!(
                "{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:expression [ shnex:conformsToShape ( {args} ) ] .\n{data}"
            );
            assert_eq!(try_validate(&graph).unwrap().conforms(), conforms, "{args}");
        }

        for (args, error) in [
            // The shape is not a shape of the shapes graph
            ("[ shnex:var \"value\" ] :Missing", "Not found shape"),
            // Each argument must produce at most one output node
            ("[ shnex:pathValues :p ] :T", "at most one node"),
            ("[ shnex:var \"value\" ] ( :T :T )", "at most one node"),
            // The arguments are a list of exactly two node expressions
            (":a", "list of two node expressions"),
        ] {
            let graph = format!(
                "{PREFIXES}{SHNEX}:S sh:targetNode :a ; sh:expression [ shnex:conformsToShape ( {args} ) ] .\n{data}"
            );
            let result = try_validate(&graph).map(|_| ());
            assert!(matches!(&result, Err(e) if e.contains(error)), "{args}: {result:?}");
        }
    }

    #[test]
    fn ill_formed_terms_are_errors_not_panics() {
        for (shapes, conforms) in [
            (":S sh:targetNode _:b ; sh:class :C .", true),
            (":S sh:targetNode :a ; sh:languageIn ( \"1\" :x ) .", false),
            (":S sh:targetNode \"x\" ; sh:disjoint :p .", true),
            (":S sh:targetNode :a ; sh:in ( :a <<( :a :b :c )>> _:b ) .", true),
        ] {
            let graph = format!("{PREFIXES}{shapes}\n:a :p :b .\n");
            assert_eq!(try_validate(&graph).unwrap().conforms(), conforms, "{shapes}");
        }

        let shapes = ":S sh:targetNode :a ; sh:hasValue <<( :a :p :b )>> .";
        assert!(try_validate(&format!("{PREFIXES}{shapes}\n")).is_err());
    }

    #[test]
    fn ill_formed_target_is_written_back() {
        // `_:b` is not a well-formed node expression, so it is not a well-formed target
        let graph = format!("{PREFIXES}:S sh:targetNode _:b ; sh:class :C .\n_:b :foo 1 .\n");

        assert!(serialize_shapes(&graph).contains("targetNode"));
    }

    #[test]
    fn target_node_expressions_target_their_output_nodes() {
        const DATA: &str = ":a :p :b .\n:b a :C .\n:T a sh:NodeShape ; sh:class :C .\n";

        for (target, results) in [
            // The constant expressions of SHACL Core
            (":a", 1),
            ("\"x\"", 1),
            // A blank node with no arcs is the empty expression, so there is no target at all
            ("[ ]", 0),
            ("[ shnex:instancesOf :C ]", 1),
            ("[ shnex:nodesMatching :T ]", 1),
            ("[ shnex:distinct ( :a :a ) ]", 1),
        ] {
            let graph = format!("{PREFIXES}{SHNEX}:S sh:targetNode {target} ; sh:class :D .\n{DATA}");
            let report = try_validate(&graph).unwrap_or_else(|e| panic!("{target}: {e}"));

            assert_eq!(report.results().len(), results, "{target}: {:?}", report.results());
        }

        // A target is declared before there is any focus node for the expression to read
        let graph = format!("{PREFIXES}{SHNEX}:S sh:targetNode [ shnex:var \"focusNode\" ] ; sh:class :D .\n{DATA}");
        let error = try_validate(&graph).expect_err("expected a rejected target node expression");
        assert!(error.contains("no focus node to read"), "{error}");
    }

    #[test]
    fn target_where_targets_the_nodes_conforming_to_its_shape() {
        const DATA: &str = ":a :p :b .\n:b a :C .\n:T a sh:NodeShape ; sh:class :C .\n";

        for (target, results) in [(":T", 1), ("[ sh:class :C ]", 1), ("[ sh:class :D ]", 0)] {
            let graph = format!("{PREFIXES}:S sh:targetWhere {target} ; sh:class :D .\n{DATA}");
            let report = try_validate(&graph).unwrap_or_else(|e| panic!("{target}: {e}"));
            assert_eq!(report.results().len(), results, "{target}: {:?}", report.results());

            let round_tripped = format!("{PREFIXES}{}\n{DATA}", serialize_shapes(&graph));
            let after = try_validate(&round_tripped).unwrap_or_else(|e| panic!("{target}: {e}\n{round_tripped}"));
            assert_eq!(after.results().len(), results, "{target} was lost:\n{round_tripped}");
        }

        let graph = format!("{PREFIXES}:S sh:targetWhere 1 ; sh:class :D .\n{DATA}");
        let error = try_validate(&graph).expect_err("expected a rejected sh:targetWhere");
        assert!(error.contains("must be a shape"), "{error}");
    }

    /// Parses and compiles `graph`, and serializes its shapes back to Turtle.
    fn serialize_shapes(graph: &str) -> String {
        let rdf = RdfData::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema = ShaclParser::new(rdf).parse().unwrap();
        let schema_ir = IRSchema::compile_with_recursion(&schema, RecursionSemantics::default()).unwrap();
        let mut writer = crate::rdf::ShaclWriter::<rudof_rdf::rdf_impl::OxigraphInMemory>::new();
        writer.register(&schema_ir).unwrap();
        let mut turtle = Vec::new();
        writer.serialize(&RDFFormat::Turtle, &mut turtle).unwrap();

        String::from_utf8(turtle).unwrap()
    }

    #[test]
    fn node_expressions_survive_a_serialization_round_trip() {
        const DATA: &str = ":a :p 1, 2, 3 ; :q :b .\n:b a :C ; :p 4 .\n:T a sh:NodeShape ; sh:class :C .\n";

        for shape in [
            // The expression itself, which is not `( true )` in any of these
            ":S sh:targetNode :a ; sh:expression [ ] .",
            ":S sh:targetNode :a ; sh:expression :a .",
            ":S sh:targetNode :a ; sh:expression 1 .",
            ":S sh:targetNode :a ; sh:expression ( 1 2 ) .",
            ":S sh:targetNode :a ; sh:expression <<( :a :p 1 )>> .",
            ":S sh:targetNode :a ; sh:expression [ shnex:var \"value\" ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:pathValues :p ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:pathValues ( :q :p ) ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:pathValues :p ; shnex:focusNode :b ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:exists [ shnex:pathValues :zzz ] ] .",
            ":S sh:targetNode :a ; sh:expression
                 [ shnex:if [ shnex:exists [ shnex:pathValues :zzz ] ] ; shnex:then true ; shnex:else false ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:distinct [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:intersection ( [ shnex:pathValues :p ] ( 1 2 ) ) ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:concat ( ( 1 ) ( 2 ) ) ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:remove ( 1 ) ; shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:limit 1 ; shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:offset 1 ; shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:orderBy [ shnex:var \"focusNode\" ] ; shnex:desc true ;
                 shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression
                 [ shnex:flatMap [ shnex:pathValues :p ] ; shnex:nodes [ shnex:pathValues :q ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:count [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:min [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:max [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:sum [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:instancesOf :C ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:nodesMatching :T ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:filterShape :T ; shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:findFirst :T ; shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:matchAll :T ; shnex:nodes [ shnex:pathValues :p ] ] .",
            ":S sh:targetNode :a ; sh:expression [ shnex:conformsToShape ( :a :T ) ] .",
            // A blank node shape, which is written back by its own label
            ":S sh:targetNode :a ; sh:expression [ shnex:nodesMatching [ sh:class :C ] ] .",
            // The expression of a node expression based constraint component
            ":S sh:targetNode :a ; sh:nodeByExpression :T .",
            ":S sh:targetNode :a ; sh:nodeByExpression [ shnex:if true ; shnex:then :T ] .",
            ":S sh:targetNode :a ; sh:deactivated [ shnex:exists [ shnex:pathValues :zzz ] ] ; sh:class :C .",
            // The expression of a target declaration
            ":S sh:targetNode [ shnex:instancesOf :C ] ; sh:class :D .",
        ] {
            let graph = format!("{PREFIXES}{SHNEX}{shape}\n{DATA}");
            let before = try_validate(&graph).unwrap_or_else(|e| panic!("{shape}: {e}"));
            assert_eq!(before.results().len(), 1, "{shape}: {:?}", before.results());

            // The writer only writes the shapes, so the data is read again with them
            let round_tripped = format!("{PREFIXES}{SHNEX}{}\n{DATA}", serialize_shapes(&graph));
            let after = try_validate(&round_tripped).unwrap_or_else(|e| panic!("{shape}: {e}\n{round_tripped}"));

            assert_eq!(
                after.results().len(),
                before.results().len(),
                "{shape} was lost on the round trip:\n{round_tripped}"
            );
        }
    }

    #[test]
    fn qualified_value_shape_sibling_in_a_cycle_is_rejected() {
        // :Q1 reaches :X only as a sibling, through :Q2, and :X reaches :Q1 again
        let graph = format!(
            "{PREFIXES}:P sh:targetNode :a ; sh:property :Q1, :Q2 .
             :Q1 sh:path :p ; sh:qualifiedValueShape :S ; sh:qualifiedMinCount 1 ;
                 sh:qualifiedValueShapesDisjoint true .
             :Q2 sh:path :q ; sh:qualifiedValueShape :X .
             :X sh:property :Q1 .\n"
        );
        let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema = ShaclParser::new(rdf).parse().unwrap();

        // Counting only the values that do not conform to :X is a negation, so no semantics accepts the cycle
        for semantics in [
            RecursionSemantics::None,
            RecursionSemantics::Lfp,
            RecursionSemantics::Gfp,
        ] {
            let error = IRSchema::compile_with_recursion(&schema, semantics).expect_err("expected a rejected cycle");
            assert!(
                error.to_string().contains("negative cycles") && error.to_string().contains(":X"),
                "{error}"
            );
        }
    }

    #[test]
    fn xone_and_qualified_max_count_in_a_cycle_are_rejected() {
        // Over `:a :p :a`, these made LFP report :a as conforming and GFP as violating
        let xone = ":S sh:xone ( :T :Any ) . :T sh:property [ sh:path :p ; sh:node :S ] . :Any sh:nodeKind sh:IRI .";
        let max_count = ":S sh:property [ sh:path :p ; sh:qualifiedValueShape :S ; sh:qualifiedMaxCount 0 ] .";
        for shapes in [xone, max_count] {
            let graph = format!("{PREFIXES}{shapes}\n");
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let schema = ShaclParser::new(rdf).parse().unwrap();

            for semantics in [
                RecursionSemantics::None,
                RecursionSemantics::Lfp,
                RecursionSemantics::Gfp,
            ] {
                let error =
                    IRSchema::compile_with_recursion(&schema, semantics).expect_err("expected a rejected cycle");
                assert!(error.to_string().contains("negative cycles"), "{error}");
            }
        }
    }

    #[test]
    fn not_in_a_cycle_is_rejected_and_a_self_loop_is_named() {
        // The sh:not cycles were accepted under LFP and GFP, and a self-loop was left out of the message
        for (shapes, shown) in [
            (":S sh:not :S .", ": [:S]"),
            (":S sh:xone ( :S ) .", ": [:S]"),
            (":S sh:not :T . :T sh:node :S .", ":T"),
            (":S sh:not :T ; sh:node :T . :T sh:node :S .", ":T"),
        ] {
            let graph = format!("{PREFIXES}{shapes}\n");
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let schema = ShaclParser::new(rdf).parse().unwrap();

            for semantics in [
                RecursionSemantics::None,
                RecursionSemantics::Lfp,
                RecursionSemantics::Gfp,
            ] {
                let error = IRSchema::compile_with_recursion(&schema, semantics)
                    .expect_err("expected a rejected cycle")
                    .to_string();
                assert!(error.contains("negative cycles") && error.contains(shown), "{error}");
            }
        }
    }

    #[test]
    fn not_of_a_positive_cycle_from_outside_it_is_stratified() {
        // :X and :Y are a positive cycle that only :A negates, so LFP cuts :X and GFP keeps it
        let graph =
            format!("{PREFIXES}:A sh:targetNode :a ; sh:not :X .\n:X sh:node :Y .\n:Y sh:node :X .\n:a :p :a .\n");
        for (semantics, results) in [(RecursionSemantics::Lfp, 0), (RecursionSemantics::Gfp, 1)] {
            let config = ShaclConfig::default().with_recursion_semantics(semantics);
            assert_eq!(
                validate_with_config(&graph, &config).results().len(),
                results,
                "{semantics:?}"
            );
        }
    }

    #[test]
    fn max_results_stops_nested_property_shapes_over_diamonds() {
        // k nested property shapes over k/2 diamonds, each doubling the results of the shapes below it
        let diamonds = |k: usize| {
            let mut graph =
                format!("{PREFIXES}:S sh:targetNode :n0 ; sh:property _:p1 .\n_:p{k} sh:path :next ; sh:class :C .\n");
            for i in 1..k {
                graph.push_str(&format!(
                    "_:p{i} sh:path :next ; sh:class :C ; sh:property _:p{} .\n",
                    i + 1
                ));
            }
            for i in 0..k / 2 {
                graph.push_str(&format!(
                    ":n{i} :next :a{i}, :b{i} .\n:a{i} :next :n{j} .\n:b{i} :next :n{j} .\n",
                    j = i + 1
                ));
            }
            graph
        };
        let config = ShaclConfig::default().with_max_results(Some(1000));

        // Under the limit, the report is the same as without it
        let report = validate_with_config(&diamonds(8), &config);
        assert_eq!(report.results().len(), 60);
        assert_eq!(report, validate_with_config(&diamonds(8), &ShaclConfig::default()));

        // 40 levels would hold about 2^22 results
        let deep = diamonds(40);
        // Over 30 nodes that all link to each other, LFP cuts each node once per path that reaches it
        // again, so the results double at each node
        let mut clique =
            format!("{PREFIXES}:S sh:targetNode :n0 ; sh:property _:p .\n_:p sh:path :next ; sh:property _:p .\n");
        // Each of the 40 sh:node results holds the 40 results of :T as details, which count too
        let mut details = format!(
            "{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path :v ; sh:node :T ] .\n\
             :T sh:property [ sh:path :p ; sh:class :C ] .\n"
        );
        for i in 0..40 {
            details.push_str(&format!(":a :v :v{i} .\n"));
            for j in 0..40 {
                details.push_str(&format!(":v{i} :p :x{j} .\n"));
                if i < 30 && j < 30 {
                    clique.push_str(&format!(":n{i} :next :n{j} .\n"));
                }
            }
        }
        for (name, graph) in [("diamonds", deep), ("clique", clique), ("details", details)] {
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let mut validator: DataValidation = rdf.clone().into();
            let schema_ir: IRSchema = ShaclParser::new(rdf).parse().unwrap().try_into().unwrap();
            let result = validator.validate(&schema_ir, &ShaclValidationMode::Native, &config);
            assert!(
                matches!(result, Err(crate::error::ValidationError::TooManyResults(_, 1000))),
                "{name}: {result:?}"
            );
        }
    }

    /// Paths at the depth limit: 64 nested `sh:zeroOrOnePath`, and 64 nested one-element sequences.
    fn paths_at_the_depth_limit() -> [String; 2] {
        [
            format!("{}:p{}", "[ sh:zeroOrOnePath ".repeat(64), " ]".repeat(64)),
            format!("{}:p{}", "( ".repeat(64), " )".repeat(64)),
        ]
    }

    #[test]
    fn result_path_at_the_depth_limit_is_read_on_a_small_stack() {
        for path in paths_at_the_depth_limit() {
            on_small_stack(move || {
                let graph = format!(
                    "{PREFIXES}:r sh:result _:x .\n\
                     _:x sh:focusNode :a ; sh:resultSeverity sh:Violation ;\n\
                     sh:sourceConstraintComponent sh:MaxCountConstraintComponent ; sh:resultPath {path} .\n"
                );
                let mut rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
                let report_node = rudof_rdf::rdf_core::term::Object::iri(rudof_iri::iri!("http://example.org/r"));
                let report = crate::validator::report::ValidationReport::parse(&mut rdf, report_node.into()).unwrap();

                assert!(report.results()[0].path().is_some());
            });
        }
    }

    #[test]
    fn result_path_at_the_depth_limit_is_written_on_a_small_stack() {
        use rudof_rdf::rdf_core::BuildRDF;
        use rudof_rdf::rdf_impl::OxigraphInMemory;

        for path in paths_at_the_depth_limit() {
            let graph = format!(
                "{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path {path} ; sh:maxCount 0 ] .\n:a :p :b .\n"
            );
            let report = try_validate(&graph).unwrap();
            assert_eq!(report.results().len(), 1, "{:?}", report.results());

            on_small_stack(move || report.to_rdf(&mut OxigraphInMemory::empty()).unwrap());
        }
    }

    #[test]
    fn paths_at_the_depth_limit_are_written_back_on_a_small_stack() {
        for path in paths_at_the_depth_limit() {
            let graph = format!(
                "{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path {path} ], [ sh:path :q ; sh:subsetOf {path} ] .\n"
            );
            let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
            let schema_ir: IRSchema = ShaclParser::new(rdf).parse().unwrap().try_into().unwrap();

            on_small_stack(move || {
                let mut writer = crate::rdf::ShaclWriter::<rudof_rdf::rdf_impl::OxigraphInMemory>::new();
                writer.register(&schema_ir).unwrap();
            });
        }
    }

    #[test]
    fn path_at_the_depth_limit_is_prefetched_on_a_small_stack() {
        use crate::validator::FetchStrategy;
        use crate::validator::processor::CachedDataValidation;

        // One graph each: the data fetched for the first path would leave nothing for the second to fetch
        let path = format!("{}:p{}", "[ sh:zeroOrMorePath ".repeat(64), " ]".repeat(64));
        for property in [
            format!("sh:path {path} ; sh:maxCount 0"),
            format!("sh:path :q ; sh:subsetOf {path}"),
        ] {
            on_small_stack(move || {
                let graph =
                    format!("{PREFIXES}:S sh:targetNode :a ; sh:property [ {property} ] .\n:a :p :b ; :q :c .\n");
                let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
                let mut validator = CachedDataValidation::new(rdf.clone());
                let schema_ir: IRSchema = ShaclParser::new(rdf).parse().unwrap().try_into().unwrap();
                let config = ShaclConfig::default().with_fetch_strategy(FetchStrategy::ByShape);

                let report = validator
                    .validate(&schema_ir, &ShaclValidationMode::Native, &config)
                    .unwrap();
                assert_eq!(report.results().len(), 1, "{:?}", report.results());
            });
        }
    }

    #[test]
    fn chain_of_patterns_at_the_nesting_limit_is_validated_on_a_small_stack() {
        // 64 levels, the most RDFRegex accepts, of the repetitions that take the most stack to compile
        let pattern = format!("a{}{}", "{2,}".repeat(13), "+".repeat(51));
        // A chain of shapes, so that the pattern is compiled and matched at many depths of the stack
        let mut graph = format!("{PREFIXES}:S0 sh:targetNode :a .\n:a :p \"b\" .\n");
        for i in 0..100 {
            graph.push_str(&format!(
                ":S{i} sh:node :S{} ; sh:property [ sh:path :p ; sh:pattern \"{pattern}\" ] .\n",
                i + 1
            ));
        }
        for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
            let graph = graph.clone();
            on_small_stack(move || {
                let rdf = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
                let mut validator: DataValidation = rdf.clone().into();
                let schema_ir: IRSchema = ShaclParser::new(rdf).parse().unwrap().try_into().unwrap();
                let report = validator.validate(&schema_ir, &mode, &ShaclConfig::default()).unwrap();
                // :S0's own pattern, and sh:node :S1
                assert_eq!(report.results().len(), 2, "{mode:?}: {:?}", report.results());
            });
        }
    }

    #[test]
    fn deeply_nested_pattern_is_an_error_on_a_small_stack() {
        on_small_stack(|| {
            // 250 nested repetitions, the most the regex crate accepts by default
            let pattern = format!("a{}", "*".repeat(250));
            let graph = format!(
                "{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path :p ; sh:pattern \"{pattern}\" ] .\n:a :p \"aa\" .\n"
            );

            let result = try_validate(&graph).map(|_| ());
            assert!(matches!(&result, Err(e) if e.contains("nested")), "{result:?}");
        });
    }

    #[test]
    fn failed_path_lookup_is_an_error() {
        use crate::validator::processor::CachedDataValidation;
        use rudof_iri::IriS;
        use rudof_rdf::rdf_impl::OxigraphEndpoint;
        use std::io::{BufRead, BufReader, Write};

        // An endpoint that answers every query with an error
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/sparql", listener.local_addr().unwrap());
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let mut reader = BufReader::new(&stream);
                let mut line = String::new();
                while reader.read_line(&mut line).unwrap_or(0) > 2 {
                    line.clear();
                }
                let _ = (&stream)
                    .write_all(b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
            }
        });

        let graph = format!("{PREFIXES}:S sh:targetNode :a ; sh:property [ sh:path :p ; sh:minCount 1 ] .\n");
        let shapes = RdfData::from_str(&graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
        let schema_ir: IRSchema = ShaclParser::new(shapes).parse().unwrap().try_into().unwrap();
        for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
            let endpoint = OxigraphEndpoint::new(&IriS::new_unchecked(&url), &Default::default()).unwrap();
            let mut validator = CachedDataValidation::new(RdfData::from_endpoint("failing", endpoint));
            // It used to conform, as :a was left out of the value nodes
            let result = validator.validate(&schema_ir, &mode, &ShaclConfig::default());
            assert!(result.is_err(), "{mode:?}: {result:?}");
        }
    }
}
