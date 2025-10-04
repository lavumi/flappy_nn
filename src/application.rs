use instant::Instant;
use wgpu::SurfaceError;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::Window,
    application::ApplicationHandler,
};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use crate::game_configs::GENE_SIZE;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use crate::wasm_bindings::render;


use crate::game_state::GameState;
use crate::renderer::*;

pub struct Application {
    gs : GameState,
    rs : RenderState,

    window: Arc<Window>,
    size: PhysicalSize<u32>,

    prev_mouse_position: PhysicalPosition<f64>,
    prev_time: Instant,


}

impl ApplicationHandler<()> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Application resumed
    }


    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, window_id: winit::window::WindowId, event: WindowEvent) {
        if window_id == self.window.id() {
            if !self.input(&event) {
                match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::KeyboardInput { event: key_event, .. } => {
                        if key_event.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) 
                            && key_event.state == ElementState::Pressed {
                            event_loop.exit();
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.resize(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        let new_size = self.window.inner_size();
                        self.resize(new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        let elapsed_time = self.prev_time.elapsed().as_millis() as f32 / 1000.0;
                        self.prev_time = Instant::now();

                        if elapsed_time > 0.2 {
                            return;
                        }
                        self.update(elapsed_time);
                        match self.render() {
                            Ok(_) => {}
                            Err(SurfaceError::Lost | SurfaceError::Outdated) => self.rs.resize(self.size),
                            Err(SurfaceError::OutOfMemory) => event_loop.exit(),
                            Err(SurfaceError::Timeout) => log::warn!("Surface timeout"),
                            Err(SurfaceError::Other) => log::warn!("Surface error: other"),
                        }
                        
                        #[cfg(target_arch = "wasm32")]
                        {
                            let arr = self.get_gene_data();
                            let str1 = format!("{:?}", arr.0);
                            let str2 = format!("{:?}", arr.1);
                            render(&str1, &str2);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Application {
    pub async fn new(
        window_attributes: winit::window::WindowAttributes,
        event_loop: &EventLoop<()>) -> Self {
        let window = Arc::new(event_loop
            .create_window(window_attributes)
            .unwrap());
        #[cfg(target_arch = "wasm32")]
        {
            // Canvas setup for web
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wgpu-wasm")?;
                    if let Some(canvas) = window.canvas() {
                        let canvas = web_sys::Element::from(canvas);
                        canvas.set_id("wasm-canvas");
                        dst.append_child(&canvas).ok()?;
                    }
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }


        let size = winit::dpi::PhysicalSize::new(
            crate::game_configs::SCREEN_SIZE[0], 
            crate::game_configs::SCREEN_SIZE[1]
        );
        let prev_mouse_position = PhysicalPosition::new(0.0, 0.0);
        let prev_time = Instant::now();

        let mut gs = GameState::default();
        gs.init();
        
        let mut rs = RenderState::new(window.clone()).await;
        rs.init_resources().await;

        Self {
            gs,
            rs,
            window,
            size,
            prev_mouse_position,
            prev_time,
        }
    }


    pub fn get_gene_data(&self) -> ([f32; GENE_SIZE], [f32;2]){
        self.gs.get_gene_data()
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        // let mut renderer = self.gs.world.write_resource::<RenderState>();
        self.rs.resize(new_size);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                self.gs.handle_keyboard_input(key_event.physical_key, key_event.state)
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
        let instances = self.gs.get_tile_instance();
        self.rs.update_mesh_instance(instances);


        let instances = self.gs.set_score_text();
        self.rs.update_text_instance(instances);

        self.rs.render()
    }

}