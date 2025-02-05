use crate::bababoi::random_double;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let samples_per_pixel = 100;
        let max_depth = 50;
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        let mut rec = crate::hittable::HitRecord::new(Point3::zero(), Vec3::zero(), 0.0);

        if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return self.ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5;
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let mut stdout = io::stdout();
        let stderr = io::stderr();

        for j in 0..self.image_height {
            write!(
                stderr.lock(),
                "\rScanlines remaining: {} ",
                self.image_height - j
            )?;
            stderr.lock().flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();

                for _ in 0..self.samples_per_pixel {
                    let pixel_sample = self.pixel00_loc
                        + (self.pixel_delta_u * (i as f64 + random_double()))
                        + (self.pixel_delta_v * (j as f64 + random_double()));

                    let ray_direction = pixel_sample - self.center;
                    let ray = Ray::new(self.center, ray_direction);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }

                write_color(&mut stdout, pixel_color, self.samples_per_pixel)?;
            }
        }

        eprintln!("\rDone.                 ");
        Ok(())
    }
}
