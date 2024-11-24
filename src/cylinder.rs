use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Point3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Cylinder {
    pub base: Point3, // Centre de la base du cylindre
    pub height: f64,  // Hauteur du cylindre
    pub radius: f64,  // Rayon du cylindre
    pub color: Color, // Couleur du cylindre
}

impl Cylinder {
    pub fn new(base: Point3, height: f64, radius: f64, color: Color) -> Cylinder {
        Cylinder {
            base,
            height,
            radius,
            color,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let axis = Vec3::new(0.0, 1.0, 0.0); // Direction verticale du cylindre
        let oc = r.origin() - self.base; // Vecteur entre l'origine du rayon et la base

        // Projections pour la surface latérale
        let a = dot(
            r.direction() - axis * dot(r.direction(), axis),
            r.direction() - axis * dot(r.direction(), axis),
        );
        let b = 2.0
            * dot(
                oc - axis * dot(oc, axis),
                r.direction() - axis * dot(r.direction(), axis),
            );
        let c =
            dot(oc - axis * dot(oc, axis), oc - axis * dot(oc, axis)) - self.radius * self.radius;

        let mut t_closest = t_max; // Pour suivre le point d'intersection le plus proche
        let mut hit_anything = false;

        // Calcul des intersections pour la surface latérale
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let mut t = (-b - sqrtd) / (2.0 * a);
            if t < t_min || t > t_max {
                t = (-b + sqrtd) / (2.0 * a);
            }

            if t >= t_min && t <= t_max {
                let point = r.at(t);
                let y = point.y() - self.base.y();
                if y >= 0.0 && y <= self.height {
                    t_closest = t;
                    rec.t = t;
                    rec.p = point;

                    // Normale pour la surface latérale
                    let outward_normal =
                        unit_vector(rec.p - self.base - axis * dot(rec.p - self.base, axis));
                    rec.set_face_normal(r, outward_normal);
                    rec.color = self.color;

                    hit_anything = true;
                }
            }
        }

        // Intersection avec le disque inférieur
        let t_disk_base = (self.base.y() - r.origin().y()) / r.direction().y();
        if t_disk_base >= t_min && t_disk_base <= t_closest {
            let p = r.at(t_disk_base);
            if (p - self.base).length_squared() <= self.radius * self.radius {
                t_closest = t_disk_base;
                rec.t = t_disk_base;
                rec.p = p;
                rec.set_face_normal(r, Vec3::new(0.0, -1.0, 0.0)); // Normale vers le bas
                rec.color = self.color;

                hit_anything = true;
            }
        }

        // Intersection avec le disque supérieur
        let top_center = self.base + Vec3::new(0.0, self.height, 0.0);
        let t_disk_top = (top_center.y() - r.origin().y()) / r.direction().y();
        if t_disk_top >= t_min && t_disk_top <= t_closest {
            let p = r.at(t_disk_top);
            if (p - top_center).length_squared() <= self.radius * self.radius {
                rec.t = t_disk_top;
                rec.p = p;
                rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0)); // Normale vers le haut
                rec.color = self.color;

                hit_anything = true;
            }
        }

        hit_anything
    }
}
