use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::components::{BgScroll, Transform};
use crate::resources::{DeltaTime, GameStage, Stage};
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct UpdateScroll;

impl<'a> System<'a> for UpdateScroll {
    type SystemData = (
        ReadStorage<'a, BgScroll>,
        WriteStorage<'a, Transform>,
        Read<'a, DeltaTime>,
        Read<'a, GameStage>
    );

    fn run(&mut self, (sc, mut tf, dt, stage): Self::SystemData) {
        if stage.0 != Stage::Run {
            return;
        }
        for ( scroll, transform) in ( &sc, &mut tf).join() {
            transform.position[0] -= dt.0;
            if transform.position[0] + transform.size[0]  / 2.0 < -6.0 {
                transform.position[0] += scroll.reposition_size;
            }
        }
    }
}