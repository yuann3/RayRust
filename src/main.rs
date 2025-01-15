use raytracer::ray::Ray;
use raytracer::sphere::hit_sphere;
use raytracer::vec3::{Color, Point3, Vec3};
use std::io::{self, Write};

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let intersection = ray.at(t);
        let normal = (intersection - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate vectors across viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate pixel delta vectors
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate upper left pixel location
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut stdout = io::stdout();
    let stderr = io::stderr();

    for j in 0..image_height {
        // Progress indicator
        write!(
            stderr.lock(),
            "\rScanlines remaining: {} ",
            image_height - j
        )?;
        stderr.lock().flush()?;

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            // Get the color for this ray
            let pixel_color = ray_color(&ray);
            write_color(&mut stdout, pixel_color)?;
        }
    }

    eprintln!("\rDone.                 ");
    Ok(())
}
