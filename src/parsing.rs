fn max(a: f32, b:f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
fn min(a: f32, b:f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn parsing(obj: String) -> Result<(Vec<Vertex>, Vec<u16>), String> {
    let mut vertices = vec![Vertex {position: (0.0, 0.0, 0.0), tex_coords: [0.0, 0.0]}];
    let mut indices = Vec::new();
    let lines: Vec<&str> = obj.split('\n').collect();
    for line in lines.iter() {
        match line.chars().next() {
            Some(c) => {
                match c {
                    'v' => {
                        let pos: Vec<Option<f32>> = line.split(' ').filter(|p| !p.is_empty()).map(|x| {
                            match x.parse::<f32>() {
                                Ok(res) => Some(res),
                                Err(_) => None,
                            }
                        }).collect();
                        if pos.len() == 4 {
                            let mut i = 1;
                            while i < pos.len() - 1 {
                                match pos.get(i) {
                                    Some(x) => {
                                        if *x == None {
                                            return Err("Vertex value must be float".to_string());
                                        }
                                    }
                                    _ => ()
                                }
                                i += 1;
                            }
                            vertices.push(Vertex {position: (pos[1].unwrap(), pos[2].unwrap(), pos[3].unwrap()), tex_coords: [0.0, 0.0]});
                        } else {
                            return Err("Your vertex must be composed of 3 points".to_string());
                        }
                    },
                    'f' => {
                        for (i, x) in line.split(' ').collect::<Vec<&str>>().iter().enumerate() {
                            if i >= 1 {
                                    match x.parse::<u16>() {
                                        Ok(nb) => indices.push(nb),
                                        Err(_) => {
                                            return Err("indices value must be an u16 number".to_string())
                                        }
                                    }
                            }
                            if i == 4 {
                                let x_3 = indices.pop().unwrap();
                                let x_1 = indices[indices.len() - 1];
                                let x_2 = indices[indices.len() - 3];
                                indices.push(x_1);
                                indices.push(x_2);
                                indices.push(x_3);
                            }
                        }
                    },
                    _ => ()
                }
            },
            None => ()
        }
    }
    let image_bounds = vertices.iter().fold([0.0, 0.0, 0.0, 0.0, 0.0, 0.0], |acc, v| [min(acc[0], v.position.0), max(acc[1], v.position.1), max(acc[2], v.position.0), min(acc[3], v.position.1), min(acc[4], v.position.2), max(acc[5], v.position.2)]);
    let size_x = image_bounds[2] - image_bounds[0];
    let size_y = image_bounds[1] - image_bounds[3];
    let size_z = image_bounds[5] - image_bounds[4];


    vertices.iter_mut().for_each(|v| {
        let t_x = (v.position.0 + image_bounds[0].abs()) / (image_bounds[2] + image_bounds[0].abs());
        let t_y = (v.position.1 + image_bounds[3].abs()) / (image_bounds[1] + image_bounds[3].abs());
        let t_z = (v.position.2 + image_bounds[4].abs()) / (image_bounds[5] + image_bounds[4].abs());
        v.tex_coords = match size_x > size_y {
            true => match size_z > size_y {
                true => [t_x, t_z],
                false => [t_x, t_y],
            },
            false => match size_z > size_x {
                true => [t_y, t_z],
                false => [t_y, t_x]
            }
        };
        v.tex_coords = [t_x, t_y];
    });
    Ok((vertices, indices))
}
