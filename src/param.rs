use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera, cube::Cube, cylinder::Cylinder, light::Light, plane::Plane, sphere::Sphere,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    pub camera: Camera,
    pub light: Light,
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub cubes: Vec<Cube>,
    pub cylinders: Vec<Cylinder>,
}

impl Scene {
    pub fn from_file(file_path: &str) -> Self {
        let json_data = fs::read_to_string(file_path).expect("Failed to read JSON file");
        serde_json::from_str(&json_data).expect("Failed to parse JSON")
    }
}
