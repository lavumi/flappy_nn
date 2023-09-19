use specs::{Entities, Entity, Join, ReadExpect, System, WriteExpect, WriteStorage};

use crate::components::{BodyType, Collider, Transform};

pub struct UpdatePhysics;

impl<'a> System<'a> for UpdatePhysics {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Transform>
    );

    fn run(&mut self, (entities, mut col, mut tf): Self::SystemData) {}
}