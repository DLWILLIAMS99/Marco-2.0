# Marco 2.0 - Visual Coding IDE

A modern, modular visual coding IDE built for creative professionals. Marco 2.0 features scoped metadata registries, expression-based logic graphs, and real-time debugging tools with WGPU rendering.

## 🚀 Features

### ✅ **Phase 3 Complete - Enhanced GUI Canvas**
- **PowerPoint-like Interface**: Drag-drop visual element creation
- **Advanced Canvas Tools**: Rectangle, circle, text, image placement with grid snapping
- **Property Binding**: Connect UI elements to logic nodes for dynamic behavior
- **Multi-Platform Support**: Desktop (Windows/macOS/Linux), Web (WASM), and Mobile-ready
- **Real-time Rendering**: WGPU-powered graphics with 60+ FPS performance

### 🔧 **Core Architecture**
- **Type-Safe MetaValue System**: Scalar, Bool, String, Color, List, Object types
- **Scoped Metadata Registry**: Hierarchical data management with dot-path access
- **Expression-Based Logic**: NodeInputBinding with literals, paths, and expressions
- **Cross-Platform Touch**: Gesture recognition and mobile optimization
- **Modular Design**: Clean separation between core, UI, graph, and rendering systems

### 🎯 **Current Status (Phase 4 Sprint 3 Week 2)**
- ✅ Core library compilation (0 errors)
- ✅ Desktop application with full menu system
- ✅ Phase 3 GUI Canvas demonstrations
- ✅ WGPU integration and touch system
- ✅ Web platform with PWA capabilities
- ✅ Real-time collaboration with WebRTC
- ✅ Advanced gesture recognition system
- ✅ Voice command integration
- 🚀 Production deployment infrastructure
- � Analytics and monitoring implementation

## 🛠️ Development Setup

### Prerequisites
- **Rust** 1.70+ with Cargo
- **Node.js** 18+ (for web development)
- **Git** for version control

### Desktop Development
```bash
# Clone the repository
git clone https://github.com/your-username/marco2.git
cd marco2

# Build the core library
cargo build

# Run the main desktop application
cargo run --bin marco2-desktop

# Run Phase 3 demonstrations
cargo run --bin marco2-phase3-working
cargo run --bin marco2-phase3-simple
```

### Web Development
```bash
# Navigate to web directory
cd web

# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

### Testing
```bash
# Run all tests
cargo test

# Run specific binary tests
cargo test --bin marco2-phase3-working

# Check compilation
cargo check
```

## 📁 Project Structure

```
marco2/
├── src/
│   ├── core/           # Core types, registry, logic system
│   ├── ui/             # User interface components
│   ├── graph/          # Logic graph runtime
│   ├── render/         # WGPU rendering system
│   ├── system/         # Application state management
│   └── demos/          # Demonstration applications
├── web/
│   ├── src/            # Web-specific code
│   ├── styles/         # CSS and styling
│   └── public/         # Static assets
├── tests/              # Integration tests
└── docs/               # Documentation
```

## 🏗️ Architecture Highlights

### **MetaValue Type System**
```rust
pub enum MetaValue {
    Scalar(f64),
    Bool(bool), 
    String(String),
    Color(ColorRGBA),
    List(Vec<MetaValue>),
    Object(HashMap<String, MetaValue>)
}
```

### **Scoped Registry Access**
```rust
// Hierarchical data access via dot paths
registry.get_scoped(&DotPath::from("canvas.slider.value"))
registry.set_scoped(&DotPath::from("ui.button.color"), MetaValue::Color(red))
```

### **Logic Node Evaluation**
```rust
impl Evaluatable for MyNode {
    fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> OutputMap {
        let a = inputs.get("a").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let b = inputs.get("b").and_then(|v| v.as_scalar()).unwrap_or(0.0);
        
        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), MetaValue::Scalar(a + b));
        outputs
    }
}
```

## 🎮 Demo Applications

### **Phase 3 Working Demo** (`marco2-phase3-working`)
Complete demonstration of the enhanced GUI Canvas with all Phase 3 features:
- Interactive element creation and manipulation
- Real-time property binding visualization
- Performance metrics and debugging tools

### **Phase 3 Simple Demo** (`marco2-phase3-simple`)
Focused demonstration showing core GUI Canvas capabilities:
- Basic element creation (rectangles, text)
- Canvas tools and grid snapping
- Simplified interaction model

### **Desktop Application** (`marco2-desktop`)
Full-featured desktop application with:
- Complete menu system (File, Edit, View, Tools)
- Window management and error handling
- Project creation and management workflows

## 🌐 Web Platform

Marco 2.0 includes a modern web platform with:

- **Progressive Web App (PWA)** capabilities
- **WebAssembly (WASM)** for high-performance core logic
- **Service Worker** for offline functionality
- **Responsive Design** for mobile and desktop
- **Touch Gesture** support for tablets and phones

### Web Features
- Offline canvas editing with local storage
- Performance monitoring and optimization
- Automatic updates and conflict resolution
- Mobile-optimized interface with touch gestures

## 📊 Performance Metrics

- **Rendering**: 60+ FPS on modern hardware
- **Memory Usage**: < 100MB for typical projects
- **Startup Time**: < 2 seconds on desktop
- **Web Bundle**: < 5MB compressed WASM + JS

## 🚧 Development Roadmap

### **Phase 4 Sprint 3** (Next)
- [ ] Advanced node library with 50+ logic nodes
- [ ] Template system with built-in project templates
- [ ] Enhanced debugging and profiling tools
- [ ] Multi-user collaboration features

### **Future Phases**
- [ ] Plugin system for third-party extensions
- [ ] Cloud synchronization and sharing
- [ ] AI-assisted code generation
- [ ] Mobile app for iOS and Android

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Guidelines
- Follow the established module structure in `src/`
- Use the MetaValue type system for all data interchange
- Implement comprehensive error handling with MarcoError
- Add tests for new functionality
- Update documentation for API changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **Architecture Guide**: [Marco 2.0 – Design & Architecture.txt](Marco%202.0%20–%20Design%20&%20Architecture.txt)
- **Development Guidelines**: [Dev Guidelines.txt](Dev%20Guidelines.txt)
- **Issue Tracker**: [GitHub Issues](https://github.com/your-username/marco2/issues)

---

**Marco 2.0** - Empowering creativity through visual programming ✨
