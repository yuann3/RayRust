use std::io::{self, Write};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // PPM header
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        // Progress indicator
        eprint!("\r Scanliens: {}", image_height - j);
        io::stderr().flush().unwrap(); // make sure immediate output

        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            // Convert to integer values;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\rDone.                 ");
}
