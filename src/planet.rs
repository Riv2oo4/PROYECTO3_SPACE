use nalgebra_glm::{Vec3, Mat4};

pub struct Planet {
    pub name: String,
    pub distance_from_sun: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub size: f32,
    pub position: Vec3,
}

impl Planet {
    pub fn new(name: &str, distance_from_sun: f32, orbit_speed: f32, rotation_speed: f32, size: f32) -> Self {
        Planet {
            name: name.to_string(),
            distance_from_sun,
            orbit_speed,
            rotation_speed,
            size,
            position: Vec3::new(distance_from_sun, 0.0, 0.0),
        }
    }

    pub fn update_position(&mut self, time: f32) {
        let angle = time * self.orbit_speed;
        self.position = Vec3::new(
            self.distance_from_sun * angle.cos(),
            0.0,
            self.distance_from_sun * angle.sin(),
        );
    }

    pub fn get_model_matrix(&self, time: f32) -> Mat4 {
        let translation = Mat4::new_translation(&self.position);
        let rotation = Mat4::from_rotation_y(time * self.rotation_speed);
        let scaling = Mat4::new_scaling(self.size);

        translation * rotation * scaling
    }
}