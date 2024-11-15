use nalgebra_glm::Mat4;
use fastnoise_lite::FastNoiseLite;

pub struct Uniforms<'a> {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32,
    pub noise: &'a FastNoiseLite,
}
