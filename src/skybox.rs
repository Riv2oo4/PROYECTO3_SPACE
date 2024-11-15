use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::color::Color;

/// Representa el cielo estrellado del sistema solar.
pub struct Skybox {
    // Aquí podrías agregar texturas o configuraciones adicionales si tienes imágenes.
}

impl Skybox {
    /// Carga las texturas del skybox desde una carpeta (opcional)
    pub fn load(_path: &str) -> Self {
        // Aquí cargarías las texturas si fuera necesario.
        Skybox {}
    }

    /// Renderiza el skybox en el fondo del framebuffer
    pub fn render(&self, framebuffer: &mut Framebuffer, _camera: &Camera) {
        framebuffer.clear_color(Color::new(30, 30, 50)); // Cambia a un tono más claro para depuración
    }
    
}
