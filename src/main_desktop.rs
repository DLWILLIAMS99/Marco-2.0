//! Marco 2.0 Desktop Application
//! 
//! Main entry point for the desktop version of Marco 2.0 visual coding IDE.
//! Features WGPU rendering, comprehensive error handling, and modular architecture.

use marco2::system::ApplicationState;
use marco2::ui::event::UIEvent;
use marco2::ui::{menu, title_bar, window, event};
use marco2::ui::panels::error_panel::ErrorPanel;
use marco2::MarcoError;
use winit::{
    event::{Event, WindowEvent, KeyEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey},
};
use std::sync::Arc;
use tracing::{info, error, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,marco2=debug")
        .init();

    info!("Starting Marco 2.0 Desktop Application");

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new()
        .with_title("Marco 2.0 - Visual Coding IDE")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_min_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)?);

    // Initialize application state
    let mut app_state = ApplicationState::new();
    
    // Initialize WGPU renderer
    pollster::block_on(async {
        if let Err(e) = app_state.initialize_renderer(&window).await {
            error!("Failed to initialize renderer: {}", e);
            return Err(e);
        }
        Ok::<(), MarcoError>(())
    })?;

    // Initialize UI components
    let mut menu_bar = menu::MenuBar::new();
    menu_bar.add_menu("File".to_string(), vec![
        menu::MenuItem::new("New Project", menu::MenuAction::NewProject),
        menu::MenuItem::new("Open Project", menu::MenuAction::OpenProject),
        menu::MenuItem::new("Save Project", menu::MenuAction::SaveProject),
        menu::MenuItem::new("Save As...", menu::MenuAction::SaveAs),
        menu::MenuItem::separator(),
        menu::MenuItem::new("Exit", menu::MenuAction::Exit),
    ]);
    
    menu_bar.add_menu("Edit".to_string(), vec![
        menu::MenuItem::new("Undo", menu::MenuAction::Undo),
        menu::MenuItem::new("Redo", menu::MenuAction::Redo),
        menu::MenuItem::separator(),
        menu::MenuItem::new("Cut", menu::MenuAction::Cut),
        menu::MenuItem::new("Copy", menu::MenuAction::Copy),
        menu::MenuItem::new("Paste", menu::MenuAction::Paste),
    ]);
    
    menu_bar.add_menu("View".to_string(), vec![
        menu::MenuItem::new("Node Library", menu::MenuAction::ToggleNodeLibrary),
        menu::MenuItem::new("Properties Panel", menu::MenuAction::TogglePropertiesPanel),
        menu::MenuItem::new("Error Log", menu::MenuAction::ToggleErrorPanel),
        menu::MenuItem::separator(),
        menu::MenuItem::new("Debug Tools", menu::MenuAction::ToggleDebugTools),
    ]);
    
    menu_bar.add_menu("Tools".to_string(), vec![
        menu::MenuItem::new("Run Tests", menu::MenuAction::RunTests),
        menu::MenuItem::new("Clear Errors", menu::MenuAction::ClearErrors),
        menu::MenuItem::new("Export Project", menu::MenuAction::ExportProject),
    ]);

    let mut title_bar = title_bar::TitleBar::new("Marco 2.0 - Visual Coding IDE", menu_bar.clone());
    let mut main_window = window::Window::new(title_bar.clone());
    // let mut error_panel = ErrorPanel::new(app_state.error_log.clone());
    
    // Placeholder for error panel functionality
    struct PlaceholderErrorPanel;
    impl PlaceholderErrorPanel {
        fn update(&self, _dt: f32, _theme: &marco2::Marco2Theme) {}
        fn add_error(&self, _error: marco2::MarcoError) {}
    }
    let error_panel = PlaceholderErrorPanel;
    
    // Event handling state
    let mut last_frame_time = std::time::Instant::now();

    info!("Application initialized successfully");

    // Main event loop
    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        info!("Close requested - shutting down gracefully");
                        target.exit();
                    }
                    
                    WindowEvent::Resized(physical_size) => {
                        info!("Window resized to: {:?}", physical_size);
                        app_state.handle_window_resize(*physical_size);
                    }
                    
                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            physical_key: PhysicalKey::Code(key_code),
                            state,
                            ..
                        },
                        ..
                    } => {
                        if state.is_pressed() {
                            match key_code {
                                KeyCode::F1 => {
                                    app_state.toggle_debug_tools();
                                }
                                KeyCode::F2 => {
                                    app_state.toggle_node_library();
                                }
                                KeyCode::F3 => {
                                    app_state.toggle_properties_panel();
                                }
                                KeyCode::F4 => {
                                    app_state.toggle_error_panel();
                                }
                                KeyCode::Escape => {
                                    if app_state.show_error_panel {
                                        app_state.toggle_error_panel();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    WindowEvent::MouseInput { button, state, .. } => {
                        // Handle mouse input for UI interactions
                        if state.is_pressed() {
                            info!("Mouse button {:?} pressed", button);
                            
                            // Simulate menu interactions for demonstration
                            match button {
                                winit::event::MouseButton::Left => {
                                    let ui_event = UIEvent::MenuItemClicked("File".to_string());
                                    if let Err(e) = handle_ui_event(&mut app_state, &ui_event) {
                                        app_state.error_log.log_error(e);
                                    }
                                }
                                winit::event::MouseButton::Right => {
                                    // Right-click could show context menu
                                    let ui_event = UIEvent::ContextMenuRequested { x: 0.0, y: 0.0 };
                                    if let Err(e) = handle_ui_event(&mut app_state, &ui_event) {
                                        app_state.error_log.log_error(e);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    WindowEvent::RedrawRequested => {
                        // Calculate delta time
                        let now = std::time::Instant::now();
                        let dt = now.duration_since(last_frame_time).as_secs_f32();
                        last_frame_time = now;
                        
                        // Update application state
                        if let Err(e) = app_state.update(dt) {
                            app_state.error_log.log_error(e);
                        }
                        
                        // Render frame
                        if let Err(e) = app_state.render() {
                            match e {
                                MarcoError::UI(ref msg) if msg.contains("Surface") => {
                                    // Surface lost, request redraw
                                    window.request_redraw();
                                }
                                _ => {
                                    app_state.error_log.log_error(e);
                                }
                            }
                        }
                        
                        // Update UI components (placeholder - no update methods available)
                        // menu_bar.update(dt, &app_state.theme);
                        // title_bar.update(dt, &app_state.theme);
                        // main_window.update(dt, &app_state.theme);
                        error_panel.update(dt, &app_state.theme);
                        
                        // Render UI components (placeholder - real rendering would use WGPU)
                        if app_state.show_error_panel {
                            let recent_errors = app_state.get_recent_errors(10);
                            for error in &recent_errors {
                                error_panel.add_error(error.clone());
                            }
                        }
                        
                        // Performance logging
                        let perf = app_state.get_performance_info();
                        if app_state.frame_count % 60 == 0 {
                            info!("Performance: {:.1} FPS, {} errors", perf.fps, perf.error_count);
                        }
                    }
                    
                    _ => {}
                }
            }
            
            Event::AboutToWait => {
                // Request redraw for continuous rendering
                window.request_redraw();
            }
            
            _ => {}
        }
    })?;

    Ok(())
}

fn handle_ui_event(app_state: &mut ApplicationState, event: &UIEvent) -> Result<(), MarcoError> {
    match event {
        UIEvent::MenuItemClicked(menu_name) => {
            info!("Menu item clicked: {}", menu_name);
            
            match menu_name.as_str() {
                "New Project" => {
                    app_state.new_project_from_template("Blank Project")?;
                }
                "Open Project" => {
                    app_state.open_project()?;
                }
                "Save Project" => {
                    app_state.save_project()?;
                }
                "Exit" => {
                    info!("Exit requested via menu");
                    std::process::exit(0);
                }
                "Debug Tools" => {
                    app_state.toggle_debug_tools();
                }
                "Error Log" => {
                    app_state.toggle_error_panel();
                }
                "Clear Errors" => {
                    app_state.clear_errors();
                }
                _ => {
                    warn!("Unhandled menu item: {}", menu_name);
                }
            }
        }
        
        UIEvent::ButtonClicked { id } => {
            info!("Button clicked: {}", id);
        }
        
        UIEvent::SliderChanged { id, value } => {
            info!("Slider {} changed to: {}", id, value);
        }
        
        UIEvent::NodeDragged { node_id, position } => {
            info!("Node {} dragged to: {:?}", node_id, position);
        }
        
        UIEvent::ErrorOccurred { error } => {
            app_state.error_log.log_error(MarcoError::Unknown(error.clone()));
        }
        
        UIEvent::ContextMenuRequested { x, y } => {
            info!("Context menu requested at: ({}, {})", x, y);
        }
        
        UIEvent::MouseClick { x: _, y: _, button: _ } => {
            // Handle mouse clicks
        }
        
        UIEvent::MouseMove { x: _, y: _ } => {
            // Handle mouse movement
        }
        
        UIEvent::KeyPress { key: _ } => {
            // Handle key presses
        }
        
        UIEvent::MenuAction(_) => {
            // Handle menu actions
        }
        
        UIEvent::Error(_) => {
            // Handle error events
        }
    }
    
    Ok(())
}
