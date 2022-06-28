use glium::glutin::{event::VirtualKeyCode, event_loop::ControlFlow};

pub fn match_event_keyboard(
    key: VirtualKeyCode,
    control_flow: &mut ControlFlow,
    speed: &mut f32,
    object: &mut [f32; 3],
    player: &mut [f32; 6],
    rotations: &mut (f32, usize, bool),
    is_textured: &mut bool,
    is_enlightened: &mut bool,
    center: &[f32; 3],
) {
    match key {
        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
        // Speed
        VirtualKeyCode::Plus | VirtualKeyCode::NumpadAdd => {
            if *speed < 1000. {
                *speed += 0.1
            }
        }
        VirtualKeyCode::Minus | VirtualKeyCode::NumpadSubtract => {
            if *speed > 0.1 {
                *speed += 0.1
            }
        }
        // Object Rotation
        VirtualKeyCode::R => rotations.1 = (rotations.1 + 1) % 7,
        VirtualKeyCode::Space => rotations.2 = !rotations.2,
        // Object Translation
        VirtualKeyCode::Right => object[0] += *speed,
        VirtualKeyCode::Left => object[0] -= *speed,
        VirtualKeyCode::PageUp => object[1] += *speed,
        VirtualKeyCode::PageDown => object[1] -= *speed,
        VirtualKeyCode::Up => object[2] += *speed,
        VirtualKeyCode::Down => object[2] -= *speed,
        // Player Translation
        VirtualKeyCode::D => player[0] += *speed,
        VirtualKeyCode::A => player[0] -= *speed,
        VirtualKeyCode::Home => player[1] += *speed,
        VirtualKeyCode::End => player[1] -= *speed,
        VirtualKeyCode::W => player[2] += *speed,
        VirtualKeyCode::S => player[2] -= *speed,
        // disable/enable textures/light
        VirtualKeyCode::T => *is_textured = !*is_textured,
        VirtualKeyCode::L => *is_enlightened = !*is_enlightened,
        // Center vision on object
        VirtualKeyCode::C => {
            player[3] = (object[0] + center[0]) - player[0];
            player[4] = (object[1] + center[1]) - player[1];
            player[5] = (object[2] + center[2]) - player[2];
        }
        _ => return,
    }
}
