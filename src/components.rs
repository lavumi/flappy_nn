use specs::*;
use specs_derive::Component;





#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum BodyType { Static, Kinematic, Dynamic }



#[derive(Component, Clone)]
pub struct Collider {
    pub aabb_offset: [f32; 4],
    pub velocity: [f32; 2],
    pub is_trigger: bool,
    pub body_type: BodyType,
}
impl Default for Collider {
    fn default() -> Self {
        Collider {
            aabb_offset: [-1.0, 0.0, -0.25, 0.25],
            velocity: [0., 0.],
            is_trigger: false,
            body_type: BodyType::Kinematic,
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
impl Transform {
    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let position = cgmath::Vector3 { x: self.position[0], y: self.position[1], z: self.position[2] };
        let translation_matrix = cgmath::Matrix4::from_translation(position);
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.size[0], self.size[1], 1.0);
        let model = (translation_matrix * scale_matrix).into();
        model
    }
}

#[derive(Component, Clone)]
pub struct BgScroll {
    pub reposition_size : f32,
}

#[derive(Component, Clone)]
pub struct PipeScroll {
    pub reposition_size : f32,
    pub pipe_index : u8,
}