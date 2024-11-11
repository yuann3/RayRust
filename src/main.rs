use std::io::{self, Write};
use raytracer::vec3::{Color, Vec3};
use raytracer::color::write_color;

fn main() -> io::Result<()> {
    // Image
    let image_width = 256;
    let image_height = 256;

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
