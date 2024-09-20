use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Default)]
pub struct InputResult {
    pub request_exit: bool,
    pub pause: bool,
    pub randomize: bool,
    pub clear: bool,
}

pub fn handle_input(input: &WinitInputHelper) -> InputResult {
    InputResult {
        request_exit: input.key_pressed(VirtualKeyCode::Escape) || input.quit(),
        pause: input.key_pressed(VirtualKeyCode::P),
        randomize: input.key_pressed(VirtualKeyCode::R),
        clear: input.key_pressed(VirtualKeyCode::C),
    }
}
