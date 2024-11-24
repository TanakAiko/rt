use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Cube {
    pub min: Point3, // Coin inférieur (minimum) du cube
    pub max: Point3, // Coin supérieur (maximum) du cube
    pub color: Color,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, color: Color) -> Cube {
        Cube { min, max, color }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        // Vérification pour chaque dimension
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i]; // Inverse de la direction
            let mut t0 = (self.min[i] - r.origin()[i]) * inv_d;
            let mut t1 = (self.max[i] - r.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1); // Inverse si nécessaire
            }

            t_min = t_min.max(t0); // Garder la borne la plus grande
            t_max = t_max.min(t1); // Garder la borne la plus petite

            if t_max <= t_min {
                return false; // Pas d'intersection
            }
        }

        rec.t = t_min;
        rec.p = r.at(rec.t);

        let outward_normal = if (rec.p.x() - self.min.x()).abs() < 1e-4 {
            // println!("Face gauche");
            Vec3::new(-1.0, 0.0, 0.0) // Face gauche
        } else if (rec.p.x() - self.max.x()).abs() < 1e-4 {
            // println!("Face droite");
            Vec3::new(1.0, 0.0, 0.0) // Face droite
        } else if (rec.p.y() - self.min.y()).abs() < 1e-4 {
            // println!("Face inferieure");
            Vec3::new(0.0, -1.0, 0.0) // Face inférieure
        } else if (rec.p.y() - self.max.y()).abs() < 1e-4 {
            // println!("Face superieure");
            Vec3::new(0.0, 1.0, 0.0) // Face supérieure
        } else if (rec.p.z() - self.min.z()).abs() < 1e-4 {
            // println!("Face avant");
            Vec3::new(0.0, 0.0, -1.0) // Face avant
        } else {
            // println!("Face arriere");
            Vec3::new(0.0, 0.0, 1.0) // Face arrière
        };

        rec.set_face_normal(r, unit_vector(outward_normal));
        rec.color = self.color;

        true
    }
}
