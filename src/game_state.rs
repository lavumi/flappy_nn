use std::collections::HashMap;
use rand::rngs::ThreadRng;
use specs::{ Join, World, WorldExt};
use winit::event::{ElementState, VirtualKeyCode};
use crate::builder::{background, pipe, player};

use crate::components::*;
use crate::renderer::InstanceTileRaw;
use crate::resources::*;
use crate::system;
use crate::system::UnifiedDispatcher;


#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Stage { Ready, Run, Pause, End }


impl Default for Stage {
    fn default() -> Self {
        Stage::Ready
    }
}


pub struct GameState {
    pub world: World,
    dispatcher: Box<dyn UnifiedDispatcher + 'static>,
    stage : Stage
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            world: World::new(),
            dispatcher: system::build(),
            stage : Stage::Ready
        }
    }
}



impl GameState {
    pub fn init(&mut self) {
        self.world.register::<Transform>();
        self.world.register::<Collider>();
        self.world.register::<Tile>();
        self.world.register::<Background>();
        self.world.register::<Player>();
        self.world.register::<Pipe>();

        self.world.insert(Camera::init_orthographic(5, 9));
        self.world.insert(DeltaTime(0.05));
        self.world.insert(GameFinished(false));
        self.world.insert(ThreadRng::default());
        self.world.insert(InputHandler::default());


        self.init_game();

    }



    fn init_game(&mut self){

        self.world.delete_all();
        background(&mut self.world);
        pipe(&mut self.world, 16.);
        pipe(&mut self.world, 8.);
        player(&mut self.world);

        let mut finished = self.world.write_resource::<GameFinished>();
        *finished = GameFinished(false);

        let mut inputs = self.world.write_resource::<InputHandler>();
        *inputs = InputHandler::default();

        self.stage = Stage::Ready;
    }

    fn check_game_finished(&mut self ) {
        let finished = self.world.read_resource::<GameFinished>();
        if finished.0 == true {
            self.stage = Stage::End;
        }
    }

    fn update_delta_time(&mut self, dt : f32 ){
        let mut delta = self.world.write_resource::<DeltaTime>();
        *delta = DeltaTime(dt);
    }

    pub fn update(&mut self, dt: f32) {
        self.check_game_finished();
        if self.stage != Stage::Run {
            return;
        }

        self.update_delta_time(dt);
        self.dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn handle_keyboard_input(&mut self, input: &winit::event::KeyboardInput) -> bool {
        // let game_stage = self.world.write_resource::<GameFinished>();
        match self.stage {
            Stage::End => {
                if input.virtual_keycode.is_none() == false &&  input.state == ElementState::Released {
                    self.init_game();
                }
                return true;
            }
            Stage::Ready | Stage::Pause => {
                if input.virtual_keycode.is_none() == false &&  input.state == ElementState::Released {
                   self.stage = Stage::Run;
                }
                return true;
            }
            Stage::Run => {
                match input.virtual_keycode {
                    Some(code) if code == VirtualKeyCode::P => {
                        if input.state == ElementState::Released {
                            self.stage = Stage::Pause;
                        }
                        return true;
                    }
                    Some(_) => {
                        let mut input_handler = self.world.write_resource::<InputHandler>();
                        input_handler.receive_keyboard_input(input.state, input.virtual_keycode.unwrap())
                    }
                    None => {
                        return false;
                    }
                }

            }
        }
    }

    pub fn get_camera_uniform(&self) -> [[f32; 4]; 4] {
        let camera = self.world.read_resource::<Camera>();
        let camera_uniform = camera.get_view_proj();
        return camera_uniform;
    }

    pub fn get_tile_instance(&self) -> HashMap<String, Vec<InstanceTileRaw>> {
        let tiles = self.world.read_storage::<Tile>();
        let transforms = self.world.read_storage::<Transform>();
        let rt_data = (&tiles, &transforms).join().collect::<Vec<_>>();

        let mut tile_instance_data_hashmap = HashMap::new();
        for (tile, transform) in rt_data {
            let atlas = tile.atlas.clone();
            let instance = InstanceTileRaw {
                uv: tile.uv.clone(),
                model: transform.get_matrix(),
            };

            tile_instance_data_hashmap
                    .entry(atlas)
                    .or_insert_with(Vec::new)
                    .push(instance);
        }

        tile_instance_data_hashmap
    }
}