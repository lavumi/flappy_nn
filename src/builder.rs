use specs::{Builder, World, WorldExt};
use crate::components::*;

pub fn background(world : &mut World){
    world.create_entity()
            .with(Tile{
                uv: [0.0, 1.0, 0.0, 1.0],
                atlas: "bg".to_string()
            })
            .with(Transform{
                position: [0., 1., 0.2],
                size: [16.0, 16.0],
            })
            .with(Scroll{})
            .build();
    world.create_entity()
            .with(Tile{
                uv: [0.0, 1.0, 0.0, 1.0],
                atlas: "bg".to_string()
            })
            .with(Transform{
                position: [16., 1., 0.2],
                size: [16.0, 16.0],
            })
            .with(Scroll{})
            .build();
    for i in 0..32 {
        let pos = i as f32 - 7.5;
        world.create_entity()
                .with( Tile {
                    uv: [0.0, 0.125, 0.7142857142,1.0 ],
                    atlas: "tile".to_string(),
                })
                .with( Transform{
                    position: [pos, -8., 0.2],
                    size: [1.0, 2.0],
                })
                .with(Scroll{})
                .build();
    }

}