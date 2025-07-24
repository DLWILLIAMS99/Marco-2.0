# Marco 2.0 WGPU Migration Plan

## Phase 1: Core Logic Preservation (Keep These)

### ✅ Preserve Existing Core Systems
These systems are platform-agnostic and should be kept:

```
src/core/                   # Complete preservation
├── types/                  # MetaValue, DotPath, ScopeId
├── registry/               # MetaRegistry with scoped access
├── logic/                  # Node system, expression evaluation
└── events/                 # Event system

src/graph/                  # Complete preservation
├── runtime/                # Graph execution engine
└── scope/                  # Scope management

src/project/                # Complete preservation
├── template/               # Template system
└── loader/                 # Project serialization
```

### ✅ Preserve Business Logic
- All node types (Add, Multiply, Constant, etc.)
- Property binding system
- Expression evaluation
- Registry operations
- Graph execution

## Phase 2: New WGPU Foundation

### 🔄 Complete Rebuild Required
```
src/ui/ → src/renderer/     # Completely new WGPU-based system
├── wgpu_context.rs         # Device, queue, surface management
├── shaders/                # WGSL shader library
├── primitives/             # GPU-accelerated shape rendering
├── text/                   # Advanced text rendering
├── canvas/                 # Professional canvas with layers
├── nodes/                  # GPU-accelerated node rendering
└── platform/               # Web, desktop, mobile abstractions
```

### 🆕 New Capabilities Unlocked
- **Cross-Platform Targets**: Web (WASM), Desktop (native), Mobile (future)
- **Professional Graphics**: Hardware-accelerated bezier curves, gradients, shadows
- **Advanced Text**: Rich typography, text on paths, advanced layouts
- **Real-time Effects**: Blur, glow, particle systems, animations
- **Performance**: Handle 10,000+ elements smoothly
- **Custom UI**: Design system matches target platform conventions

## Phase 3: Implementation Approach

### Step 1: New Cargo.toml with WGPU Dependencies
```toml
[dependencies]
# Core logic (preserve)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
uuid = { version = "1.0", features = ["v4", "serde"] }

# WGPU rendering stack
wgpu = "0.19"
winit = "0.29"
pollster = "0.3"
bytemuck = { version = "1.14", features = ["derive"] }
glam = "0.25"  # Math library for 3D transforms

# Advanced text rendering
cosmic-text = "0.10"  # Modern text shaping and layout
fontdue = "0.8"       # Font rasterization

# Platform-specific
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"

# Asset loading and image processing
image = "0.24"
```

### Step 2: Core Architecture
```rust
// New main architecture
pub struct Marco2App {
    // Preserve core logic
    registry: Arc<MetaRegistry>,
    graph_runtime: GraphRuntime,
    
    // New WGPU renderer
    renderer: Marco2Renderer,
    
    // Mode-specific canvases
    logic_canvas: LogicCanvas,
    gui_canvas: GuiCanvas,
    
    // Cross-platform event handling
    platform: PlatformAdapter,
}

pub struct Marco2Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    
    // Rendering pipelines
    shape_pipeline: ShapeRenderPipeline,
    text_pipeline: TextRenderPipeline,
    node_pipeline: NodeRenderPipeline,
    effect_pipeline: EffectRenderPipeline,
}
```

### Step 3: Platform-Specific Targets

#### Desktop (Windows/macOS/Linux)
```rust
// src/main_desktop.rs
fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Marco 2.0")
        .with_inner_size(LogicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap();
    
    let mut app = Marco2App::new_desktop(window);
    
    event_loop.run(move |event, _, control_flow| {
        app.handle_event(event, control_flow);
    });
}
```

#### Web (WASM)
```rust
// src/main_web.rs
#[wasm_bindgen(start)]
pub fn main() {
    console_log::init().expect("Initialize logger");
    
    let event_loop = EventLoop::new();
    let window = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("marco2-canvas"))
        .expect("Canvas element not found");
    
    let window = WindowBuilder::new()
        .with_canvas(Some(window.into()))
        .build(&event_loop)
        .unwrap();
    
    let mut app = Marco2App::new_web(window);
    
    event_loop.run(move |event, _, control_flow| {
        app.handle_event(event, control_flow);
    });
}
```

#### Mobile (Future)
```rust
// src/main_mobile.rs - Future implementation
// Will use same core with mobile-optimized UI
```

## Phase 4: Migration Benefits

### For Non-Programmer Users
1. **Platform Flexibility**: Design once, deploy to web, desktop, and mobile
2. **Professional Tools**: Advanced selection, alignment, and design tools
3. **Visual Effects**: Modern UI effects that match commercial apps
4. **Performance**: Smooth interaction even with complex designs
5. **Export Options**: Generate web apps, desktop apps, or mobile apps

### For the Codebase
1. **Future-Proof**: Modern graphics API with long-term support
2. **Performance**: GPU acceleration for all rendering
3. **Extensibility**: Easy to add new visual features and effects
4. **Cross-Platform**: Single codebase for all targets
5. **Professional Grade**: Matches capabilities of commercial design tools

## Implementation Timeline

### Week 1-2: Foundation
- Set up WGPU context and basic rendering
- Implement primitive shape rendering (rectangles, circles, lines)
- Basic text rendering system
- Mouse/touch input handling

### Week 3-4: Core Canvas
- Implement LogicCanvas with GPU-accelerated nodes
- Professional selection and transformation tools
- Bezier connection rendering
- Grid and guide systems

### Week 5-6: GUI Canvas
- Advanced GUI element rendering
- Layer system with effects
- Professional design tools (alignment, distribution)
- Property panel integration

### Week 7-8: Platform Targets
- Web WASM build target
- Desktop optimization
- Asset pipeline and loading
- Performance profiling and optimization

### Week 9-10: Advanced Features
- Animation system
- Visual effects (shadows, blur, gradients)
- Template system integration
- Export functionality

## Migration Decision

Given your vision of enabling non-programmers to create sophisticated applications without platform limitations, I strongly recommend the **clean slate approach** with careful preservation of your core logic systems.

This approach will:
1. ✅ Preserve all your valuable business logic
2. ✅ Unlock professional-grade design capabilities
3. ✅ Enable true cross-platform deployment
4. ✅ Future-proof the architecture
5. ✅ Provide a foundation for unlimited creative potential

Would you like me to start implementing this migration plan? I can begin with the new Cargo.toml and basic WGPU foundation while ensuring all your core logic systems are preserved and enhanced.
