//! Integration tests for RudofMcpService.
//!
//! These tests verify the service handlers work correctly
//!
//! Note: Since rmcp's RequestContext is difficult to construct
//! in tests (Peer is crate-private), we focus on testing:
//! - Service initialization and state
//! - Capabilities via get_info()
//! - Internal helper methods (completions, subscriptions)
//! - Tool/prompt router availability

use rmcp::ServerHandler;
use rudof_mcp::service::RudofMcpService;

// =============================================================================
// Service Initialization Tests
// =============================================================================

mod initialization_tests {
    use super::*;

    /// Test that service initializes with valid default state
    #[test]
    fn test_service_initialization() {
        let service = RudofMcpService::new();

        // Verify the service is properly initialized
        // by checking that get_info returns valid data
        let info = service.get_info();

        assert!(!info.server_info.name.is_empty(), "Server name should be set");
    }

    /// Test that try_new returns Ok for valid configuration
    #[test]
    fn test_try_new_succeeds() {
        let result = RudofMcpService::try_new();
        assert!(result.is_ok(), "try_new should succeed with valid configuration");
    }
}

// =============================================================================
// Tool Router Tests
// =============================================================================

mod tool_router_tests {
    use super::*;

    /// Test that tool router is initialized
    #[test]
    fn test_tool_router_exists() {
        let service = RudofMcpService::new();
        // The tool_router field should be accessible
        let _router = &service.tool_router;
    }

    /// Test that annotated_tools returns tools
    #[test]
    fn test_annotated_tools_not_empty() {
        use rudof_mcp::service::annotated_tools;

        let tools = annotated_tools();

        assert!(!tools.is_empty(), "Should have at least one tool defined");
    }

    /// Test tools have required fields
    #[test]
    fn test_tools_have_name_and_description() {
        use rudof_mcp::service::annotated_tools;

        let tools = annotated_tools();

        for tool in tools {
            assert!(!tool.name.is_empty(), "Tool name should not be empty");
            assert!(
                !tool.description.as_ref().unwrap().is_empty(),
                "Tool description should not be empty"
            );
        }
    }

    /// Test expected tools are present
    #[test]
    fn test_expected_tools_present() {
        use rudof_mcp::service::annotated_tools;

        let tools = annotated_tools();
        let tool_names: Vec<_> = tools.iter().map(|t| t.name.to_string()).collect();

        // Verify some expected tools exist
        let expected_tools = vec!["load_rdf_data_from_sources", "export_rdf_data", "node_info"];

        for expected in expected_tools {
            assert!(
                tool_names.iter().any(|n| n == expected),
                "Expected tool '{}' should be present",
                expected
            );
        }
    }
}

// =============================================================================
// Prompt Router Tests
// =============================================================================

mod prompt_router_tests {
    use super::*;

    /// Test that prompt router is initialized
    #[test]
    fn test_prompt_router_exists() {
        let service = RudofMcpService::new();
        let _router = &service.prompt_router;
    }

    /// Test that prompt router lists prompts
    #[test]
    fn test_prompt_router_has_prompts() {
        let service = RudofMcpService::new();
        let prompts = service.prompt_router.list_all();

        assert!(!prompts.is_empty(), "Should have at least one prompt defined");
    }

    /// Test prompts have required fields
    #[test]
    fn test_prompts_have_name() {
        let service = RudofMcpService::new();
        let prompts = service.prompt_router.list_all();

        for prompt in &prompts {
            assert!(!prompt.name.is_empty(), "Prompt name should not be empty");
        }
    }

    /// Test expected prompts are present
    #[test]
    fn test_expected_prompts_present() {
        let service = RudofMcpService::new();
        let prompts = service.prompt_router.list_all();
        let prompt_names: Vec<_> = prompts.iter().map(|p| p.name.to_string()).collect();

        let expected_prompts = vec!["explore_rdf_node", "analyze_rdf_data", "validation_guide"];

        for expected in expected_prompts {
            assert!(
                prompt_names.iter().any(|n| n == expected),
                "Expected prompt '{}' should be present",
                expected
            );
        }
    }
}

// =============================================================================
// Completion Tests
// =============================================================================

mod completion_tests {
    use super::*;

