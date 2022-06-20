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

pub fn parsing(obj: String) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = vec![Vertex {position: (0.0, 0.0, 0.0)}];
    let mut indices = Vec::new();
    let lines: Vec<&str> = obj.split('\n').collect();
    for line in lines.iter() {
        match line.chars().next() {
            Some(c) => {
                match c {
                    'v' => {
                        let pos: Vec<&str> = line.split(' ').collect();
                        vertices.push(Vertex {position: (pos[1].parse::<f32>().unwrap(), pos[2].parse::<f32>().unwrap(), pos[3].parse::<f32>().unwrap())});
                    },
                    'f' => {
                        for (i, x) in line.split(' ').collect::<Vec<&str>>().iter().enumerate() {
                            match x.parse::<u16>() {
                                Ok(nb) => indices.push(nb),
                                Err(_) => ()
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
    (vertices, indices)
}
