#[derive(Copy, Clone)]
pub struct ColoredVertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
}
implement_vertex!(ColoredVertex, position, normal, color);

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 3],
}
implement_vertex!(TexturedVertex, position, normal, uv);
