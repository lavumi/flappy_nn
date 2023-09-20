use specs::{Join, Read, System, WriteStorage};

use crate::components::{ Player, Transform};
use crate::game_configs::{GRAVITY, JUMP_FORCE};
use crate::resources::{DeltaTime, InputHandler};

pub struct UpdatePlayer;

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>
    );

    fn run(&mut self, (mut players, mut tf, input, dt): Self::SystemData) {
        for ( player, transform) in ( &mut players, &mut tf).join() {
            if input.jump == true {
                player.jump = true;
            }

            player.force = if player.jump {
                player.jump = false;
                 JUMP_FORCE
            } else {
                player.force - GRAVITY * dt.0
            };

            transform.position[1] += player.force;
        }
    }
}