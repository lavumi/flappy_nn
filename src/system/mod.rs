pub use dispatcher::UnifiedDispatcher;
pub use update_camera::UpdateCamera;
pub use update_physics::UpdatePhysics;

mod update_camera;
mod update_physics;
mod dispatcher;


pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}