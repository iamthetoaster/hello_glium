// Module for generating equidistant points on a sphere


// credit to @bduvenhage https://github.com/bduvenhage/Bits-O-Cpp/blob/master/geomtry/main_3D_fibo.cpp
pub fn sphere(number_of_points: usize) -> Vec<[f32; 3]> {
    let mut result: Vec<[f32; 3]> = Vec::new();

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

pub fn icosahedron() -> Vec<[[f32; 3]; 3]> {
    let pts = [
        [0.0, 0.0, 0.0],
        [0.000000, -0.525731, 0.850651],
        [0.850651, 0.000000, 0.525731],
        [0.850651, 0.000000, -0.525731],
        [-0.850651, 0.000000, -0.525731],
        [-0.850651, 0.000000, 0.525731],
        [-0.525731, 0.850651, 0.000000],
        [0.525731, 0.850651, 0.000000],
        [0.525731, -0.850651, 0.000000],
        [-0.525731, -0.850651, 0.000000],
        [0.000000, -0.525731, -0.850651],
        [0.000000, 0.525731, -0.850651],
        [0.000000, 0.525731, 0.850651]
    ];
    let ico = vec! {
        [pts[2], pts[3], pts[7]],
        [pts[2], pts[8], pts[3]],
        [pts[4], pts[5], pts[6]],
        [pts[5], pts[4], pts[9]],
        [pts[7], pts[6], pts[12]],
        [pts[6], pts[7], pts[11]],
        [pts[10], pts[11], pts[3]],
        [pts[11], pts[10], pts[4]],
        [pts[8], pts[9], pts[10]],
        [pts[9], pts[8], pts[1]],
        [pts[12], pts[1], pts[2]],
        [pts[1], pts[12], pts[5]],
        [pts[7], pts[3], pts[11]],
        [pts[2], pts[7], pts[12]],
        [pts[4], pts[6], pts[11]],
        [pts[6], pts[5], pts[12]],
        [pts[3], pts[8], pts[10]],
        [pts[8], pts[2], pts[1]],
        [pts[4], pts[10], pts[9]],
        [pts[5], pts[9], pts[1]],
    };
    return ico;
}

pub fn icosphere(subdivisions: usize) -> Vec<[[f32; 3]; 3]> {
    return icosahedron();
}