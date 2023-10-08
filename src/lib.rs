#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::application::Application;
use crate::winit_state::WinitState;

mod renderer;
pub mod winit_state;
pub mod application;
mod components;
mod resources;
mod system;
mod game_state;
mod builder;
mod game_configs;
mod utils;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn start(){
    let title = "wgpu_wasm";
    let width = game_configs::SCREEN_SIZE[0];
    let height = game_configs::SCREEN_SIZE[1];

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

            #[wasm_bindgen(module = "/defined-in-js.js")]
            extern "C" {
                fn render(gene : &str,pos : &str);
            }

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = console)]
                fn log(s: &str);
            }
        } else {
            env_logger::init();
        }
    }

    let (wb, event_loop) = WinitState::create(title, width, height );
    // let asset_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/").to_string();
    let mut application = Application::new(wb, &event_loop).await;
    event_loop.run(move |event, _,control_flow| {
        application.run(&event, control_flow);
        #[cfg(target_arch = "wasm32")]
        {
            let arr = application.get_gene_data();
            let str1 = format!("{:?}", arr.0);
            let str2 = format!("{:?}", arr.1);
            render(&str1 , &str2);
        }
    });



}


// #[cfg(target_arch = "wasm32")]
// fn main() {
//     use wasm_bindgen::prelude::*;
//
//
//
//     // lifted from the `console_log` example
//
//
//     #[wasm_bindgen(start)]
//     pub fn run() {
//         log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"
//
//         let x = MyClass::new();
//         assert_eq!(x.number(), 42);
//         x.set_number(10);
//         log(&x.render());
//     }
//
// }