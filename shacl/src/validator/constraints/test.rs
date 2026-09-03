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
    fn cautious_mode_rejects_an_ungrounded_cycle() {
        // Least fixpoint: nothing in the cycle can be justified without
        // assuming itself, so it's assumed non-conformant.
        let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Cautious);
        let report = validate_with_config(MUTUAL_CYCLE_GRAPH, &config);
        assert!(!report.conforms());
    }

    #[test]
    fn brave_mode_accepts_a_self_consistent_cycle() {
        // Greatest fixpoint: the assignment "everyone in the cycle conforms"
        // is self-consistent, so it's accepted.
        let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Brave);
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
        let cautious_config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Cautious);
        let cautious = validate_with_config(graph, &cautious_config);
        assert!(cautious.conforms(), "{:?}", cautious.results());

        let brave_config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Brave);
        let brave = validate_with_config(graph, &brave_config);
        assert!(brave.conforms(), "{:?}", brave.results());
    }

    /// A terminating chain and a separate pure cycle against the same
    /// self-recursive shape, both reachable in one `:targetClass` batch —
    /// this combination previously triggered a regression (found during
    /// development of this feature, before it ever shipped) where batching
    /// multiple independent `:targetClass` instances together made sibling
    /// targets falsely look like ancestors of each other, cutting the chain
    /// nodes as if they were cyclic. Every chain node must conform; every
    /// cycle node must not (cautious/LFP).
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

        let config = ShaclConfig::default().with_recursion_semantics(RecursionSemantics::Cautious);
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
}
