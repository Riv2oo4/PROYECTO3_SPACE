use nalgebra_glm::{ Vec3,Mat4};


pub struct Planet {
    pub position: Vec3,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub current_angle: f32,
    pub size: f32,
    pub shader_index: usize,
}

impl Planet {
    pub fn new(orbit_radius: f32, orbit_speed: f32, size: f32, shader_index: usize) -> Self {
        Planet {
            position: Vec3::zeros(),
            orbit_radius,
            orbit_speed,
            current_angle: 0.0,
            size,
            shader_index,
        }
    }

    pub fn update_position(&mut self, delta_time: f32) {
        self.current_angle += self.orbit_speed * delta_time;
        self.position = Vec3::new(
            self.orbit_radius * self.current_angle.cos(),
            0.0,
            self.orbit_radius * self.current_angle.sin(),
        );
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        Mat4::new_translation(&self.position) * Mat4::new_scaling(self.size)
    }
}
