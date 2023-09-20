use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::components::{BgScroll, PipeScroll, Transform};
use crate::resources::{DeltaTime, GameStage, Stage};
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct UpdatePipe;

const HOLE_SIZE: f32 = 2.0;

impl<'a> System<'a> for UpdatePipe {
    type SystemData = (
        ReadStorage<'a, PipeScroll>,
        WriteStorage<'a, Transform>,
        Read<'a, DeltaTime>,
        Write<'a, ThreadRng>,
        Read<'a, GameStage>
    );

    fn run(&mut self, (sc, mut tf, dt, mut rng, stage): Self::SystemData) {
        if stage.0 != Stage::Run {
            return;
        }
        let mut rand = -1.0 as f32;
        for ( scroll, transform) in ( &sc, &mut tf).join() {
            transform.position[0] -= dt.0;
            if transform.position[0] + transform.size[0]  / 2.0 < -6.0 {
                if rand < 0.0 {
                    rand = rng.gen_range(3.0..7.0);
                }
                transform.position[0] += scroll.reposition_size;

                match scroll.pipe_index {
                    0 => {
                        transform.position[1] = rand - 6.0;
                    }
                    1 => {
                        transform.position[1] = (rand - 6.0) * 0.5 - 4.0;
                        transform.size[1] = rand;
                    }
                    2 => {
                        transform.position[1] = rand + HOLE_SIZE - 4.0;
                    }
                    3 => {
                        transform.position[1] = (rand + HOLE_SIZE -4.0)  * 0.5 + 5.5;
                        transform.size[1] = 13.0 - (rand + HOLE_SIZE );
                    }
                    _ => {}
                }
            }
        }
    }
}