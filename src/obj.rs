use nalgebra_glm::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone)]
pub struct Obj {
    vertices: Vec<Vec3>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(filename))?;
        let reader = BufReader::new(file);
        let mut vertices = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        Ok(Obj { vertices })
    }
}
