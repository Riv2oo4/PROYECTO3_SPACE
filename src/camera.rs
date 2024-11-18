use nalgebra_glm::{Vec3, cross, normalize, rotate_vec3};


pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * distance;
        self.center += direction * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        let direction = (self.center - self.eye).normalize();
        let right = normalize(&cross(&direction, &self.up));
        self.eye += right * distance;
        self.center += right * distance;
    }

    pub fn move_up(&mut self, distance: f32) {
        self.eye += self.up * distance;
        self.center += self.up * distance;
    }



    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        let direction = self.center - self.eye;

        // Rotación horizontal
        let rotated_horizontally = rotate_vec3(&direction, yaw, &self.up);

        // Rotación vertical
        let right = normalize(&cross(&rotated_horizontally, &self.up));
        let rotated = rotate_vec3(&rotated_horizontally, pitch, &right);

        self.center = self.eye + rotated;
    }
}
