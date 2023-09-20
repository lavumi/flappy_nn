pub use dispatcher::UnifiedDispatcher;
pub use update_camera::UpdateCamera;
pub use update_physics::UpdatePhysics;
pub use update_scroll::UpdateScroll;

mod update_camera;
mod update_physics;
mod dispatcher;
mod update_scroll;


pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}