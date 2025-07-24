//! Marco 2.0 Main Entry Point
//! 
//! Cross-platform visual coding IDE with WGPU-powered rendering.
//! This is the default desktop entry point.

mod core;
mod graph;
mod devtools;
mod project;
mod renderer;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Default to desktop for now, but can be conditional based on features
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing::info!("Starting Marco 2.0 Desktop");
        // Will implement desktop main once we have the WGPU foundation
        println!("Marco 2.0 - Desktop version starting soon!");
        println!("WGPU migration in progress...");
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        tracing::info!("Starting Marco 2.0 Web");
        println!("Marco 2.0 - Web version starting soon!");
    }
}
