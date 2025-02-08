use raytracer::camera::Camera;
use raytracer::hittable_list::HittableList;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vec3::{Color, Point3, Vec3};
use std::io;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    )));

    let center_material = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        center_material,
    )));

    let glass_outer = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        glass_outer,
    )));

    let air_bubble = Dielectric::new(1.0 / 1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        air_bubble,
    )));

    let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world)?;

    Ok(())
}
