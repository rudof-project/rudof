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

        assert!(
            !info.server_info.name.is_empty(),
            "Server name should be set"
        );
    }

    /// Test that try_new returns Ok for valid configuration
    #[test]
    fn test_try_new_succeeds() {
        let result = RudofMcpService::try_new();
        assert!(
            result.is_ok(),
            "try_new should succeed with valid configuration"
        );
    }

    /// Test service is Clone
    #[test]
    fn test_service_is_clone() {
        let service = RudofMcpService::new();
        let _cloned = service.clone();
        // If we get here, Clone is implemented
    }

    /// Test service is Send + Sync (required for async)
    #[test]
    fn test_service_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RudofMcpService>();
    }

    /// Test default impl works
    #[test]
    fn test_service_default() {
        let service = RudofMcpService::default();
        let info = service.get_info();
        assert!(!info.server_info.name.is_empty());
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

        for tool in &tools {
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

        assert!(
            !prompts.is_empty(),
            "Should have at least one prompt defined"
        );
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

        let expected_prompts = vec![
            "explore_rdf_node",
            "analyze_rdf_data",
            "validation_guide",
            "sparql_builder",
        ];

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
        assert!(
            completions.contains(&"both".to_string()),
            "Should include 'both' mode"
        );
        assert!(completions.contains(&"outgoing".to_string()));
        assert!(completions.contains(&"incoming".to_string()));
    }

    /// Test prompt argument completions for boolean args
    #[test]
    fn test_prompt_argument_completions_boolean() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "verbose");

        assert!(!completions.is_empty());
        assert!(completions.contains(&"true".to_string()));
        assert!(completions.contains(&"false".to_string()));
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

        let completions =
            service.get_resource_uri_completions("http://example.org/resource", "format");

        assert!(
            completions.is_empty(),
            "Non-rudof URIs should have no completions"
        );
    }

    /// Test unknown argument returns empty completions
    #[test]
    fn test_unknown_argument_no_completions() {
        let service = RudofMcpService::new();

        let completions = service.get_prompt_argument_completions("any", "unknown_arg");

        assert!(
            completions.is_empty(),
            "Unknown arguments should have no completions"
        );
    }
}

// =============================================================================
// Resource Subscription Management Tests (using block_on for simplicity)
// =============================================================================

mod resource_subscription_management_tests {
    use super::*;

    /// Helper to run async code in tests
    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    /// Test subscribing and getting subscribers
    #[test]
    fn test_subscribe_and_get_subscribers() {
        let service = RudofMcpService::new();
        let uri = "rudof://test-resource".to_string();
        let subscriber_id = "test-subscriber".to_string();

        block_on(async {
            // Initially no subscribers
            let subscribers = service.get_resource_subscribers(&uri).await;
            assert!(
                subscribers.is_empty(),
                "Should have no subscribers initially"
            );

            // Add subscription
            service
                .subscribe_resource(uri.clone(), subscriber_id.clone())
                .await;

            // Should have subscriber now
            let subscribers = service.get_resource_subscribers(&uri).await;
            assert_eq!(subscribers.len(), 1, "Should have one subscriber");
            assert!(
                subscribers.contains(&subscriber_id),
                "Should contain our subscriber"
            );
        });
    }

    /// Test unsubscribing removes subscriber
    #[test]
    fn test_unsubscribe_removes_subscriber() {
        let service = RudofMcpService::new();
        let uri = "rudof://test-resource".to_string();
        let subscriber_id = "test-subscriber".to_string();

        block_on(async {
            // Add subscription
            service
                .subscribe_resource(uri.clone(), subscriber_id.clone())
                .await;

            // Verify subscriber exists
            let subscribers = service.get_resource_subscribers(&uri).await;
            assert_eq!(subscribers.len(), 1);

            // Unsubscribe
            service.unsubscribe_resource(&uri, &subscriber_id).await;

            // Should be empty now
            let subscribers = service.get_resource_subscribers(&uri).await;
            assert!(
                subscribers.is_empty(),
                "Should have no subscribers after unsubscribe"
            );
        });
    }

    /// Test multiple subscribers
    #[test]
    fn test_multiple_subscribers() {
        let service = RudofMcpService::new();
        let uri = "rudof://test-resource".to_string();

        block_on(async {
            // Add multiple subscribers
            service
                .subscribe_resource(uri.clone(), "sub1".to_string())
                .await;
            service
                .subscribe_resource(uri.clone(), "sub2".to_string())
                .await;
            service
                .subscribe_resource(uri.clone(), "sub3".to_string())
                .await;

            let subscribers = service.get_resource_subscribers(&uri).await;
            assert_eq!(subscribers.len(), 3, "Should have three subscribers");
        });
    }

