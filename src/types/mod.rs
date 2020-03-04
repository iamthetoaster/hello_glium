// #[derive(Copy, Clone)]
// pub struct ColoredVertex {
//     position: [f32; 4],
//     normal: [f32; 3],
//     color: [f32; 4],
// }
// implement_vertex!(ColoredVertex, position, normal, color);

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    position: [f32; 4],
    normal: [f32; 4],
    uv: [f32; 3],
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

implement_vertex!(TexturedVertex, position, normal, uv);
