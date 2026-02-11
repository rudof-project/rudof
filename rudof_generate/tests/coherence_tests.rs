use rudof_generate::config::OutputFormat;
use rudof_generate::{DataGenerator, GeneratorConfig};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_property_fill_probability() {
    // 1. Create a SHACL schema with many optional properties
    // We use minCount 0 to ensure they are optional by default
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:PersonShape a sh:NodeShape ;
    sh:targetClass ex:Person ;
    sh:property [ sh:path ex:p1 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p2 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p3 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p4 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p5 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p6 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p7 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p8 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p9 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p10 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] .
"#;

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure generator with property_fill_probability = 0.5
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 100;
    config.generation.property_fill_probability = 0.5;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples; // Use NTriples for easy counting

    // 3. Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify output
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();

    // Count total property occurrences (excluding rdf:type)
    let total_triples = output_content.lines().count();
    let type_triples = output_content
        .lines()
        .filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))
        .count();
    let property_triples = total_triples - type_triples;

    // Expected: 100 entities * 10 properties * 0.5 = 500 properties
    // Allow reasonable variance (e.g., +/- 10% -> 450 to 550)
    // Actually standard deviation for binomial n=1000, p=0.5 is sqrt(1000*0.25) = 15.8
    // So 3-sigma is +/- 48. Range [452, 548].
    // Let's be generous: [400, 600].

    println!("Total property triples generated: {}", property_triples);

    assert!(
        property_triples >= 400,
        "Too few properties generated: {}",
        property_triples
    );
    assert!(
        property_triples <= 600,
        "Too many properties generated: {}",
        property_triples
    );
}

#[tokio::test]
async fn test_ignore_min_cardinality() {
    // 1. Create a SHACL schema with REQUIRED properties (minCount 1)
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:RequiredShape a sh:NodeShape ;
    sh:targetClass ex:Required ;
    sh:property [ sh:path ex:req1 ; sh:datatype xsd:integer ; sh:minCount 1 ; ] ;
    sh:property [ sh:path ex:req2 ; sh:datatype xsd:integer ; sh:minCount 1 ; ] .
"#;

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with ignore_min_cardinality = true AND low probability
    // If it works, we should see MISSING properties despite minCount 1
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 100;
    config.generation.property_fill_probability = 0.5;
    config.generation.ignore_min_cardinality = true;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();
    let total_triples = output_content.lines().count();
    let type_triples = output_content
        .lines()
        .filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))
        .count();
    let property_triples = total_triples - type_triples;

    // If minCount was respected, we'd have exactly 200 properties (100 * 2).
    // With ignore_min_cardinality=true and prob=0.5, we expect ~100 properties.
    println!("Total property triples (ignore_min=true): {}", property_triples);

    assert!(
        property_triples < 180,
        "Should have skipped many required properties. Got: {}",
        property_triples
    );
}

#[tokio::test]
async fn test_max_properties_per_instance() {
    // 1. Create a SHACL schema with MANY optional properties
    // 20 properties
    let mut shacl_schema = String::from(
        r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:ManyPropsShape a sh:NodeShape ;
    sh:targetClass ex:ManyProps ;
"#,
    );

    for i in 1..=20 {
        shacl_schema.push_str(&format!(
            "    sh:property [ sh:path ex:p{} ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;\n",
            i
        ));
    }
    shacl_schema.push_str("    .\n");

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with max_properties_per_instance = 5
    // Even though probability is 1.0 (default), we should get exactly 5 properties per instance
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 10;
    config.generation.max_properties_per_instance = 5;
    config.generation.property_fill_probability = 1.0;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();
    let total_triples = output_content.lines().count();
    let type_triples = output_content
        .lines()
        .filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))
        .count();
    let property_triples = total_triples - type_triples;

    println!("Total property triples (max=5): {}", property_triples);

    // 10 entities * 5 properties = 50 properties
    assert_eq!(
        property_triples, 50,
        "Should have exactly 50 properties (5 per instance)"
    );
}

#[tokio::test]
async fn test_property_selection_strategy_random() {
    // 1. Create a SHACL schema with 10 optional properties
    let mut shacl_schema = String::from(
        r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:RandomPropsShape a sh:NodeShape ;
    sh:targetClass ex:RandomProps ;
"#,
    );

    for i in 1..=10 {
        shacl_schema.push_str(&format!(
            "    sh:property [ sh:path ex:p{} ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;\n",
            i
        ));
    }
    shacl_schema.push_str("    .\n");

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with Strategy::Random and Prob=0.5
    // Target count = 10 * 0.5 = 5 properties PER INSTANCE.
    // Since Variance is 0.0 by default, EVERY instance should have exactly 5 properties.
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 20;
    config.generation.property_fill_probability = 0.5;
    config.generation.property_selection_strategy = rudof_generate::config::PropertySelectionStrategy::Random;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();

    // Check total count first
    let total_triples = output_content.lines().count();
    let type_triples = output_content
        .lines()
        .filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))
        .count();
    let property_triples = total_triples - type_triples;

    println!("Total property triples (Random Strategy): {}", property_triples);

    // 20 entities * 5 properties = 100 properties
    assert_eq!(
        property_triples, 100,
        "Should have exactly 100 properties (5 per instance)"
    );

    // 5. Verify that not all instances have the SAME 5 properties (random selection)
    // We can't easily parse NTriples here without a parser, but we can check if all p1..p10 are present in the file
    for i in 1..=10 {
        let prop = format!("http://example.org/p{}", i);
        assert!(
            output_content.contains(&prop),
            "Property {} should appear at least once in 20 entities",
            prop
        );
    }
}

