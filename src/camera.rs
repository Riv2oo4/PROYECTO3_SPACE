use nalgebra_glm::{Vec3, cross, normalize, rotate_vec3};
use std::f32::consts::PI;

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

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos(),
        );

        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        let distance = (self.center - self.eye).magnitude();

        let min_distance = 10.0; // Distancia mínima para evitar colisiones
        let max_distance = 1000.0; // Distancia máxima para evitar perder de vista el objeto

        let new_distance = distance - delta; // Invertimos el sentido de delta
        if new_distance > min_distance && new_distance < max_distance {
            self.eye += direction * delta;
            self.has_changed = true;
        }
    }

    pub fn move_center(&mut self, direction: Vec3) {
        let radius_vector = self.center - self.eye;
        let radius = radius_vector.magnitude();

        let angle_x = direction.x * 0.05;
        let angle_y = direction.y * 0.05;

        let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

        let right = rotated.cross(&self.up).normalize();
        let final_rotated = rotate_vec3(&rotated, angle_y, &right);

        self.center = self.eye + final_rotated.normalize() * radius;
        self.has_changed = true;
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
