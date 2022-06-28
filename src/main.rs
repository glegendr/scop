#[macro_use]
extern crate glium;
extern crate image;
extern crate glam;

mod parsing;
mod matrix;
mod event;

use std::{env, fs, process};
use event::match_event_keyboard;
use parsing::parsing;
use matrix::Matrix;
use std::io::Cursor;
use glium::{glutin, Surface, glutin::event::VirtualKeyCode};

const VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec2 tex_coords;
    in vec3 normal;

    out vec2 v_tex_coords;
    out vec3 v_normal;
    out vec3 v_position;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        v_tex_coords = tex_coords;
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 150

    in vec3 v_normal;
    in vec2 v_tex_coords;
    in vec3 v_position;

    out vec4 color;

    uniform vec3 u_light;
    uniform bool is_textured;
    uniform bool is_enlightened;
    uniform sampler2D tex;

    vec4 get_enlightened_color(vec4 base_color, float strength) {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    
        vec3 dark_color = vec3(base_color[0] - strength, base_color[1] - strength, base_color[2] - strength);
        vec3 regular_color = vec3(base_color[0], base_color[1], base_color[2]);
        vec3 specular_color = vec3(base_color[0] + strength, base_color[1] + strength, base_color[2] + strength);
        return vec4(dark_color + diffuse * regular_color + specular * specular_color, 1.0);
    }

    void main() {
        vec4 raw_color;
        float strength;

        if (is_textured) {
            raw_color = texture(tex, v_tex_coords);
            strength = 0.4;
        } else {
            float grey = (float((gl_PrimitiveID) % 5) / 10.) * 0.4 + 0.02;
            raw_color = vec4(grey, grey, grey, 1.0);
            strength = 0.02;
        }

        if (is_enlightened) {
            color = get_enlightened_color(raw_color, strength);
        } else {
            color = raw_color;
        }
    }
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("add an obj file in argument");
        process::exit(1);
    }
    let (vertices, normals, indices_parsing, center) = match fs::read_to_string(args[1].clone()) {
        Ok(contents) => {
            match parsing(contents) {
                Ok(x) => x,
                Err(e) => {
                    println!("{e}");
                    process::exit(1)
                }
            }
        }
        Err(_) => {
            println!("Something went wrong when reading the file");
            process::exit(1)
        }
    };

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let normals = glium::VertexBuffer::new(&display, &normals).unwrap();
    let mut indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices_parsing,
    )
    .unwrap();

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let image = match image::load(Cursor::new(&include_bytes!("../resources/kitten.png")),
        image::ImageFormat::Png) {
            Err(_) => {
                println!("Error parsing texture");
                process::exit(1)
            },
            Ok(img) => img.to_rgba8()
        };
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap(); 

    let mut rotations: (f32, usize, bool) = (0.0, 0, true);
    let mut object: [f32; 3] = [-center[0], -center[1], -center[2]];
    let mut player: [f32; 6] = [0.0, 0.0, -5., 0.0, 0.0, 1.];
    let mut last_mouse_position: [f64; 2] = [0.0, 0.0];
    let mut is_textured: bool = false;
    let mut is_enlightened: bool = false;
    let mut speed: f32 = 0.1;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        if rotations.2 {
            rotations.0 += 0.005;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let model = Matrix::from_translation([-center[0], -center[1], -center[2]])
            .rotate(rotations.1, rotations.0)
            .multiply(&Matrix::from_translation(center))
            .translate([object[0], object[1], object[2]]);

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
                &uniform! {
                    model: model.to_cols_array_2d(),
                    view: view,
                    perspective: perspective,
                    u_light: light,
                    tex: &texture,
                    is_textured: is_textured,
                    is_enlightened: is_enlightened,
                },
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
                            // change object type
                            VirtualKeyCode::O => {
                                indices = match indices.get_primitives_type() {
                                    glium::index::PrimitiveType::TrianglesList => glium::IndexBuffer::new(
                                        &display,
                                        glium::index::PrimitiveType::LinesList,
                                        &indices_parsing,
                                    )
                                    .unwrap(),
                                    _ => glium::IndexBuffer::new(
                                        &display,
                                        glium::index::PrimitiveType::TrianglesList,
                                        &indices_parsing,
                                    )
                                    .unwrap()
                                }
                            },
                            _ => match_event_keyboard(
                                key,
                                control_flow,
                                &mut speed,
                                &mut object,
                                &mut player,
                                &mut rotations,
                                &mut is_textured,
                                &mut is_enlightened,
                                &center
                            )
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
                        let mult = if player[5] > 0.0 {
                            -1.0
                        } else {
                            1.0
                        };
                        player[3] -= (((last_mouse_position[0] - x) as f32) * speed / 100.0) * player[5];
                        player[4] -= (((last_mouse_position[1] - y) as f32) * speed / 100.0) * player[5] * mult;
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
