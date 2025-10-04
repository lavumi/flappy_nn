#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::application::Application;
use crate::winit_state::WinitState;

// Re-export renderer from engine
pub use engine::renderer;

pub mod winit_state;
pub mod application;
mod components;
mod resources;
mod system;
mod game_state;
mod builder;
mod game_configs;

#[cfg(target_arch = "wasm32")]
mod wasm_bindings;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn start(){
    let title = "wgpu_wasm";
    let width = game_configs::SCREEN_SIZE[0];
    let height = game_configs::SCREEN_SIZE[1];

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let (wb, event_loop) = WinitState::create(title, width, height );
    // let asset_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/").to_string();
    let mut application = Application::new(wb, &event_loop).await;
    event_loop.run_app(&mut application).unwrap();
}

