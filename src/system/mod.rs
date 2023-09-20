pub use dispatcher::UnifiedDispatcher;
pub use update_camera::UpdateCamera;
pub use check_collision::CheckCollision;
pub use scroll_background::ScrollBackground;
pub use scroll_pipe::UpdatePipe;
pub use update_player::UpdatePlayer;
pub use check_game_stage::CheckGameStage;
pub use update_animation::UpdateAnimation;


mod update_camera;
mod check_collision;
mod dispatcher;
mod scroll_background;
mod scroll_pipe;
mod update_player;
mod check_game_stage;
mod update_animation;


pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}