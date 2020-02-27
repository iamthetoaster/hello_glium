use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use crate::types;

pub fn parse_obj(file_name: &str) {
    let f = File::open(file_name)
        .expect(&format!("File {} not found", file_name));

    let file = BufReader::new(&f);

    let mut verts: Vec<[f32; 4]> = Vec::new();
    let mut normals: Vec<[f32; 4]> = Vec::new();
    let mut uvs: Vec<[f32; 3]> = Vec::new();
    let mut faces: Vec<types::TexturedVertex> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l);

        if l.starts_with("v ") {
            let mut contents = l[2..].split_whitespace();
            
            let mut vert: [f32;4] = [0.0; 4];
            
            for i in 0..4 {
                vert[i] = match contents.next() {
                    Some(value) => value.parse().unwrap(),
                    None => 0.0f32
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
                    Some(vert) => {
                        let split = vert.split("/");
                        
                    },
                    None => break
                };
            }
        }
    }

}
