use specs::*;
use specs_derive::Component;
use crate::game_configs::GENE_SIZE;


#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum BodyType { Static, Kinematic, Dynamic }

#[derive(Component, Clone)]
pub struct Collider {
    pub aabb_offset: [f32; 4],
}
impl Default for Collider {
    fn default() -> Self {
        Collider {
            aabb_offset: [-1.0, 0.0, -0.25, 0.25],
        }
    }
}

#[derive(Component, Clone)]
pub struct Tile {
    pub uv: [f32; 4],
    pub atlas: String,
}

#[derive(Component, Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub size: [f32; 2],
}

#[derive(Component, Clone)]
pub struct Text {
    pub content: String,
    pub color : [f32;3]
}

#[derive(Component, Clone)]
pub struct Background {
    pub reposition_size : f32,
}

#[derive(Component, Clone)]
pub struct Pipe {
    pub reposition_size : f32,
    pub pipe_index : u8,
}

#[derive(Component, Clone)]
pub struct PipeTarget {}

#[derive(Component, Clone, Default)]
pub struct Player {
    pub force: f32,
    pub jump : bool,
}

#[derive(Component, Clone, Default)]
pub struct Animation {
    pub index : u32,
    pub delta : f32,
}





//region [ Neural Network ]
#[derive(Component, Clone, Default)]
pub struct NeuralLayer {
    pub weights: Vec<Vec<f32>>,
    pub values : Vec<f32>,
    pub bias : Vec<f32>
}


#[derive(Component, Clone)]
pub struct DNA {
    pub hidden_layers:[usize;2],
    pub genes:[f32;GENE_SIZE],
    pub index:usize,
}


//endregion