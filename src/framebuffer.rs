use crate::color::Color;
use crate::obj::Obj;
use nalgebra_glm::{Mat4, Vec3, Vec4};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0x000000); // Negro
    }

    pub fn render_model(
        &mut self,
        model: &Obj,
        model_matrix: &Mat4,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        color: Color,
    ) {
        let mvp = projection_matrix * view_matrix * model_matrix;

        for vertex in &model.vertices {
            // Convertir el vector 3D en un vector homogéneo (4D)
            let vertex_homogeneous = Vec4::new(vertex.x, vertex.y, vertex.z, 1.0);
            let transformed = mvp * vertex_homogeneous;

            if transformed.w == 0.0 {
                continue;
            }

            // Convertir coordenadas homogéneas en Normalized Device Coordinates (NDC)
            let ndc = Vec3::new(
                transformed.x / transformed.w,
                transformed.y / transformed.w,
                transformed.z / transformed.w,
            );

            // Convertir NDC en coordenadas de pantalla
            let screen_x = ((ndc.x + 1.0) * 0.5 * self.width as f32) as i32;
            let screen_y = ((1.0 - ndc.y) * 0.5 * self.height as f32) as i32;

            self.set_pixel(screen_x, screen_y, color.to_hex());
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            self.buffer[index] = color;
        }
    }
}
