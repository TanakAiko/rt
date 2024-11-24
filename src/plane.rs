use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{self, Point3, Vec3},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Plane {
    pub origine: Point3,
    pub width: i32,
    pub height: i32,
    pub color: Color,
}

impl Plane {
    pub fn new(origine: Point3, width: i32, height: i32, color: Color) -> Self {
        Plane {
            origine,
            width,
            height,
            color,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Définir une normale au plan (par défaut (0, 1, 0), perpendiculaire au sol)
        let normal = Vec3::new(0.0, 1.0, 0.0);

        // Calcul du dénominateur pour éviter une division par zéro
        let denom = vec3::dot(ray.direction(), normal);
        if denom.abs() < 1e-8 {
            // Le rayon est parallèle au plan
            return false;
        }

        // Calcul de t (intersection rayon-plan)
        let t = vec3::dot(self.origine - ray.origin(), normal) / denom;

        // Vérifier si t est dans les bornes acceptables
        if t < t_min || t > t_max {
            return false;
        }

        // Calcul du point d'intersection
        let p_hit = ray.at(t);

        // Vérifier si le point est dans les limites du plan (facultatif)
        let local_hit = p_hit - self.origine; // Position relative à l'origine du plan
        if local_hit.x() < 0.0
            || local_hit.x() > self.width as f64
            || local_hit.z() < 0.0
            || local_hit.z() > self.height as f64
        {
            return false;
        }

        // Mettre à jour le HitRecord
        rec.t = t;
        rec.p = p_hit;
        rec.set_face_normal(ray, normal);
        rec.color = self.color;
        true
    }
}
