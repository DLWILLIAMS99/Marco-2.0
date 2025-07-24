# Marco 2.0 - Copilot Instructions

## Project Overview
Marco 2.0 is a visual coding IDE built on Rust, featuring scoped metadata registries, expression-based logic graphs, and real-time debugging tools. The architecture emphasizes type safety, modularity, and introspective capabilities.

## Core Architecture Patterns

### Module Structure
Follow the established module layout:
- `core/types` - Foundational types (`DotPath`, `MetaValue`, `ScopeId`)
- `core/registry` - Metadata registry with scoped access
- `core/logic` - Logic nodes (`NodeInputBinding`, `BindingExpr`, `Evaluatable`)
- `graph/runtime` - Logic graph execution per scope
- `devtools/debug` - Debugging overlays and inspection tools

### Type System Rules
```rust
// Always use MetaValue for registry data
pub enum MetaValue {
    Scalar(f64), Bool(bool), String(String), 
    Color(ColorRGBA), List(Vec<MetaValue>),
    Object(HashMap<String, MetaValue>)
}

// Registry access via scoped dot paths
registry.get_scoped(&DotPath::from("canvas.slider.value"))
```

### Logic Node Pattern
All logic nodes implement `Evaluatable`:
```rust
impl Evaluatable for MyNode {
    fn evaluate(&self, inputs, _ctx) -> HashMap<String, MetaValue> {
        // Never panic - use safe unwrapping with defaults
        let a = inputs.get("a").and_then(|v| v.as_scalar()).unwrap_or(0.0);

### NodeInputBinding Variants
NodeInputBinding::Literal(MetaValue::Bool(true))
NodeInputBinding::Path(DotPath::from("canvas.toggle.active"))
NodeInputBinding::Expression(BindingExpr::Binary(...))
```

### Expression Trees
Use `Box` for nested expressions:
BindingExpr::Binary(
    BinaryOp::Greater,
    Box::new(BindingExpr::Ref(DotPath::from("a"))),
)
```
## Critical Safety Rules
- **All structs derive**: `Debug, Clone, Serialize, Deserialize`
- **Never use `.unwrap()`** except when guarded or unreachable
- **Registry access only via `DotPath`** - no raw strings
- **No panics in logic evaluation** - always provide fallbacks

## Snapshot & Template System
- Use hash-compressed snapshots: `HashKey → MetaValue`
- Store `DotPath` separately from snapshots via `.metamap.json`

## Devtools Conventions
- Add hotkey toggles for debug panels (e.g., Ctrl+D)
- Color coding: Blue=added, Red=removed, Yellow=modified
- All debug overlays are opt-in and non-intrusive

## Debugging & Validation Workflow
When implementing UI interactions or logic flows:
- **Add validation logs** at key interaction points (button clicks, state changes)
- **Use `tracing::info!()` macros** to track execution flow
- **Create test harnesses** for complex interactions before full UI integration
- **Implement assertion helpers** that can verify expected state transitions

    assert!(self.is_valid_state(), "Invalid state before button action");
    
    assert!(self.action_completed(), "Action failed to complete");
}
```rust
// Template maintains separate scopes
    is_editable: true
}
```

// All paths rebased to single scope, overlay removed
FlattenedObject {
}
```
- Track merge history for potential "unmerge" operations
- Rebase all internal `DotPath` references during flatten

## Scaffolding Architecture Principles


### 1. Trait-First Design
Define interfaces before implementations:
```rust
// Define the contract first
pub trait Evaluatable {
    fn evaluate(&self, inputs: &InputMap, ctx: &EvalContext) -> OutputMap;
}

// Then implement specific nodes
impl Evaluatable for ConditionalNode { /* ... */ }
```

### 2. Extensible Core Types
Build `MetaValue` and `BindingExpr` to accommodate future variants without breaking changes:
```rust
#[non_exhaustive] // Allows adding variants later
pub enum MetaValue {
    Scalar(f64),
    // ... existing variants
    // Reserved space for: Matrix, Texture, Curve, etc.
}
```

### 3. Modular Component Boundaries
Structure modules with clear separation:
- `core/` - Never depends on UI or project-specific logic
- `graph/` - Can use core, but not UI or persistence
- `ui/` - Can use core and graph, but not persistence internals
- `project/` - Top-level orchestration only

### 4. Future-Proof APIs
Design public interfaces for extension:
```rust
// Good: Extensible config pattern
pub struct NodeConfig {
    pub inputs: Vec<InputSpec>,
    pub outputs: Vec<OutputSpec>,
    pub metadata: HashMap<String, MetaValue>, // Extensible
}

// Avoid: Hard-coded parameter lists that require signature changes
```

## Build Features
Use feature flags for optional components:
- `remote_mounts` - Serial/WebSocket adapters
- `devtools` - Inspector and tracer tools
- `ai_tools` - AI-assisted generation
- `plugin_support` - External plugin APIs

When implementing new features, reference `Prompt Cues.txt` for detailed type usage examples and `Dev Guidelines.txt` for architectural standards.
