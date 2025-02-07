use raytracer::camera::Camera;
use raytracer::hittable_list::HittableList;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::{Color, Point3};
use std::io;

fn main() -> io::Result<()> {
    // World
    let mut world = HittableList::new();

    // Ground - large diffuse sphere
    let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0)); // Yellow-ish
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    )));

    // Center sphere - diffuse blue
    let center_material = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        center_material,
    )));

    // Left sphere - glass
    let left_material = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        left_material,
    )));

    // Right sphere - brushed metal (high fuzz)
    let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    )));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world)?;

    Ok(())
}
