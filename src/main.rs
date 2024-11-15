use nalgebra_glm::Vec3;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod camera;
mod color;

use framebuffer::Framebuffer;
use color::Color;

pub struct Planet {
    position: Vec3,
    orbit_radius: f32,
    orbit_speed: f32,
    current_angle: f32,
    color: Color,
    size: f32,
}

fn main() {
    let width = 800;
    let height = 800;

    let mut framebuffer = Framebuffer::new(width, height);
    let mut window = Window::new("Sistema Solar", width, height, WindowOptions::default()).unwrap();

    // Planetas
    let mut planets = vec![
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            current_angle: 0.0,
            color: Color::new(255, 200, 0), // Sol
            size: 50.0,
        },
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 100.0,
            orbit_speed: 0.02,
            current_angle: 0.0,
            color: Color::new(0, 100, 255), // Planeta 1
            size: 10.0,
        },
        Planet {
            position: Vec3::zeros(),
            orbit_radius: 180.0,
            orbit_speed: 0.01,
            current_angle: 0.0,
            color: Color::new(255, 0, 0), // Planeta 2
            size: 15.0,
        },
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();

        // Dibujar órbitas
        for planet in &planets[1..] {
            framebuffer.draw_circle(
                framebuffer.width as i32 / 2,
                framebuffer.height as i32 / 2,
                planet.orbit_radius as i32,
                Color::new(100, 100, 100),
            );
        }

        // Actualizar y dibujar planetas
        for planet in planets.iter_mut() {
            if planet.orbit_radius > 0.0 {
                planet.current_angle += planet.orbit_speed;
                planet.position = Vec3::new(
                    planet.orbit_radius * planet.current_angle.cos(),
                    0.0,
                    planet.orbit_radius * planet.current_angle.sin(),
                );
            }

            framebuffer.draw_circle(
                framebuffer.width as i32 / 2 + planet.position.x as i32,
                framebuffer.height as i32 / 2 + planet.position.z as i32,
                planet.size as i32,
                planet.color,
            );
        }

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}
