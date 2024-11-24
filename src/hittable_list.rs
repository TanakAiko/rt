use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::hittable::{HitRecord, Hittable};
use crate::param::Scene;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn set_scene(&mut self, scene: Scene) {
        // Ajouter les sphères
        for sphere in scene.spheres {
            self.add(Box::new(Sphere::new(
                sphere.center,
                sphere.radius,
                sphere.color,
            )));
        }

        // Ajouter les plans
        for plane in scene.planes {
            self.add(Box::new(Plane::new(
                plane.origine,
                plane.width,
                plane.height,
                plane.color,
            )));
        }

        // Ajouter les cubes
        for cube in scene.cubes {
            self.add(Box::new(Cube::new(cube.min, cube.max, cube.color)));
        }

        // Ajouter les cylindres
        for cylinder in scene.cylinders {
            self.add(Box::new(Cylinder::new(
                cylinder.base,
                cylinder.height,
                cylinder.radius,
                cylinder.color,
            )));
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
