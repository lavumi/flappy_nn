use specs::{ Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Scroll, Transform};
use crate::resources::DeltaTime;

pub struct UpdateScroll;

impl<'a> System<'a> for UpdateScroll {
    type SystemData = (
        ReadStorage<'a, Scroll>,
        WriteStorage<'a, Transform>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (sc, mut tf, dt): Self::SystemData) {
        for ( _, transform) in ( &sc, &mut tf).join() {
            transform.position[0] -= dt.0;
            if transform.position[0] + transform.size[0]  / 2.0 < -6.0 {
                transform.position[0] += 32.;
            }
        }
    }
}