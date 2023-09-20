use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::components::{ Player, Transform};
use crate::game_configs::{GRAVITY, JUMP_FORCE};
use crate::resources::{DeltaTime, GameStage, InputHandler, Stage};

pub struct UpdatePlayer;

impl<'a> System<'a> for UpdatePlayer {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler>,
        Read<'a, DeltaTime>,
        Read<'a, GameStage>
    );

    fn run(&mut self, (mut players, mut tf, input, dt, stage): Self::SystemData) {
        if stage.0 != Stage::Run {
            return;
        }
        for ( player, transform) in ( &mut players, &mut tf).join() {
            if input.attack1 == true {
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