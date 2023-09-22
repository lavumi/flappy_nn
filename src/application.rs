use instant::Instant;
use wgpu::SurfaceError;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::game_state::GameState;
use crate::renderer::*;

pub struct Application {
    gs : GameState,
    rs : RenderState,

    window: Window,
    size: PhysicalSize<u32>,

    prev_mouse_position: PhysicalPosition<f64>,
    prev_time: Instant,


}

impl Application {
    pub async fn new(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>) -> Self {
        let window = window_builder
            .build(&event_loop)
            .unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            // use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(1600, 900));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wgpu-wasm")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    canvas.set_id("wasm-canvas");
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }


        let mut gs = GameState::default();
        gs.init();
        let mut rs = RenderState::new(&window).await;
        rs.init_resources().await;




        let size = window.inner_size();
        let prev_mouse_position = PhysicalPosition::new(0.0, 0.0);
        let prev_time = Instant::now();


        Self {
            gs,
            rs,
            window,
            size,
            prev_mouse_position,
            prev_time,
        }
    }

    pub fn run(
        &mut self,
        event: &Event<'_, ()>,
        control_flow: &mut ControlFlow, // TODO: Figure out if we actually will use this...
    ) {
        match event {
            Event::WindowEvent { ref event, window_id, }
            if window_id == &self.window.id() => {
                if !self.input(event) {
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == &self.window.id() => {
                let elapsed_time = self.prev_time.elapsed().as_millis() as f32 / 1000.0;
                self.prev_time = Instant::now();

                //todo fix 처음 시작할때 elapse time 이 한순간 튀는데 이거 원인 찾아보자. + 처음 켜져마자 게임이 시작되면 안되는데...
                if elapsed_time > 0.2 {
                    return;
                }
                self.update(elapsed_time);
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => self.rs.resize(self.size),
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                self.window.request_redraw();
            }
            _ => {}
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        // let mut renderer = self.gs.world.write_resource::<RenderState>();
        self.rs.resize(new_size);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                self.gs.handle_keyboard_input(input)
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.prev_mouse_position = position.clone();
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => {
                        // self.toggle_full_screen();
                    }
                    _ => {}
                }
                false
            }
            _ => false,
        }
    }

    fn update(&mut self, dt: f32) {
        self.gs.update(dt);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        //1. update camera
        let camera_uniform = self.gs.get_camera_uniform();
        self.rs.update_camera_buffer(camera_uniform);


        // //2. update meshes
        // let rt_map_tiles = self.gs.get_map_instance();
        // self.rs.update_map_instance(rt_map_tiles);
        //
        //
        let instances = self.gs.get_tile_instance();
        self.rs.update_mesh_instance_bulk(instances);

        self.rs.render()
    }
}