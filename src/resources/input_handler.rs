use winit::event::{ElementState, VirtualKeyCode};

pub struct InputHandler{
    pub attack1: bool,
}


impl Default for InputHandler {
    fn default() -> Self {
        InputHandler {
            attack1: false
        }
    }
}


impl InputHandler {
    pub fn receive_keyboard_input(&mut self, state : ElementState, virtual_keycode: VirtualKeyCode) -> bool {
        match virtual_keycode {
            VirtualKeyCode::Space => {
                match state {
                    ElementState::Pressed => {
                        self.attack1 = true;
                    }
                    ElementState::Released => {
                        self.attack1 = false;
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