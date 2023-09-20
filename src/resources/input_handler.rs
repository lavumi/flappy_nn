use winit::event::{ElementState, VirtualKeyCode};

pub struct InputHandler{
    pub jump: bool,
}


impl Default for InputHandler {
    fn default() -> Self {
        InputHandler {
            jump: false
        }
    }
}


impl InputHandler {
    pub fn receive_keyboard_input(&mut self, state : ElementState, virtual_keycode: VirtualKeyCode) -> bool {
        match virtual_keycode {
            VirtualKeyCode::Space => {
                match state {
                    ElementState::Pressed => {
                        self.jump = true;
                    }
                    ElementState::Released => {
                        self.jump = false;
                    }
                }
                true
            }
            _ => {
                false
            }
        }
    }
}