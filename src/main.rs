use output::Output;

pub mod output;
pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod common;
pub mod camera;
pub mod plane;
pub mod light;
pub mod cube;

fn main() {
    println!("おはよう世界！");

    let mut out = Output::new();
    out.init_pixel_color();

    out.edit_image();

    out.generate_ppm_file();
}
