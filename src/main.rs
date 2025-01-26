use raytracer::camera::Camera;
use raytracer::hittable_list::HittableList;
use raytracer::sphere::Sphere;
use raytracer::vec3::Point3;
use std::io;

fn main() -> io::Result<()> {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    // Render
    cam.render(&world)?;

    Ok(())
}
