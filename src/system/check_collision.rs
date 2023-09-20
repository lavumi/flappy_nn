use specs::{Entities, Join, ReadStorage, System};

use crate::components::{  Pipe, Player, Transform};

pub struct CheckCollision;





impl<'a> System<'a> for CheckCollision {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pipe>,
        ReadStorage<'a, Transform>
    );

    fn run(&mut self, (entities,players, pipes,   transforms): Self::SystemData) {

        for ( e, _, player_tr) in  (&entities, &players, &transforms).join() {
            let pt =player_tr.position;
            for (_, pipe_tr) in  ( & pipes, &transforms).join() {
                let obstacle_point = [
                    if pt[0] > pipe_tr.position[0] + pipe_tr.size[0] * 0.5 {
                        pipe_tr.position[0] + pipe_tr.size[0] * 0.5
                    }
                    else if pt[0] < pipe_tr.position[0] - pipe_tr.size[0] * 0.5 {
                        pipe_tr.position[0] - pipe_tr.size[0] * 0.5
                    }
                    else {
                        pt[0]
                    },
                    if pt[1] > pipe_tr.position[1] + pipe_tr.size[1] * 0.5 {
                        pipe_tr.position[1] + pipe_tr.size[1] * 0.5
                    }
                    else if pt[1] < pipe_tr.position[1] - pipe_tr.size[1] * 0.5 {
                        pipe_tr.position[1] - pipe_tr.size[1] * 0.5
                    }
                    else {
                        pt[1]
                    }
                ];

                let dist_pow = (obstacle_point[0] - pt[0]) * (obstacle_point[0] - pt[0]) + (obstacle_point[1] - pt[1]) * (obstacle_point[1] - pt[1]);
                if dist_pow < 0.2 {
                    entities.delete(e).expect("delete player fail!!!");
                    continue;
                }

            }
        }
    }
}