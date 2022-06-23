#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

pub fn parsing(obj: String) -> Result<(Vec<Vertex>, Vec<u16>), String> {
    let mut vertices = vec![Vertex {position: (0.0, 0.0, 0.0)}];
    let mut indices = Vec::new();
    let lines: Vec<&str> = obj.split('\n').collect();
    for line in lines.iter() {
        match line.chars().next() {
            Some(c) => {
                match c {
                    'v' => {
                        let pos: Vec<Option<f32>> = line.split(' ').map(|x| {
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
                            vertices.push(Vertex {position: (pos[1].unwrap(), pos[2].unwrap(), pos[3].unwrap())});
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
                            if i == 3 {
                                break;
                            }
                        }
                    },
                    _ => ()
                }
            },
            None => ()
        }
    }
    //println!("{:?}", vertices);
    //println!("{:?}", indices.len());
    Ok((vertices, indices))
}
