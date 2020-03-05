use std::ops;

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub uv: [f32; 3],
}

impl TexturedVertex {
    pub const fn new(position: [f32; 4], normal: [f32; 4], uv: [f32; 3]) -> TexturedVertex {
        return TexturedVertex {position: position, normal: normal, uv: uv};
    }
}

impl std::fmt::Display for TexturedVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->std::fmt::Result {
        write!(f, 
            "({}, {}, {}, {}), ({}, {}, {}), ({}, {}, {}, {})", 
            self.position[0], 
            self.position[1], 
            self.position[2], 
            self.position[3], 
            self.uv[0], 
            self.uv[1], 
            self.uv[2],
            self.normal[0], 
            self.normal[1], 
            self.normal[2], 
            self.normal[3], 
        )
    }
}

impl ops::Add for &TexturedVertex {
    type Output = TexturedVertex;
    fn add(self, other: Self) -> TexturedVertex {
        let mut position = [0.0, 0.0, 0.0, 0.0f32];
        let mut normal = [0.0, 0.0, 0.0, 0.0f32];
        let mut uv = [0.0, 0.0, 0.0f32];

        for i in 0..4 {
            position[i] = self.position[i] + other.position[i];
            normal[i] = self.normal[i] + other.normal[i];
        }
        for i in 0..3 {
            uv[i] = self.uv[i] + other.uv[i];
        }

        TexturedVertex{position: position, normal: normal, uv: uv}
    }
}

impl ops::Sub for &TexturedVertex {
    type Output = TexturedVertex;
    fn sub(self, other: Self) -> TexturedVertex {
        let mut position = [0.0, 0.0, 0.0, 0.0f32];
        let mut normal = [0.0, 0.0, 0.0, 0.0f32];
        let mut uv = [0.0, 0.0, 0.0f32];

        for i in 0..4 {
            position[i] = self.position[i] - other.position[i];
            normal[i] = self.normal[i] - other.normal[i];
        }
        for i in 0..3 {
            uv[i] = self.uv[i] - other.uv[i];
        }

        TexturedVertex{position: position, normal: normal, uv: uv}
    }
}

impl ops::Mul<f32> for &TexturedVertex {
    type Output = TexturedVertex;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut position = [0.0, 0.0, 0.0, 0.0f32];
        let mut normal = [0.0, 0.0, 0.0, 0.0f32];
        let mut uv = [0.0, 0.0, 0.0f32];

        for i in 0..4 {
            position[i] = self.position[i] * rhs;
            normal[i] = self.normal[i] * rhs;
        }
        for i in 0..3 {
            uv[i] = self.uv[i] * rhs;
        }

        TexturedVertex{position: position, normal: normal, uv: uv}
    }
}

impl ops::Div<f32> for &TexturedVertex {
    type Output = TexturedVertex;

    fn div(self, rhs: f32) -> Self::Output {
        let mut position = [0.0, 0.0, 0.0, 0.0f32];
        let mut normal = [0.0, 0.0, 0.0, 0.0f32];
        let mut uv = [0.0, 0.0, 0.0f32];

        for i in 0..4 {
            position[i] = self.position[i] / rhs;
            normal[i] = self.normal[i] / rhs;
        }
        for i in 0..3 {
            uv[i] = self.uv[i] / rhs;
        }

        TexturedVertex{position: position, normal: normal, uv: uv}
    }
}

impl ops::Neg for &TexturedVertex {
    type Output = TexturedVertex;

    fn neg(self) -> Self::Output {
        let mut position = [0.0, 0.0, 0.0, 0.0f32];
        let mut normal = [0.0, 0.0, 0.0, 0.0f32];
        let mut uv = [0.0, 0.0, 0.0f32];

        for i in 0..4 {
            position[i] = -self.position[i];
            normal[i] = -self.normal[i];
        }
        for i in 0..3 {
            uv[i] = -self.uv[i];
        }

        TexturedVertex{position: position, normal: normal, uv: uv}
    }
}

implement_vertex!(TexturedVertex, position, normal, uv);