    /// Test subscribing to different resources
    #[test]
    fn test_subscribe_different_resources() {
        let service = RudofMcpService::new();

        block_on(async {
            service
                .subscribe_resource("rudof://resource1".to_string(), "sub1".to_string())
                .await;
            service
                .subscribe_resource("rudof://resource2".to_string(), "sub2".to_string())
                .await;

            let subs1 = service.get_resource_subscribers("rudof://resource1").await;
            let subs2 = service.get_resource_subscribers("rudof://resource2").await;

            assert_eq!(subs1.len(), 1);
            assert_eq!(subs2.len(), 1);
            assert!(subs1.contains(&"sub1".to_string()));
            assert!(subs2.contains(&"sub2".to_string()));
        });
    }

    /// Test unsubscribing non-existent subscriber does nothing
    #[test]
    fn test_unsubscribe_nonexistent() {
        let service = RudofMcpService::new();
        let uri = "rudof://test-resource".to_string();

        block_on(async {
            // Add one subscriber
            service
                .subscribe_resource(uri.clone(), "sub1".to_string())
                .await;

            // Unsubscribe non-existent subscriber
            service.unsubscribe_resource(&uri, "nonexistent").await;

            // Original subscriber should still be there
            let subscribers = service.get_resource_subscribers(&uri).await;
            assert_eq!(subscribers.len(), 1);
            assert!(subscribers.contains(&"sub1".to_string()));
        });
    }

    /// Test getting subscribers for unknown resource returns empty
    #[test]
    fn test_get_subscribers_unknown_resource() {
        let service = RudofMcpService::new();

        block_on(async {
            let subscribers = service.get_resource_subscribers("rudof://unknown").await;
            assert!(subscribers.is_empty());
        });
    }
}

// =============================================================================
// Task Store Tests
// =============================================================================

mod task_store_tests {
    use super::*;

    /// Helper to run async code in tests
    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    /// Test task store is initialized
    #[test]
    fn test_task_store_exists() {
        let service = RudofMcpService::new();
        let _store = &service.task_store;
    }

    /// Test task store is empty initially
    #[test]
    fn test_task_store_initially_empty() {
        let service = RudofMcpService::new();

        block_on(async {
            let tasks = service.task_store.list(None).await;
            assert!(
                tasks.tasks.is_empty(),
                "Task store should be empty initially"
            );
        });
    }

    /// Test task enqueue creates a task
    #[test]
    fn test_task_enqueue() {
        let service = RudofMcpService::new();

        block_on(async {
            let result = service.task_store.enqueue().await;

            // Should have created a task
            assert!(
                !result.task.task_id.is_empty(),
                "Task ID should not be empty"
            );
        });
    }

    /// Test enqueued task appears in list
    #[test]
    fn test_enqueued_task_in_list() {
        let service = RudofMcpService::new();

        block_on(async {
            // Enqueue a task
            let enqueue_result = service.task_store.enqueue().await;
            let task_id = enqueue_result.task.task_id.clone();

            // List tasks
            let list_result = service.task_store.list(None).await;

            // Should contain our task
            let task_ids: Vec<_> = list_result.tasks.iter().map(|t| &t.task_id).collect();
            assert!(
                task_ids.contains(&&task_id),
                "Enqueued task should appear in list"
            );
        });
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

// =============================================================================
// Context Storage Tests
// =============================================================================

mod context_storage_tests {
    use super::*;

    /// Helper to run async code in tests
    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    /// Test initial context is None
    #[test]
    fn test_initial_context_is_none() {
        let service = RudofMcpService::new();

        block_on(async {
            let ctx = service.current_context.read().await;
            assert!(ctx.is_none(), "Initial context should be None");
        });
    }
}

// =============================================================================
// Rudof State Tests
// =============================================================================

mod rudof_state_tests {
    use super::*;

    /// Helper to run async code in tests
    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    /// Test rudof instance is accessible
    #[test]
    fn test_rudof_accessible() {
        let service = RudofMcpService::new();

        block_on(async {
            // Should be able to lock rudof
            let _rudof = service.rudof.lock().await;
            // If we get here, we can access the rudof instance
        });
    }

    /// Test rudof can be locked multiple times (via Arc)
    #[test]
    fn test_rudof_multiple_services() {
        let service1 = RudofMcpService::new();
        let service2 = service1.clone();

        block_on(async {
            // Both services should share the same rudof instance
            let guard1 = service1.rudof.lock().await;
            drop(guard1); // Release lock

            let _guard2 = service2.rudof.lock().await;
            // If we get here, Arc sharing works correctly
        });
    }
}
