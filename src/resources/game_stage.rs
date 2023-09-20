#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Stage { Ready, Run, Pause, End }


impl Default for Stage {
    fn default() -> Self {
        Stage::Ready
    }
}


#[derive(Default)]
pub struct GameStage(pub Stage);