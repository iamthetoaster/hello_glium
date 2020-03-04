use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use crate::types::TexturedVertex;

pub fn parse_uv_obj(file_name: &str) -> Vec<TexturedVertex> {
    let f = File::open(file_name)
        .expect(&format!("File {} not found", file_name));

    let file = BufReader::new(&f);

    let mut verts: Vec<[f32; 4]> = vec!{[0.0, 0.0, 0.0, 1.0]};
    let mut normals: Vec<[f32; 4]> = vec!{[0.0, 0.0, 0.0, 0.0]};
    let mut uvs: Vec<[f32; 3]> = vec!{[0.0, 0.0, 0.0]};
    let mut triangles: Vec<TexturedVertex> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();

        if l.starts_with("v ") {
            let mut contents = l[2..].split_whitespace();
            
            let mut vert: [f32;4] = [0.0; 4];
            
            for i in 0..4 {
                vert[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 1.0f32
                };
            }

            verts.push(vert);
        } else if l.starts_with("vn ") {
            let mut contents = l[3..].split_whitespace();
            
            let mut norm: [f32;4] = [0.0; 4];
            
            for i in 0..4 {
                norm[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 0.0f32
                };
            }

            normals.push(norm);
        } else if l.starts_with("vt ") {
            let mut contents = l[3..].split_whitespace();
            
            let mut uv: [f32;3] = [0.0; 3];
            
            for i in 0..3 {
                uv[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 0.0f32
                };
            }

            uvs.push(uv);
        } else if l.starts_with("f ") {
            let mut contents = l[2..].split_whitespace();

            loop {
                match contents.next() {
                    Some(value) => {
                        let indices = value.split("/");
                        let mut indices = indices.map(|val| val.parse::<usize>());
                        let position = verts[
                            match indices.next() {
                                Some(val) => match val {
                                    Ok(v) => v,
                                    _ => 0
                                },
                                None => 0
                            }
                        ];
                        let uv = uvs[
                            match indices.next() {
                                Some(val) => match val {
                                    Ok(v) => v,
                                    _ => 0
                                },
                                None => 0
                            }
                        ];
                        let normal = normals[
                            match indices.next() {
                                Some(val) => match val {
                                    Ok(v) => v,
                                    _ => 0
                                },
                                None => 0
                            }
                        ];

                        triangles.push(TexturedVertex::new(position, normal, uv)); 
                        println!("pushing vert {}", triangles.len());
                    },
                    None => break
                }
            }
        }
    }
    return triangles;
}
