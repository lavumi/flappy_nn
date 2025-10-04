// Re-export generic systems from engine
pub use engine::systems::*;

pub use dispatcher::UnifiedDispatcher;
pub use check_collision::CheckCollision;
pub use scroll_background::ScrollBackground;
pub use scroll_pipe::UpdatePipe;
pub use update_player::UpdatePlayer;
pub use check_game_stage::CheckGameStage;
pub use process_nn::ProcessNN;

mod check_collision;
mod dispatcher;
mod scroll_background;
mod scroll_pipe;
mod update_player;
mod check_game_stage;
mod process_nn;


pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}