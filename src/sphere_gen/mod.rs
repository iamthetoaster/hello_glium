// Module for generating equidistant points on a sphere

pub fn sphere(number_of_points: usize) -> Vec<[f32; 3]> {
    let mut result: Vec<[f32; 3]> = vec!();

    let two_phi = 5_f32.sqrt() + 1.0;
    let golden_angle = two_phi * std::f32::consts::PI;

    for i in 0..number_of_points {
        let latitude: f32 = (-1.0 + 2.0 * i as f32 / (number_of_points as f32 + 1.0)).asin();
        let longitude: f32 = golden_angle * i as f32;

        let (x, y, z) = (
            longitude.cos() * latitude.cos(),
            longitude.sin() * latitude.cos(),
            latitude.sin()
        );

        result.push([x, y, z]);
    }

    return result;
}

fn square_distance(a: [f32; 3], b: [f32; 3]) -> f32{
    (
        (a[0] - b[0]).powi(2) +
        (a[1] - b[1]).powi(2) +
        (a[2] - b[2]).powi(2)
    )
}

pub fn delaunay(points: &Vec<[f32; 3]>) -> Vec<[[f32;3]; 3]> {
    let (
        mut max_x, mut max_y, mut max_z, 
        mut min_x, mut min_y, mut min_z
    ) = (
        std::f32::NEG_INFINITY, std::f32::NEG_INFINITY, std::f32::NEG_INFINITY, 
        std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY
    );

    for point in points {
        max_x = max_x.max(point[0]);
        max_y = max_y.max(point[1]);
        max_z = max_z.max(point[2]);
        min_x = min_x.min(point[0]);
        min_y = min_y.min(point[1]);
        min_z = min_z.min(point[2]);
    }

    max_x += 1.0; max_y += 1.0; max_z += 1.0;
    min_x -= 1.0; min_y -= 1.0; min_z -= 1.0;

    let super_triangle = [
        [min_x, min_y, min_z],
        [min_x, max_y, min_z],
        [max_x, max_y, max_z]
    ];

    let mut triangulation: Vec<[[f32;3]; 3]> = vec!(super_triangle);
    
    for point in points {
        let mut bad_triangles: Vec<[[f32;3]; 3]> = vec!();
        let mut bad_triangle_indicies: Vec<usize> = vec!();
        let mut index = 0_usize;
        for triangle in &triangulation {
            let (mut center_x, mut center_y, mut center_z) = (0_f32, 0_f32, 0_f32);
            for corner in triangle {
                center_x += corner[0];
                center_y += corner[1];
                center_z += corner[2];
            }
            center_x /= 3.0;
            center_y /= 3.0;
            center_z /= 3.0;
            let center = [center_x, center_y, center_z];
            if square_distance(center, *point) < square_distance(center, triangle[0]) {
                bad_triangles.push(*triangle);
                bad_triangle_indicies.push(index);
            }
            index += 1;
        }
        let mut polygon: Vec<[[f32; 3]; 2]> = vec!();
        for triangle in bad_triangles {
            for i in 0..triangle.len() {
                let a = triangle[i];
                let b = triangle[(i + 1) % 3];
                if !(polygon.contains(&[a, b]) || polygon.contains(&[b, a])){
                    polygon.push([a, b])
                }
            }
        }
        for index in bad_triangle_indicies {
            triangulation.remove(index);
        }
        for edge in polygon {
            let new_triangle = [edge[0], edge[1], *point];
            triangulation.push(new_triangle);
        }
    }
    let mut supers: Vec<usize> = vec!();
    for i in 0..triangulation.len() {
        for corner_1 in &triangulation[i] {
            for corner_2 in &super_triangle {
                if corner_1 == corner_2 {
                    supers.push(i);
                }
            }
        }
    }
    for i in supers {
        triangulation.remove(i);
    }

    return triangulation;
}