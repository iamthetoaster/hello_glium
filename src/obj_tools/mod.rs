use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use crate::types::TexturedVertex;

pub fn parse_uv_obj(file_name: &str) -> Vec<TexturedVertex> {
    let f = File::open(file_name)
        .expect(&format!("File {} not found", file_name));

    let file = BufReader::new(&f);

    // An obj is essentially a collection of vertices, normals, and uvs, with triangles built from them
    // obj indexing is counted from 1, startign each array with a default value allows a naiive way of parsing files which do not have values for uvs and or normals
    let mut verts: Vec<[f32; 4]> = vec!{[0.0, 0.0, 0.0, 1.0]}; // Points have w of 0
    let mut normals: Vec<[f32; 4]> = vec!{[0.0, 0.0, 0.0, 0.0]}; // Directions have w of 0
    let mut uvs: Vec<[f32; 3]> = vec!{[0.0, 0.0, 0.0]};
    let mut triangles: Vec<TexturedVertex> = Vec::new();

    // Each item in the obj is on its own line, so we can iterate through the lines to collect them. 
    for line in file.lines() {
        let l = line.unwrap();

        // Using ifs instead of a match statement allows us to ignore lines with data we are choosing not to parse, eg group lines etc
        if l.starts_with("v ") {
            // We clip off the label of a given line, the remainder should be space separated
            let mut contents = l[2..].split_whitespace(); 
            
            let mut vert: [f32;4] = [0.0; 4];
            
            for i in 0..4 {
                vert[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 1.0f32 // data should be at least 3 dimensional, the fourth dimensional component should be 1
                };
            }

            verts.push(vert);
        } else if l.starts_with("vn ") {
            let mut contents = l[3..].split_whitespace();
            
            let mut norm: [f32;4] = [0.0; 4];
            
            for i in 0..4 {
                norm[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 0.0f32 // data should be at least 3 dimensional, the fourth dimensional component should be 0
                };
            }

            normals.push(norm);
        } else if l.starts_with("vt ") {
            let mut contents = l[3..].split_whitespace();
            
            let mut uv: [f32;3] = [0.0; 3];
            
            for i in 0..3 {
                uv[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 0.0f32 // in most cases this will be two dimensional data, there's no rhyme or reason to how I'm handling other cases here
                };
            }

            uvs.push(uv);
        } else if l.starts_with("f ") {
            let mut contents = l[2..].split_whitespace();
            // Apparently people like to export models with "quads" because they're "good for loop selection"
            // So we have to handle that some way instead of assuming everything is a tri
            let mut face: Vec<TexturedVertex> = Vec::new();
            loop {
                match contents.next() {
                    Some(value) => {
                        let indices = value.split("/"); // In a face, the space separated values are sets of slash separated values!
                        let mut indices = indices.map(|val| val.parse::<usize>());
                        let position = verts[
                            match indices.next() {
                                Some(val) => match val { // it's possible to do these matches earlier, I might move them later 
                                    Ok(v) => v,
                                    _ => 0
                                },
                                None => 0 
                                // basically in all cases where it doesn't have an element we are looking for, we use the default value for that element
                                // which is stored in element 0 of that vector
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
                        face.push(TexturedVertex::new(position, normal, uv));
                        
                    },
                    None => break
                }
            }

            // naiive fan triangularization
            // leave me alone it's like five lines of code and most of the time I'll only get quads or tris
            // but yeah this gets messed up for concave faces and doesn't give you the fewest possible tris
            for i in 1..(face.len() - 1) {
                triangles.push(face[0]);
                triangles.push(face[i]);
                triangles.push(face[i + 1]);
            }
        }
    }
    return triangles;
}

// Projects any model onto a sphere 
pub fn sphereize(model: &Vec<TexturedVertex>) -> Vec<TexturedVertex> {
    model.into_iter().map(|vert| {
        let [mut x, mut y, mut z, _] = vert.position;
        let len = (x * x + y * y + z * z).recip().sqrt();
        x *= len;
        y *= len;
        z *= len;
        
        TexturedVertex::new([x, y, z, 1.0], [x, y, z, 0.0], vert.uv)
    }).collect()
}
