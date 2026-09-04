use crate::config::{CardinalityStrategy, EntityDistribution, GenerationConfig};
use crate::field_generators::{FieldGenerationManager, GenerationContext};
use crate::shape_processing::ShapeInfo;
use crate::unified_constraints::UnifiedConstraint;
use crate::{DataGeneratorError, Result};
use oxrdf::{Literal, NamedNode, NamedOrBlankNode, Term, Triple};
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_impl::OxigraphInMemory;
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Parallel data generator that creates synthetic RDF data
pub struct ParallelGenerator {
    config: GenerationConfig,
    field_manager: FieldGenerationManager,
    shapes: Arc<RwLock<HashMap<String, ShapeInfo>>>,
    generated_entities: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl ParallelGenerator {
    pub fn new(config: &crate::config::GeneratorConfig) -> Result<Self> {
        let field_manager = FieldGenerationManager::new(config.field_generators.clone())?;

        Ok(Self {
            config: config.generation.clone(),
            field_manager,
            shapes: Arc::new(RwLock::new(HashMap::new())),
            generated_entities: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Set the shapes to generate data for
    pub async fn set_shapes(&mut self, shape_infos: Vec<crate::shape_processing::ShapeInfo>) -> Result<()> {
        let mut shapes = self.shapes.write().await;
        shapes.clear();

        // Store the processed ShapeInfo directly
        for shape_info in shape_infos {
            let shape_id = shape_info.declaration.id.to_string();
            shapes.insert(shape_id, shape_info);
        }

        Ok(())
    }

    /// Generate synthetic data in parallel
    pub async fn generate_data(&self, config: &GenerationConfig) -> Result<OxigraphInMemory> {
        // Every random decision below derives its generator from this seed and
        // the coordinates of the decision, so a seeded run is reproducible
        // regardless of how the per-shape tasks interleave.
        if let Some(seed) = config.seed {
            tracing::info!("Using random seed: {}", seed);
        }

        let mut graph = OxigraphInMemory::default();

        // Calculate entity distribution
        let entity_counts = self.calculate_entity_distribution(config).await?;

        // Generate entities for each shape in parallel
        let generation_tasks: Vec<_> = entity_counts
            .into_iter()
            .map(|(shape_id, count)| {
                let shapes = Arc::clone(&self.shapes);
                let generated_entities = Arc::clone(&self.generated_entities);

                async move {
                    self.generate_entities_for_shape(&shape_id, count, shapes, generated_entities)
                        .await
                }
            })
            .collect();

        // Wait for all entity generation tasks to complete
        let entity_results: Result<Vec<Vec<Triple>>> = futures::future::try_join_all(generation_tasks).await;
        let all_triples = entity_results?;

        // Add all triples to the graph
        for triples in all_triples {
            for triple in triples {
                graph
                    .add_triple(triple.subject, triple.predicate, triple.object)
                    .map_err(|e| DataGeneratorError::GraphGeneration(format!("Failed to add triple: {e}")))?;
            }
        }

        // Generate relationships between entities
        self.generate_relationships(&mut graph).await?;

        Ok(graph)
    }

    /// Calculate how many entities to generate for each shape
    async fn calculate_entity_distribution(&self, config: &GenerationConfig) -> Result<HashMap<String, usize>> {
        let shapes = self.shapes.read().await;
        let mut distribution = HashMap::new();

        match &config.entity_distribution {
            EntityDistribution::Equal => {
                let num_shapes = shapes.len();
                if num_shapes == 0 {
                    return Ok(distribution);
                }

                let base_count = config.entity_count / num_shapes;
                let remainder = config.entity_count % num_shapes;

                // Sorted, because the remainder is handed to the first few shapes
                // and `HashMap` iteration order varies between processes. Left
                // unsorted, which shapes receive the extra entity changes from run
                // to run, and with it the size of the generated graph.
                let mut shape_ids: Vec<&String> = shapes.keys().collect();
                shape_ids.sort();

                for (i, shape_id) in shape_ids.into_iter().enumerate() {
                    let count = if i < remainder { base_count + 1 } else { base_count };
                    distribution.insert(shape_id.clone(), count);
                }
            },
            EntityDistribution::Weighted(weights) => {
                let total_weight: f64 = weights.values().sum();
                if total_weight <= 0.0 {
                    return Err(DataGeneratorError::Config("Total weight must be positive".to_string()));
                }

                for shape_id in shapes.keys() {
                    let weight = weights.get(shape_id).unwrap_or(&1.0);
                    let count = ((weight / total_weight) * config.entity_count as f64).round() as usize;
                    distribution.insert(shape_id.clone(), count);
                }
            },
            EntityDistribution::Custom(custom_counts) => {
                for shape_id in shapes.keys() {
                    let count = custom_counts.get(shape_id).unwrap_or(&0);
                    distribution.insert(shape_id.clone(), *count);
                }
            },
        }

        Ok(distribution)
    }

    /// Generate entities for a specific shape
    async fn generate_entities_for_shape(
        &self,
        shape_id: &str,
        count: usize,
        shapes: Arc<RwLock<HashMap<String, ShapeInfo>>>,
        generated_entities: Arc<RwLock<HashMap<String, Vec<String>>>>,
    ) -> Result<Vec<Triple>> {
        let shape_info = {
            let shapes_guard = shapes.read().await;
            shapes_guard
                .get(shape_id)
                .ok_or_else(|| DataGeneratorError::GraphGeneration(format!("Shape not found: {shape_id}")))?
                .clone()
        };

        // Generate entities in batches for better memory usage
        let batch_size = 100;
        let mut all_triples = Vec::new();
        let mut entity_iris = Vec::new();

        for batch_start in (0..count).step_by(batch_size) {
            let batch_end = (batch_start + batch_size).min(count);

            // Generate entities sequentially to handle async
            let mut batch_triples = Vec::new();
            for entity_index in batch_start..batch_end {
                let entity_triples = self.generate_single_entity(&shape_info, entity_index).await?;
                batch_triples.push(entity_triples);
            }

            // Collect entity IRIs and triples
            for (i, triples) in batch_triples.into_iter().enumerate() {
                let entity_iri = format!("{}-{}", shape_id, batch_start + i + 1);
                entity_iris.push(entity_iri);
                all_triples.extend(triples);
            }
        }

        // Store generated entity IRIs
        {
            let mut entities_guard = generated_entities.write().await;
            entities_guard.insert(shape_id.to_string(), entity_iris);
        }

        Ok(all_triples)
    }

    /// Get effective configuration for a specific shape, applying overrides if present
    fn get_effective_config(&self, shape_id: &str) -> GenerationConfig {
        let mut config = self.config.clone();

        if let Some(override_config) = self.config.type_overrides.get(shape_id) {
            if let Some(val) = override_config.property_fill_probability {
                config.property_fill_probability = val;
            }
            if let Some(val) = override_config.ignore_min_cardinality {
                config.ignore_min_cardinality = val;
            }
            if let Some(val) = override_config.max_properties_per_instance {
                config.max_properties_per_instance = val;
            }
            if let Some(val) = override_config.property_selection_strategy {
                config.property_selection_strategy = val;
            }
            if let Some(val) = override_config.property_count_variance {
                config.property_count_variance = val;
            }
            if let Some(val) = &override_config.excluded_properties {
                config.excluded_properties = val.clone();
            }
        }
        config
    }

    /// A random number generator for one decision during generation.
    ///
    /// Generation runs one task per shape, so a single shared generator would
    /// either have to be locked -- serialising the phase the parallelism exists
    /// for -- or produce a different stream depending on how the tasks
    /// interleaved. Instead each decision derives its own generator from the
    /// configured seed and the coordinates of the decision itself, so the
    /// stream a value comes from does not depend on scheduling and the same
    /// seed reproduces the same graph.
    ///
    /// Without a seed the generator is drawn from entropy, which keeps the
    /// unseeded behaviour that callers who do not ask for reproducibility get.
    fn derive_rng(&self, salt: &str, discriminator: &str, index: usize) -> StdRng {
        match self.config.seed {
            Some(seed) => {
                let mut hasher = DefaultHasher::new();
                seed.hash(&mut hasher);
                salt.hash(&mut hasher);
                discriminator.hash(&mut hasher);
                index.hash(&mut hasher);
                StdRng::seed_from_u64(hasher.finish())
            },
            None => StdRng::from_entropy(),
        }
    }

    /// The IRI that entities of `shape_info` are typed with.
    ///
    /// A schema that declares a target -- `sh:targetClass`, or a ShEx `rdf:type`
    /// value set -- is asking for its instances to carry that class, and the
    /// targets it declares select nothing unless they do. Shapes without a
    /// target fall back to the shape IRI, which is the only identifier
    /// available for them.
    fn type_iri_for<'a>(shape_info: &'a ShapeInfo, shape_id: &'a str) -> &'a str {
        shape_info.target_class.as_deref().unwrap_or(shape_id)
    }

    /// Generate a single entity
    async fn generate_single_entity(&self, shape_info: &ShapeInfo, entity_index: usize) -> Result<Vec<Triple>> {
        let mut triples = Vec::new();
        let shape_id = &shape_info.declaration.id.to_string();
        let entity_iri = format!("{}-{}", shape_id, entity_index + 1);
        let entity_node = NamedNode::new_unchecked(&entity_iri);

        // Get effective configuration for this shape
        let config = self.get_effective_config(shape_id);

        // Add type triple. Entities are typed with the class the shape targets,
        // so that the targets declared by the source schema select them; the
        // shape IRI is only a fallback for shapes that declare no target.
        triples.push(Triple::new(
            NamedOrBlankNode::NamedNode(entity_node.clone()),
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Term::NamedNode(NamedNode::new_unchecked(Self::type_iri_for(shape_info, shape_id))),
        ));

        // Generate property triples

        // 0. Calculate effective probability with variance for THIS instance
        let mut variance_multiplier = 1.0;
        if config.property_count_variance > 0.0 {
            use rand::Rng;
            let mut rng = self.derive_rng("variance", shape_id, entity_index);
            // random range [-variance, +variance]
            let variance_factor = rng.gen_range(-config.property_count_variance..=config.property_count_variance);
            variance_multiplier = 1.0 + variance_factor;
        }

        let effective_probability = (config.property_fill_probability * variance_multiplier).clamp(0.0, 1.0);

        // 1. Separate properties into mandatory and candidates
        let mut properties_to_generate = Vec::new();
        let mut candidates = Vec::new();
        let mut mandatory_properties = Vec::new();

        for property_info in &shape_info.properties {
            // Check if property is explicitly excluded
            if config.excluded_properties.contains(&property_info.property_iri) {
                continue;
            }

            // Properties whose range is another shape belong to the relationship
            // linking phase, which resolves them against the entity registries once
            // every shape has been materialised. Emitting them here as well produced
            // two values for every such property -- one pointing at a synthetic
            // nested entity, one at a registry entity -- which violates any
            // declared maximum cardinality of 1.
            if property_info.shape_ref.is_some() {
                continue;
            }

            let min_card = property_info.min_cardinality.unwrap_or(0);
            let effective_min = if config.ignore_min_cardinality { 0 } else { min_card };

            if effective_min > 0 {
                mandatory_properties.push(property_info);
            } else {
                candidates.push(property_info);
            }
        }

        // 2. Select from candidates based on strategy
        use crate::config::PropertySelectionStrategy;
        use rand::seq::SliceRandom;

        match config.property_selection_strategy {
            PropertySelectionStrategy::All | PropertySelectionStrategy::Weighted => {
                // Weighted is treated as All for now (future work)
                // Independent probability check for each candidate using effective_probability
                use rand::Rng;
                let mut rng = self.derive_rng("fill", shape_id, entity_index);
                for prop in candidates {
                    let roll: f64 = rng.r#gen();
                    if roll <= effective_probability {
                        properties_to_generate.push(prop);
                    }
                }
            },
            PropertySelectionStrategy::Random => {
                // Fixed count based on probability
                // Target count includes mandatory properties?
                // Spec says: "num_props = round(property_fill_probability * total_props)"
                // This usually implies total *resulting* properties.

                let total_available = shape_info.properties.len();
                let target_count = (effective_probability * total_available as f64).round() as usize;

                // We must include mandatory ones
                let mandatory_count = mandatory_properties.len();

                // How many candidates to pick?
                // If target <= mandatory, we pick 0 candidates (and just have mandatory).
                // If target > mandatory, we pick (target - mandatory) candidates.
                let mut slots_for_candidates = target_count.saturating_sub(mandatory_count);

                // Clamp to available candidates
                slots_for_candidates = slots_for_candidates.min(candidates.len());

                if slots_for_candidates > 0 {
                    let mut rng = self.derive_rng("select", shape_id, entity_index);
                    candidates.shuffle(&mut rng);
                    for slot in candidates.iter().take(slots_for_candidates) {
                        properties_to_generate.push(slot);
                    }
                }
            },
        }

        // Add mandatory properties
        properties_to_generate.extend(mandatory_properties);

        // 2. Apply max_properties_per_instance limit
        if config.max_properties_per_instance > 0 && properties_to_generate.len() > config.max_properties_per_instance {
            use rand::seq::SliceRandom;
            let mut rng = self.derive_rng("truncate", shape_id, entity_index);
            properties_to_generate.shuffle(&mut rng);
            properties_to_generate.truncate(config.max_properties_per_instance);
        }

        // 3. Generate triples for selected properties
        for property_info in properties_to_generate {
            // Handle cardinality for both data and object properties
            let num_values = self.calculate_property_value_count(
                &property_info.property_iri,
                property_info.min_cardinality,
                property_info.max_cardinality,
                entity_index,
                &config,
            );

            // If we decided to include it but calculated 0 values (e.g. random 0..X),
            // force at least 1 if we passed the probability check?
            // The spec says "generate property with cardinality >= effective_min".
            // calculate_property_value_count respects min_cardinality.

            for value_idx in 0..num_values {
                if let Some(shape_ref) = &property_info.shape_ref {
                    // Object property with shape reference - generate nested entity
                    let nested_entity_iri = format!("{shape_ref}-{entity_index}-{value_idx}");
                    let nested_entity_node = NamedNode::new_unchecked(&nested_entity_iri);

                    // Add triple linking to nested entity
                    triples.push(Triple::new(
                        NamedOrBlankNode::NamedNode(entity_node.clone()),
                        NamedNode::new_unchecked(&property_info.property_iri),
                        Term::NamedNode(nested_entity_node.clone()),
                    ));

                    // Generate nested entity properties if shape is available
                    if let Some(nested_shape_info) = self.get_shape_info(shape_ref).await {
                        let nested_triples = self.generate_nested_entity_properties(
                            &nested_entity_node,
                            &nested_shape_info,
                            entity_index,
                        )?;
                        triples.extend(nested_triples);
                    }
                } else if let Some(datatype) = &property_info.datatype {
                    // Data property with literal value
                    let mut context = GenerationContext::new(
                        property_info.property_iri.clone(),
                        datatype.clone(),
                        format!("{entity_iri}-{value_idx}"),
                    )
                    .with_seed(self.config.seed);

                    // Add constraint parameters to context
                    let constraint_params = self.constraints_to_parameters(&property_info.constraints);
                    for (key, value) in constraint_params {
                        context.parameters.insert(key, value);
                    }

                    let literal_value = self.field_manager.generate_field(&context)?;

                    // Create proper typed literal based on datatype
                    let literal_term = self.create_typed_literal(&literal_value, datatype)?;

                    triples.push(Triple::new(
                        NamedOrBlankNode::NamedNode(entity_node.clone()),
                        NamedNode::new_unchecked(&property_info.property_iri),
                        literal_term,
                    ));
                }
            }
        }

        Ok(triples)
    }

    /// Calculate how many values to generate for a property based on cardinality
    fn calculate_property_value_count(
        &self,
        property_iri: &str,
        min_cardinality: Option<i32>,
        max_cardinality: Option<i32>,
        entity_index: usize,
        config: &GenerationConfig,
    ) -> usize {
        let min_card_raw = min_cardinality.unwrap_or(1).max(0) as usize;
        // Apply ignore_min_cardinality: if true, treat min as 0
        let min_card = if config.ignore_min_cardinality { 0 } else { min_card_raw };
        let max_card = match max_cardinality {
            Some(-1) => 5, // Unbounded, but cap at reasonable limit for properties
            Some(max) => (max as usize).max(min_card),
            None => 1,
        };

        match self.config.cardinality_strategy {
            CardinalityStrategy::Minimum => min_card,
            CardinalityStrategy::Maximum => max_card,
            CardinalityStrategy::Random => {
                use rand::Rng;
                let mut rng = self.derive_rng("value_count", property_iri, entity_index);
                if min_card == max_card {
                    min_card
                } else {
                    rng.gen_range(min_card..=max_card)
                }
            },
            CardinalityStrategy::Balanced => {
                if min_card == max_card {
                    min_card
                } else {
                    min_card + (entity_index % (max_card - min_card + 1))
                }
            },
        }
    }

    /// Create a properly typed literal based on datatype
    fn create_typed_literal(&self, value: &str, datatype: &str) -> Result<Term> {
        match datatype {
            "http://www.w3.org/2001/XMLSchema#string" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            "http://www.w3.org/2001/XMLSchema#integer" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            "http://www.w3.org/2001/XMLSchema#decimal" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            "http://www.w3.org/2001/XMLSchema#boolean" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            "http://www.w3.org/2001/XMLSchema#date" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            "http://www.w3.org/2001/XMLSchema#dateTime" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            "http://www.w3.org/2001/XMLSchema#anyURI" => Ok(Term::Literal(Literal::new_typed_literal(
                value,
                NamedNode::new_unchecked(datatype),
            ))),
            _ => {
                // Default to string literal for unknown types
                Ok(Term::Literal(Literal::new_typed_literal(
                    value,
                    NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#string"),
                )))
            },
        }
    }

    /// Generate relationships between entities
    async fn generate_relationships(&self, graph: &mut OxigraphInMemory) -> Result<()> {
        let shapes = self.shapes.read().await;
        let generated_entities = self.generated_entities.read().await;

        for (shape_id, shape_info) in shapes.iter() {
            if let Some(from_entities) = generated_entities.get(shape_id) {
                for dependency in &shape_info.dependencies {
                    if let Some(to_entities) = generated_entities.get(&dependency.target_shape) {
                        self.generate_relationships_for_dependency(graph, from_entities, to_entities, dependency)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate relationships for a specific dependency
    fn generate_relationships_for_dependency(
        &self,
        graph: &mut OxigraphInMemory,
        from_entities: &[String],
        to_entities: &[String],
        dependency: &crate::shape_processing::ShapeDependency,
    ) -> Result<()> {
        for (idx, from_iri) in from_entities.iter().enumerate() {
            if to_entities.is_empty() {
                continue;
            }

            // Calculate number of relationships based on cardinality strategy
            let num_relationships = self.calculate_relationship_count(
                &dependency.property,
                dependency.min_cardinality,
                dependency.max_cardinality,
                to_entities.len(),
                idx,
            );

            // Select target entities
            let mut selected_targets = Vec::new();
            for i in 0..num_relationships {
                let target_idx = (idx + i) % to_entities.len();
                selected_targets.push(&to_entities[target_idx]);
            }

            // Create relationship triples
            for to_iri in selected_targets {
                let triple = Triple::new(
                    NamedOrBlankNode::NamedNode(NamedNode::new_unchecked(from_iri)),
                    NamedNode::new_unchecked(&dependency.property),
                    Term::NamedNode(NamedNode::new_unchecked(to_iri)),
                );

                graph
                    .add_triple(triple.subject, triple.predicate, triple.object)
                    .map_err(|e| {
                        DataGeneratorError::GraphGeneration(format!("Failed to add relationship triple: {e}"))
                    })?;
            }
        }

        Ok(())
    }

    /// Calculate the number of relationships to create based on cardinality and strategy
    fn calculate_relationship_count(
        &self,
        property_iri: &str,
        min_cardinality: Option<i32>,
        max_cardinality: Option<i32>,
        available_targets: usize,
        entity_index: usize,
    ) -> usize {
        let min_card = min_cardinality.unwrap_or(1).max(0) as usize;
        let max_card = match max_cardinality {
            Some(-1) => available_targets.min(20), // Unbounded, but cap at reasonable limit
            Some(max) => (max as usize).min(available_targets),
            None => 1.min(available_targets),
        };

        let max_card = max_card.max(min_card);

        match self.config.cardinality_strategy {
            CardinalityStrategy::Minimum => min_card,
            CardinalityStrategy::Maximum => max_card,
            CardinalityStrategy::Random => {
                use rand::Rng;
                let mut rng = self.derive_rng("relationship_count", property_iri, entity_index);
                if min_card == max_card {
                    min_card
                } else {
                    rng.gen_range(min_card..=max_card)
                }
            },
            CardinalityStrategy::Balanced => {
                // Use a deterministic but varied approach based on entity index
                if min_card == max_card {
                    min_card
                } else {
                    min_card + (entity_index % (max_card - min_card + 1))
                }
            },
        }
    }

    /// Get shape info by shape ID
    async fn get_shape_info(&self, shape_id: &str) -> Option<ShapeInfo> {
        let shapes = self.shapes.read().await;
        shapes.get(shape_id).cloned()
    }

    /// Generate properties for nested entity
    fn generate_nested_entity_properties(
        &self,
        entity_node: &NamedNode,
        shape_info: &ShapeInfo,
        parent_entity_index: usize,
    ) -> Result<Vec<Triple>> {
        let mut triples = Vec::new();
        let shape_id = &shape_info.declaration.id.to_string();

        // Add type triple for nested entity
        triples.push(Triple::new(
            NamedOrBlankNode::NamedNode(entity_node.clone()),
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            Term::NamedNode(NamedNode::new_unchecked(Self::type_iri_for(shape_info, shape_id))),
        ));

        // Generate properties for nested entity (only data properties to avoid infinite recursion)
        for property_info in &shape_info.properties {
            if property_info.shape_ref.is_some() {
                // Skip nested object properties to avoid infinite recursion
                continue;
            }

            if let Some(datatype) = &property_info.datatype {
                let num_values = self.calculate_property_value_count(
                    &property_info.property_iri,
                    property_info.min_cardinality,
                    property_info.max_cardinality,
                    parent_entity_index,
                    &self.config,
                );

                for value_idx in 0..num_values {
                    let context = GenerationContext::new(
                        property_info.property_iri.clone(),
                        datatype.clone(),
                        format!("{}-nested-{}", entity_node.as_str(), value_idx),
                    )
                    .with_seed(self.config.seed);

                    let literal_value = self.field_manager.generate_field(&context)?;
                    let literal_term = self.create_typed_literal(&literal_value, datatype)?;

                    triples.push(Triple::new(
                        NamedOrBlankNode::NamedNode(entity_node.clone()),
                        NamedNode::new_unchecked(&property_info.property_iri),
                        literal_term,
                    ));
                }
            }
        }

        Ok(triples)
    }

    /// Convert unified constraints to generation context parameters
    fn constraints_to_parameters(&self, constraints: &[UnifiedConstraint]) -> HashMap<String, Value> {
        let mut params = HashMap::new();

        for constraint in constraints {
            match constraint {
                UnifiedConstraint::MinInclusive(crate::unified_constraints::Value::Literal(val, _)) => {
                    if let Ok(i) = val.parse::<i64>() {
                        params.insert("min".to_string(), json!(i));
                    } else if let Ok(f) = val.parse::<f64>() {
                        params.insert("min".to_string(), json!(f));
                    }
                },
                UnifiedConstraint::MaxInclusive(crate::unified_constraints::Value::Literal(val, _)) => {
                    if let Ok(i) = val.parse::<i64>() {
                        params.insert("max".to_string(), json!(i));
                    } else if let Ok(f) = val.parse::<f64>() {
                        params.insert("max".to_string(), json!(f));
                    }
                },
                UnifiedConstraint::MinExclusive(crate::unified_constraints::Value::Literal(val, _)) => {
                    if let Ok(i) = val.parse::<i64>() {
                        params.insert("min".to_string(), json!(i + 1));
                    } else if let Ok(f) = val.parse::<f64>() {
                        params.insert("min".to_string(), json!(f + 0.001));
                    }
                },
                UnifiedConstraint::MaxExclusive(crate::unified_constraints::Value::Literal(val, _)) => {
                    if let Ok(i) = val.parse::<i64>() {
                        params.insert("max".to_string(), json!(i - 1));
                    } else if let Ok(f) = val.parse::<f64>() {
                        params.insert("max".to_string(), json!(f - 0.001));
                    }
                },
                UnifiedConstraint::MinLength(len) => {
                    params.insert("min_length".to_string(), json!(*len));
                },
                UnifiedConstraint::MaxLength(len) => {
                    params.insert("max_length".to_string(), json!(*len));
                },
                UnifiedConstraint::Pattern(pattern) => {
                    params.insert("pattern".to_string(), json!(pattern));
                },
                UnifiedConstraint::In(values) => {
                    let json_values: Vec<Value> = values
                        .iter()
                        .map(|v| match v {
                            crate::unified_constraints::Value::Literal(val, _) => json!(val),
                            crate::unified_constraints::Value::Iri(iri) => json!(iri),
                            _ => json!(null),
                        })
                        .collect();
                    params.insert("values".to_string(), json!(json_values));
                },
                UnifiedConstraint::HasValue(value) => match value {
                    crate::unified_constraints::Value::Literal(val, _) => {
                        params.insert("fixed_value".to_string(), json!(val));
                    },
                    crate::unified_constraints::Value::Iri(iri) => {
                        params.insert("fixed_value".to_string(), json!(iri));
                    },
                    _ => {},
                },
                _ => {}, // Other constraints not implemented yet
            }
        }

        params
    }
}
