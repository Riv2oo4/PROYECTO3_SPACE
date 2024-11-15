use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod camera;
mod color;
mod obj;

use framebuffer::Framebuffer;
use camera::Camera;
use color::Color;
use obj::Obj;

pub struct Planet {
    position: Vec3,
    orbit_radius: f32,
    orbit_speed: f32,
    current_angle: f32,
    size: f32,
    color: Color,
    model: Obj, // Modelo de la esfera
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_projection_matrix(width: f32, height: f32) -> Mat4 {
    let fov = 45.0 * std::f32::consts::PI / 180.0;
    let aspect_ratio = width / height;
    perspective(fov, aspect_ratio, 0.1, 1000.0)
}

fn main() {
    let width = 800;
    let height = 800;

    let mut framebuffer = Framebuffer::new(width, height);
    let mut window = Window::new("Sistema Solar", width, height, WindowOptions::default()).unwrap();

    let sphere_model = Obj::load("assets/models/sphere.obj").expect("No se pudo cargar sphere.obj");

    // Planetas y Sol
    let mut planets = vec![
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            current_angle: 0.0,
            size: 1.5,
            color: Color::new(255, 200, 0), // Sol
            model: sphere_model.clone(),
        },
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 100.0,
            orbit_speed: 0.001,
            current_angle: 0.0,
            size: 0.5,
            color: Color::new(0, 100, 255), // Planeta 1
            model: sphere_model.clone(),
        },
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 180.0,
            orbit_speed: 0.0008,
            current_angle: 0.0,
            size: 0.7,
            color: Color::new(255, 0, 0), // Planeta 2
            model: sphere_model.clone(),
        },
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 260.0,
            orbit_speed: 0.0006,
            current_angle: 0.0,
            size: 0.9,
            color: Color::new(255, 165, 0), // Planeta 3 (Naranja)
            model: sphere_model.clone(),
        },
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 350.0,
            orbit_speed: 0.0004,
            current_angle: 0.0,
            size: 1.2,
            color: Color::new(100, 255, 100), // Planeta 4 (Verde)
            model: sphere_model.clone(),
        },
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 500.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();

        // Manejo de entrada para movimiento de cámara
        if window.is_key_down(Key::W) {
            camera.move_forward(5.0);
        }
        if window.is_key_down(Key::S) {
            camera.move_forward(-5.0);
        }
        if window.is_key_down(Key::A) {
            camera.move_right(-5.0);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(5.0);
        }
        if window.is_key_down(Key::Space) {
            camera.move_up(5.0);
        }
        if window.is_key_down(Key::LeftShift) {
            camera.move_up(-5.0);
        }
        if window.is_key_down(Key::Left) {
            camera.rotate(0.05, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.rotate(-0.05, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.rotate(0.0, 0.05);
        }
        if window.is_key_down(Key::Down) {
            camera.rotate(0.0, -0.05);
        }

        // Actualizar y renderizar planetas
        for planet in planets.iter_mut() {
            if planet.orbit_radius > 0.0 {
                planet.current_angle += planet.orbit_speed;
                planet.position = Vec3::new(
                    planet.orbit_radius * planet.current_angle.cos(),
                    0.0,
                    planet.orbit_radius * planet.current_angle.sin(),
                );
            }

            // Matriz de modelo
            let model_matrix = Mat4::new_translation(&planet.position)
                * Mat4::new_scaling(planet.size);

            // Renderizar el planeta
            framebuffer.render_model(
                &planet.model,
                &model_matrix,
                &create_view_matrix(camera.eye, camera.center, camera.up),
                &create_projection_matrix(width as f32, height as f32),
                planet.color,
            );
        }

        // Actualizar el buffer de la ventana
        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}
