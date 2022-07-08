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
    pub position: (f32, f32, f32),
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);


pub fn parsing(obj: String) -> Result<(Vec<Vertex>, Vec<Normal>, Vec<u16>, [f32; 3]), String> {
    let mut vertices = vec![Vertex {position: (0.0, 0.0, 0.0), tex_coords: [0.0, 0.0]}];
    let mut textures = vec![Vertex {position: (0.0, 0.0, 0.0), tex_coords: [0.0, 0.0]}];
    let mut normals = vec![Normal {normal: (0.0, 0.0, 0.0)}];
    let mut indices = Vec::new();
    let lines: Vec<&str> = obj.split('\n').collect();
    for line in lines.iter() {
        let chunk = line.split(' ').map(|ch| ch.trim()).filter(|ch| !ch.is_empty()).collect::<Vec<&str>>();
        let mut chunk_iter = chunk.into_iter();
        match chunk_iter.next() {
            // vertices / normal and textures
            v@(Some("v") | Some("vn") | Some("vt")) => {
                let pos: Vec<f32> = chunk_iter.fold(Ok(Vec::new()), |acc: Result<Vec<f32>, String>, x| {
                    match acc {
                        Ok(mut acc) => {
                            acc.push(x.parse::<f32>().map_err(|_| format!("Vertex/Normal/Texture value must be float {x:?}"))?);
                            Ok(acc)
                        },
                        _ => acc
                    }
                })?;
                if let Some("vt") = v {
                    textures.push(Vertex {
                        position: (0., 0., 0.),
                        tex_coords: [
                            *pos.get(0).ok_or(String::from("Your texture must be composed of 2 points"))?,
                            *pos.get(1).ok_or(String::from("Your texture must be composed of 2 points"))?,
                        ]
                    });
                    continue
                }
                let pos = (
                    *pos.get(0).ok_or(String::from("Your vertex/normal must be composed of 3 points"))?,
                    *pos.get(1).ok_or(String::from("Your vertex/normal must be composed of 3 points"))?,
                    *pos.get(2).ok_or(String::from("Your vertex/normal must be composed of 3 points"))?
                );
                if let Some("v") = v {
                    vertices.push(Vertex {
                        position: pos,
                        tex_coords: [0.0, 0.0]
                    });
                } else {
                    normals.push(Normal {
                        normal: pos
                    })
                }
            },
            // faces
            Some("f") => {
                for (i, chunk) in chunk_iter.enumerate() {
                    //TODO handle Vertice/Texture/Normal
                    let x: Vec<&str> = chunk.split('/').collect();
                    let indice = x.get(0).ok_or(String::from("No indice given"))?.parse::<u16>().map_err(|_| format!("indices value must be an u16 number {x:?}"))?;
                    indices.push(indice);

                    // apply texture to vertex
                    if let Some(vertex) = vertices.get_mut(indice as usize) {
                        if let Some(tex_indice_s) = x.get(1) {
                            if let Some(tex) = textures.get(tex_indice_s.parse::<usize>().map_err(|_| format!("indices value must be an usize number {x:?}"))?) {
                                vertex.tex_coords = tex.tex_coords;
                            }
                        }
                    }

                    if i == 3 {
                        let x_3 = indices.pop().unwrap();
                        let x_1 = indices[indices.len() - 1];
                        let x_2 = indices[indices.len() - 3];
                        indices.push(x_1);
                        indices.push(x_2);
                        indices.push(x_3);
                    }

                }
            },
            _ => {}
        }
    }

    // define distance between origin and object's center
    let mut iter = vertices.iter();
    iter.next();
    let mut center = [0., 0., 0.];
    if let Some(fst) = iter.next() {
        let image_bounds = iter.fold(
            [fst.position.0, fst.position.1, fst.position.0, fst.position.1, fst.position.2, fst.position.2],
            |acc, v| [min(acc[0], v.position.0), max(acc[1], v.position.1), max(acc[2], v.position.0), min(acc[3], v.position.1), min(acc[4], v.position.2), max(acc[5], v.position.2)]);
        let size_y = image_bounds[1] - image_bounds[3];
        let size_z = image_bounds[5] - image_bounds[4];
        let size_x = image_bounds[2] - image_bounds[0];
        center = [size_x / 2.0 + image_bounds[0], size_y / 2.0 + image_bounds[3], size_z / 2.0 + image_bounds[4]];

        if textures.len() == 1 {
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
                        true => [t_z, t_y],
                        false => [t_z, t_y]
                    }
                };
            });
        }

    }
    Ok((vertices, normals, indices, center))
}
