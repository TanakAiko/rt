use crate::{
    camera::Camera,
    color::{self, Color},
    common,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    light::Light,
    param::Scene,
    ray::Ray,
    vec3::{self, Point3, Vec3},
};
use std::fs::OpenOptions;
use std::io::Write;

const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMAGE_WIDTH: i32 = 800;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

#[derive(Debug)]
pub struct Output {
    pub format: String,
    pub resolution: (u32, u32),
    pub max_color_value: u32,
    pub pixel_color: Vec<(u32, u32, u32)>,
}

impl Output {
    pub fn new() -> Self {
        Output {
            format: "P3".to_string(),
            resolution: (800, 600),
            max_color_value: 255,
            pixel_color: Vec::new(),
        }
    }

    pub fn init_pixel_color(&mut self) {
        for _ in 0..self.resolution.0 * self.resolution.1 {
            self.pixel_color.push((150, 5, 15));
        }
    }

    pub fn edit_image(&mut self) {
        self.pixel_color.clear();

        let scene = Scene::from_file("scene.json");

        // World

        let mut world = HittableList::new();
        world.set_scene(scene.clone());

        // Light source
        let light = Light::new(
            scene.light.position,
            scene.light.color,
            scene.light.intensity,
        );

        // Camera
        let cam = Camera::new(
            scene.camera.origin,
            scene.camera.look_at,
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            ASPECT_RATIO,
        );

        for j in (0..IMAGE_HEIGHT).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, &light);
                }
                color::write_color(self, pixel_color, SAMPLES_PER_PIXEL);
            }
        }
        eprint!("\nDone.\n");
    }

    pub fn generate_ppm_file(&self) {
        let mut code = String::new();

        code.push_str(&format!("{}\n", self.format));
        code.push_str(&format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT));
        code.push_str(&format!("{}\n", self.max_color_value));
        //println!("Pixel_color: {:?}", self.pixel_color);
        for (r, g, b) in self.pixel_color.iter() {
            code.push_str(&format!("{} {} {}\n", r, g, b));
        }

        code.trim_end().to_string();

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Réinitialise le fichier à chaque exécution
            .open("output.ppm")
            .expect("Erreur lors de l'ouverture du fichier");

        write!(file, "{}", code).expect("Writing to file error");
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, light: &Light) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let lighting = compute_lighting(&rec.p, &rec.normal, world, light);
        return rec.color * lighting;
    }
    Color::new(0.5, 0.7, 1.0) * light.color * light.intensity
}

fn compute_lighting(point: &Point3, normal: &Vec3, world: &dyn Hittable, light: &Light) -> Color {
    let light_dir = vec3::unit_vector(light.position - *point); // Direction vers la lumière
    let shadow_ray = Ray::new(*point, light_dir); // Rayon vers la lumière

    // Vérifier s'il y a un obstacle entre le point et la lumière
    let mut temp_rec = HitRecord::new();
    if world.hit(&shadow_ray, 0.001, common::INFINITY, &mut temp_rec) {
        return Color::new(0.4, 0.4, 0.4); // Ombre complète (pas de lumière)
    }

    // Calcul de l'éclairage diffus
    let diff = vec3::dot(*normal, light_dir).max(0.4);
    //println!("diff: {}", diff);
    diff * light.color * light.intensity
}