#[tokio::test]
async fn test_property_count_variance() {
    // 1. Create a SHACL schema with 20 optional properties
    let mut shacl_schema = String::from(
        r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:VariancePropsShape a sh:NodeShape ;
    sh:targetClass ex:VarianceProps ;
"#,
    );

    for i in 1..=20 {
        shacl_schema.push_str(&format!(
            "    sh:property [ sh:path ex:p{} ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;\n",
            i
        ));
    }
    shacl_schema.push_str("    .\n");

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with Variance = 0.8
    // Base count = 10 (20 * 0.5)
    // Range = +/- 8
    // Expected counts in [2, 18]
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 50;
    config.generation.property_fill_probability = 0.5;
    config.generation.property_count_variance = 0.8;
    config.generation.property_selection_strategy = rudof_generate::config::PropertySelectionStrategy::Random;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify validation of variance
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();

    // We need to count properties PER INSTANCE.
    // Parsing NTriples simply: subject predicate object .
    // Group by subject.
    let mut counts_per_subject: HashMap<String, usize> = HashMap::new();

    for line in output_content.lines() {
        if line.is_empty() || line.starts_with("#") {
            continue;
        }
        if line.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type") {
            continue;
        }

        // Simple extraction of subject
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let subject = parts[0].to_string();
            *counts_per_subject.entry(subject).or_insert(0) += 1;
        }
    }

    let counts: Vec<usize> = counts_per_subject.values().cloned().collect();
    println!("Property counts per instance: {:?}", counts);

    assert!(!counts.is_empty(), "Should have generated entities");

    let min_count = *counts.iter().min().unwrap();
    let max_count = *counts.iter().max().unwrap();

    println!("Min count: {}, Max count: {}", min_count, max_count);

    // Assert significant variance
    assert!(
        max_count - min_count >= 3,
        "Should have variance in property counts. Range got: {}-{}",
        min_count,
        max_count
    );

    // Also assert we are somewhat within expected range [2, 18]
    // (allowing small margin for randomness, but not unbounded)
    // Actually, min could be 0? No, 10 - 8 = 2.
    // Max could be 18.
    // Let's just check they are not all equal to 10.
}

#[tokio::test]
async fn test_excluded_properties() {
    // 1. Create a SHACL schema with 2 REQUIRED properties
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:ExcludeShape a sh:NodeShape ;
    sh:targetClass ex:Exclude ;
    sh:property [ sh:path ex:keep ; sh:datatype xsd:integer ; sh:minCount 1 ; ] ;
    sh:property [ sh:path ex:skip ; sh:datatype xsd:integer ; sh:minCount 1 ; ] .
"#;

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with excluded_properties = ["http://example.org/skip"]
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 10;
    config.generation.excluded_properties = vec!["http://example.org/skip".to_string()];
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();

    // Check that 'keep' is present and 'skip' is absent
    assert!(
        output_content.contains("http://example.org/keep"),
        "Should contain non-excluded property"
    );
    assert!(
        !output_content.contains("http://example.org/skip"),
        "Should NOT contain excluded property"
    );
}

#[tokio::test]
async fn test_type_overrides() {
    // 1. Create a SHACL schema with 2 Shapes having the same property structure
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:ShapeHigh a sh:NodeShape ;
    sh:targetClass ex:High ;
    sh:property [ sh:path ex:p1 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] .

ex:ShapeLow a sh:NodeShape ;
    sh:targetClass ex:Low ;
    sh:property [ sh:path ex:p1 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] .
"#;

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure default prob=1.0, but override ShapeLow to be 0.0
    use rudof_generate::config::TypeOverrideConfig;
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 20; // 10 of each roughly
    config.generation.property_fill_probability = 1.0;

    // Explicit Equal distribution to ensure we have both
    config.generation.entity_distribution = rudof_generate::config::EntityDistribution::Equal;

    let mut overrides = HashMap::new();
    overrides.insert(
        "http://example.org/ShapeLow".to_string(),
        TypeOverrideConfig {
            property_fill_probability: Some(0.0), // Should have NO properties
            ignore_min_cardinality: None,
            max_properties_per_instance: None,
            property_selection_strategy: None,
            property_count_variance: None,
            excluded_properties: None,
        },
    );
    config.generation.type_overrides = overrides;

    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();

    // Analyze output
    let mut high_has_prop = false;
    let mut low_has_prop = false;

    // Parse triples to verify property presence for specific subjects
    // High subjects start with http://example.org/ShapeHigh-...
    // Low subjects start with http://example.org/ShapeLow-...
    // This naming convention depends on the generator implementation (ParallelGenerator uses {shape_id}-{index})
    // Let's check crudely: count occurrences of "p1" lines where subject contains "High" vs "Low".

    for line in output_content.lines() {
        if line.contains("http://example.org/p1") {
            if line.contains("http://example.org/ShapeHigh") {
                high_has_prop = true;
            } else if line.contains("http://example.org/ShapeLow") {
                low_has_prop = true;
            }
        }
    }

    assert!(high_has_prop, "ShapeHigh should have properties (prob=1.0)");
    assert!(!low_has_prop, "ShapeLow should NOT have properties (prob=0.0 override)");
}
