# Data Generator SHACL Integration Roadmap

This document outlines the changes needed to extend the current ShEx-only data generator to support both ShEx and SHACL schema formats through a unified architecture.

## Current State Analysis

The data generator currently:
- Only supports ShEx schemas via `shex_ast` and `shex_compact`
- Has a `ShapeProcessor` that works exclusively with ShEx AST types
- Uses `ShapeInfo`, `ShapeDependency`, and `PropertyInfo` structs tied to ShEx
- Processes dependencies and generates synthetic data based on ShEx constraints

## Goal

Create a unified system that can:
- Load and process both ShEx and SHACL schemas
- Extract constraints from both formats into a common representation
- Generate synthetic data based on constraints from either schema type
- Maintain backward compatibility with existing ShEx workflows

## Phase 1: Core Infrastructure Changes

### 1.1 Add SHACL Dependencies
**File:** `Cargo.toml`
```toml
# Add to [dependencies]
shacl_ast = { workspace = true }
```

### 1.2 Create Unified Constraint Model
**New File:** `src/unified_constraints.rs`

Create a schema-agnostic constraint model that can represent constraints from both ShEx and SHACL:

```rust
/// Unified constraint model that abstracts over ShEx and SHACL
pub struct UnifiedConstraintModel {
    pub shapes: HashMap<String, UnifiedShape>,
    pub dependencies: HashMap<String, Vec<String>>,
}

pub struct UnifiedShape {
    pub id: String,
    pub target_class: Option<String>,
    pub properties: Vec<UnifiedPropertyConstraint>,
    pub closed: bool,
}

pub struct UnifiedPropertyConstraint {
    pub property_iri: String,
    pub constraints: Vec<UnifiedConstraint>,
    pub min_cardinality: Option<u32>,
    pub max_cardinality: Option<u32>,
}

pub enum UnifiedConstraint {
    Datatype(String),
    ShapeReference(String),
    NodeKind(NodeKind),
    Pattern(String),
    MinInclusive(Value),
    MaxInclusive(Value),
    MinExclusive(Value),
    MaxExclusive(Value),
    MinLength(u32),
    MaxLength(u32),
    In(Vec<Value>),
    HasValue(Value),
}

pub enum NodeKind {
    IRI,
    BlankNode,
    Literal,
    BlankNodeOrIRI,
    BlankNodeOrLiteral,
    IRIOrLiteral,
}

pub enum Value {
    IRI(String),
    Literal(String, Option<String>), // value, datatype
    BlankNode(String),
}
```

### 1.3 Create Schema Format Converters
**New Directory:** `src/converters/`

#### 1.3.1 ShEx to Unified Converter
**New File:** `src/converters/shex_to_unified.rs`

```rust
pub struct ShExToUnified;

impl ShExToUnified {
    pub async fn convert_file<P: AsRef<Path>>(&self, shex_path: P) -> Result<UnifiedConstraintModel>
    pub async fn convert_schema(&self, schema_data: &str) -> Result<UnifiedConstraintModel>
    
    // Private methods for converting ShEx AST elements
    fn convert_shape_decl(&self, shape_decl: &ShapeDecl) -> UnifiedShape
    fn convert_shape_expr(&self, shape_expr: &ShapeExpr) -> Vec<UnifiedPropertyConstraint>
    fn convert_triple_expr(&self, triple_expr: &TripleExpr) -> UnifiedPropertyConstraint
    fn convert_node_constraint(&self, node_constraint: &NodeConstraint) -> Vec<UnifiedConstraint>
}
```

#### 1.3.2 SHACL to Unified Converter  
**New File:** `src/converters/shacl_to_unified.rs`

```rust
pub struct ShaclToUnified;

impl ShaclToUnified {
    pub async fn convert_file<P: AsRef<Path>>(&self, shacl_path: P) -> Result<UnifiedConstraintModel>
    pub async fn convert_schema(&self, schema_data: &str) -> Result<UnifiedConstraintModel>
    
    // Private methods for converting SHACL elements
    fn convert_shape(&self, shape: &Shape) -> UnifiedShape
    fn convert_property_shape(&self, prop_shape: &PropertyShape) -> UnifiedPropertyConstraint
    fn convert_constraints(&self, shape: &Shape) -> Vec<UnifiedConstraint>
}
```

#### 1.3.3 Converter Module
**New File:** `src/converters/mod.rs`

```rust
pub mod shex_to_unified;
pub mod shacl_to_unified;

pub use shex_to_unified::ShExToUnified;
pub use shacl_to_unified::ShaclToUnified;
```

## Phase 2: Refactor Core Components

### 2.1 Enhanced Shape Processor
**File:** `src/shape_processing.rs`

Refactor to use the unified constraint model:

