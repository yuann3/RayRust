use crate::bababoi::random_double;
use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Lambertian;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            image_height: 0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.lookfrom;

        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        let mut rec = HitRecord::new(
            Point3::zero(),
            Vec3::zero(),
            Lambertian::new(Color::zero()),
            0.0,
        );

        if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(ray, &rec) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::zero();
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
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

                write_color(&mut io::stdout(), pixel_color, self.samples_per_pixel)?;
            }
        }

        eprintln!("\nDone.");
        Ok(())
    }
}
