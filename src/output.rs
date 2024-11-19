use crate::{
    camera::Camera,
    color::{self, Color},
    common,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
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

        // World

        let mut world = HittableList::new();
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        // Camera

        let cam = Camera::new(
            Point3::new(-2.0, 3.0, 1.0),
            Point3::new(0.0, 0.0, -1.0),
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
                    pixel_color += ray_color(&r, &world);
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

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, common::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
