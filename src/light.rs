use crate::{color::Color, vec3::Point3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Light {
    pub position: Point3,
    pub color: Color,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: Point3, color: Color, intensity: f64) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }
}
