use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod planet; 
use triangle::rasterize_parallel;
use std::time::Instant;
use crate::color::Color;
use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use shaders::{earth_shader, jupiter_shader, mars_shader,sun_shader, saturn_shader};
use planet::Planet; 
use fastnoise_lite::{FastNoiseLite, NoiseType};

pub struct Uniforms<'a> {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: &'a FastNoiseLite,
}

fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * std::f32::consts::PI / 180.0;
    let aspect_ratio = window_width / window_height;
    perspective(fov, aspect_ratio, 0.1, 1000.0)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_index: usize,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = shaders::vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }
    triangles.sort_by(|a, b| {
        let z_a = (a[0].transformed_position.z + a[1].transformed_position.z + a[2].transformed_position.z) / 3.0;
        let z_b = (b[0].transformed_position.z + b[1].transformed_position.z + b[2].transformed_position.z) / 3.0;
        z_b.partial_cmp(&z_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    let fragments = rasterize_parallel(&triangles);

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let color = match shader_index {
                0 => sun_shader( uniforms),
                1 => earth_shader(&fragment, uniforms),
                2 => mars_shader(&fragment, uniforms),
                3 => jupiter_shader(&fragment, uniforms),
                4 => saturn_shader(&fragment, uniforms),
                _ => Color::black(),
            };

            framebuffer.set_current_color(color.to_hex());
            framebuffer.point_with_emission(x, y, fragment.depth, 0);
        }
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
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
}


fn main() {
    let window_width = 800;
    let window_height = 800;
    let framebuffer_width = 800;
    let framebuffer_height = 800;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar con Shaders",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 500.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let sphere_model = Obj::load("assets/models/sphere.obj").expect("No se pudo cargar sphere.obj");
    let vertex_arrays = sphere_model.get_vertex_array();

    let mut planets = vec![
        Planet::new(0.0, 0.0, 1.5, 0),   
        Planet::new(25.0, 0.001, 0.5, 1), 
        Planet::new(45.0, 0.0008, 0.7, 2), 
        Planet::new(65.0, 0.0006, 0.9, 3), 
        Planet::new(90.0, 0.0004, 1.2, 4), 
    ];

    let noise = create_noise();
    let  time = 0;
    let mut frame_count = 0; 
    let mut last_time = Instant::now(); 

    while window.is_open() {
        if last_time.elapsed().as_secs_f32() >= 1.0 {
            let fps = frame_count;
            window.set_title(&format!("Sistema Solar"));
            frame_count = 0;
            last_time = Instant::now();
        }
        

        handle_input(&window, &mut camera);


        framebuffer.clear();

        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);

        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        for planet in planets.iter_mut() {
            planet.update_position(1.0);

            let model_matrix = planet.get_model_matrix();
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: &noise,
            };

            render(&mut framebuffer, &uniforms, &vertex_arrays, planet.shader_index);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}