use std::io::{self, Write};
use raytracer::vec3::{Vec3, Color, Point3};
use raytracer::color::write_color;

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height 
                       * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate vectors across viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_width, 0.0);

    // Calculate pixel delta vectors
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate upper left pixel location
    let viewport_upper_left = camera_center 
                            - Vec3::new(0.0, 0.0, focal_length) 
                            - viewport_u/2.0
                            - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left 
                    + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut stdout = io::stdout();
    let stderr = io::stderr();

    for j in 0..image_height {
        // Progress indicator
        write!(stderr.lock(), "\rScanlines remaining: {} ", image_height - j)?;
        stderr.lock().flush()?;

        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0
            );
            write_color(&mut stdout, pixel_color)?;
        }
    }

    eprintln!("\rDone.                 ");
    Ok(())
}
