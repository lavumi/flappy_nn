use specs::prelude::World;

#[cfg(not(target_arch = "wasm32"))]
pub use multi_thread::*;
#[cfg(target_arch = "wasm32")]
pub use single_thread::*;

use super::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod multi_thread;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: *mut World);
}

construct_dispatcher!(
    (UpdateCamera, "update_camera", &[]),
    (UpdatePhysics, "update_physics", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}