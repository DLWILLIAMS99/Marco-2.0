//! UI Module - Visual Node Editor and IDE Interface
//! 
//! Contains the main application, node editor, debug overlays, and cross-platform UI systems.

pub mod app;
pub mod node_editor;
pub mod debug_overlay;
pub mod property_panel;
pub mod gui_canvas;
pub mod gui_canvas_simple;
pub mod components;
pub mod theme;
pub mod utils;
pub mod panels;
pub mod menu;
pub mod title_bar;
pub mod window;
pub mod event;
pub mod nodes;
pub mod visual_node_editor;
pub mod template_creator;
pub mod node_library_panel;
pub mod template_gallery;
pub mod integrated_ide;

// Phase 4 Sprint 1: Cross-Platform UI Systems
pub mod responsive;
pub mod touch;
pub mod mobile_canvas;
pub mod cross_platform;

// Re-export core UI types that exist
pub use app::{Marco2App, AppMode};
pub use node_editor::{NodePos, NodeLayout, NodeType, VisualConnection, NodeEditorEvent, NodeEditorResponse};
pub use debug_overlay::DebugOverlay;
pub use property_panel::{PropertyPanel, PropertyPanelResponse};
pub use gui_canvas::{GuiCanvasDesigner, GuiCanvasEvent, GuiCanvasResponse, CanvasTool, GuiElementType};
pub use gui_canvas_simple::SimpleGuiCanvas;
pub use theme::Marco2Theme;
pub use visual_node_editor::VisualNodeEditor;
pub use template_creator::{TemplateCreator, TemplateCategory, GuiElement};
pub use node_library_panel::NodeLibraryPanel;
pub use template_gallery::TemplateGallery;
pub use integrated_ide::{IntegratedIDE, IDEMode};

// Phase 4 Sprint 1: Cross-Platform UI System Re-exports
pub use responsive::{ResponsiveLayout, ScreenSize, LayoutBreakpoint};
pub use touch::{TouchHandler, TouchId, TouchGesture, TouchTool};
pub use mobile_canvas::{MobileCanvasDesigner, ScreenOrientation, MobileContextMenu};
pub use cross_platform::{CrossPlatformUI, PlatformInfo, PlatformType, UIRenderConfig, MouseButton, PerformanceLevel};
