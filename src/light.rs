use crate::{color::Color, vec3::Point3};

pub struct Light {
    pub position: Point3,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point3, intensity: Color) -> Self {
        Light {
            position,
            intensity,
        }
    }
}
