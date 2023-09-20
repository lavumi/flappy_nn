use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::components::{ Pipe, Transform};
use crate::resources::{DeltaTime};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::game_configs::{GAME_SPEED, HOLE_SIZE};

pub struct UpdatePipe;



impl<'a> System<'a> for UpdatePipe {
    type SystemData = (
        ReadStorage<'a, Pipe>,
        WriteStorage<'a, Transform>,
        Read<'a, DeltaTime>,
        Write<'a, ThreadRng>
    );

    fn run(&mut self, (sc, mut tf, dt, mut rng): Self::SystemData) {
        let mut rand = -1.0 as f32;
        for ( scroll, transform) in ( &sc, &mut tf).join() {
            transform.position[0] -= dt.0 * GAME_SPEED;
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