```rust
pub struct ShapeProcessor {
    unified_model: UnifiedConstraintModel,
    shex_converter: ShExToUnified,
    shacl_converter: ShaclToUnified,
}

impl ShapeProcessor {
    // New methods
    pub async fn load_shex_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()>
    pub async fn load_shacl_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()>
    pub async fn load_schema_auto<P: AsRef<Path>>(&mut self, path: P) -> Result<()> // Auto-detect format
    
    // Unified methods that work with both schema types
    pub fn get_unified_shapes(&self) -> &HashMap<String, UnifiedShape>
    pub fn get_dependencies(&self) -> &HashMap<String, Vec<String>>
    pub fn analyze_dependencies(&mut self) -> Result<()>
    
    // Legacy compatibility methods (deprecated but maintained)
    #[deprecated(note = "Use load_shex_schema instead")]
    pub async fn extract_shapes<P: AsRef<Path>>(&mut self, shex_path: P) -> Result<Vec<ShapeDecl>>
}
```

### 2.2 Update Data Generation Pipeline
**File:** `src/parallel_generation.rs`

Modify to work with unified constraints:

```rust
pub struct ParallelGenerator {
    config: GeneratorConfig,
    // Remove ShEx-specific fields, use unified model
}

impl ParallelGenerator {
    pub fn generate_from_unified_model(&self, model: &UnifiedConstraintModel) -> Result<Vec<GeneratedData>>
    
    // Private methods
    fn generate_for_shape(&self, shape: &UnifiedShape) -> Result<GeneratedData>
    fn generate_property_value(&self, constraint: &UnifiedPropertyConstraint) -> Result<Value>
    fn apply_constraint(&self, constraint: &UnifiedConstraint, value: &mut Value) -> Result<()>
}
```

### 2.3 Update Main Generator Interface
**File:** `src/lib.rs`

Add support for both schema types:

```rust
impl DataGenerator {
    // New methods for schema loading
    pub async fn load_shex_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()>
    pub async fn load_shacl_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()>
    pub async fn load_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()> // Auto-detect
    
    // Enhanced generation method
    pub async fn generate_data(&mut self) -> Result<Vec<GeneratedData>>
}
```

## Phase 3: Configuration and CLI Updates

### 3.1 Configuration Enhancements
**File:** `src/config.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    // Add schema format specification
    pub schema_format: Option<SchemaFormat>, // None = auto-detect
    
    // ... existing fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaFormat {
    ShEx,
    SHACL,
}
```

### 3.2 CLI Updates
If there's a CLI component, update it to accept both schema types:

```bash
# Examples of new CLI usage
data-generator --shex schema.shex --output data.ttl
data-generator --shacl schema.ttl --output data.ttl  
data-generator --schema schema.ttl --format auto --output data.ttl
```

## Phase 4: Testing and Validation

### 4.1 Unit Tests
Create comprehensive tests for:
- ShEx to unified conversion
- SHACL to unified conversion  
- Unified constraint model operations
- Data generation from unified constraints

### 4.2 Integration Tests
- Test with real ShEx schemas
- Test with real SHACL schemas
- Test with mixed scenarios
- Validate generated data against original schemas

### 4.3 Performance Testing
- Compare performance between ShEx and SHACL processing
- Ensure unified model doesn't introduce significant overhead

## Phase 5: Documentation and Examples

### 5.1 Update Documentation
- API documentation for new unified approach
- Migration guide for users of the old ShEx-only API
- Examples showing both ShEx and SHACL usage

### 5.2 Example Schemas
Create example schemas in both formats that demonstrate equivalent constraints.

## Implementation Priority

1. **High Priority (Core Functionality)**
   - Unified constraint model (`unified_constraints.rs`)
   - Basic ShEx converter (`shex_to_unified.rs`)
   - Basic SHACL converter (`shacl_to_unified.rs`)
   - Refactored `ShapeProcessor`

2. **Medium Priority (Full Integration)**
   - Complete constraint coverage in converters
   - Updated parallel generation
   - Configuration updates
   - Comprehensive testing

3. **Low Priority (Polish)**
   - Auto-detection of schema formats
   - Performance optimizations
   - Advanced constraint features
   - CLI enhancements

## Backward Compatibility Strategy

To maintain backward compatibility:
1. Keep existing public API methods but mark as deprecated
2. Implement legacy methods as wrappers around new unified approach
3. Provide clear migration path in documentation
4. Support both old and new APIs during transition period

## Risk Mitigation

1. **SHACL AST Familiarity**: Start with basic SHACL constraint mapping
2. **Performance Impact**: Profile unified model overhead
3. **Complexity Management**: Implement incrementally with extensive testing
4. **API Stability**: Use feature flags for new functionality during development

## Success Criteria

1. ✅ Data generator works with both ShEx and SHACL schemas
2. ✅ Generated data quality is equivalent to ShEx-only version
3. ✅ Performance impact is minimal (< 10% overhead)
4. ✅ Existing ShEx workflows continue to work unchanged
5. ✅ Comprehensive test coverage for both schema types
6. ✅ Clear documentation and examples for both formats
