//! Marco 2.0 Web Entry Point
//! 
//! Web application using WGPU for cross-platform rendering.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod core;
mod graph;
mod devtools;
mod project;
mod renderer;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logging
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    
    tracing::info!("Marco 2.0 Web starting...");
    
    // Web implementation will be added once WGPU foundation is stable
    web_sys::console::log_1(&"Marco 2.0 - Web version starting soon!".into());
    web_sys::console::log_1(&"WGPU migration in progress...".into());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    println!("This is the web entry point, but not running on wasm32");
}