    /// Test prompt argument completions for format
    #[test]
    fn test_prompt_argument_completions_format() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "format");

        assert!(!completions.is_empty(), "Should have format completions");
        assert!(
            completions.contains(&"turtle".to_string()),
            "Should include turtle format"
        );
    }

    /// Test prompt argument completions for rdf_format
    #[test]
    fn test_prompt_argument_completions_rdf_format() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "rdf_format");

        assert!(!completions.is_empty());
        assert!(completions.contains(&"turtle".to_string()));
        assert!(completions.contains(&"ntriples".to_string()));
        assert!(completions.contains(&"rdfxml".to_string()));
    }

    /// Test prompt argument completions for schema_format
    #[test]
    fn test_prompt_argument_completions_schema_format() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "schema_format");

        assert!(!completions.is_empty());
        assert!(completions.contains(&"shexc".to_string()));
        assert!(completions.contains(&"shexj".to_string()));
    }

    /// Test prompt argument completions for mode
    #[test]
    fn test_prompt_argument_completions_mode() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "mode");

        assert!(!completions.is_empty(), "Should have mode completions");
        assert!(completions.contains(&"both".to_string()), "Should include 'both' mode");
        assert!(completions.contains(&"outgoing".to_string()));
        assert!(completions.contains(&"incoming".to_string()));
    }

    /// Test prompt argument completions for technology
    #[test]
    fn test_prompt_argument_completions_technology() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("validation_guide", "technology");

        assert!(!completions.is_empty());
        assert!(completions.contains(&"shex".to_string()));
        assert!(completions.contains(&"shacl".to_string()));
    }

    /// Test resource URI completions for format
    #[test]
    fn test_resource_uri_completions() {
        let service = RudofMcpService::new();

        let completions = service.get_resource_uri_completions("rudof://current-data", "format");

        assert!(
            !completions.is_empty(),
            "Should have format completions for rudof:// URIs"
        );
        assert!(completions.contains(&"turtle".to_string()));
    }

    /// Test resource URI completions for endpoint
    #[test]
    fn test_resource_uri_completions_endpoint() {
        let service = RudofMcpService::new();

        let completions = service.get_resource_uri_completions("rudof://query", "endpoint");

        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.contains("wikidata")));
    }

    /// Test non-rudof URI returns empty completions
    #[test]
    fn test_non_rudof_uri_no_completions() {
        let service = RudofMcpService::new();

        let completions = service.get_resource_uri_completions("http://example.org/resource", "format");

        assert!(completions.is_empty(), "Non-rudof URIs should have no completions");
    }

    /// Test unknown argument returns empty completions
    #[test]
    fn test_unknown_argument_no_completions() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "unknown_arg");

        assert!(completions.is_empty(), "Unknown arguments should have no completions");
    }
}

// =============================================================================
// Log Level Tests
// =============================================================================

mod log_level_tests {
    use super::*;
    use rmcp::model::LoggingLevel;

    /// Helper to run async code in tests
    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    /// Test initial log level is None
    #[test]
    fn test_initial_log_level_is_none() {
        let service = RudofMcpService::new();

        block_on(async {
            let level = service.current_min_log_level.read().await;
            assert!(level.is_none(), "Initial log level should be None");
        });
    }

    /// Test log level can be set
    #[test]
    fn test_log_level_can_be_set() {
        let service = RudofMcpService::new();

        block_on(async {
            {
                let mut level = service.current_min_log_level.write().await;
                *level = Some(LoggingLevel::Debug);
            }

            let level = service.current_min_log_level.read().await;
            assert_eq!(*level, Some(LoggingLevel::Debug));
        });
    }

    /// Test all log levels can be stored
    #[test]
    fn test_all_log_levels() {
        let service = RudofMcpService::new();

        let levels = vec![
            LoggingLevel::Debug,
            LoggingLevel::Info,
            LoggingLevel::Notice,
            LoggingLevel::Warning,
            LoggingLevel::Error,
            LoggingLevel::Critical,
            LoggingLevel::Alert,
            LoggingLevel::Emergency,
        ];

        block_on(async {
            for level in levels {
                {
                    let mut stored = service.current_min_log_level.write().await;
                    *stored = Some(level);
                }

                let stored = service.current_min_log_level.read().await;
                assert_eq!(*stored, Some(level));
            }
        });
    }
}
