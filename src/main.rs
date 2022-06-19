#[macro_use]
extern crate glium;

mod teapot;

pub fn mult_m(a: [[f32; 4]; 4], b: &[[f32; 4]; 4], model_position: &[f32; 4]) -> [[f32; 4]; 4] {
    let mut out = [
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
        [0., 0., 0., 0.],
    ];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    out[3] = *model_position;
    out
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        uniform mat4 transformmodel;

        void main() {
            mat4 modeltransformed = transformmodel * model;
            mat4 modelview = view * modeltransformed;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.0, 0.6, 0.0);
            vec3 regular_color = vec3(0.0, 1.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut rotations: (f32, usize, bool) = (0.0, 0, true);
    let mut object: [f32; 3] = [0.0, 0.0, 250.0];
    let mut player: [f32; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let mut last_mouse_position: [f64; 2] = [0.0, 0.0];
    let speed: f32 = 5.0;

    let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        if rotations.2 {
            rotations.0 += 0.001;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let model_position = [object[0], object[1], object[2], 1.0f32];
        let transformmodel = match rotations.1 {
            0 => mult_m(model, &[
                [rotations.0.cos(), 0.0, -rotations.0.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [rotations.0.sin(), 0.0, rotations.0.cos(), 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], &model_position),
            1 => mult_m(model, &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, rotations.0.cos(), -rotations.0.sin(), 0.0],
                [0.0, rotations.0.sin(), rotations.0.cos(), 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], &model_position),
            2 => mult_m(model, &[
                [rotations.0.cos(), -rotations.0.sin(), 0.0, 0.0],
                [rotations.0.sin(), rotations.0.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], &model_position),
            3 => mult_m([
                    [rotations.0.cos(), 0.0, -rotations.0.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [rotations.0.sin(), 0.0, rotations.0.cos(), 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ], &[
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, rotations.0.cos(), -rotations.0.sin(), 0.0],
                    [0.0, rotations.0.sin(), rotations.0.cos(), 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ], &model_position),
            4 => mult_m([
                    [rotations.0.cos(), 0.0, -rotations.0.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [rotations.0.sin(), 0.0, rotations.0.cos(), 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ], &[
                    [rotations.0.cos(), -rotations.0.sin(), 0.0, 0.0],
                    [rotations.0.sin(), rotations.0.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ], &model_position),
            5 => mult_m([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, rotations.0.cos(), -rotations.0.sin(), 0.0],
                    [0.0, rotations.0.sin(), rotations.0.cos(), 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ], &[
                    [rotations.0.cos(), -rotations.0.sin(), 0.0, 0.0],
                    [rotations.0.sin(), rotations.0.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ], &model_position),
            _ => mult_m(mult_m([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, rotations.0.cos(), -rotations.0.sin(), 0.0],
                [0.0, rotations.0.sin(), rotations.0.cos(), 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], &[
                [rotations.0.cos(), -rotations.0.sin(), 0.0, 0.0],
                [rotations.0.sin(), rotations.0.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], &model_position), &[
                [rotations.0.cos(), 0.0, -rotations.0.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [rotations.0.sin(), 0.0, rotations.0.cos(), 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], &model_position),
        };

        let view = view_matrix(&[player[0], player[1], player[2]], &[player[3], player[4], player[5]], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { model: model, view: view, perspective: perspective, u_light: light, transformmodel: transformmodel },
                &params,
            )
            .unwrap();
        target.finish().unwrap();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => if let Some(key) = input.virtual_keycode {
                    if input.state == glutin::event::ElementState::Pressed {
                        match key {
                           glutin::event::VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,
                           // Object Rotation
                           glutin::event::VirtualKeyCode::R => rotations.1 = (rotations.1 + 1) % 7,
                           glutin::event::VirtualKeyCode::Space => rotations.2 = !rotations.2,
                           // Object Translation
                           glutin::event::VirtualKeyCode::Right => object[0] += speed,
                           glutin::event::VirtualKeyCode::Left => object[0] -= speed,
                           glutin::event::VirtualKeyCode::PageUp => object[1] += speed,
                           glutin::event::VirtualKeyCode::PageDown => object[1] -= speed,
                           glutin::event::VirtualKeyCode::Up => object[2] += speed,
                           glutin::event::VirtualKeyCode::Down => object[2] -= speed,
                           // Player Translation
                           glutin::event::VirtualKeyCode::D => player[0] += speed,
                           glutin::event::VirtualKeyCode::A => player[0] -= speed,
                           glutin::event::VirtualKeyCode::Home => player[1] += speed,
                           glutin::event::VirtualKeyCode::End => player[1] -= speed,
                           glutin::event::VirtualKeyCode::W => player[2] += speed,
                           glutin::event::VirtualKeyCode::S => player[2] -= speed,
                           // Center vision on object
                           glutin::event::VirtualKeyCode::C => {
                            player[3] = player[0] - object[0];
                            player[4] = player[1] - object[1];
                            player[5] = if player[2] - object[2] < 0.0 {
                                1.0
                            } else {
                                -1.0
                            };
                           }
                           _ => return,
                        }

                    }
                }
                // Mouse
                glutin::event::WindowEvent::CursorMoved { position, ..} => match position {
                    glutin::dpi::PhysicalPosition { x, y } => {
                        if last_mouse_position[0] == 0.0 && last_mouse_position[1] == 0.0 {
                            last_mouse_position[0] = x;
                            last_mouse_position[1] = y;
                            return ;
                        }
                        player[3] -= (((last_mouse_position[0] - x) as f32) * speed / 1000.0) * player[5];
                        player[4] += ((last_mouse_position[1] - y) as f32) * speed / 1000.0;
                        last_mouse_position[0] = x;
                        last_mouse_position[1] = y;
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
    });
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
